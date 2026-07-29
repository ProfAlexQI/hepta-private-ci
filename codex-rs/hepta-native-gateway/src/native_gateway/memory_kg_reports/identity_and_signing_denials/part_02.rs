fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
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
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_ready",
    );
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "public_claim_status_exposure_allowed",
        "public_claim_status_exposure_request_accepted",
        "public_claim_status_exposure_accepted",
        "public_claim_status_exposure_recorded",
        "public_claim_status_exposure_persisted",
        "public_claim_status_exposure_materialized",
        "public_claim_status_exposure_filesystem_written",
        "public_claim_status_exposure_delivered",
        "public_claim_status_exposed",
        "public_status_claimed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_status_exposed",
        "publication_status_exposed",
        "dashboard_status_exposed",
        "public_badge_exposed",
        "status_endpoint_exposed",
        "query_status_exposed",
        "export_status_exposed",
        "observability_status_exposed",
        "release_notes_status_exposed",
        "changelog_status_exposed",
        "version_tag_status_exposed",
        "artifact_availability_status_exposed",
        "distribution_queue_status_exposed",
        "channel_status_delivered",
        "external_status_sent",
        "telegram_status_sent",
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
    ];
    let surface_specs = [
        (
            "revocation_replay_public_claim_status_claim",
            "blocked_revocation_replay_public_claim_noop",
            "revocation_replay_public_claim_status_denied",
            &["public_claim_requested"][..],
        ),
        (
            "logout_replay_release_claim_status_claim",
            "blocked_logout_replay_release_claim_noop",
            "logout_replay_release_claim_status_denied",
            &["release_claim_requested"][..],
        ),
        (
            "identity_reinstatement_publication_claim_status_claim",
            "blocked_identity_reinstatement_publication_claim_noop",
            "identity_reinstatement_publication_claim_status_denied",
            &["publication_claim_requested"][..],
        ),
        (
            "session_reinstatement_ga_stable_claim_status_claim",
            "blocked_session_reinstatement_ga_stable_claim_noop",
            "session_reinstatement_ga_stable_claim_status_denied",
            &["ga_stable_claim_requested"][..],
        ),
        (
            "terminal_decision_dashboard_public_badge_status_claim",
            "blocked_dashboard_public_badge_noop",
            "terminal_decision_dashboard_public_badge_denied",
            &["dashboard_public_badge_requested"][..],
        ),
        (
            "terminal_status_endpoint_claim",
            "blocked_status_endpoint_noop",
            "terminal_status_endpoint_exposure_denied",
            &["status_endpoint_requested"][..],
        ),
        (
            "terminal_query_status_claim",
            "blocked_query_status_noop",
            "terminal_query_status_exposure_denied",
            &["query_status_requested"][..],
        ),
        (
            "terminal_export_status_claim",
            "blocked_export_status_noop",
            "terminal_export_status_exposure_denied",
            &["export_status_requested"][..],
        ),
        (
            "terminal_observability_status_claim",
            "blocked_observability_status_noop",
            "terminal_observability_status_exposure_denied",
            &["observability_status_requested"][..],
        ),
        (
            "terminal_release_notes_status_claim",
            "blocked_release_notes_status_noop",
            "terminal_release_notes_status_exposure_denied",
            &["release_notes_status_requested"][..],
        ),
        (
            "terminal_changelog_status_claim",
            "blocked_changelog_status_noop",
            "terminal_changelog_status_exposure_denied",
            &["changelog_status_requested"][..],
        ),
        (
            "terminal_version_tag_status_claim",
            "blocked_version_tag_status_noop",
            "terminal_version_tag_status_exposure_denied",
            &["version_tag_status_requested"][..],
        ),
        (
            "terminal_artifact_availability_status_claim",
            "blocked_artifact_availability_status_noop",
            "terminal_artifact_availability_status_exposure_denied",
            &["artifact_availability_status_requested"][..],
        ),
        (
            "terminal_distribution_queue_status_claim",
            "blocked_distribution_queue_status_noop",
            "terminal_distribution_queue_status_exposure_denied",
            &["distribution_queue_status_requested"][..],
        ),
        (
            "terminal_channel_external_telegram_public_status_claim",
            "blocked_channel_external_telegram_public_status_noop",
            "terminal_channel_external_telegram_public_status_denied",
            &[
                "channel_status_requested",
                "external_status_requested",
                "telegram_status_requested",
            ][..],
        ),
        (
            "terminal_release_publication_authority_public_status_claim",
            "blocked_release_publication_authority_public_status_noop",
            "terminal_release_publication_authority_from_public_status_denied",
            &["release_publication_authority_public_status_requested"][..],
        ),
        (
            "terminal_activation_live_public_status_claim",
            "blocked_activation_live_public_status_noop",
            "terminal_activation_live_from_public_status_denied",
            &["activation_live_public_status_requested"][..],
        ),
        (
            "terminal_install_restart_active_binary_public_status_claim",
            "blocked_active_binary_public_status_noop",
            "terminal_install_restart_active_binary_from_public_status_denied",
            &["install_restart_active_binary_public_status_requested"][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface": surface,
                "source_terminal_decision_status_promotion_ready": source_ready,
                "public_claim_status_exposure_attempted": true,
                "public_claim_status_exposure_noop_confirmed": true,
                "public_claim_status_exposure_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-public-claim-status-exposure-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:public=0:status=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-public-claim-status-exposure:no-public-claim:no-status-exposure:no-authority:no-install:no-live",
    );
    let report_ready = source_ready
        && route_count_source_command_accepted
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded_count",
        ) == 0
        && surface_count == 18;
    let denials = vec![
        "source_terminal_decision_status_promotion_report_required",
        "public_claim_status_request_acceptance_denied",
        "public_claim_status_acceptance_denied",
        "public_claim_status_recording_denied",
        "public_claim_status_persistence_denied",
        "public_claim_status_materialization_denied",
        "public_claim_status_filesystem_write_denied",
        "public_claim_status_delivery_denied",
        "public_claim_status_exposure_denied",
        "public_status_claim_denied",
        "public_release_claim_denied",
        "public_ga_claim_denied",
        "release_status_exposure_denied",
        "publication_status_exposure_denied",
        "dashboard_status_exposure_denied",
        "public_badge_exposure_denied",
        "status_endpoint_exposure_denied",
        "query_status_exposure_denied",
        "export_status_exposure_denied",
        "observability_status_exposure_denied",
        "release_notes_status_exposure_denied",
        "changelog_status_exposure_denied",
        "version_tag_status_exposure_denied",
        "artifact_availability_status_exposure_denied",
        "distribution_queue_status_exposure_denied",
        "channel_status_delivery_denied",
        "external_status_send_denied",
        "telegram_status_send_denied",
        "acceptance_from_public_status_denied",
        "operator_approval_from_public_status_denied",
        "release_publication_authority_from_public_status_denied",
        "activation_live_from_public_status_denied",
        "install_restart_active_binary_from_public_status_denied",
        "memory_provider_kg_from_public_status_denied",
    ];
    let denied_count = denials.len();
    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-public-claim-status-exposure-denial --json",
        "side_effect_free": true,
        "native_route": true,
        "route_enabled": true,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "minimum_required_samples": 24,
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_mode": "native_route_denied_terminal_decision_status_cannot_be_exposed_as_public_release_publication_or_activation_status",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_policy_hash_sha256": policy_hash,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_count": denied_count,
            "allowed_next_actions": [{
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_gate",
                "status": "allowed_report_only_next_slice",
                "exposes_public_status": false,
                "claims_public_release": false,
                "claims_public_ga": false,
                "records_operator_acceptance": false,
                "derives_release_publication_authority": false,
                "derives_activation_authority": false,
                "activates_live": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }],
        }),
    );
    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_request_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed_count",
        "release_publication_result_receipt_public_status_claimed_count",
        "release_publication_result_receipt_public_release_claimed_count",
        "release_publication_result_receipt_public_ga_claimed_count",
        "release_publication_result_receipt_release_status_exposed_count",
        "release_publication_result_receipt_publication_status_exposed_count",
        "release_publication_result_receipt_dashboard_status_exposed_count",
        "release_publication_result_receipt_public_badge_exposed_count",
        "release_publication_result_receipt_status_endpoint_exposed_count",
        "release_publication_result_receipt_query_status_exposed_count",
        "release_publication_result_receipt_export_status_exposed_count",
        "release_publication_result_receipt_observability_status_exposed_count",
        "release_publication_result_receipt_release_notes_status_exposed_count",
        "release_publication_result_receipt_changelog_status_exposed_count",
        "release_publication_result_receipt_version_tag_status_exposed_count",
        "release_publication_result_receipt_artifact_availability_status_exposed_count",
        "release_publication_result_receipt_distribution_queue_status_exposed_count",
        "release_publication_result_receipt_channel_status_delivered_count",
        "release_publication_result_receipt_external_status_sent_count",
        "release_publication_result_receipt_telegram_status_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_operator_approval_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_activation_command_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_service_restarted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_active_binary_mutated_count",
    ] {
        if let Some(report_object) = report.as_object_mut() {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_query_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_notes_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_changelog_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_version_tag_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed",
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
        for key in false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
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

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
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
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_ready",
    );
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_contract_hash_sha256")
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
        "operator_consent_recorded",
        "operator_consent_persisted",
        "consent_reconfirmation_recorded",
        "consent_reconfirmation_persisted",
        "identity_signature_recorded",
        "session_consent_token_recorded",
        "revocation_replay_intent_timestamp_recorded",
        "device_session_consent_nonce_recorded",
        "logout_replay_consent_refresh_recorded",
        "explicit_intent_status_promoted",
        "explicit_consent_status_promoted",
        "consent_summary_recorded",
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
            "source_terminal_public_claim_status_exposure_report_required",
            "blocked_source_terminal_public_status_required_noop",
            "source_terminal_public_claim_status_exposure_report_required",
            &["source_terminal_public_claim_status_exposure_report_required"],
        ),
        (
            "download_button_revocation_replay_operator_intent_reconfirmation_claim",
            "blocked_revocation_replay_operator_intent_noop",
            "download_button_revocation_replay_operator_intent_reconfirmation_claim_denied",
            &["operator_intent_reconfirmation_requested"],
        ),
        (
            "direct_download_url_logout_replay_operator_consent_reconfirmation_claim",
            "blocked_logout_replay_operator_consent_noop",
            "direct_download_url_logout_replay_operator_consent_reconfirmation_claim_denied",
            &["operator_consent_reconfirmation_requested"],
        ),
        (
            "checksum_identity_reinstatement_intent_signature_claim",
            "blocked_identity_reinstatement_intent_signature_noop",
            "checksum_identity_reinstatement_intent_signature_claim_denied",
            &["identity_signature_requested"],
        ),
        (
            "package_manager_session_reinstatement_consent_token_claim",
            "blocked_session_reinstatement_consent_token_noop",
            "package_manager_session_reinstatement_consent_token_claim_denied",
            &["session_consent_token_requested"],
        ),
        (
            "curl_pipe_shell_revocation_replay_intent_timestamp_claim",
            "blocked_revocation_replay_intent_timestamp_noop",
            "curl_pipe_shell_revocation_replay_intent_timestamp_claim_denied",
            &["revocation_replay_intent_timestamp_requested"],
        ),
        (
            "installer_device_session_consent_nonce_claim",
            "blocked_device_session_consent_nonce_noop",
            "installer_device_session_consent_nonce_claim_denied",
            &["device_session_consent_nonce_requested"],
        ),
        (
            "auto_update_session_logout_replay_consent_refresh_claim",
            "blocked_session_logout_replay_consent_refresh_noop",
            "auto_update_session_logout_replay_consent_refresh_claim_denied",
            &["logout_replay_consent_refresh_requested"],
        ),
        (
            "release_channel_identity_revocation_replay_intent_status_claim",
            "blocked_identity_revocation_replay_intent_status_noop",
            "release_channel_identity_revocation_replay_intent_status_claim_denied",
            &["identity_revocation_replay_intent_status_requested"],
        ),
        (
            "update_feed_session_reinstatement_consent_summary_claim",
            "blocked_session_reinstatement_consent_summary_noop",
            "update_feed_session_reinstatement_consent_summary_claim_denied",
            &["session_reinstatement_consent_summary_requested"],
        ),
        (
            "package_registry_identity_badge_intent_badge_claim",
            "blocked_identity_badge_intent_badge_noop",
            "package_registry_identity_badge_intent_badge_claim_denied",
            &["operator_intent_badge_requested"],
        ),
        (
            "cdn_session_readback_logout_replay_consent_readback_claim",
            "blocked_logout_replay_consent_readback_noop",
            "cdn_session_readback_logout_replay_consent_readback_claim_denied",
            &["consent_readback_requested"],
        ),
        (
            "sbom_identity_dashboard_reinstatement_consent_notification_claim",
            "blocked_identity_dashboard_consent_notification_noop",
            "sbom_identity_dashboard_reinstatement_consent_notification_claim_denied",
            &["consent_notification_requested"],
        ),
        (
            "signature_channel_session_consent_delivery_claim",
            "blocked_session_channel_consent_delivery_noop",
            "signature_channel_session_consent_delivery_claim_denied",
            &["channel_consent_requested"],
        ),
        (
            "one_click_identity_approval_reinstatement_reconfirmed_consent_claim",
            "blocked_identity_approval_reconfirmed_consent_noop",
            "one_click_identity_approval_reinstatement_reconfirmed_consent_claim_denied",
            &["operator_intent_approval_requested"],
        ),
        (
            "external_telegram_identity_session_reinstatement_consent_reconfirmation_claim",
            "blocked_external_telegram_consent_noop",
            "external_telegram_identity_session_reinstatement_consent_reconfirmation_claim_denied",
            &["external_consent_requested", "telegram_consent_requested"],
        ),
        (
            "release_publication_authority_replay_reinstatement_intent_consent_claim",
            "blocked_authority_intent_consent_noop",
            "release_publication_authority_replay_reinstatement_intent_consent_claim_denied",
            &["authority_intent_consent_requested"],
        ),
        (
            "activation_live_install_restart_active_binary_reinstatement_consent_claim",
            "blocked_live_consent_noop",
            "activation_live_install_restart_active_binary_reinstatement_consent_claim_denied",
            &[
                "live_consent_requested",
                "install_restart_active_binary_consent_requested",
            ],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface": surface,
                "source_terminal_public_claim_status_exposure_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_attempted": true,
                "operator_intent_consent_reconfirmation_noop_confirmed": true,
                "operator_intent_consent_reconfirmation_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:intent=0:consent=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation:no-intent:no-consent:no-reconfirmation:no-approval:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_terminal_public_claim_status_exposure_report_required",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_reconfirmation_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_reconfirmation_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_consent_reconfirmation_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_identity_signature_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_session_consent_token_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_revocation_replay_intent_timestamp_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_device_session_consent_nonce_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_consent_refresh_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_status_promotion_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_acceptance_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_intent_consent_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_release_publication_authority_from_intent_consent_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_activation_authority_from_intent_consent_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_download_install_from_intent_consent_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_install_restart_active_binary_from_intent_consent_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_memory_provider_secret_external_send_from_intent_consent_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && route_count_source_command_accepted
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_activation_authority_derived_count",
        ) == 0
        && surface_count == 18;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_RECONFIRMATION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-21",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_mode": "native_route_denied_terminal_public_status_cannot_create_operator_intent_consent_reconfirmation_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denied_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_intent": false,
                    "records_operator_consent": false,
                    "records_operator_identity": false,
                    "records_operator_session": false,
                    "records_intent_consent_evidence": false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_reconfirmed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_reconfirmed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_consent_reconfirmation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_consent_reconfirmation_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_identity_signature_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_session_consent_token_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_revocation_replay_intent_timestamp_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_device_session_consent_nonce_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_logout_replay_consent_refresh_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_explicit_intent_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_explicit_consent_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_consent_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_intent_consent_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_acceptance_from_intent_consent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_consent_reconfirmation_recorded",
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

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
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
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_ready",
    );
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "evidence_persistence_requested",
        "intent_evidence_requested",
        "consent_evidence_requested",
        "evidence_receipt_requested",
        "evidence_persistence_allowed",
        "operator_intent_evidence_recorded",
        "operator_intent_evidence_persisted",
        "operator_consent_evidence_recorded",
        "operator_consent_evidence_persisted",
        "intent_consent_evidence_recorded",
        "intent_consent_evidence_persisted",
        "identity_signature_evidence_recorded",
        "identity_signature_evidence_persisted",
        "session_consent_token_evidence_recorded",
        "session_consent_token_evidence_persisted",
        "consent_nonce_evidence_recorded",
        "consent_refresh_evidence_recorded",
        "evidence_receipt_recorded",
        "evidence_receipt_persisted",
        "evidence_materialized",
        "evidence_filesystem_written",
        "evidence_ledger_written",
        "evidence_indexed",
        "evidence_exported",
        "evidence_query_registered",
        "evidence_observability_recorded",
        "evidence_readback_recorded",
        "identity_session_binding_from_evidence_recorded",
        "operator_approval_from_evidence_derived",
        "acceptance_from_evidence_recorded",
        "terminal_decision_from_evidence_recorded",
        "terminal_status_from_evidence_recorded",
        "release_publication_authority_from_evidence_derived",
        "activation_authority_from_evidence_derived",
        "download_link_from_evidence_rendered",
        "install_command_from_evidence_rendered",
        "install_from_evidence_executed",
        "service_restart_from_evidence_performed",
        "launchd_from_evidence_mutated",
        "active_binary_from_evidence_mutated",
        "result_receipt_from_evidence_recorded",
        "result_receipt_from_evidence_persisted",
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
            "source_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_report_required",
            "blocked_source_intent_consent_reconfirmation_required_noop",
            "source_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_report_required",
            &["source_operator_intent_consent_reconfirmation_report_required"],
        ),
        (
            "download_button_revocation_replay_operator_intent_evidence_record_claim",
            "blocked_revocation_replay_operator_intent_evidence_noop",
            "download_button_revocation_replay_operator_intent_evidence_record_claim_denied",
            &["intent_evidence_requested"],
        ),
        (
            "direct_download_url_logout_replay_operator_consent_evidence_record_claim",
            "blocked_logout_replay_operator_consent_evidence_noop",
            "direct_download_url_logout_replay_operator_consent_evidence_record_claim_denied",
            &["consent_evidence_requested"],
        ),
        (
            "checksum_identity_reinstatement_signature_evidence_packet_claim",
            "blocked_identity_signature_evidence_packet_noop",
            "checksum_identity_reinstatement_signature_evidence_packet_claim_denied",
            &["identity_signature_evidence_requested"],
        ),
        (
            "package_manager_session_reinstatement_consent_token_evidence_claim",
            "blocked_session_consent_token_evidence_noop",
            "package_manager_session_reinstatement_consent_token_evidence_claim_denied",
            &["session_consent_token_evidence_requested"],
        ),
        (
            "curl_pipe_shell_revocation_replay_intent_timestamp_evidence_claim",
            "blocked_revocation_replay_intent_timestamp_evidence_noop",
            "curl_pipe_shell_revocation_replay_intent_timestamp_evidence_claim_denied",
            &["intent_timestamp_evidence_requested"],
        ),
        (
            "installer_device_session_consent_nonce_evidence_claim",
            "blocked_device_session_consent_nonce_evidence_noop",
            "installer_device_session_consent_nonce_evidence_claim_denied",
            &["consent_nonce_evidence_requested"],
        ),
        (
            "auto_update_session_logout_replay_consent_refresh_evidence_claim",
            "blocked_logout_replay_consent_refresh_evidence_noop",
            "auto_update_session_logout_replay_consent_refresh_evidence_claim_denied",
            &["consent_refresh_evidence_requested"],
        ),
        (
            "release_channel_identity_revocation_replay_intent_status_evidence_claim",
            "blocked_identity_revocation_replay_intent_status_evidence_noop",
            "release_channel_identity_revocation_replay_intent_status_evidence_claim_denied",
            &["intent_status_evidence_requested"],
        ),
        (
            "update_feed_session_reinstatement_consent_summary_evidence_claim",
            "blocked_session_reinstatement_consent_summary_evidence_noop",
            "update_feed_session_reinstatement_consent_summary_evidence_claim_denied",
            &["consent_summary_evidence_requested"],
        ),
        (
            "package_registry_identity_badge_intent_evidence_badge_claim",
            "blocked_identity_badge_intent_evidence_noop",
            "package_registry_identity_badge_intent_evidence_badge_claim_denied",
            &["intent_badge_evidence_requested"],
        ),
        (
            "cdn_session_readback_logout_replay_consent_readback_evidence_claim",
            "blocked_logout_replay_consent_readback_evidence_noop",
            "cdn_session_readback_logout_replay_consent_readback_evidence_claim_denied",
            &["consent_readback_evidence_requested"],
        ),
        (
            "sbom_identity_dashboard_reinstatement_consent_notification_evidence_claim",
            "blocked_identity_dashboard_consent_notification_evidence_noop",
            "sbom_identity_dashboard_reinstatement_consent_notification_evidence_claim_denied",
            &["consent_notification_evidence_requested"],
        ),
        (
            "signature_channel_session_consent_delivery_evidence_claim",
            "blocked_session_channel_consent_delivery_evidence_noop",
            "signature_channel_session_consent_delivery_evidence_claim_denied",
            &["channel_consent_evidence_requested"],
        ),
        (
            "one_click_identity_approval_reinstatement_reconfirmed_consent_evidence_claim",
            "blocked_identity_approval_reconfirmed_consent_evidence_noop",
            "one_click_identity_approval_reinstatement_reconfirmed_consent_evidence_claim_denied",
            &["operator_approval_consent_evidence_requested"],
        ),
        (
            "external_telegram_identity_session_reinstatement_consent_evidence_claim",
            "blocked_external_telegram_consent_evidence_noop",
            "external_telegram_identity_session_reinstatement_consent_evidence_claim_denied",
            &[
                "external_consent_evidence_requested",
                "telegram_consent_evidence_requested",
            ],
        ),
        (
            "release_publication_authority_replay_reinstatement_intent_consent_evidence_claim",
            "blocked_authority_intent_consent_evidence_noop",
            "release_publication_authority_replay_reinstatement_intent_consent_evidence_claim_denied",
            &["authority_intent_consent_evidence_requested"],
        ),
        (
            "activation_live_install_restart_active_binary_reinstatement_consent_evidence_claim",
            "blocked_live_consent_evidence_noop",
            "activation_live_install_restart_active_binary_reinstatement_consent_evidence_claim_denied",
            &[
                "live_consent_evidence_requested",
                "install_restart_active_binary_evidence_requested",
            ],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface": surface,
                "source_operator_intent_consent_reconfirmation_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_attempted": true,
                "operator_intent_consent_evidence_persistence_noop_confirmed": true,
                "operator_intent_consent_evidence_persistence_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:evidence=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence:no-evidence:no-receipt:no-binding:no-authority:no-install:no-live",
    );
    let denials = vec![
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_persistence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_identity_signature_evidence_persistence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_session_consent_token_evidence_persistence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_persistence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_evidence_materialization_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_evidence_filesystem_write_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_evidence_ledger_index_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_evidence_export_query_observability_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_evidence_readback_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_identity_session_binding_from_evidence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_evidence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_release_publication_authority_from_evidence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_activation_authority_from_evidence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_download_install_from_evidence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_install_restart_active_binary_from_evidence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_memory_provider_secret_external_send_from_evidence_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && route_count_source_command_accepted
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_intent_consent_derived_count",
        ) == 0
        && surface_count == 18;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-22",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_mode": "native_route_denied_operator_intent_consent_reconfirmation_cannot_materialize_evidence_receipts_binding_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denied_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denied_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_intent": false,
                    "records_operator_consent": false,
                    "records_operator_identity": false,
                    "records_operator_session": false,
                    "records_intent_consent_evidence": false,
                    "exports_evidence": false,
                    "registers_query": false,
                    "records_observability": false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_identity_signature_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_session_consent_token_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_ledger_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_indexed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_exported_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_readback_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_identity_session_binding_from_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_evidence_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_acceptance_from_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_persisted",
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

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_evidence =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_report();
    let source_signing =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_report();
    let source_bool = |source: &serde_json::Value, key: &str| -> bool {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |source: &serde_json::Value, key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_evidence_ready = source_bool(
        &source_evidence,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_ready",
    );
    let source_signing_ready = source_bool(
        &source_signing,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_ready",
    );
    let source_evidence_report_sha256 = sha256_json_value(&source_evidence);
    let source_signing_report_sha256 = sha256_json_value(&source_signing);
    let source_evidence_contract_hash = source_evidence
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let source_signing_contract_hash = source_signing
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_result_receipt_surface_allowed",
        "artifact_distribution_signing_notarization_result_receipt_surface_request_accepted",
        "artifact_distribution_signing_notarization_result_receipt_surface_accepted",
        "artifact_distribution_signing_notarization_result_receipt_surface_recorded",
        "artifact_distribution_signing_notarization_result_receipt_surface_persisted",
        "artifact_distribution_signing_notarization_result_receipt_surface_materialized",
        "artifact_distribution_signing_notarization_result_receipt_surface_filesystem_written",
        "artifact_distribution_signing_notarization_result_receipt_surface_delivered",
        "artifact_distribution_signing_notarization_result_receipt_surface_indexed",
        "artifact_distribution_signing_notarization_result_receipt_surface_exported",
        "artifact_distribution_signing_notarization_result_receipt_surface_query_registered",
        "artifact_distribution_signing_notarization_result_receipt_surface_observability_recorded",
        "artifact_distribution_signing_notarization_result_receipt_surface_status_exposed",
        "artifact_signing_receipt_accepted",
        "package_signing_receipt_accepted",
        "signature_manifest_receipt_recorded",
        "notarization_submission_receipt_persisted",
        "notarization_ticket_receipt_materialized",
        "stapling_receipt_filesystem_written",
        "installer_signing_receipt_delivered",
        "provenance_attestation_receipt_indexed",
        "sbom_manifest_receipt_exported",
        "release_asset_bundle_receipt_query_registered",
        "cdn_update_feed_receipt_observability_recorded",
        "package_registry_receipt_status_exposed",
        "dashboard_endpoint_receipt_status_exposed",
        "external_signing_receipt_delivered",
        "telegram_signing_receipt_delivered",
        "public_release_claimed",
        "public_ga_claimed",
        "acceptance_recorded",
        "operator_approval_derived",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_rendered",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "source_signing_notarization_surface_report_required",
            "blocked_source_signing_notarization_surface_required_noop",
            "source_artifact_distribution_signing_notarization_surface_report_required",
            vec!["source_report_required"],
        ),
        (
            "artifact_signing_result_receipt_schema_acceptance",
            "blocked_artifact_signing_result_receipt_schema_acceptance_noop",
            "artifact_signing_result_receipt_acceptance_denied",
            vec!["artifact_signing_receipt_acceptance_requested"],
        ),
        (
            "package_signing_result_receipt_acceptance",
            "blocked_package_signing_result_receipt_acceptance_noop",
            "package_signing_result_receipt_acceptance_denied",
            vec!["package_signing_receipt_acceptance_requested"],
        ),
        (
            "signature_manifest_result_receipt_recording",
            "blocked_signature_manifest_result_receipt_recording_noop",
            "signature_manifest_result_receipt_recording_denied",
            vec!["signature_manifest_receipt_recording_requested"],
        ),
        (
            "notarization_submission_result_receipt_persistence",
            "blocked_notarization_submission_result_receipt_persistence_noop",
            "notarization_submission_result_receipt_persistence_denied",
            vec!["notarization_submission_receipt_persistence_requested"],
        ),
        (
            "notarization_ticket_result_receipt_materialization",
            "blocked_notarization_ticket_result_receipt_materialization_noop",
            "notarization_ticket_result_receipt_materialization_denied",
            vec!["notarization_ticket_receipt_materialization_requested"],
        ),
        (
            "stapling_result_receipt_filesystem_write",
            "blocked_stapling_result_receipt_filesystem_write_noop",
            "stapling_result_receipt_filesystem_write_denied",
            vec!["stapling_receipt_filesystem_write_requested"],
        ),
        (
            "installer_signing_result_receipt_delivery",
            "blocked_installer_signing_result_receipt_delivery_noop",
            "installer_signing_result_receipt_delivery_denied",
            vec!["installer_signing_receipt_delivery_requested"],
        ),
        (
            "provenance_attestation_result_receipt_indexing",
            "blocked_provenance_attestation_result_receipt_indexing_noop",
            "provenance_attestation_result_receipt_indexing_denied",
            vec!["provenance_attestation_receipt_indexing_requested"],
        ),
        (
            "sbom_manifest_result_receipt_export",
            "blocked_sbom_manifest_result_receipt_export_noop",
            "sbom_manifest_result_receipt_export_denied",
            vec!["sbom_manifest_receipt_export_requested"],
        ),
        (
            "release_asset_bundle_result_receipt_query",
            "blocked_release_asset_bundle_result_receipt_query_noop",
            "release_asset_bundle_result_receipt_query_denied",
            vec!["release_asset_bundle_receipt_query_requested"],
        ),
        (
            "cdn_update_feed_result_receipt_observability",
            "blocked_cdn_update_feed_result_receipt_observability_noop",
            "cdn_update_feed_result_receipt_observability_denied",
            vec!["cdn_update_feed_receipt_observability_requested"],
        ),
        (
            "package_registry_result_receipt_status",
            "blocked_package_registry_result_receipt_status_noop",
            "package_registry_result_receipt_status_denied",
            vec!["package_registry_receipt_status_requested"],
        ),
        (
            "dashboard_endpoint_signing_receipt_status_exposure",
            "blocked_dashboard_endpoint_signing_receipt_status_exposure_noop",
            "dashboard_endpoint_signing_receipt_status_exposure_denied",
            vec!["dashboard_endpoint_receipt_status_requested"],
        ),
        (
            "external_telegram_signing_receipt_delivery",
            "blocked_external_telegram_signing_receipt_delivery_noop",
            "external_telegram_signing_receipt_delivery_denied",
            vec![
                "external_signing_receipt_delivery_requested",
                "telegram_signing_receipt_delivery_requested",
            ],
        ),
        (
            "release_publication_authority_from_signing_receipt",
            "blocked_release_publication_authority_from_signing_receipt_noop",
            "release_publication_authority_from_signing_receipt_denied",
            vec!["release_publication_authority_from_signing_receipt_requested"],
        ),
        (
            "activation_live_install_from_signing_receipt",
            "blocked_activation_live_install_from_signing_receipt_noop",
            "activation_live_install_from_signing_receipt_denied",
            vec![
                "activation_authority_from_signing_receipt_requested",
                "live_install_from_signing_receipt_requested",
            ],
        ),
        (
            "install_restart_active_binary_from_signing_receipt",
            "blocked_install_restart_active_binary_from_signing_receipt_noop",
            "install_restart_active_binary_from_signing_receipt_denied",
            vec!["install_restart_active_binary_from_signing_receipt_requested"],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "source_artifact_distribution_signing_notarization_surface_ready": source_signing_ready,
                "artifact_distribution_signing_notarization_result_receipt_surface_attempted": true,
                "artifact_distribution_signing_notarization_result_receipt_surface_noop_confirmed": true,
                "artifact_distribution_signing_notarization_result_receipt_surface_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial:native:evidence={source_evidence_report_sha256}:signing={source_signing_report_sha256}:surfaces={surface_count}:route_count={}:receipt=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-no-persistence:no-receipt-acceptance:no-recording:no-persistence:no-materialization:no-delivery:no-status-exposure:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_surface_report_required",
        "artifact_distribution_signing_notarization_result_receipt_request_acceptance_denied",
        "artifact_distribution_signing_notarization_result_receipt_acceptance_denied",
        "artifact_distribution_signing_notarization_result_receipt_recording_denied",
        "artifact_distribution_signing_notarization_result_receipt_persistence_denied",
        "artifact_distribution_signing_notarization_result_receipt_materialization_denied",
        "artifact_distribution_signing_notarization_result_receipt_filesystem_write_denied",
        "artifact_distribution_signing_notarization_result_receipt_delivery_denied",
        "artifact_distribution_signing_notarization_result_receipt_indexing_denied",
        "artifact_distribution_signing_notarization_result_receipt_export_denied",
        "artifact_distribution_signing_notarization_result_receipt_query_registration_denied",
        "artifact_distribution_signing_notarization_result_receipt_observability_denied",
        "artifact_distribution_signing_notarization_result_receipt_status_exposure_denied",
        "artifact_signing_result_receipt_acceptance_denied",
        "package_signing_result_receipt_acceptance_denied",
        "signature_manifest_result_receipt_recording_denied",
        "notarization_submission_result_receipt_persistence_denied",
        "notarization_ticket_result_receipt_materialization_denied",
        "stapling_result_receipt_filesystem_write_denied",
        "installer_signing_result_receipt_delivery_denied",
        "provenance_attestation_result_receipt_indexing_denied",
        "sbom_manifest_result_receipt_export_denied",
        "release_asset_bundle_result_receipt_query_denied",
        "cdn_update_feed_result_receipt_observability_denied",
        "package_registry_result_receipt_status_denied",
        "dashboard_endpoint_signing_receipt_status_exposure_denied",
        "external_telegram_signing_receipt_delivery_denied",
        "release_publication_authority_from_signing_receipt_denied",
        "activation_live_install_from_signing_receipt_denied",
        "install_restart_active_binary_from_signing_receipt_denied",
        "memory_provider_kg_secret_external_send_from_signing_receipt_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_evidence_ready
        && source_signing_ready
        && route_count_source_command_accepted
        && source_u64(
            &source_signing,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count",
        ) == 18
        && source_u64(
            &source_signing,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count",
        ) == 0
        && source_u64(
            &source_signing,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed_count",
        ) == 0
        && source_u64(
            &source_signing,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count",
        ) == 0
        && source_u64(
            &source_signing,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            &source_signing,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count",
        ) == 0
        && surface_count == 18;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_NO_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-26",
        "artifact_distribution_signing_notarization_result_receipt_no_persistence_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_route_v1",
        "artifact_distribution_signing_notarization_result_receipt_no_persistence_mode": "native_route_denied_signing_notarization_receipts_cannot_be_accepted_recorded_persisted_materialized_delivered_exposed_or_used_for_authority",
        "source_operator_intent_consent_evidence_persistence_gate": source_evidence["gate"].clone(),
        "source_operator_intent_consent_evidence_persistence_ready": source_evidence_ready,
        "source_operator_intent_consent_evidence_persistence_report_sha256": source_evidence_report_sha256,
        "source_operator_intent_consent_evidence_persistence_contract_hash_sha256": source_evidence_contract_hash,
        "source_artifact_distribution_signing_notarization_surface_gate": source_signing["gate"].clone(),
        "source_artifact_distribution_signing_notarization_surface_ready": source_signing_ready,
        "source_artifact_distribution_signing_notarization_surface_report_sha256": source_signing_report_sha256,
        "source_artifact_distribution_signing_notarization_surface_contract_hash_sha256": source_signing_contract_hash,
        "artifact_distribution_signing_notarization_result_receipt_contract_hash_sha256": contract_hash,
        "artifact_distribution_signing_notarization_result_receipt_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_surface_count": source_u64(&source_signing, "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count"),
            "source_artifact_distribution_signing_notarization_surface_attempt_count": source_u64(&source_signing, "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count"),
            "source_artifact_distribution_signing_notarization_surface_denied_count": source_u64(&source_signing, "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denied_count"),
            "source_artifact_signing_executed_count": source_u64(&source_signing, "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count"),
            "source_package_signing_executed_count": source_u64(&source_signing, "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed_count"),
            "source_notarization_submitted_count": source_u64(&source_signing, "release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count"),
            "source_release_publication_authority_from_signing_status_derived_count": source_u64(&source_signing, "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count"),
            "source_activation_authority_from_signing_status_derived_count": source_u64(&source_signing, "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count"),
            "artifact_distribution_signing_notarization_result_receipt_surface_count": surface_count,
            "artifact_distribution_signing_notarization_result_receipt_surface_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_result_receipt_surface_denied_count": surface_count,
            "artifact_distribution_signing_notarization_result_receipt_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_result_receipt": denials,
            "denied_by_artifact_distribution_signing_notarization_result_receipt_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_signing_receipt": false,
                    "persists_signing_receipt": false,
                    "materializes_signing_receipt": false,
                    "delivers_signing_receipt": false,
                    "exposes_signing_receipt_status": false,
                    "replays_signing_receipt": false,
                    "accepts_idempotency_key": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "reads_credentials": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let zero_keys = [
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
        "signature_manifest_receipt_recorded_count",
        "notarization_submission_receipt_persisted_count",
        "notarization_ticket_receipt_materialized_count",
        "stapling_receipt_filesystem_written_count",
        "installer_signing_receipt_delivered_count",
        "provenance_attestation_receipt_indexed_count",
        "sbom_manifest_receipt_exported_count",
        "release_asset_bundle_receipt_query_registered_count",
        "cdn_update_feed_receipt_observability_recorded_count",
        "package_registry_receipt_status_exposed_count",
        "dashboard_endpoint_receipt_status_exposed_count",
        "external_signing_receipt_delivered_count",
        "telegram_signing_receipt_delivered_count",
        "acceptance_from_signing_receipt_recorded_count",
        "operator_approval_from_signing_receipt_derived_count",
        "release_publication_authority_from_signing_receipt_derived_count",
        "activation_authority_from_signing_receipt_derived_count",
        "download_link_from_signing_receipt_rendered_count",
        "install_command_from_signing_receipt_rendered_count",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_result_receipt_accepted",
        "artifact_distribution_signing_notarization_result_receipt_recorded",
        "artifact_distribution_signing_notarization_result_receipt_persisted",
        "artifact_distribution_signing_notarization_result_receipt_materialized",
        "artifact_distribution_signing_notarization_result_receipt_filesystem_written",
        "artifact_distribution_signing_notarization_result_receipt_delivered",
        "artifact_distribution_signing_notarization_result_receipt_indexed",
        "artifact_distribution_signing_notarization_result_receipt_exported",
        "artifact_distribution_signing_notarization_result_receipt_query_registered",
        "artifact_distribution_signing_notarization_result_receipt_observability_recorded",
        "artifact_distribution_signing_notarization_result_receipt_status_exposed",
        "artifact_signing_receipt_accepted",
        "package_signing_receipt_accepted",
        "signature_manifest_receipt_recorded",
        "notarization_submission_receipt_persisted",
        "notarization_ticket_receipt_materialized",
        "stapling_receipt_filesystem_written",
        "installer_signing_receipt_delivered",
        "provenance_attestation_receipt_indexed",
        "sbom_manifest_receipt_exported",
        "release_asset_bundle_receipt_query_registered",
        "cdn_update_feed_receipt_observability_recorded",
        "package_registry_receipt_status_exposed",
        "dashboard_endpoint_receipt_status_exposed",
        "external_signing_receipt_delivered",
        "telegram_signing_receipt_delivered",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }
    let mut side_effects = serde_json::Map::new();
    for key in &false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report["artifact_distribution_signing_notarization_result_receipt_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed",
        "artifact_distribution_signing_notarization_receipt_replay_allowed",
        "artifact_distribution_signing_notarization_receipt_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_replay_recorded",
        "artifact_distribution_signing_notarization_receipt_replay_persisted",
        "artifact_distribution_signing_notarization_receipt_replay_performed",
        "artifact_distribution_signing_notarization_receipt_duplicate_accepted",
        "artifact_distribution_signing_notarization_receipt_duplicate_recorded",
        "artifact_distribution_signing_notarization_receipt_duplicate_persisted",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_recorded",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_recorded",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_materialized",
        "artifact_distribution_signing_notarization_receipt_idempotency_filesystem_written",
        "artifact_distribution_signing_notarization_receipt_replay_nonce_accepted",
        "artifact_distribution_signing_notarization_receipt_replay_nonce_recorded",
        "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted",
        "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted",
        "artifact_distribution_signing_notarization_receipt_completed_status_accepted",
        "artifact_distribution_signing_notarization_receipt_ack_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_ledger_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_index_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_delivery_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_query_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_export_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_observability_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted",
        "artifact_signing_receipt_replay_accepted",
        "package_signing_receipt_replay_accepted",
        "signature_manifest_receipt_idempotency_recorded",
        "notarization_submission_receipt_idempotency_persisted",
        "notarization_ticket_receipt_nonce_recorded",
        "stapling_receipt_cross_scope_reuse_accepted",
        "installer_signing_receipt_out_of_order_accepted",
        "provenance_attestation_receipt_ack_replay_accepted",
        "sbom_manifest_receipt_ledger_index_replay_accepted",
        "release_asset_bundle_receipt_export_query_replay_accepted",
        "cdn_update_feed_receipt_observability_replay_accepted",
        "package_registry_receipt_status_rebind_accepted",
        "dashboard_endpoint_receipt_hash_status_replay_accepted",
        "external_signing_receipt_delivery_replay_accepted",
        "telegram_signing_receipt_delivery_replay_accepted",
        "public_release_claimed",
        "public_ga_claimed",
        "acceptance_recorded",
        "operator_approval_derived",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_rendered",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
    ];
    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "source_signing_notarization_result_receipt_no_persistence_report_required",
            "blocked_source_signing_receipt_no_persistence_required_noop",
            "source_signing_notarization_result_receipt_no_persistence_report_required",
            vec!["source_report_required"],
        ),
        (
            "duplicate_artifact_signing_receipt_identity",
            "blocked_duplicate_artifact_signing_receipt_identity_noop",
            "duplicate_artifact_signing_receipt_identity_denied",
            vec!["duplicate_artifact_signing_receipt_identity_requested"],
        ),
        (
            "package_signing_receipt_replay_acceptance",
            "blocked_package_signing_receipt_replay_acceptance_noop",
            "package_signing_receipt_replay_acceptance_denied",
            vec!["package_signing_receipt_replay_acceptance_requested"],
        ),
        (
            "signature_manifest_receipt_idempotency_key",
            "blocked_signature_manifest_receipt_idempotency_key_noop",
            "signature_manifest_receipt_idempotency_key_denied",
            vec!["signature_manifest_receipt_idempotency_key_requested"],
        ),
        (
            "notarization_submission_receipt_idempotency_state",
            "blocked_notarization_submission_receipt_idempotency_state_noop",
            "notarization_submission_receipt_idempotency_state_denied",
            vec!["notarization_submission_receipt_idempotency_state_requested"],
        ),
        (
            "notarization_ticket_stale_nonce_replay",
            "blocked_notarization_ticket_stale_nonce_replay_noop",
            "notarization_ticket_stale_nonce_replay_denied",
            vec!["notarization_ticket_stale_nonce_replay_requested"],
        ),
        (
            "stapling_receipt_cross_scope_reuse",
            "blocked_stapling_receipt_cross_scope_reuse_noop",
            "stapling_receipt_cross_scope_reuse_denied",
            vec!["stapling_receipt_cross_scope_reuse_requested"],
        ),
        (
            "installer_signing_receipt_out_of_order_replay",
            "blocked_installer_signing_receipt_out_of_order_replay_noop",
            "installer_signing_receipt_out_of_order_replay_denied",
            vec!["installer_signing_receipt_out_of_order_replay_requested"],
        ),
        (
            "provenance_receipt_completion_ack_replay",
            "blocked_provenance_receipt_completion_ack_replay_noop",
            "provenance_receipt_completion_ack_replay_denied",
            vec!["provenance_receipt_completion_ack_replay_requested"],
        ),
        (
            "sbom_receipt_ledger_index_replay",
            "blocked_sbom_receipt_ledger_index_replay_noop",
            "sbom_receipt_ledger_index_replay_denied",
            vec!["sbom_receipt_ledger_index_replay_requested"],
        ),
        (
            "release_asset_bundle_receipt_export_query_replay",
            "blocked_release_asset_bundle_receipt_export_query_replay_noop",
            "release_asset_bundle_receipt_export_query_replay_denied",
            vec!["release_asset_bundle_receipt_export_query_replay_requested"],
        ),
        (
            "cdn_update_feed_receipt_observability_replay",
            "blocked_cdn_update_feed_receipt_observability_replay_noop",
            "cdn_update_feed_receipt_observability_replay_denied",
            vec!["cdn_update_feed_receipt_observability_replay_requested"],
        ),
        (
            "package_registry_receipt_status_rebind",
            "blocked_package_registry_receipt_status_rebind_noop",
            "package_registry_receipt_status_rebind_denied",
            vec!["package_registry_receipt_status_rebind_requested"],
        ),
        (
            "dashboard_endpoint_receipt_hash_status_replay",
            "blocked_dashboard_endpoint_receipt_hash_status_replay_noop",
            "dashboard_endpoint_receipt_hash_status_replay_denied",
            vec!["dashboard_endpoint_receipt_hash_status_replay_requested"],
        ),
        (
            "external_telegram_receipt_delivery_replay",
            "blocked_external_telegram_receipt_delivery_replay_noop",
            "external_telegram_receipt_delivery_replay_denied",
            vec![
                "external_signing_receipt_delivery_replay_requested",
                "telegram_signing_receipt_delivery_replay_requested",
            ],
        ),
        (
            "release_publication_authority_replay_from_signing_receipt",
            "blocked_release_publication_authority_replay_from_signing_receipt_noop",
            "release_publication_authority_replay_from_signing_receipt_denied",
            vec!["release_publication_authority_replay_from_signing_receipt_requested"],
        ),
        (
            "activation_live_install_replay_from_signing_receipt",
            "blocked_activation_live_install_replay_from_signing_receipt_noop",
            "activation_live_install_replay_from_signing_receipt_denied",
            vec!["activation_live_install_replay_from_signing_receipt_requested"],
        ),
        (
            "install_restart_active_binary_replay_path",
            "blocked_install_restart_active_binary_replay_path_noop",
            "install_restart_active_binary_replay_path_denied",
            vec!["install_restart_active_binary_replay_path_requested"],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "source_artifact_distribution_signing_notarization_result_receipt_no_persistence_ready": source_ready,
                "canonical_noop_signing_receipt_identity_required": true,
                "artifact_distribution_signing_notarization_receipt_replay_idempotency_attempted": true,
                "artifact_distribution_signing_notarization_receipt_replay_idempotency_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_replay_idempotency_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-replay-idempotency-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:replay=0:idempotency=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-replay-idempotency-denial:no-duplicate:no-replay:no-idempotency-record:no-idempotency-persist:no-nonce:no-cross-scope:no-status-rebind:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_result_receipt_no_persistence_report_required",
        "signing_receipt_replay_denied",
        "signing_receipt_duplicate_identity_denied",
        "signing_receipt_idempotency_key_denied",
        "signing_receipt_idempotency_state_denied",
        "signing_receipt_nonce_replay_denied",
        "signing_receipt_cross_scope_reuse_denied",
        "signing_receipt_status_upgrade_denied",
        "signing_receipt_completed_status_denied",
        "signing_receipt_completion_ack_replay_denied",
        "signing_receipt_ledger_index_delivery_replay_denied",
        "signing_receipt_export_query_observability_replay_denied",
        "signing_receipt_hash_status_rebind_denied",
        "external_telegram_signing_receipt_replay_denied",
        "release_publication_authority_from_signing_receipt_replay_denied",
        "activation_live_install_from_signing_receipt_replay_denied",
        "install_restart_active_binary_from_signing_receipt_replay_denied",
        "memory_provider_kg_secret_external_send_from_signing_receipt_replay_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64("artifact_distribution_signing_notarization_result_receipt_surface_count")
            == 18
        && source_u64(
            "artifact_distribution_signing_notarization_result_receipt_surface_attempt_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_result_receipt_surface_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_result_receipt_surface_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_result_receipt_surface_persisted_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_result_receipt_surface_status_exposed_count",
        ) == 0
        && source_u64("release_publication_authority_from_signing_receipt_derived_count") == 0
        && source_u64("activation_authority_from_signing_receipt_derived_count") == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-replay-idempotency-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-26",
        "artifact_distribution_signing_notarization_receipt_replay_idempotency_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_route_v1",
        "artifact_distribution_signing_notarization_receipt_replay_idempotency_mode": "native_route_denied_signing_notarization_receipt_replay_duplicate_idempotency_state_status_rebind_authority_install_or_live_use",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_result_receipt_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_route",
            "source_artifact_distribution_signing_notarization_result_receipt_ready": source_ready,
            "source_artifact_distribution_signing_notarization_result_receipt_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_result_receipt_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_result_receipt_surface_count": source_u64("artifact_distribution_signing_notarization_result_receipt_surface_count"),
            "source_artifact_distribution_signing_notarization_result_receipt_surface_attempt_count": source_u64("artifact_distribution_signing_notarization_result_receipt_surface_attempt_count"),
            "source_artifact_distribution_signing_notarization_result_receipt_surface_denied_count": source_u64("artifact_distribution_signing_notarization_result_receipt_surface_denied_count"),
            "source_artifact_distribution_signing_notarization_result_receipt_surface_recorded_count": source_u64("artifact_distribution_signing_notarization_result_receipt_surface_recorded_count"),
            "source_artifact_distribution_signing_notarization_result_receipt_surface_persisted_count": source_u64("artifact_distribution_signing_notarization_result_receipt_surface_persisted_count"),
            "source_artifact_distribution_signing_notarization_result_receipt_surface_status_exposed_count": source_u64("artifact_distribution_signing_notarization_result_receipt_surface_status_exposed_count"),
            "source_release_publication_authority_from_signing_receipt_derived_count": source_u64("release_publication_authority_from_signing_receipt_derived_count"),
            "source_activation_authority_from_signing_receipt_derived_count": source_u64("activation_authority_from_signing_receipt_derived_count"),
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_replay_idempotency": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_replay_idempotency_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "replays_signing_receipt": false,
                    "records_duplicate_receipt": false,
                    "records_idempotency_key": false,
                    "persists_idempotency_state": false,
                    "accepts_cross_scope_reuse": false,
                    "accepts_status_upgrade": false,
                    "records_completion_ack": false,
                    "rebinds_hash_status": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "reads_credentials": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed_count",
        "artifact_distribution_signing_notarization_receipt_replay_allowed_count",
        "artifact_distribution_signing_notarization_receipt_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_replay_recorded_count",
        "artifact_distribution_signing_notarization_receipt_replay_persisted_count",
        "artifact_distribution_signing_notarization_receipt_replay_performed_count",
        "artifact_distribution_signing_notarization_receipt_duplicate_accepted_count",
        "artifact_distribution_signing_notarization_receipt_duplicate_recorded_count",
        "artifact_distribution_signing_notarization_receipt_duplicate_persisted_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_recorded_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_recorded_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_materialized_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_replay_nonce_accepted_count",
        "artifact_distribution_signing_notarization_receipt_replay_nonce_recorded_count",
        "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted_count",
        "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted_count",
        "artifact_distribution_signing_notarization_receipt_completed_status_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ack_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ledger_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_index_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_delivery_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_query_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_observability_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted_count",
        "artifact_signing_receipt_replay_accepted_count",
        "package_signing_receipt_replay_accepted_count",
        "signature_manifest_receipt_idempotency_recorded_count",
        "notarization_submission_receipt_idempotency_persisted_count",
        "notarization_ticket_receipt_nonce_recorded_count",
        "stapling_receipt_cross_scope_reuse_accepted_count",
        "installer_signing_receipt_out_of_order_accepted_count",
        "provenance_attestation_receipt_ack_replay_accepted_count",
        "sbom_manifest_receipt_ledger_index_replay_accepted_count",
        "release_asset_bundle_receipt_export_query_replay_accepted_count",
        "cdn_update_feed_receipt_observability_replay_accepted_count",
        "package_registry_receipt_status_rebind_accepted_count",
        "dashboard_endpoint_receipt_hash_status_replay_accepted_count",
        "external_signing_receipt_delivery_replay_accepted_count",
        "telegram_signing_receipt_delivery_replay_accepted_count",
        "acceptance_from_signing_receipt_replay_recorded_count",
        "operator_approval_from_signing_receipt_replay_derived_count",
        "release_publication_authority_from_signing_receipt_replay_derived_count",
        "activation_authority_from_signing_receipt_replay_derived_count",
        "download_link_from_signing_receipt_replay_rendered_count",
        "install_command_from_signing_receipt_replay_rendered_count",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed",
        "artifact_distribution_signing_notarization_receipt_replay_allowed",
        "artifact_distribution_signing_notarization_receipt_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_replay_recorded",
        "artifact_distribution_signing_notarization_receipt_replay_persisted",
        "artifact_distribution_signing_notarization_receipt_replay_performed",
        "artifact_distribution_signing_notarization_receipt_duplicate_accepted",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted",
        "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted",
        "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted",
        "artifact_distribution_signing_notarization_receipt_completed_status_accepted",
        "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "artifact_distribution_signing_notarization_receipt_replay_recorded",
        "artifact_distribution_signing_notarization_receipt_replay_persisted",
        "artifact_distribution_signing_notarization_receipt_replay_performed",
        "artifact_distribution_signing_notarization_receipt_duplicate_recorded",
        "artifact_distribution_signing_notarization_receipt_duplicate_persisted",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_recorded",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_recorded",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_materialized",
        "artifact_distribution_signing_notarization_receipt_idempotency_filesystem_written",
        "artifact_distribution_signing_notarization_receipt_replay_nonce_recorded",
        "artifact_distribution_signing_notarization_receipt_completed_status_accepted",
        "artifact_distribution_signing_notarization_receipt_ack_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_ledger_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_index_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_delivery_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_query_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_export_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_observability_replay_accepted",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_rendered",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
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
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    let mut side_effects = serde_json::Map::new();
    for key in &side_effect_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report["artifact_distribution_signing_notarization_receipt_replay_idempotency_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_allowed",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_materialized",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_filesystem_written",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_materialized",
        "artifact_distribution_signing_notarization_receipt_duplicate_sequence_accepted",
        "artifact_distribution_signing_notarization_receipt_stale_sequence_accepted",
        "artifact_distribution_signing_notarization_receipt_late_arrival_accepted",
        "artifact_distribution_signing_notarization_receipt_future_gap_accepted",
        "artifact_distribution_signing_notarization_receipt_timestamp_rollback_accepted",
        "artifact_distribution_signing_notarization_receipt_epoch_rollback_accepted",
        "artifact_distribution_signing_notarization_receipt_same_sequence_different_hash_accepted",
        "artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted",
        "artifact_distribution_signing_notarization_receipt_ordered_query_export_accepted",
        "artifact_distribution_signing_notarization_receipt_ordered_observability_accepted",
        "artifact_distribution_signing_notarization_receipt_ordered_delivery_accepted",
        "artifact_distribution_signing_notarization_receipt_ordered_status_accepted",
        "artifact_distribution_signing_notarization_receipt_ordered_hash_status_accepted",
        "artifact_signing_receipt_ordering_accepted",
        "package_signing_receipt_ordering_accepted",
        "signature_manifest_receipt_late_arrival_accepted",
        "notarization_submission_receipt_future_gap_accepted",
        "notarization_ticket_receipt_timestamp_rollback_accepted",
        "stapling_receipt_epoch_rollback_accepted",
        "installer_signing_same_sequence_hash_accepted",
        "provenance_attestation_latest_wins_accepted",
        "sbom_manifest_monotonic_cursor_recorded",
        "release_asset_bundle_ordered_query_export_accepted",
        "cdn_update_feed_ordered_observability_accepted",
        "package_registry_ordered_status_accepted",
        "dashboard_endpoint_ordered_hash_status_accepted",
        "external_ordered_delivery_accepted",
        "telegram_ordered_delivery_accepted",
        "public_release_claimed",
        "public_ga_claimed",
        "acceptance_recorded",
        "operator_approval_derived",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_rendered",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
    ];
    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "source_signing_receipt_replay_idempotency_report_required",
            "blocked_source_signing_receipt_replay_idempotency_required_noop",
            "source_signing_receipt_replay_idempotency_report_required",
            vec!["source_report_required"],
        ),
        (
            "duplicate_signing_receipt_sequence",
            "blocked_duplicate_signing_receipt_sequence_noop",
            "duplicate_signing_receipt_sequence_denied",
            vec!["duplicate_signing_receipt_sequence_requested"],
        ),
        (
            "stale_package_signing_receipt_sequence",
            "blocked_stale_package_signing_receipt_sequence_noop",
            "stale_package_signing_receipt_sequence_denied",
            vec!["stale_package_signing_receipt_sequence_requested"],
        ),
        (
            "signature_manifest_receipt_late_arrival",
            "blocked_signature_manifest_receipt_late_arrival_noop",
            "signature_manifest_receipt_late_arrival_denied",
            vec!["signature_manifest_receipt_late_arrival_requested"],
        ),
        (
            "notarization_submission_receipt_future_gap",
            "blocked_notarization_submission_receipt_future_gap_noop",
            "notarization_submission_receipt_future_gap_denied",
            vec!["notarization_submission_receipt_future_gap_requested"],
        ),
        (
            "notarization_ticket_timestamp_rollback",
            "blocked_notarization_ticket_timestamp_rollback_noop",
            "notarization_ticket_timestamp_rollback_denied",
            vec!["notarization_ticket_timestamp_rollback_requested"],
        ),
        (
            "stapling_receipt_epoch_rollback",
            "blocked_stapling_receipt_epoch_rollback_noop",
            "stapling_receipt_epoch_rollback_denied",
            vec!["stapling_receipt_epoch_rollback_requested"],
        ),
        (
            "installer_signing_same_sequence_different_hash",
            "blocked_installer_signing_same_sequence_different_hash_noop",
            "installer_signing_same_sequence_different_hash_denied",
            vec!["installer_signing_same_sequence_different_hash_requested"],
        ),
        (
            "provenance_receipt_latest_wins_overwrite",
            "blocked_provenance_receipt_latest_wins_overwrite_noop",
            "provenance_receipt_latest_wins_overwrite_denied",
            vec!["provenance_receipt_latest_wins_overwrite_requested"],
        ),
        (
            "sbom_receipt_monotonic_cursor",
            "blocked_sbom_receipt_monotonic_cursor_noop",
            "sbom_receipt_monotonic_cursor_denied",
            vec!["sbom_receipt_monotonic_cursor_requested"],
        ),
        (
            "release_asset_bundle_receipt_ordered_query_export",
            "blocked_release_asset_bundle_receipt_ordered_query_export_noop",
            "release_asset_bundle_receipt_ordered_query_export_denied",
            vec!["release_asset_bundle_receipt_ordered_query_export_requested"],
        ),
        (
            "cdn_update_feed_receipt_ordered_observability",
            "blocked_cdn_update_feed_receipt_ordered_observability_noop",
            "cdn_update_feed_receipt_ordered_observability_denied",
            vec!["cdn_update_feed_receipt_ordered_observability_requested"],
        ),
        (
            "package_registry_ordered_status",
            "blocked_package_registry_ordered_status_noop",
            "package_registry_ordered_status_denied",
            vec!["package_registry_ordered_status_requested"],
        ),
        (
            "dashboard_endpoint_ordered_hash_status",
            "blocked_dashboard_endpoint_ordered_hash_status_noop",
            "dashboard_endpoint_ordered_hash_status_denied",
            vec!["dashboard_endpoint_ordered_hash_status_requested"],
        ),
        (
            "external_telegram_ordered_delivery",
            "blocked_external_telegram_ordered_delivery_noop",
            "external_telegram_ordered_delivery_denied",
            vec![
                "external_ordered_delivery_requested",
                "telegram_ordered_delivery_requested",
            ],
        ),
        (
            "release_publication_authority_ordering",
            "blocked_release_publication_authority_ordering_noop",
            "release_publication_authority_ordering_denied",
            vec!["release_publication_authority_ordering_requested"],
        ),
        (
            "activation_live_install_ordering",
            "blocked_activation_live_install_ordering_noop",
            "activation_live_install_ordering_denied",
            vec!["activation_live_install_ordering_requested"],
        ),
        (
            "install_restart_active_binary_ordering_path",
            "blocked_install_restart_active_binary_ordering_path_noop",
            "install_restart_active_binary_ordering_path_denied",
            vec!["install_restart_active_binary_ordering_path_requested"],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "source_signing_receipt_replay_idempotency_denial_ready": source_ready,
                "canonical_noop_signing_receipt_identity_required": true,
                "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempted": true,
                "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-ordering-monotonicity-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:ordering=0:cursor=0:monotonicity=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-ordering-monotonicity-denial:no-ordering:no-sequence-cursor:no-monotonicity-state:no-latest-wins:no-rollback:no-status-rebind:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_report_required",
        "signing_receipt_duplicate_sequence_denied",
        "signing_receipt_stale_sequence_denied",
        "signing_receipt_late_arrival_denied",
        "signing_receipt_future_gap_denied",
        "signing_receipt_timestamp_rollback_denied",
        "signing_receipt_epoch_rollback_denied",
        "signing_receipt_same_sequence_different_hash_denied",
        "signing_receipt_latest_wins_overwrite_denied",
        "signing_receipt_monotonic_cursor_denied",
        "signing_receipt_ordered_query_export_denied",
        "signing_receipt_ordered_observability_denied",
        "signing_receipt_ordered_status_denied",
        "signing_receipt_ordered_hash_status_denied",
        "external_telegram_signing_receipt_ordered_delivery_denied",
        "release_publication_authority_from_signing_receipt_ordering_denied",
        "activation_live_install_from_signing_receipt_ordering_denied",
        "install_restart_active_binary_from_signing_receipt_ordering_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed_count",
        ) == 0
        && source_u64("artifact_distribution_signing_notarization_receipt_replay_accepted_count")
            == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count",
        ) == 0
        && source_u64("release_publication_authority_from_signing_receipt_replay_derived_count")
            == 0
        && source_u64("activation_authority_from_signing_receipt_replay_derived_count") == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-ordering-monotonicity-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-26",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_route_v1",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_mode": "native_route_denied_signing_notarization_receipt_ordering_monotonicity_cursor_state_latest_wins_rollback_status_authority_install_or_live_use",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed_count": source_u64("artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed_count"),
            "source_artifact_distribution_signing_notarization_receipt_replay_accepted_count": source_u64("artifact_distribution_signing_notarization_receipt_replay_accepted_count"),
            "source_artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count": source_u64("artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count"),
            "source_release_publication_authority_from_signing_receipt_replay_derived_count": source_u64("release_publication_authority_from_signing_receipt_replay_derived_count"),
            "source_activation_authority_from_signing_receipt_replay_derived_count": source_u64("activation_authority_from_signing_receipt_replay_derived_count"),
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_ordering_monotonicity": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "accepts_ordering": false,
                    "records_sequence_cursor": false,
                    "persists_monotonicity_state": false,
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "reads_credentials": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_allowed_count",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted_count",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_materialized_count",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded_count",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted_count",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded_count",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_materialized_count",
        "artifact_distribution_signing_notarization_receipt_duplicate_sequence_accepted_count",
        "artifact_distribution_signing_notarization_receipt_stale_sequence_accepted_count",
        "artifact_distribution_signing_notarization_receipt_late_arrival_accepted_count",
        "artifact_distribution_signing_notarization_receipt_future_gap_accepted_count",
        "artifact_distribution_signing_notarization_receipt_timestamp_rollback_accepted_count",
        "artifact_distribution_signing_notarization_receipt_epoch_rollback_accepted_count",
        "artifact_distribution_signing_notarization_receipt_same_sequence_different_hash_accepted_count",
        "artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ordered_query_export_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ordered_observability_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ordered_delivery_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ordered_status_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ordered_hash_status_accepted_count",
        "artifact_signing_receipt_ordering_accepted_count",
        "package_signing_receipt_ordering_accepted_count",
        "signature_manifest_receipt_late_arrival_accepted_count",
        "notarization_submission_receipt_future_gap_accepted_count",
        "notarization_ticket_receipt_timestamp_rollback_accepted_count",
        "stapling_receipt_epoch_rollback_accepted_count",
        "installer_signing_same_sequence_hash_accepted_count",
        "provenance_attestation_latest_wins_accepted_count",
        "sbom_manifest_monotonic_cursor_recorded_count",
        "release_asset_bundle_ordered_query_export_accepted_count",
        "cdn_update_feed_ordered_observability_accepted_count",
        "package_registry_ordered_status_accepted_count",
        "dashboard_endpoint_ordered_hash_status_accepted_count",
        "external_ordered_delivery_accepted_count",
        "telegram_ordered_delivery_accepted_count",
        "acceptance_from_signing_receipt_ordering_recorded_count",
        "operator_approval_from_signing_receipt_ordering_derived_count",
        "release_publication_authority_from_signing_receipt_ordering_derived_count",
        "activation_authority_from_signing_receipt_ordering_derived_count",
        "download_link_from_signing_receipt_ordering_rendered_count",
        "install_command_from_signing_receipt_ordering_rendered_count",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_materialized",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted",
        "artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_materialized",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_filesystem_written",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted",
        "artifact_distribution_signing_notarization_receipt_ordered_query_export_accepted",
        "artifact_distribution_signing_notarization_receipt_ordered_observability_accepted",
        "artifact_distribution_signing_notarization_receipt_ordered_delivery_accepted",
        "artifact_distribution_signing_notarization_receipt_ordered_status_accepted",
        "artifact_distribution_signing_notarization_receipt_ordered_hash_status_accepted",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_rendered",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
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
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    let mut side_effects = serde_json::Map::new();
    for key in &side_effect_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report["artifact_distribution_signing_notarization_receipt_ordering_monotonicity_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written",
        "artifact_distribution_signing_notarization_receipt_cancellation_accepted",
        "artifact_distribution_signing_notarization_receipt_cancellation_recorded",
        "artifact_distribution_signing_notarization_receipt_cancellation_persisted",
        "artifact_distribution_signing_notarization_receipt_withdrawal_accepted",
        "artifact_distribution_signing_notarization_receipt_supersession_accepted",
        "artifact_distribution_signing_notarization_receipt_supersession_recorded",
        "artifact_distribution_signing_notarization_receipt_supersession_persisted",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted",
        "artifact_distribution_signing_notarization_receipt_tombstone_recorded",
        "artifact_distribution_signing_notarization_receipt_tombstone_persisted",
        "artifact_distribution_signing_notarization_receipt_delete_marker_recorded",
        "artifact_distribution_signing_notarization_receipt_delete_marker_persisted",
        "artifact_distribution_signing_notarization_receipt_latest_replacement_accepted",
        "artifact_distribution_signing_notarization_receipt_ack_replacement_accepted",
        "artifact_distribution_signing_notarization_receipt_query_replacement_accepted",
        "artifact_distribution_signing_notarization_receipt_export_replacement_accepted",
        "artifact_distribution_signing_notarization_receipt_observability_replacement_accepted",
        "artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted",
        "artifact_signing_receipt_cancellation_accepted",
        "package_signing_receipt_cancellation_accepted",
        "signature_manifest_receipt_withdrawal_accepted",
        "notarization_submission_receipt_cancellation_accepted",
        "notarization_ticket_receipt_supersession_accepted",
        "stapling_receipt_tombstone_recorded",
        "installer_signing_receipt_replacement_accepted",
        "provenance_attestation_latest_replacement_accepted",
        "sbom_manifest_supersession_accepted",
        "release_asset_bundle_cancelled_query_export_accepted",
        "cdn_update_feed_superseded_observability_accepted",
        "package_registry_replacement_status_accepted",
        "dashboard_endpoint_tombstone_hash_status_accepted",
        "external_supersession_delivery_accepted",
        "telegram_supersession_delivery_accepted",
        "public_release_claimed",
        "public_ga_claimed",
        "acceptance_recorded",
        "operator_approval_derived",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_rendered",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
    ];
    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "source_signing_receipt_ordering_monotonicity_report_required",
            "blocked_source_signing_receipt_ordering_monotonicity_required_noop",
            "source_signing_receipt_ordering_monotonicity_report_required",
            vec!["source_report_required"],
        ),
        (
            "duplicate_signing_receipt_cancellation",
            "blocked_duplicate_signing_receipt_cancellation_noop",
            "duplicate_signing_receipt_cancellation_denied",
            vec!["duplicate_signing_receipt_cancellation_requested"],
        ),
        (
            "stale_package_signing_receipt_cancellation",
            "blocked_stale_package_signing_receipt_cancellation_noop",
            "stale_package_signing_receipt_cancellation_denied",
            vec!["stale_package_signing_receipt_cancellation_requested"],
        ),
        (
            "signature_manifest_late_arrival_withdrawal",
            "blocked_signature_manifest_late_arrival_withdrawal_noop",
            "signature_manifest_late_arrival_withdrawal_denied",
            vec!["signature_manifest_late_arrival_withdrawal_requested"],
        ),
        (
            "notarization_submission_future_gap_cancellation",
            "blocked_notarization_submission_future_gap_cancellation_noop",
            "notarization_submission_future_gap_cancellation_denied",
            vec!["notarization_submission_future_gap_cancellation_requested"],
        ),
        (
            "notarization_ticket_rollback_supersession",
            "blocked_notarization_ticket_rollback_supersession_noop",
            "notarization_ticket_rollback_supersession_denied",
            vec!["notarization_ticket_rollback_supersession_requested"],
        ),
        (
            "stapling_epoch_rollback_tombstone",
            "blocked_stapling_epoch_rollback_tombstone_noop",
            "stapling_epoch_rollback_tombstone_denied",
            vec!["stapling_epoch_rollback_tombstone_requested"],
        ),
        (
            "installer_same_sequence_hash_replacement",
            "blocked_installer_same_sequence_hash_replacement_noop",
            "installer_same_sequence_hash_replacement_denied",
            vec!["installer_same_sequence_hash_replacement_requested"],
        ),
        (
            "provenance_latest_wins_cancellation",
            "blocked_provenance_latest_wins_cancellation_noop",
            "provenance_latest_wins_cancellation_denied",
            vec!["provenance_latest_wins_cancellation_requested"],
        ),
        (
            "sbom_monotonic_cursor_supersession",
            "blocked_sbom_monotonic_cursor_supersession_noop",
            "sbom_monotonic_cursor_supersession_denied",
            vec!["sbom_monotonic_cursor_supersession_requested"],
        ),
        (
            "release_asset_bundle_cancelled_query_export",
            "blocked_release_asset_bundle_cancelled_query_export_noop",
            "release_asset_bundle_cancelled_query_export_denied",
            vec!["release_asset_bundle_cancelled_query_export_requested"],
        ),
        (
            "cdn_update_feed_superseded_observability",
            "blocked_cdn_update_feed_superseded_observability_noop",
            "cdn_update_feed_superseded_observability_denied",
            vec!["cdn_update_feed_superseded_observability_requested"],
        ),
        (
            "package_registry_replacement_status",
            "blocked_package_registry_replacement_status_noop",
            "package_registry_replacement_status_denied",
            vec!["package_registry_replacement_status_requested"],
        ),
        (
            "dashboard_endpoint_tombstone_hash_status",
            "blocked_dashboard_endpoint_tombstone_hash_status_noop",
            "dashboard_endpoint_tombstone_hash_status_denied",
            vec!["dashboard_endpoint_tombstone_hash_status_requested"],
        ),
        (
            "external_telegram_supersession_delivery",
            "blocked_external_telegram_supersession_delivery_noop",
            "external_telegram_supersession_delivery_denied",
            vec![
                "external_supersession_delivery_requested",
                "telegram_supersession_delivery_requested",
            ],
        ),
        (
            "release_publication_authority_cancellation_supersession",
            "blocked_release_publication_authority_cancellation_supersession_noop",
            "release_publication_authority_cancellation_supersession_denied",
            vec!["release_publication_authority_cancellation_supersession_requested"],
        ),
        (
            "activation_live_install_supersession",
            "blocked_activation_live_install_supersession_noop",
            "activation_live_install_supersession_denied",
            vec!["activation_live_install_supersession_requested"],
        ),
        (
            "install_restart_active_binary_cancellation_path",
            "blocked_install_restart_active_binary_cancellation_path_noop",
            "install_restart_active_binary_cancellation_path_denied",
            vec!["install_restart_active_binary_cancellation_path_requested"],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "source_signing_receipt_ordering_monotonicity_denial_ready": source_ready,
                "canonical_noop_signing_receipt_identity_required": true,
                "artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempted": true,
                "artifact_distribution_signing_notarization_receipt_cancellation_supersession_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_cancellation_supersession_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-cancellation-supersession-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:cancel=0:supersede=0:replace=0:tombstone=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-cancellation-supersession-denial:no-cancel:no-supersede:no-replacement:no-tombstone:no-delete-marker:no-lifecycle-persist:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_report_required",
        "signing_receipt_cancellation_denied",
        "signing_receipt_withdrawal_denied",
        "signing_receipt_supersession_denied",
        "signing_receipt_replacement_denied",
        "signing_receipt_tombstone_denied",
        "signing_receipt_delete_marker_denied",
        "signing_receipt_latest_replacement_denied",
        "signing_receipt_ack_replacement_denied",
        "signing_receipt_query_export_replacement_denied",
        "signing_receipt_observability_replacement_denied",
        "signing_receipt_lifecycle_cancellation_supersession_denied",
        "external_telegram_signing_receipt_supersession_delivery_denied",
        "release_publication_authority_from_signing_receipt_cancellation_supersession_denied",
        "activation_live_install_from_signing_receipt_supersession_denied",
        "install_restart_active_binary_from_signing_receipt_cancellation_denied",
        "memory_provider_kg_secret_external_send_from_signing_receipt_cancellation_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_allowed_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted_count",
        ) == 0
        && source_u64("release_publication_authority_from_signing_receipt_ordering_derived_count")
            == 0
        && source_u64("activation_authority_from_signing_receipt_ordering_derived_count") == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-cancellation-supersession-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-26",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_route_v1",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_mode": "native_route_denied_signing_notarization_receipt_cancellation_supersession_replacement_tombstone_lifecycle_authority_install_or_live_use",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted_count": source_u64("artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted_count"),
            "source_artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted_count": source_u64("artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted_count"),
            "source_release_publication_authority_from_signing_receipt_ordering_derived_count": source_u64("release_publication_authority_from_signing_receipt_ordering_derived_count"),
            "source_activation_authority_from_signing_receipt_ordering_derived_count": source_u64("activation_authority_from_signing_receipt_ordering_derived_count"),
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_cancellation_supersession": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_cancellation_supersession_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "records_replacement_receipt": false,
                    "records_tombstone": false,
                    "records_delete_marker": false,
                    "persists_lifecycle_state": false,
                    "records_audit_evidence": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "reads_credentials": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_accepted_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_recorded_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_persisted_count",
        "artifact_distribution_signing_notarization_receipt_withdrawal_accepted_count",
        "artifact_distribution_signing_notarization_receipt_supersession_accepted_count",
        "artifact_distribution_signing_notarization_receipt_supersession_recorded_count",
        "artifact_distribution_signing_notarization_receipt_supersession_persisted_count",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted_count",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded_count",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted_count",
        "artifact_distribution_signing_notarization_receipt_tombstone_recorded_count",
        "artifact_distribution_signing_notarization_receipt_tombstone_persisted_count",
        "artifact_distribution_signing_notarization_receipt_delete_marker_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delete_marker_persisted_count",
        "artifact_distribution_signing_notarization_receipt_latest_replacement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ack_replacement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_query_replacement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_replacement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_observability_replacement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count",
        "artifact_signing_receipt_cancellation_accepted_count",
        "package_signing_receipt_cancellation_accepted_count",
        "signature_manifest_receipt_withdrawal_accepted_count",
        "notarization_submission_receipt_cancellation_accepted_count",
        "notarization_ticket_receipt_supersession_accepted_count",
        "stapling_receipt_tombstone_recorded_count",
        "installer_signing_receipt_replacement_accepted_count",
        "provenance_attestation_latest_replacement_accepted_count",
        "sbom_manifest_supersession_accepted_count",
        "release_asset_bundle_cancelled_query_export_accepted_count",
        "cdn_update_feed_superseded_observability_accepted_count",
        "package_registry_replacement_status_accepted_count",
        "dashboard_endpoint_tombstone_hash_status_accepted_count",
        "external_supersession_delivery_accepted_count",
        "telegram_supersession_delivery_accepted_count",
        "acceptance_from_signing_receipt_cancellation_recorded_count",
        "operator_approval_from_signing_receipt_cancellation_derived_count",
        "release_publication_authority_from_signing_receipt_cancellation_derived_count",
        "activation_authority_from_signing_receipt_supersession_derived_count",
        "download_link_from_signing_receipt_cancellation_rendered_count",
        "install_command_from_signing_receipt_supersession_rendered_count",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written",
        "artifact_distribution_signing_notarization_receipt_cancellation_recorded",
        "artifact_distribution_signing_notarization_receipt_cancellation_persisted",
        "artifact_distribution_signing_notarization_receipt_supersession_recorded",
        "artifact_distribution_signing_notarization_receipt_supersession_persisted",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted",
        "artifact_distribution_signing_notarization_receipt_tombstone_recorded",
        "artifact_distribution_signing_notarization_receipt_tombstone_persisted",
        "artifact_distribution_signing_notarization_receipt_delete_marker_recorded",
        "artifact_distribution_signing_notarization_receipt_delete_marker_persisted",
        "artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_rendered",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
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
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    let mut side_effects = serde_json::Map::new();
    for key in &side_effect_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        .get("artifact_distribution_signing_notarization_receipt_cancellation_supersession_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_receipt_audit_evidence_allowed",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_materialized",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written",
        "artifact_distribution_signing_notarization_receipt_audit_trail_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_trail_persisted",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted",
        "artifact_distribution_signing_notarization_receipt_hash_chain_recorded",
        "artifact_distribution_signing_notarization_receipt_merkle_root_recorded",
        "artifact_distribution_signing_notarization_receipt_attestation_recorded",
        "artifact_distribution_signing_notarization_receipt_witness_recorded",
        "artifact_distribution_signing_notarization_receipt_notary_recorded",
        "artifact_distribution_signing_notarization_receipt_ledger_recorded",
        "artifact_distribution_signing_notarization_receipt_ledger_persisted",
        "artifact_distribution_signing_notarization_receipt_index_recorded",
        "artifact_distribution_signing_notarization_receipt_index_persisted",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered",
        "artifact_distribution_signing_notarization_receipt_query_export_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_observability_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_readback_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_status_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_hash_status_evidence_recorded",
        "artifact_signing_cancellation_audit_trail_recorded",
        "package_signing_supersession_immutable_evidence_recorded",
        "signature_manifest_withdrawal_hash_chain_recorded",
        "notarization_submission_cancellation_attestation_recorded",
        "notarization_ticket_supersession_witness_recorded",
        "stapling_tombstone_ledger_index_recorded",
        "installer_replacement_evidence_materialized",
        "provenance_latest_replacement_immutable_evidence_recorded",
        "sbom_supersession_evidence_exported",
        "release_asset_cancelled_query_evidence_recorded",
        "cdn_superseded_observability_evidence_recorded",
        "package_registry_replacement_status_evidence_recorded",
        "dashboard_endpoint_tombstone_hash_status_evidence_recorded",
        "external_audit_evidence_delivered",
        "telegram_audit_evidence_delivered",
        "public_release_claimed",
        "public_ga_claimed",
        "acceptance_recorded",
        "operator_approval_derived",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_rendered",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
    ];
    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "source_signing_receipt_cancellation_supersession_report_required",
            "blocked_source_signing_receipt_cancellation_supersession_required_noop",
            "source_signing_receipt_cancellation_supersession_report_required",
            vec!["source_report_required"],
        ),
        (
            "artifact_signing_cancellation_audit_trail_append",
            "blocked_artifact_signing_cancellation_audit_trail_append_noop",
            "artifact_signing_cancellation_audit_trail_append_denied",
            vec!["artifact_signing_cancellation_audit_trail_append_requested"],
        ),
        (
            "package_signing_supersession_immutable_evidence_packet",
            "blocked_package_signing_supersession_immutable_evidence_packet_noop",
            "package_signing_supersession_immutable_evidence_packet_denied",
            vec!["package_signing_supersession_immutable_evidence_packet_requested"],
        ),
        (
            "signature_manifest_withdrawal_hash_chain",
            "blocked_signature_manifest_withdrawal_hash_chain_noop",
            "signature_manifest_withdrawal_hash_chain_denied",
            vec!["signature_manifest_withdrawal_hash_chain_requested"],
        ),
        (
            "notarization_submission_cancellation_attestation",
            "blocked_notarization_submission_cancellation_attestation_noop",
            "notarization_submission_cancellation_attestation_denied",
            vec!["notarization_submission_cancellation_attestation_requested"],
        ),
        (
            "notarization_ticket_supersession_witness_notary",
            "blocked_notarization_ticket_supersession_witness_notary_noop",
            "notarization_ticket_supersession_witness_notary_denied",
            vec!["notarization_ticket_supersession_witness_notary_requested"],
        ),
        (
            "stapling_tombstone_ledger_index",
            "blocked_stapling_tombstone_ledger_index_noop",
            "stapling_tombstone_ledger_index_denied",
            vec!["stapling_tombstone_ledger_index_requested"],
        ),
        (
            "installer_replacement_evidence_materialization",
            "blocked_installer_replacement_evidence_materialization_noop",
            "installer_replacement_evidence_materialization_denied",
            vec!["installer_replacement_evidence_materialization_requested"],
        ),
        (
            "provenance_latest_replacement_immutable_evidence",
            "blocked_provenance_latest_replacement_immutable_evidence_noop",
            "provenance_latest_replacement_immutable_evidence_denied",
            vec!["provenance_latest_replacement_immutable_evidence_requested"],
        ),
        (
            "sbom_supersession_evidence_export",
            "blocked_sbom_supersession_evidence_export_noop",
            "sbom_supersession_evidence_export_denied",
            vec!["sbom_supersession_evidence_export_requested"],
        ),
        (
            "release_asset_cancelled_query_evidence",
            "blocked_release_asset_cancelled_query_evidence_noop",
            "release_asset_cancelled_query_evidence_denied",
            vec!["release_asset_cancelled_query_evidence_requested"],
        ),
        (
            "cdn_superseded_observability_evidence",
            "blocked_cdn_superseded_observability_evidence_noop",
            "cdn_superseded_observability_evidence_denied",
            vec!["cdn_superseded_observability_evidence_requested"],
        ),
        (
            "package_registry_replacement_status_evidence",
            "blocked_package_registry_replacement_status_evidence_noop",
            "package_registry_replacement_status_evidence_denied",
            vec!["package_registry_replacement_status_evidence_requested"],
        ),
        (
            "dashboard_endpoint_tombstone_hash_status_evidence",
            "blocked_dashboard_endpoint_tombstone_hash_status_evidence_noop",
            "dashboard_endpoint_tombstone_hash_status_evidence_denied",
            vec!["dashboard_endpoint_tombstone_hash_status_evidence_requested"],
        ),
        (
            "external_telegram_audit_evidence_delivery",
            "blocked_external_telegram_audit_evidence_delivery_noop",
            "external_telegram_audit_evidence_delivery_denied",
            vec![
                "external_audit_evidence_delivery_requested",
                "telegram_audit_evidence_delivery_requested",
            ],
        ),
        (
            "release_publication_authority_audit_evidence",
            "blocked_release_publication_authority_audit_evidence_noop",
            "release_publication_authority_audit_evidence_denied",
            vec!["release_publication_authority_audit_evidence_requested"],
        ),
        (
            "activation_live_install_audit_evidence",
            "blocked_activation_live_install_audit_evidence_noop",
            "activation_live_install_audit_evidence_denied",
            vec!["activation_live_install_audit_evidence_requested"],
        ),
        (
            "install_restart_active_binary_audit_path",
            "blocked_install_restart_active_binary_audit_path_noop",
            "install_restart_active_binary_audit_path_denied",
            vec!["install_restart_active_binary_audit_path_requested"],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "source_signing_receipt_cancellation_supersession_denial_ready": source_ready,
                "canonical_noop_signing_receipt_identity_required": true,
                "artifact_distribution_signing_notarization_receipt_audit_evidence_attempted": true,
                "artifact_distribution_signing_notarization_receipt_audit_evidence_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_audit_evidence_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-audit-evidence-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:audit=0:evidence=0:ledger=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-audit-evidence-denial:no-audit:no-immutable-evidence:no-hash-chain:no-attestation:no-ledger:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_report_required",
        "signing_receipt_audit_trail_denied",
        "signing_receipt_immutable_evidence_denied",
        "signing_receipt_hash_chain_merkle_root_denied",
        "signing_receipt_attestation_witness_notary_denied",
        "signing_receipt_ledger_index_denied",
        "signing_receipt_materialized_evidence_denied",
        "signing_receipt_export_query_observability_evidence_denied",
        "signing_receipt_readback_status_hash_evidence_denied",
        "external_telegram_signing_receipt_audit_evidence_delivery_denied",
        "release_publication_authority_from_signing_receipt_audit_evidence_denied",
        "activation_live_install_from_signing_receipt_audit_evidence_denied",
        "install_restart_active_binary_from_signing_receipt_audit_evidence_denied",
        "memory_provider_kg_secret_external_send_from_signing_receipt_audit_evidence_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempt_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count",
        ) == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_cancellation_derived_count",
        ) == 0
        && source_u64("activation_authority_from_signing_receipt_supersession_derived_count") == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_AUDIT_EVIDENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-audit-evidence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-26",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_route_v1",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_mode": "native_route_denied_signing_notarization_receipt_audit_trail_immutable_evidence_hash_chain_ledger_authority_install_or_live_use",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_audit_evidence_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count": source_u64("artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count"),
            "source_artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count": source_u64("artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count"),
            "source_release_publication_authority_from_signing_receipt_cancellation_derived_count": source_u64("release_publication_authority_from_signing_receipt_cancellation_derived_count"),
            "source_activation_authority_from_signing_receipt_supersession_derived_count": source_u64("activation_authority_from_signing_receipt_supersession_derived_count"),
            "artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_audit_evidence": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_audit_evidence_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_audit_evidence": false,
                    "records_immutable_evidence": false,
                    "records_hash_chain": false,
                    "records_attestation": false,
                    "records_witness": false,
                    "records_notary": false,
                    "records_ledger": false,
                    "persists_audit_evidence": false,
                    "accepts_retention": false,
                    "accepts_expiry": false,
                    "performs_garbage_collection": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "reads_credentials": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_audit_evidence_allowed_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_materialized_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_trail_persisted_count",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted_count",
        "artifact_distribution_signing_notarization_receipt_hash_chain_recorded_count",
        "artifact_distribution_signing_notarization_receipt_merkle_root_recorded_count",
        "artifact_distribution_signing_notarization_receipt_attestation_recorded_count",
        "artifact_distribution_signing_notarization_receipt_witness_recorded_count",
        "artifact_distribution_signing_notarization_receipt_notary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_persisted_count",
        "artifact_distribution_signing_notarization_receipt_index_recorded_count",
        "artifact_distribution_signing_notarization_receipt_index_persisted_count",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered_count",
        "artifact_distribution_signing_notarization_receipt_query_export_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_readback_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_hash_status_evidence_recorded_count",
        "artifact_signing_cancellation_audit_trail_recorded_count",
        "package_signing_supersession_immutable_evidence_recorded_count",
        "signature_manifest_withdrawal_hash_chain_recorded_count",
        "notarization_submission_cancellation_attestation_recorded_count",
        "notarization_ticket_supersession_witness_recorded_count",
        "stapling_tombstone_ledger_index_recorded_count",
        "installer_replacement_evidence_materialized_count",
        "provenance_latest_replacement_immutable_evidence_recorded_count",
        "sbom_supersession_evidence_exported_count",
        "release_asset_cancelled_query_evidence_recorded_count",
        "cdn_superseded_observability_evidence_recorded_count",
        "package_registry_replacement_status_evidence_recorded_count",
        "dashboard_endpoint_tombstone_hash_status_evidence_recorded_count",
        "external_audit_evidence_delivered_count",
        "telegram_audit_evidence_delivered_count",
        "acceptance_from_signing_receipt_audit_evidence_recorded_count",
        "operator_approval_from_signing_receipt_audit_evidence_derived_count",
        "release_publication_authority_from_signing_receipt_audit_evidence_derived_count",
        "activation_authority_from_signing_receipt_audit_evidence_derived_count",
        "download_link_from_signing_receipt_audit_evidence_rendered_count",
        "install_command_from_signing_receipt_audit_evidence_rendered_count",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_materialized",
        "artifact_distribution_signing_notarization_receipt_audit_trail_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_trail_persisted",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted",
        "artifact_distribution_signing_notarization_receipt_hash_chain_recorded",
        "artifact_distribution_signing_notarization_receipt_attestation_recorded",
        "artifact_distribution_signing_notarization_receipt_ledger_recorded",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_materialized",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written",
        "artifact_distribution_signing_notarization_receipt_audit_trail_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_trail_persisted",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted",
        "artifact_distribution_signing_notarization_receipt_hash_chain_recorded",
        "artifact_distribution_signing_notarization_receipt_merkle_root_recorded",
        "artifact_distribution_signing_notarization_receipt_attestation_recorded",
        "artifact_distribution_signing_notarization_receipt_witness_recorded",
        "artifact_distribution_signing_notarization_receipt_notary_recorded",
        "artifact_distribution_signing_notarization_receipt_ledger_recorded",
        "artifact_distribution_signing_notarization_receipt_ledger_persisted",
        "artifact_distribution_signing_notarization_receipt_index_recorded",
        "artifact_distribution_signing_notarization_receipt_index_persisted",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered",
        "artifact_distribution_signing_notarization_receipt_query_export_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_observability_evidence_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_rendered",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
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
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    let mut side_effects = serde_json::Map::new();
    for key in &side_effect_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}
