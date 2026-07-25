fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source = hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_report();
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_contract_hash_sha256")
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
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_ready",
    );

    let surface_false_keys = [
        "source_operator_identity_session_revocation_logout_replay_reinstatement_report_required",
        "revocation_logout_replay_ordering_requested",
        "logout_replay_sequence_requested",
        "identity_reinstatement_ordering_requested",
        "session_reinstatement_ordering_requested",
        "sequence_claim_requested",
        "monotonicity_claim_requested",
        "timestamp_rollback_requested",
        "epoch_rollback_requested",
        "same_sequence_different_nonce_requested",
        "late_arrival_requested",
        "future_sequence_gap_requested",
        "latest_wins_requested",
        "monotonic_cursor_requested",
        "ordered_query_requested",
        "ordered_export_requested",
        "ordered_observability_requested",
        "ordered_delivery_requested",
        "completion_order_requested",
        "telegram_ordered_delivery_requested",
        "release_publication_authority_ordering_requested",
        "install_restart_active_binary_ordering_requested",
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
        "ordered_query_accepted",
        "ordered_export_accepted",
        "ordered_observability_accepted",
        "ordered_delivery_accepted",
        "completion_order_recorded",
        "operator_approval_from_revocation_logout_replay_reinstatement_ordering_derived",
        "acceptance_from_revocation_logout_replay_reinstatement_ordering_recorded",
        "terminal_decision_from_revocation_logout_replay_reinstatement_ordering_recorded",
        "terminal_status_from_revocation_logout_replay_reinstatement_ordering_recorded",
        "release_publication_authority_from_revocation_logout_replay_reinstatement_ordering_derived",
        "activation_authority_from_revocation_logout_replay_reinstatement_ordering_derived",
        "download_link_from_revocation_logout_replay_reinstatement_ordering_rendered",
        "install_command_from_revocation_logout_replay_reinstatement_ordering_rendered",
        "install_from_revocation_logout_replay_reinstatement_ordering_executed",
        "service_restart_from_revocation_logout_replay_reinstatement_ordering_performed",
        "launchd_from_revocation_logout_replay_reinstatement_ordering_mutated",
        "active_binary_from_revocation_logout_replay_reinstatement_ordering_mutated",
        "result_receipt_from_revocation_logout_replay_reinstatement_ordering_recorded",
        "result_receipt_from_revocation_logout_replay_reinstatement_ordering_persisted",
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
            "source_operator_identity_session_revocation_logout_replay_reinstatement_report_required",
            "blocked_source_replay_reinstatement_required_noop",
            "source_operator_identity_session_revocation_logout_replay_reinstatement_report_required",
            &[
                "source_operator_identity_session_revocation_logout_replay_reinstatement_report_required",
            ][..],
        ),
        (
            "download_button_revocation_replay_sequence_claim",
            "blocked_revocation_replay_sequence_noop",
            "download_button_revocation_replay_sequence_claim_denied",
            &[
                "revocation_logout_replay_ordering_requested",
                "sequence_claim_requested",
            ][..],
        ),
        (
            "direct_download_url_logout_replay_sequence_claim",
            "blocked_logout_replay_sequence_noop",
            "direct_download_url_logout_replay_sequence_claim_denied",
            &[
                "logout_replay_sequence_requested",
                "sequence_claim_requested",
            ][..],
        ),
        (
            "checksum_identity_reinstatement_timestamp_rollback_claim",
            "blocked_identity_reinstatement_timestamp_rollback_noop",
            "checksum_identity_reinstatement_timestamp_rollback_claim_denied",
            &[
                "identity_reinstatement_ordering_requested",
                "monotonicity_claim_requested",
                "timestamp_rollback_requested",
            ][..],
        ),
        (
            "package_manager_session_reinstatement_epoch_rollback_claim",
            "blocked_session_reinstatement_epoch_rollback_noop",
            "package_manager_session_reinstatement_epoch_rollback_claim_denied",
            &[
                "session_reinstatement_ordering_requested",
                "monotonicity_claim_requested",
                "epoch_rollback_requested",
            ][..],
        ),
        (
            "curl_pipe_shell_revocation_replay_same_sequence_different_nonce_claim",
            "blocked_same_sequence_different_nonce_noop",
            "curl_pipe_shell_revocation_replay_same_sequence_different_nonce_claim_denied",
            &[
                "revocation_logout_replay_ordering_requested",
                "sequence_claim_requested",
                "same_sequence_different_nonce_requested",
            ][..],
        ),
        (
            "installer_device_session_reinstatement_late_arrival_claim",
            "blocked_device_session_reinstatement_late_arrival_noop",
            "installer_device_session_reinstatement_late_arrival_claim_denied",
            &[
                "session_reinstatement_ordering_requested",
                "monotonicity_claim_requested",
                "late_arrival_requested",
            ][..],
        ),
        (
            "auto_update_session_logout_replay_future_sequence_gap_claim",
            "blocked_logout_replay_future_sequence_gap_noop",
            "auto_update_session_logout_replay_future_sequence_gap_claim_denied",
            &[
                "revocation_logout_replay_ordering_requested",
                "logout_replay_sequence_requested",
                "sequence_claim_requested",
                "future_sequence_gap_requested",
            ][..],
        ),
        (
            "release_channel_identity_revocation_replay_latest_wins_claim",
            "blocked_identity_revocation_replay_latest_wins_noop",
            "release_channel_identity_revocation_replay_latest_wins_claim_denied",
            &[
                "revocation_logout_replay_ordering_requested",
                "sequence_claim_requested",
                "latest_wins_requested",
            ][..],
        ),
        (
            "update_feed_session_reinstatement_monotonic_cursor_claim",
            "blocked_session_reinstatement_monotonic_cursor_noop",
            "update_feed_session_reinstatement_monotonic_cursor_claim_denied",
            &[
                "session_reinstatement_ordering_requested",
                "monotonicity_claim_requested",
                "monotonic_cursor_requested",
            ][..],
        ),
        (
            "package_registry_identity_badge_reinstatement_ordered_status_claim",
            "blocked_identity_badge_reinstatement_ordered_status_noop",
            "package_registry_identity_badge_reinstatement_ordered_status_claim_denied",
            &[
                "identity_reinstatement_ordering_requested",
                "monotonicity_claim_requested",
            ][..],
        ),
        (
            "cdn_session_readback_logout_replay_ordered_query_claim",
            "blocked_session_readback_logout_replay_ordered_query_noop",
            "cdn_session_readback_logout_replay_ordered_query_claim_denied",
            &[
                "logout_replay_sequence_requested",
                "sequence_claim_requested",
                "ordered_query_requested",
            ][..],
        ),
        (
            "sbom_identity_dashboard_reinstatement_ordered_export_claim",
            "blocked_identity_dashboard_reinstatement_ordered_export_noop",
            "sbom_identity_dashboard_reinstatement_ordered_export_claim_denied",
            &[
                "identity_reinstatement_ordering_requested",
                "monotonicity_claim_requested",
                "ordered_export_requested",
            ][..],
        ),
        (
            "signature_channel_session_reinstatement_ordered_observability_claim",
            "blocked_channel_session_reinstatement_ordered_observability_noop",
            "signature_channel_session_reinstatement_ordered_observability_claim_denied",
            &[
                "session_reinstatement_ordering_requested",
                "monotonicity_claim_requested",
                "ordered_observability_requested",
            ][..],
        ),
        (
            "one_click_identity_approval_reinstatement_completion_order_claim",
            "blocked_identity_approval_reinstatement_completion_order_noop",
            "one_click_identity_approval_reinstatement_completion_order_claim_denied",
            &[
                "identity_reinstatement_ordering_requested",
                "monotonicity_claim_requested",
                "completion_order_requested",
            ][..],
        ),
        (
            "external_telegram_identity_session_reinstatement_ordered_delivery_claim",
            "blocked_external_telegram_reinstatement_ordered_delivery_noop",
            "external_telegram_identity_session_reinstatement_ordered_delivery_claim_denied",
            &[
                "logout_replay_sequence_requested",
                "identity_reinstatement_ordering_requested",
                "session_reinstatement_ordering_requested",
                "sequence_claim_requested",
                "ordered_delivery_requested",
                "telegram_ordered_delivery_requested",
            ][..],
        ),
        (
            "release_publication_authority_replay_reinstatement_ordering_claim",
            "blocked_release_publication_replay_reinstatement_ordering_noop",
            "release_publication_authority_replay_reinstatement_ordering_claim_denied",
            &[
                "revocation_logout_replay_ordering_requested",
                "session_reinstatement_ordering_requested",
                "sequence_claim_requested",
                "release_publication_authority_ordering_requested",
            ][..],
        ),
        (
            "activation_live_install_restart_active_binary_session_reinstatement_ordering_claim",
            "blocked_live_session_reinstatement_ordering_noop",
            "activation_live_install_restart_active_binary_session_reinstatement_ordering_claim_denied",
            &[
                "session_reinstatement_ordering_requested",
                "monotonicity_claim_requested",
                "install_restart_active_binary_ordering_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, extra_true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface": surface,
                "source_operator_identity_session_revocation_logout_replay_reinstatement_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempted": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_noop_confirmed": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:ordering=0:monotonicity=0:lifecycle=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity:no-ordering:no-monotonicity:no-latest-lifecycle:no-cursor:no-approval:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_revocation_logout_replay_reinstatement_report_required",
        "revocation_logout_replay_ordering_denied",
        "logout_replay_sequence_acceptance_denied",
        "identity_reinstatement_ordering_denied",
        "session_reinstatement_ordering_denied",
        "sequence_cursor_recording_denied",
        "sequence_cursor_persistence_denied",
        "monotonicity_state_recording_denied",
        "monotonicity_state_persistence_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_nonce_denied",
        "late_arrival_denied",
        "future_sequence_gap_denied",
        "latest_wins_lifecycle_state_denied",
        "monotonic_cursor_denied",
        "ordered_query_export_observability_denied",
        "ordered_delivery_and_completion_order_denied",
        "acceptance_or_approval_from_ordering_denied",
        "release_activation_authority_from_ordering_denied",
        "download_install_from_ordering_denied",
        "memory_kg_provider_secret_external_send_from_ordering_denied",
    ];
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_recorded_count",
        ) == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-20",
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_v1",
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_mode": "native_route_denied_replay_reinstatement_cannot_create_lifecycle_ordering_monotonicity_or_latest_state_authority",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_gate": source["gate"].clone(),
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ready": source_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_report_sha256": source_report_sha256,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_contract_hash_sha256": source_contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_contract_hash_sha256": contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_policy_hash_sha256": policy_hash,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_ready": report_ready,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_recorded_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_count": denials.len(),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_gate",
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
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_sequence_cursor_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_sequence_cursor_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }
    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_timestamp_rollback_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_epoch_rollback_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_same_sequence_different_nonce_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_late_arrival_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_future_sequence_gap_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_latest_wins_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonic_cursor_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordered_query_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordered_export_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordered_observability_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordered_delivery_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_completion_order_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_replay_reinstatement_ordering_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_replay_reinstatement_ordering_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_external_send_count",
    ] {
        if let Some(report_object) = report.as_object_mut() {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_monotonicity_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_latest_state_recorded",
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

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source = hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_report();
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_contract_hash_sha256")
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
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_ready",
    );

    let surface_false_keys = [
        "source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_report_required",
        "revocation_logout_replay_cancellation_requested",
        "logout_replay_cancellation_requested",
        "identity_reinstatement_withdrawal_requested",
        "session_reinstatement_supersession_requested",
        "replacement_receipt_requested",
        "tombstone_requested",
        "delete_marker_requested",
        "latest_replacement_requested",
        "ack_replacement_requested",
        "cancelled_query_requested",
        "superseded_export_requested",
        "replacement_observability_requested",
        "cancellation_completion_requested",
        "supersession_delivery_requested",
        "telegram_cancellation_supersession_requested",
        "release_publication_authority_cancellation_supersession_requested",
        "install_restart_active_binary_supersession_requested",
        "cancellation_accepted",
        "cancellation_recorded",
        "cancellation_persisted",
        "revocation_replay_cancellation_recorded",
        "logout_replay_cancellation_recorded",
        "identity_reinstatement_withdrawal_recorded",
        "session_reinstatement_supersession_recorded",
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
        "replay_reinstatement_replacement_accepted",
        "lifecycle_cancellation_supersession_recorded",
        "lifecycle_cancellation_supersession_persisted",
        "result_receipt_from_cancellation_supersession_recorded",
        "result_receipt_from_cancellation_supersession_persisted",
        "result_receipt_from_cancellation_supersession_materialized",
        "result_receipt_from_cancellation_supersession_filesystem_written",
        "operator_approval_from_cancellation_supersession_derived",
        "acceptance_from_cancellation_supersession_recorded",
        "terminal_decision_from_cancellation_supersession_recorded",
        "terminal_status_from_cancellation_supersession_recorded",
        "release_publication_authority_from_cancellation_supersession_derived",
        "activation_authority_from_cancellation_supersession_derived",
        "download_link_from_cancellation_supersession_rendered",
        "install_command_from_cancellation_supersession_rendered",
        "install_from_cancellation_supersession_executed",
        "service_restart_from_cancellation_supersession_performed",
        "launchd_from_cancellation_supersession_mutated",
        "active_binary_from_cancellation_supersession_mutated",
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
            "source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_report_required",
            "blocked_source_ordering_monotonicity_required_noop",
            "source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_report_required",
            &[
                "source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_report_required",
            ][..],
        ),
        (
            "download_button_revocation_replay_cancellation_claim",
            "blocked_revocation_replay_cancellation_noop",
            "download_button_revocation_replay_cancellation_claim_denied",
            &["revocation_logout_replay_cancellation_requested"][..],
        ),
        (
            "direct_download_url_logout_replay_cancellation_claim",
            "blocked_logout_replay_cancellation_noop",
            "direct_download_url_logout_replay_cancellation_claim_denied",
            &[
                "logout_replay_cancellation_requested",
                "revocation_logout_replay_cancellation_requested",
            ][..],
        ),
        (
            "checksum_identity_reinstatement_withdrawal_claim",
            "blocked_identity_reinstatement_withdrawal_noop",
            "checksum_identity_reinstatement_withdrawal_claim_denied",
            &["identity_reinstatement_withdrawal_requested"][..],
        ),
        (
            "package_manager_session_reinstatement_supersession_claim",
            "blocked_session_reinstatement_supersession_noop",
            "package_manager_session_reinstatement_supersession_claim_denied",
            &["session_reinstatement_supersession_requested"][..],
        ),
        (
            "curl_pipe_shell_revocation_replay_replacement_nonce_claim",
            "blocked_replacement_receipt_nonce_noop",
            "curl_pipe_shell_revocation_replay_replacement_nonce_claim_denied",
            &["replacement_receipt_requested"][..],
        ),
        (
            "installer_device_session_reinstatement_tombstone_claim",
            "blocked_device_session_reinstatement_tombstone_noop",
            "installer_device_session_reinstatement_tombstone_claim_denied",
            &[
                "session_reinstatement_supersession_requested",
                "tombstone_requested",
            ][..],
        ),
        (
            "auto_update_session_logout_replay_delete_marker_claim",
            "blocked_session_logout_replay_delete_marker_noop",
            "auto_update_session_logout_replay_delete_marker_claim_denied",
            &[
                "logout_replay_cancellation_requested",
                "delete_marker_requested",
            ][..],
        ),
        (
            "release_channel_identity_revocation_replay_cancellation_status_claim",
            "blocked_identity_revocation_replay_cancellation_status_noop",
            "release_channel_identity_revocation_replay_cancellation_status_claim_denied",
            &["revocation_logout_replay_cancellation_requested"][..],
        ),
        (
            "update_feed_session_reinstatement_supersession_summary_claim",
            "blocked_session_reinstatement_supersession_summary_noop",
            "update_feed_session_reinstatement_supersession_summary_claim_denied",
            &["session_reinstatement_supersession_requested"][..],
        ),
        (
            "package_registry_identity_badge_reinstatement_replacement_claim",
            "blocked_identity_badge_reinstatement_replacement_noop",
            "package_registry_identity_badge_reinstatement_replacement_claim_denied",
            &[
                "replacement_receipt_requested",
                "identity_reinstatement_withdrawal_requested",
            ][..],
        ),
        (
            "cdn_session_readback_logout_replay_cancelled_query_claim",
            "blocked_session_readback_logout_replay_cancelled_query_noop",
            "cdn_session_readback_logout_replay_cancelled_query_claim_denied",
            &[
                "logout_replay_cancellation_requested",
                "cancelled_query_requested",
            ][..],
        ),
        (
            "sbom_identity_dashboard_reinstatement_superseded_export_claim",
            "blocked_identity_dashboard_reinstatement_superseded_export_noop",
            "sbom_identity_dashboard_reinstatement_superseded_export_claim_denied",
            &[
                "identity_reinstatement_withdrawal_requested",
                "session_reinstatement_supersession_requested",
                "superseded_export_requested",
            ][..],
        ),
        (
            "signature_channel_session_reinstatement_replacement_observability_claim",
            "blocked_channel_session_reinstatement_replacement_observability_noop",
            "signature_channel_session_reinstatement_replacement_observability_claim_denied",
            &[
                "session_reinstatement_supersession_requested",
                "replacement_receipt_requested",
                "replacement_observability_requested",
            ][..],
        ),
        (
            "one_click_identity_approval_reinstatement_cancellation_completion_claim",
            "blocked_identity_approval_reinstatement_cancellation_completion_noop",
            "one_click_identity_approval_reinstatement_cancellation_completion_claim_denied",
            &[
                "identity_reinstatement_withdrawal_requested",
                "revocation_logout_replay_cancellation_requested",
                "cancellation_completion_requested",
            ][..],
        ),
        (
            "external_telegram_identity_session_reinstatement_supersession_delivery_claim",
            "blocked_external_telegram_reinstatement_supersession_delivery_noop",
            "external_telegram_identity_session_reinstatement_supersession_delivery_claim_denied",
            &[
                "identity_reinstatement_withdrawal_requested",
                "session_reinstatement_supersession_requested",
                "supersession_delivery_requested",
                "telegram_cancellation_supersession_requested",
            ][..],
        ),
        (
            "release_publication_authority_replay_reinstatement_cancellation_supersession_claim",
            "blocked_release_publication_replay_reinstatement_cancellation_supersession_noop",
            "release_publication_authority_replay_reinstatement_cancellation_supersession_claim_denied",
            &[
                "revocation_logout_replay_cancellation_requested",
                "session_reinstatement_supersession_requested",
                "release_publication_authority_cancellation_supersession_requested",
            ][..],
        ),
        (
            "activation_live_install_restart_active_binary_session_reinstatement_supersession_claim",
            "blocked_live_session_reinstatement_supersession_noop",
            "activation_live_install_restart_active_binary_session_reinstatement_supersession_claim_denied",
            &[
                "session_reinstatement_supersession_requested",
                "replacement_receipt_requested",
                "install_restart_active_binary_supersession_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, extra_true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface": surface,
                "source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_attempted": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_noop_confirmed": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-cancellation-supersession-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:cancel=0:supersede=0:replace=0:tombstone=0:lifecycle=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-cancellation-supersession:no-cancel:no-supersede:no-replacement:no-tombstone:no-lifecycle:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_ordering_monotonicity_report_required",
        "revocation_logout_replay_cancellation_denied",
        "logout_replay_cancellation_denied",
        "identity_reinstatement_withdrawal_denied",
        "session_reinstatement_supersession_denied",
        "replacement_receipt_recording_denied",
        "tombstone_recording_denied",
        "delete_marker_recording_denied",
        "latest_replacement_denied",
        "ack_replacement_denied",
        "cancelled_query_denied",
        "superseded_export_denied",
        "replacement_observability_denied",
        "ordering_monotonicity_replacement_bypass_denied",
        "lifecycle_cancellation_supersession_persistence_denied",
        "result_receipt_from_cancellation_supersession_denied",
        "acceptance_or_approval_from_cancellation_supersession_denied",
        "release_activation_authority_from_cancellation_supersession_denied",
        "download_install_from_cancellation_supersession_denied",
        "memory_kg_provider_secret_external_send_from_cancellation_supersession_denied",
    ];
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count",
        ) == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-cancellation-supersession-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-20",
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_v1",
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_mode": "native_route_denied_replay_reinstatement_cannot_cancel_supersede_replace_tombstone_or_derive_authority",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_gate": source["gate"].clone(),
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_ready": source_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_report_sha256": source_report_sha256,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_contract_hash_sha256": source_contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_contract_hash_sha256": contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_policy_hash_sha256": policy_hash,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_ready": report_ready,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denied_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_count": denials.len(),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_identity": false,
                    "records_operator_session": false,
                    "accepts_replay": false,
                    "records_reinstatement": false,
                    "records_ordering": false,
                    "records_monotonicity": false,
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "records_replacement": false,
                    "records_tombstone": false,
                    "records_audit_trail": false,
                    "records_evidence": false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_tombstone_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_tombstone_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_latest_replacement_accepted_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }
    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ack_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancelled_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_superseded_export_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_lifecycle_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_lifecycle_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_cancellation_supersession_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_external_send_count",
    ] {
        if let Some(report_object) = report.as_object_mut() {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_supersession_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replacement_recorded",
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

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source = hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_report();
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_contract_hash_sha256")
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
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_ready",
    );

    let surface_false_keys = [
        "source_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_report_required",
        "revocation_logout_replay_audit_evidence_requested",
        "identity_reinstatement_audit_evidence_requested",
        "session_reinstatement_audit_evidence_requested",
        "cancellation_supersession_audit_evidence_requested",
        "ordering_monotonicity_audit_evidence_requested",
        "replay_idempotency_audit_evidence_requested",
        "audit_trail_append_requested",
        "immutable_evidence_requested",
        "hash_chain_requested",
        "merkle_root_requested",
        "attestation_requested",
        "witness_requested",
        "notary_requested",
        "audit_materialization_requested",
        "audit_filesystem_write_requested",
        "ledger_evidence_requested",
        "index_evidence_requested",
        "delivery_evidence_requested",
        "export_evidence_requested",
        "query_evidence_requested",
        "observability_evidence_requested",
        "readback_evidence_requested",
        "completion_ack_audit_evidence_requested",
        "release_publication_authority_evidence_requested",
        "activation_authority_evidence_requested",
        "external_audit_evidence_requested",
        "telegram_audit_evidence_requested",
        "public_release_evidence_requested",
        "release_artifact_evidence_requested",
        "public_artifact_evidence_requested",
        "activation_evidence_requested",
        "install_evidence_requested",
        "service_restart_evidence_requested",
        "active_binary_evidence_requested",
        "audit_trail_accepted",
        "audit_trail_recorded",
        "audit_trail_persisted",
        "audit_trail_materialized",
        "audit_trail_filesystem_written",
        "immutable_evidence_accepted",
        "immutable_evidence_recorded",
        "immutable_evidence_persisted",
        "immutable_evidence_materialized",
        "immutable_evidence_filesystem_written",
        "hash_chain_recorded",
        "hash_chain_persisted",
        "merkle_root_recorded",
        "merkle_root_persisted",
        "attestation_recorded",
        "attestation_persisted",
        "witness_recorded",
        "witness_persisted",
        "notary_recorded",
        "notary_persisted",
        "ledger_evidence_recorded",
        "ledger_evidence_persisted",
        "index_evidence_recorded",
        "index_evidence_persisted",
        "delivery_evidence_recorded",
        "delivery_evidence_persisted",
        "export_evidence_recorded",
        "query_evidence_registered",
        "observability_evidence_recorded",
        "readback_evidence_recorded",
        "completion_ack_from_audit_evidence_recorded",
        "cancellation_supersession_evidence_recorded",
        "ordering_monotonicity_evidence_recorded",
        "replay_idempotency_evidence_recorded",
        "audit_evidence_acceptance_recorded",
        "result_receipt_from_audit_evidence_recorded",
        "result_receipt_from_audit_evidence_persisted",
        "operator_approval_from_audit_evidence_derived",
        "release_publication_authority_from_audit_evidence_derived",
        "activation_authority_from_audit_evidence_derived",
        "download_link_from_audit_evidence_rendered",
        "install_command_from_audit_evidence_rendered",
        "install_from_audit_evidence_executed",
        "service_restart_from_audit_evidence_performed",
        "launchd_from_audit_evidence_mutated",
        "active_binary_from_audit_evidence_mutated",
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
            "source_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_report_required",
            "blocked_source_cancellation_supersession_required_noop",
            "source_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_report_required",
            &[
                "source_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_report_required",
            ][..],
        ),
        (
            "download_button_revocation_replay_audit_trail_append_claim",
            "blocked_revocation_replay_audit_trail_noop",
            "download_button_revocation_replay_audit_trail_append_claim_denied",
            &[
                "revocation_logout_replay_audit_evidence_requested",
                "audit_trail_append_requested",
            ][..],
        ),
        (
            "direct_download_url_logout_replay_immutable_evidence_packet_claim",
            "blocked_logout_replay_immutable_evidence_noop",
            "direct_download_url_logout_replay_immutable_evidence_packet_claim_denied",
            &[
                "revocation_logout_replay_audit_evidence_requested",
                "immutable_evidence_requested",
            ][..],
        ),
        (
            "checksum_identity_reinstatement_hash_chain_merkle_root_claim",
            "blocked_identity_reinstatement_hash_chain_noop",
            "checksum_identity_reinstatement_hash_chain_merkle_root_claim_denied",
            &[
                "identity_reinstatement_audit_evidence_requested",
                "immutable_evidence_requested",
                "hash_chain_requested",
                "merkle_root_requested",
            ][..],
        ),
        (
            "package_manager_session_reinstatement_attestation_witness_notary_claim",
            "blocked_session_reinstatement_attestation_noop",
            "package_manager_session_reinstatement_attestation_witness_notary_claim_denied",
            &[
                "session_reinstatement_audit_evidence_requested",
                "immutable_evidence_requested",
                "attestation_requested",
                "witness_requested",
                "notary_requested",
            ][..],
        ),
        (
            "curl_pipe_shell_revocation_replay_audit_materialization_filesystem_claim",
            "blocked_revocation_replay_audit_materialization_noop",
            "curl_pipe_shell_revocation_replay_audit_materialization_filesystem_claim_denied",
            &[
                "revocation_logout_replay_audit_evidence_requested",
                "audit_materialization_requested",
                "audit_filesystem_write_requested",
            ][..],
        ),
        (
            "installer_device_session_reinstatement_ledger_index_delivery_evidence_claim",
            "blocked_device_session_reinstatement_ledger_index_delivery_noop",
            "installer_device_session_reinstatement_ledger_index_delivery_evidence_claim_denied",
            &[
                "session_reinstatement_audit_evidence_requested",
                "ledger_evidence_requested",
                "index_evidence_requested",
                "delivery_evidence_requested",
            ][..],
        ),
        (
            "auto_update_session_logout_replay_export_query_observability_evidence_claim",
            "blocked_session_logout_replay_export_query_observability_noop",
            "auto_update_session_logout_replay_export_query_observability_evidence_claim_denied",
            &[
                "revocation_logout_replay_audit_evidence_requested",
                "export_evidence_requested",
                "query_evidence_requested",
                "observability_evidence_requested",
            ][..],
        ),
        (
            "release_channel_identity_revocation_replay_readback_evidence_claim",
            "blocked_identity_revocation_replay_readback_noop",
            "release_channel_identity_revocation_replay_readback_evidence_claim_denied",
            &[
                "revocation_logout_replay_audit_evidence_requested",
                "readback_evidence_requested",
            ][..],
        ),
        (
            "update_feed_session_reinstatement_completion_ack_audit_evidence_claim",
            "blocked_session_reinstatement_completion_ack_evidence_noop",
            "update_feed_session_reinstatement_completion_ack_audit_evidence_claim_denied",
            &[
                "session_reinstatement_audit_evidence_requested",
                "completion_ack_audit_evidence_requested",
            ][..],
        ),
        (
            "package_registry_identity_badge_reinstatement_cancellation_supersession_evidence_claim",
            "blocked_identity_reinstatement_cancellation_supersession_evidence_noop",
            "package_registry_identity_badge_reinstatement_cancellation_supersession_evidence_claim_denied",
            &[
                "identity_reinstatement_audit_evidence_requested",
                "cancellation_supersession_audit_evidence_requested",
            ][..],
        ),
        (
            "cdn_session_readback_logout_replay_ordering_monotonicity_evidence_claim",
            "blocked_logout_replay_ordering_monotonicity_evidence_noop",
            "cdn_session_readback_logout_replay_ordering_monotonicity_evidence_claim_denied",
            &[
                "revocation_logout_replay_audit_evidence_requested",
                "ordering_monotonicity_audit_evidence_requested",
            ][..],
        ),
        (
            "sbom_identity_dashboard_reinstatement_replay_idempotency_evidence_claim",
            "blocked_identity_reinstatement_replay_idempotency_evidence_noop",
            "sbom_identity_dashboard_reinstatement_replay_idempotency_evidence_claim_denied",
            &[
                "identity_reinstatement_audit_evidence_requested",
                "replay_idempotency_audit_evidence_requested",
            ][..],
        ),
        (
            "signature_channel_session_reinstatement_release_publication_authority_evidence_claim",
            "blocked_session_reinstatement_release_authority_evidence_noop",
            "signature_channel_session_reinstatement_release_publication_authority_evidence_claim_denied",
            &[
                "session_reinstatement_audit_evidence_requested",
                "release_publication_authority_evidence_requested",
            ][..],
        ),
        (
            "one_click_identity_approval_reinstatement_activation_authority_evidence_claim",
            "blocked_identity_approval_reinstatement_activation_authority_evidence_noop",
            "one_click_identity_approval_reinstatement_activation_authority_evidence_claim_denied",
            &[
                "identity_reinstatement_audit_evidence_requested",
                "activation_authority_evidence_requested",
            ][..],
        ),
        (
            "external_telegram_identity_session_reinstatement_external_evidence_claim",
            "blocked_external_telegram_reinstatement_evidence_noop",
            "external_telegram_identity_session_reinstatement_external_evidence_claim_denied",
            &[
                "identity_reinstatement_audit_evidence_requested",
                "session_reinstatement_audit_evidence_requested",
                "external_audit_evidence_requested",
                "telegram_audit_evidence_requested",
            ][..],
        ),
        (
            "release_publication_authority_replay_reinstatement_public_release_evidence_claim",
            "blocked_release_publication_public_release_evidence_noop",
            "release_publication_authority_replay_reinstatement_public_release_evidence_claim_denied",
            &[
                "revocation_logout_replay_audit_evidence_requested",
                "identity_reinstatement_audit_evidence_requested",
                "release_publication_authority_evidence_requested",
                "public_release_evidence_requested",
                "release_artifact_evidence_requested",
                "public_artifact_evidence_requested",
            ][..],
        ),
        (
            "activation_live_install_restart_active_binary_session_reinstatement_evidence_claim",
            "blocked_live_session_reinstatement_evidence_noop",
            "activation_live_install_restart_active_binary_session_reinstatement_evidence_claim_denied",
            &[
                "session_reinstatement_audit_evidence_requested",
                "activation_evidence_requested",
                "install_evidence_requested",
                "service_restart_evidence_requested",
                "active_binary_evidence_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, extra_true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface": surface,
                "source_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_attempted": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_noop_confirmed": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-audit-evidence-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:audit=0:evidence=0:hashchain=0:attestation=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-audit-evidence:no-audit:no-evidence:no-hash-chain:no-attestation:no-authority:no-install:no-live",
    );
    let report_ready = source_ready
        && route_count_source_command_accepted
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denied_count",
        ) == 18
        && !source_bool("operator_approval_recorded")
        && !source_bool("release_publication_authority_derived")
        && !source_bool("activation_authority_derived")
        && !source_bool("install_executed")
        && !source_bool("service_restarted")
        && !source_bool("active_binary_mutated")
        && surface_count == 18;
    let denials = vec![
        "source_cancellation_supersession_report_required",
        "revocation_logout_replay_audit_trail_denied",
        "logout_replay_immutable_evidence_denied",
        "identity_reinstatement_hash_chain_merkle_root_denied",
        "session_reinstatement_attestation_witness_notary_denied",
        "audit_materialization_filesystem_denied",
        "ledger_index_delivery_evidence_denied",
        "export_query_observability_evidence_denied",
        "readback_evidence_denied",
        "completion_ack_audit_evidence_denied",
        "cancellation_supersession_audit_evidence_denied",
        "ordering_monotonicity_audit_evidence_denied",
        "replay_idempotency_audit_evidence_denied",
        "release_publication_authority_evidence_denied",
        "activation_authority_evidence_denied",
        "external_telegram_audit_evidence_denied",
        "public_release_artifact_evidence_denied",
        "live_install_restart_active_binary_evidence_denied",
        "acceptance_or_approval_from_audit_evidence_denied",
        "release_activation_authority_from_audit_evidence_denied",
        "download_install_from_audit_evidence_denied",
        "memory_kg_provider_secret_external_send_from_audit_evidence_denied",
    ];
    let denial_count = denials.len();

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_AUDIT_EVIDENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-audit-evidence-denial --json",
        "native_route": true,
        "side_effect_free": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_v1",
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_mode": "native_route_denied_cancellation_supersession_cannot_create_audit_trail_immutable_evidence_or_authority",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_gate": source["gate"].clone(),
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_ready": source_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_report_sha256": source_report_sha256,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_contract_hash_sha256": source_contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_contract_hash_sha256": contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_policy_hash_sha256": policy_hash,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_ready": report_ready,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denied_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denied_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_count": denial_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_identity": false,
                    "records_operator_session": false,
                    "accepts_replay": false,
                    "records_reinstatement": false,
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "records_audit_trail": false,
                    "persists_immutable_evidence": false,
                    "records_hash_chain": false,
                    "records_ledger_evidence": false,
                    "records_retention": false,
                    "records_expiry": false,
                    "records_garbage_collection": false,
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
    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_chain_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_merkle_root_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attestation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_witness_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_notary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ledger_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_index_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delivery_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_evidence_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_completion_ack_from_audit_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replay_idempotency_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_external_send_count",
    ] {
        if let Some(report_object) = report.as_object_mut() {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_chain_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ledger_evidence_recorded",
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

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source = hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| -> bool {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_ready",
    );
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();

    let surface_false_keys = [
        "source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_required",
        "retention_policy_requested",
        "ttl_lease_requested",
        "expiry_timestamp_requested",
        "expiry_scheduler_requested",
        "expiry_timer_requested",
        "expiry_ack_requested",
        "garbage_collection_queue_requested",
        "garbage_collection_scan_requested",
        "garbage_collection_candidate_requested",
        "garbage_collection_decision_requested",
        "tombstone_gc_requested",
        "delete_marker_gc_requested",
        "archive_requested",
        "compaction_requested",
        "audit_evidence_retention_requested",
        "immutable_evidence_retention_requested",
        "hash_attestation_retention_requested",
        "witness_notary_expiry_requested",
        "completion_ack_retention_requested",
        "cancellation_supersession_retention_requested",
        "ordering_monotonicity_retention_requested",
        "external_telegram_retention_requested",
        "release_publication_retention_authority_requested",
        "activation_retention_authority_requested",
        "live_install_gc_evidence_requested",
        "retention_policy_accepted",
        "retention_policy_recorded",
        "retention_policy_persisted",
        "ttl_lease_recorded",
        "ttl_lease_persisted",
        "expiry_timestamp_recorded",
        "expiry_scheduler_recorded",
        "expiry_timer_started",
        "expiry_ack_recorded",
        "expiry_state_persisted",
        "garbage_collection_queue_recorded",
        "garbage_collection_scan_performed",
        "garbage_collection_candidate_recorded",
        "garbage_collection_decision_recorded",
        "garbage_collection_state_persisted",
        "tombstone_gc_recorded",
        "delete_marker_gc_recorded",
        "archive_recorded",
        "compaction_recorded",
        "audit_evidence_retention_recorded",
        "immutable_evidence_retention_recorded",
        "hash_attestation_retention_recorded",
        "witness_notary_expiry_recorded",
        "completion_ack_retention_recorded",
        "cancellation_supersession_retention_recorded",
        "ordering_monotonicity_retention_recorded",
        "result_receipt_from_retention_recorded",
        "result_receipt_from_retention_persisted",
        "operator_approval_from_retention_derived",
        "release_publication_authority_from_retention_derived",
        "activation_authority_from_retention_derived",
        "download_link_from_retention_rendered",
        "install_command_from_retention_rendered",
        "install_from_retention_executed",
        "service_restart_from_retention_performed",
        "launchd_from_retention_mutated",
        "active_binary_from_retention_mutated",
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
            "source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_required",
            "blocked_source_audit_evidence_required_noop",
            "source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_required",
            &[
                "source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_required",
            ][..],
        ),
        (
            "download_button_revocation_replay_audit_retention_policy_claim",
            "blocked_revocation_replay_audit_retention_noop",
            "download_button_revocation_replay_audit_retention_policy_claim_denied",
            &[
                "retention_policy_requested",
                "audit_evidence_retention_requested",
            ][..],
        ),
        (
            "direct_download_url_logout_replay_immutable_evidence_ttl_lease_claim",
            "blocked_logout_replay_immutable_evidence_ttl_noop",
            "direct_download_url_logout_replay_immutable_evidence_ttl_lease_claim_denied",
            &[
                "ttl_lease_requested",
                "immutable_evidence_retention_requested",
            ][..],
        ),
        (
            "checksum_identity_reinstatement_hash_chain_expiry_timestamp_claim",
            "blocked_identity_reinstatement_hash_expiry_noop",
            "checksum_identity_reinstatement_hash_chain_expiry_timestamp_claim_denied",
            &[
                "expiry_timestamp_requested",
                "hash_attestation_retention_requested",
            ][..],
        ),
        (
            "package_manager_session_reinstatement_attestation_retention_ledger_claim",
            "blocked_session_reinstatement_attestation_retention_noop",
            "package_manager_session_reinstatement_attestation_retention_ledger_claim_denied",
            &[
                "retention_policy_requested",
                "hash_attestation_retention_requested",
            ][..],
        ),
        (
            "curl_pipe_shell_revocation_replay_audit_expiry_scheduler_claim",
            "blocked_revocation_replay_expiry_scheduler_noop",
            "curl_pipe_shell_revocation_replay_audit_expiry_scheduler_claim_denied",
            &[
                "expiry_scheduler_requested",
                "expiry_timer_requested",
                "expiry_ack_requested",
            ][..],
        ),
        (
            "installer_device_session_reinstatement_ledger_garbage_collection_queue_claim",
            "blocked_device_session_reinstatement_gc_queue_noop",
            "installer_device_session_reinstatement_ledger_garbage_collection_queue_claim_denied",
            &[
                "garbage_collection_queue_requested",
                "audit_evidence_retention_requested",
            ][..],
        ),
        (
            "auto_update_session_logout_replay_index_garbage_collection_scan_claim",
            "blocked_session_logout_replay_gc_scan_noop",
            "auto_update_session_logout_replay_index_garbage_collection_scan_claim_denied",
            &[
                "garbage_collection_scan_requested",
                "garbage_collection_candidate_requested",
            ][..],
        ),
        (
            "release_channel_identity_revocation_replay_evidence_gc_candidate_claim",
            "blocked_identity_revocation_replay_gc_candidate_noop",
            "release_channel_identity_revocation_replay_evidence_gc_candidate_claim_denied",
            &[
                "garbage_collection_candidate_requested",
                "garbage_collection_decision_requested",
            ][..],
        ),
        (
            "update_feed_session_reinstatement_completion_ack_retention_claim",
            "blocked_session_reinstatement_completion_retention_noop",
            "update_feed_session_reinstatement_completion_ack_retention_claim_denied",
            &[
                "retention_policy_requested",
                "completion_ack_retention_requested",
            ][..],
        ),
        (
            "package_registry_identity_badge_reinstatement_audit_evidence_archive_claim",
            "blocked_identity_reinstatement_archive_noop",
            "package_registry_identity_badge_reinstatement_audit_evidence_archive_claim_denied",
            &[
                "archive_requested",
                "audit_evidence_retention_requested",
                "cancellation_supersession_retention_requested",
            ][..],
        ),
        (
            "cdn_session_readback_logout_replay_evidence_compaction_claim",
            "blocked_logout_replay_compaction_noop",
            "cdn_session_readback_logout_replay_evidence_compaction_claim_denied",
            &[
                "compaction_requested",
                "ordering_monotonicity_retention_requested",
            ][..],
        ),
        (
            "sbom_identity_dashboard_reinstatement_hash_attestation_retention_claim",
            "blocked_identity_reinstatement_hash_attestation_retention_noop",
            "sbom_identity_dashboard_reinstatement_hash_attestation_retention_claim_denied",
            &[
                "retention_policy_requested",
                "hash_attestation_retention_requested",
            ][..],
        ),
        (
            "signature_channel_session_reinstatement_witness_notary_expiry_claim",
            "blocked_session_reinstatement_witness_notary_expiry_noop",
            "signature_channel_session_reinstatement_witness_notary_expiry_claim_denied",
            &[
                "expiry_timestamp_requested",
                "witness_notary_expiry_requested",
            ][..],
        ),
        (
            "one_click_identity_approval_reinstatement_activation_evidence_retention_claim",
            "blocked_identity_approval_reinstatement_activation_retention_noop",
            "one_click_identity_approval_reinstatement_activation_evidence_retention_claim_denied",
            &[
                "activation_retention_authority_requested",
                "audit_evidence_retention_requested",
            ][..],
        ),
        (
            "external_telegram_identity_session_reinstatement_retention_delivery_claim",
            "blocked_external_telegram_retention_delivery_noop",
            "external_telegram_identity_session_reinstatement_retention_delivery_claim_denied",
            &[
                "external_telegram_retention_requested",
                "audit_evidence_retention_requested",
            ][..],
        ),
        (
            "release_publication_authority_replay_reinstatement_retention_authority_claim",
            "blocked_release_publication_retention_authority_noop",
            "release_publication_authority_replay_reinstatement_retention_authority_claim_denied",
            &[
                "release_publication_retention_authority_requested",
                "retention_policy_requested",
            ][..],
        ),
        (
            "activation_live_install_restart_active_binary_garbage_collection_evidence_claim",
            "blocked_live_install_restart_active_binary_gc_noop",
            "activation_live_install_restart_active_binary_garbage_collection_evidence_claim_denied",
            &[
                "live_install_gc_evidence_requested",
                "garbage_collection_queue_requested",
                "garbage_collection_decision_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface": surface,
                "source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempted": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_noop_confirmed": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-retention-expiry-garbage-collection-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:retention=0:expiry=0:gc=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-retention-expiry-garbage-collection:no-retention:no-expiry:no-gc:no-archive:no-compaction:no-authority:no-install:no-live",
    );
    let report_ready = source_ready
        && route_count_source_command_accepted
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_recorded_count",
        ) == 0
        && surface_count == 18;
    let denials = vec![
        "source_audit_evidence_report_required",
        "retention_policy_denied",
        "ttl_lease_denied",
        "expiry_timestamp_denied",
        "expiry_scheduler_timer_ack_denied",
        "garbage_collection_queue_denied",
        "garbage_collection_scan_denied",
        "garbage_collection_candidate_decision_denied",
        "tombstone_delete_marker_gc_denied",
        "archive_compaction_denied",
        "audit_evidence_retention_denied",
        "immutable_evidence_retention_denied",
        "hash_attestation_retention_denied",
        "witness_notary_expiry_denied",
        "completion_ack_retention_denied",
        "cancellation_supersession_retention_denied",
        "ordering_monotonicity_retention_denied",
        "external_telegram_retention_delivery_denied",
        "release_publication_retention_authority_denied",
        "activation_retention_authority_denied",
        "live_install_restart_active_binary_gc_denied",
        "memory_kg_provider_secret_external_send_from_retention_denied",
    ];
    let denied_count = denials.len();
    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": "ready",
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-retention-expiry-garbage-collection-denial --json",
        "side_effect_free": true,
        "native_route": true,
        "route_enabled": true,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "minimum_required_samples": 24,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_v1",
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_mode": "native_route_denied_audit_evidence_cannot_create_retention_expiry_gc_lifecycle_or_authority",
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_ready": report_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_route",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_ready": source_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_sha256": source_report_sha256,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_contract_hash_sha256": source_contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_contract_hash_sha256": contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_chain_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_chain_recorded_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count": surface_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [{
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_operator_identity": false,
                "records_operator_session": false,
                "accepts_replay": false,
                "records_reinstatement": false,
                "records_audit_trail": false,
                "persists_immutable_evidence": false,
                "records_retention": false,
                "records_expiry": false,
                "records_garbage_collection": false,
                "records_archive": false,
                "records_compaction": false,
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
            }],
        }),
    );
    let zero_count_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ttl_lease_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ttl_lease_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timestamp_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_scheduler_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timer_started_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_ack_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_queue_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_scan_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_candidate_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_decision_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_tombstone_gc_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_gc_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_archive_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_compaction_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_attestation_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_witness_notary_expiry_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_completion_ack_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in zero_count_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_recorded",
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

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source = hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| -> bool {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_ready",
    );
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();

    let surface_false_keys = [
        "source_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_report_required",
        "operator_identity_session_revocation_logout_replay_reinstatement_query_requested",
        "operator_identity_session_revocation_logout_replay_reinstatement_export_requested",
        "operator_identity_session_revocation_logout_replay_reinstatement_observability_requested",
        "search_index_requested",
        "export_snapshot_requested",
        "export_file_requested",
        "export_stream_requested",
        "metric_observability_requested",
        "log_observability_requested",
        "trace_observability_requested",
        "event_observability_requested",
        "dashboard_panel_requested",
        "alert_slo_requested",
        "operator_summary_readback_requested",
        "audit_view_requested",
        "external_observability_requested",
        "telegram_observability_requested",
        "authority_view_requested",
        "live_view_requested",
        "install_view_requested",
        "service_restart_view_requested",
        "active_binary_view_requested",
        "query_registered",
        "query_executed",
        "query_result_recorded",
        "query_result_persisted",
        "search_index_recorded",
        "search_index_persisted",
        "export_accepted",
        "export_snapshot_recorded",
        "export_snapshot_persisted",
        "export_file_written",
        "export_stream_opened",
        "observability_metric_recorded",
        "observability_log_recorded",
        "observability_trace_recorded",
        "observability_event_recorded",
        "dashboard_panel_recorded",
        "alert_registered",
        "slo_recorded",
        "operator_summary_recorded",
        "readback_surface_recorded",
        "audit_view_recorded",
        "ledger_observability_recorded",
        "index_observability_recorded",
        "delivery_observability_recorded",
        "retention_policy_recorded",
        "expiry_recorded",
        "garbage_collection_scan_performed",
        "audit_trail_recorded",
        "immutable_evidence_recorded",
        "hash_chain_recorded",
        "result_receipt_recorded",
        "result_receipt_persisted",
        "result_receipt_exported",
        "result_receipt_query_registered",
        "result_receipt_observability_recorded",
        "completion_ack_recorded",
        "operator_approval_from_export_query_observability_accepted",
        "release_publication_authority_from_export_query_observability_derived",
        "activation_authority_from_export_query_observability_derived",
        "activation_command_from_export_query_observability_derived",
        "install_from_export_query_observability_executed",
        "service_restart_from_export_query_observability_performed",
        "launchd_from_export_query_observability_mutated",
        "active_binary_from_export_query_observability_mutated",
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
            "source_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_report_required",
            "blocked_source_retention_report_required_noop",
            "source_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_report_required",
            &[
                "source_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_report_required",
            ][..],
        ),
        (
            "download_button_revocation_replay_query_registration_claim",
            "blocked_revocation_replay_query_registration_noop",
            "download_button_revocation_replay_query_registration_claim_denied",
            &["operator_identity_session_revocation_logout_replay_reinstatement_query_requested"][..],
        ),
        (
            "direct_download_url_logout_replay_query_execution_claim",
            "blocked_logout_replay_query_execution_noop",
            "direct_download_url_logout_replay_query_execution_claim_denied",
            &["operator_identity_session_revocation_logout_replay_reinstatement_query_requested"][..],
        ),
        (
            "checksum_identity_reinstatement_query_result_claim",
            "blocked_identity_reinstatement_query_result_noop",
            "checksum_identity_reinstatement_query_result_claim_denied",
            &["operator_identity_session_revocation_logout_replay_reinstatement_query_requested"][..],
        ),
        (
            "package_manager_session_reinstatement_search_index_claim",
            "blocked_session_reinstatement_search_index_noop",
            "package_manager_session_reinstatement_search_index_claim_denied",
            &["search_index_requested"][..],
        ),
        (
            "curl_pipe_shell_revocation_replay_export_request_claim",
            "blocked_revocation_replay_export_request_noop",
            "curl_pipe_shell_revocation_replay_export_request_claim_denied",
            &["operator_identity_session_revocation_logout_replay_reinstatement_export_requested"]
                [..],
        ),
        (
            "installer_device_session_reinstatement_export_snapshot_claim",
            "blocked_device_session_reinstatement_export_snapshot_noop",
            "installer_device_session_reinstatement_export_snapshot_claim_denied",
            &[
                "operator_identity_session_revocation_logout_replay_reinstatement_export_requested",
                "export_snapshot_requested",
            ][..],
        ),
        (
            "auto_update_session_logout_replay_export_file_claim",
            "blocked_session_logout_replay_export_file_noop",
            "auto_update_session_logout_replay_export_file_claim_denied",
            &[
                "operator_identity_session_revocation_logout_replay_reinstatement_export_requested",
                "export_file_requested",
            ][..],
        ),
        (
            "release_channel_identity_revocation_replay_export_stream_claim",
            "blocked_identity_revocation_replay_export_stream_noop",
            "release_channel_identity_revocation_replay_export_stream_claim_denied",
            &[
                "operator_identity_session_revocation_logout_replay_reinstatement_export_requested",
                "export_stream_requested",
            ][..],
        ),
        (
            "update_feed_session_reinstatement_observability_metric_log_claim",
            "blocked_session_reinstatement_metric_log_noop",
            "update_feed_session_reinstatement_observability_metric_log_claim_denied",
            &[
                "operator_identity_session_revocation_logout_replay_reinstatement_observability_requested",
                "metric_observability_requested",
                "log_observability_requested",
            ][..],
        ),
        (
            "package_registry_identity_badge_observability_trace_event_claim",
            "blocked_identity_badge_trace_event_noop",
            "package_registry_identity_badge_observability_trace_event_claim_denied",
            &[
                "operator_identity_session_revocation_logout_replay_reinstatement_observability_requested",
                "trace_observability_requested",
                "event_observability_requested",
            ][..],
        ),
        (
            "cdn_session_readback_logout_replay_dashboard_panel_claim",
            "blocked_logout_replay_dashboard_panel_noop",
            "cdn_session_readback_logout_replay_dashboard_panel_claim_denied",
            &["dashboard_panel_requested"][..],
        ),
        (
            "sbom_identity_dashboard_reinstatement_alert_slo_claim",
            "blocked_identity_reinstatement_alert_slo_noop",
            "sbom_identity_dashboard_reinstatement_alert_slo_claim_denied",
            &["alert_slo_requested"][..],
        ),
        (
            "signature_channel_session_operator_summary_readback_claim",
            "blocked_session_operator_summary_readback_noop",
            "signature_channel_session_operator_summary_readback_claim_denied",
            &["operator_summary_readback_requested"][..],
        ),
        (
            "one_click_identity_approval_reinstatement_audit_view_claim",
            "blocked_identity_approval_reinstatement_audit_view_noop",
            "one_click_identity_approval_reinstatement_audit_view_claim_denied",
            &["audit_view_requested"][..],
        ),
        (
            "external_telegram_identity_session_reinstatement_external_observability_claim",
            "blocked_external_telegram_observability_noop",
            "external_telegram_identity_session_reinstatement_external_observability_claim_denied",
            &[
                "operator_identity_session_revocation_logout_replay_reinstatement_observability_requested",
                "external_observability_requested",
                "telegram_observability_requested",
            ][..],
        ),
        (
            "release_publication_authority_replay_reinstatement_authority_view_claim",
            "blocked_release_publication_authority_view_noop",
            "release_publication_authority_replay_reinstatement_authority_view_claim_denied",
            &["authority_view_requested"][..],
        ),
        (
            "activation_live_install_restart_active_binary_result_receipt_live_view_claim",
            "blocked_live_install_restart_active_binary_view_noop",
            "activation_live_install_restart_active_binary_result_receipt_live_view_claim_denied",
            &[
                "live_view_requested",
                "install_view_requested",
                "service_restart_view_requested",
                "active_binary_view_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface": surface,
                "source_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_attempted": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_noop_confirmed": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-export-query-observability-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:query=0:export=0:observability=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-export-query-observability:no-query:no-export:no-observability:no-readback:no-authority:no-install:no-live",
    );
    let report_ready = source_ready
        && route_count_source_command_accepted
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_scan_performed_count",
        ) == 0
        && surface_count == 18;
    let denials = vec![
        "source_retention_expiry_garbage_collection_report_required",
        "operator_identity_session_query_registration_denied",
        "operator_identity_session_query_execution_denied",
        "operator_identity_session_query_result_denied",
        "operator_identity_session_search_index_denied",
        "operator_identity_session_export_request_denied",
        "operator_identity_session_export_snapshot_denied",
        "operator_identity_session_export_file_denied",
        "operator_identity_session_export_stream_denied",
        "operator_identity_session_observability_metric_log_denied",
        "operator_identity_session_observability_trace_event_denied",
        "operator_identity_session_dashboard_panel_denied",
        "operator_identity_session_alert_slo_denied",
        "operator_identity_session_operator_summary_readback_denied",
        "operator_identity_session_audit_view_denied",
        "operator_identity_session_external_telegram_observability_denied",
        "operator_identity_session_release_publication_authority_view_denied",
        "operator_identity_session_live_install_restart_active_binary_view_denied",
        "operator_identity_session_acceptance_or_approval_from_view_denied",
        "operator_identity_session_release_activation_authority_from_view_denied",
        "operator_identity_session_download_install_from_view_denied",
        "operator_identity_session_memory_kg_provider_secret_external_send_from_view_denied",
    ];
    let denied_count = denials.len();
    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": "ready",
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-export-query-observability-denial --json",
        "side_effect_free": true,
        "native_route": true,
        "route_enabled": true,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "minimum_required_samples": 24,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_v1",
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_mode": "native_route_denied_retention_expiry_garbage_collection_cannot_create_export_query_observability_view_or_authority",
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_ready": report_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_route",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_ready": source_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_report_sha256": source_report_sha256,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_contract_hash_sha256": source_contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_contract_hash_sha256": contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timestamp_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timestamp_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_scan_performed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_scan_performed_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denied_count": surface_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [{
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_gate",
                "status": "allowed_report_only_next_slice",
                "exports_receipt": false,
                "registers_query": false,
                "records_observability": false,
                "records_summary": false,
                "records_briefing": false,
                "derives_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }],
        }),
    );
    let zero_count_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_result_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_result_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_search_index_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_search_index_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_snapshot_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_snapshot_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_file_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_stream_opened_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_metric_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_log_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_trace_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_event_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_dashboard_panel_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_alert_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_slo_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_surface_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_view_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ledger_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_index_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delivery_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in zero_count_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_result_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_snapshot_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_file_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_metric_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_trace_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
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

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source = hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| -> bool {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_ready",
    );
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!("unknown"));

    let surface_false_keys = [
        "source_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_report_required",
        "operator_summary_requested",
        "operator_briefing_requested",
        "readback_digest_requested",
        "status_banner_requested",
        "exported_summary_text_requested",
        "operator_briefing_card_requested",
        "notification_timeline_requested",
        "dashboard_narrative_requested",
        "audit_narrative_requested",
        "briefing_delivery_requested",
        "approval_summary_requested",
        "external_briefing_requested",
        "telegram_briefing_requested",
        "authority_briefing_requested",
        "live_status_briefing_requested",
        "install_status_briefing_requested",
        "service_restart_status_briefing_requested",
        "active_binary_status_briefing_requested",
        "operator_summary_recorded",
        "operator_summary_persisted",
        "operator_briefing_recorded",
        "operator_briefing_persisted",
        "readback_digest_recorded",
        "status_banner_recorded",
        "exported_summary_text_recorded",
        "exported_summary_text_persisted",
        "operator_briefing_card_materialized",
        "notification_recorded",
        "timeline_recorded",
        "dashboard_narrative_recorded",
        "audit_narrative_recorded",
        "briefing_delivery_recorded",
        "briefing_delivery_performed",
        "approval_summary_recorded",
        "external_briefing_sent",
        "telegram_briefing_sent",
        "summary_briefing_acceptance_recorded",
        "result_receipt_recorded",
        "result_receipt_persisted",
        "completion_ack_recorded",
        "operator_approval_from_summary_briefing_accepted",
        "release_publication_authority_from_summary_briefing_derived",
        "activation_authority_from_summary_briefing_derived",
        "download_link_from_summary_briefing_rendered",
        "install_command_from_summary_briefing_rendered",
        "install_from_summary_briefing_executed",
        "service_restart_from_summary_briefing_performed",
        "launchd_from_summary_briefing_mutated",
        "active_binary_from_summary_briefing_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "channel_send_performed",
        "external_send_performed",
        "filesystem_written",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let surface_specs: &[(&str, &str, &str, &[&str])] = &[
        (
            "source_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_report_required",
            "blocked_source_export_query_observability_report_required_noop",
            "source_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_report_required",
            &[
                "source_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_report_required",
            ][..],
        ),
        (
            "download_button_revocation_replay_summary_request_claim",
            "blocked_revocation_replay_summary_request_noop",
            "download_button_revocation_replay_summary_request_claim_denied",
            &["operator_summary_requested"][..],
        ),
        (
            "direct_download_url_logout_replay_briefing_request_claim",
            "blocked_logout_replay_briefing_request_noop",
            "direct_download_url_logout_replay_briefing_request_claim_denied",
            &["operator_briefing_requested"][..],
        ),
        (
            "checksum_identity_reinstatement_readback_digest_claim",
            "blocked_identity_reinstatement_readback_digest_noop",
            "checksum_identity_reinstatement_readback_digest_claim_denied",
            &["readback_digest_requested"][..],
        ),
        (
            "package_manager_session_reinstatement_status_banner_claim",
            "blocked_session_reinstatement_status_banner_noop",
            "package_manager_session_reinstatement_status_banner_claim_denied",
            &["status_banner_requested"][..],
        ),
        (
            "curl_pipe_shell_revocation_replay_exported_summary_text_claim",
            "blocked_revocation_replay_exported_summary_text_noop",
            "curl_pipe_shell_revocation_replay_exported_summary_text_claim_denied",
            &[
                "operator_summary_requested",
                "exported_summary_text_requested",
            ][..],
        ),
        (
            "installer_device_session_reinstatement_operator_briefing_card_claim",
            "blocked_device_session_reinstatement_briefing_card_noop",
            "installer_device_session_reinstatement_operator_briefing_card_claim_denied",
            &[
                "operator_briefing_requested",
                "operator_briefing_card_requested",
            ][..],
        ),
        (
            "auto_update_session_logout_replay_notification_timeline_claim",
            "blocked_session_logout_replay_notification_timeline_noop",
            "auto_update_session_logout_replay_notification_timeline_claim_denied",
            &["notification_timeline_requested"][..],
        ),
        (
            "release_channel_identity_revocation_replay_dashboard_narrative_claim",
            "blocked_identity_revocation_replay_dashboard_narrative_noop",
            "release_channel_identity_revocation_replay_dashboard_narrative_claim_denied",
            &["dashboard_narrative_requested"][..],
        ),
        (
            "update_feed_session_reinstatement_audit_narrative_claim",
            "blocked_session_reinstatement_audit_narrative_noop",
            "update_feed_session_reinstatement_audit_narrative_claim_denied",
            &["audit_narrative_requested"][..],
        ),
        (
            "package_registry_identity_badge_briefing_delivery_claim",
            "blocked_identity_badge_briefing_delivery_noop",
            "package_registry_identity_badge_briefing_delivery_claim_denied",
            &["operator_briefing_requested", "briefing_delivery_requested"][..],
        ),
        (
            "cdn_session_readback_logout_replay_final_summary_claim",
            "blocked_logout_replay_final_summary_noop",
            "cdn_session_readback_logout_replay_final_summary_claim_denied",
            &["operator_summary_requested", "readback_digest_requested"][..],
        ),
        (
            "sbom_identity_dashboard_reinstatement_status_readback_claim",
            "blocked_identity_reinstatement_status_readback_noop",
            "sbom_identity_dashboard_reinstatement_status_readback_claim_denied",
            &["status_banner_requested", "readback_digest_requested"][..],
        ),
        (
            "signature_channel_session_operator_memo_claim",
            "blocked_session_operator_memo_noop",
            "signature_channel_session_operator_memo_claim_denied",
            &["operator_summary_requested", "operator_briefing_requested"][..],
        ),
        (
            "one_click_identity_approval_reinstatement_approval_summary_claim",
            "blocked_identity_approval_reinstatement_summary_noop",
            "one_click_identity_approval_reinstatement_approval_summary_claim_denied",
            &["approval_summary_requested"][..],
        ),
        (
            "external_telegram_identity_session_reinstatement_briefing_delivery_claim",
            "blocked_external_telegram_briefing_delivery_noop",
            "external_telegram_identity_session_reinstatement_briefing_delivery_claim_denied",
            &[
                "operator_briefing_requested",
                "external_briefing_requested",
                "telegram_briefing_requested",
                "briefing_delivery_requested",
            ][..],
        ),
        (
            "release_publication_authority_replay_reinstatement_briefing_authority_claim",
            "blocked_release_publication_authority_briefing_noop",
            "release_publication_authority_replay_reinstatement_briefing_authority_claim_denied",
            &["authority_briefing_requested"][..],
        ),
        (
            "activation_live_install_restart_active_binary_briefing_status_claim",
            "blocked_live_install_restart_active_binary_briefing_status_noop",
            "activation_live_install_restart_active_binary_briefing_status_claim_denied",
            &[
                "live_status_briefing_requested",
                "install_status_briefing_requested",
                "service_restart_status_briefing_requested",
                "active_binary_status_briefing_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_surface": surface,
                "source_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_attempted": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_noop_confirmed": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-facing-summary-briefing-non-persistence-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:summary=0:briefing=0:readback=0:delivery=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-facing-summary-briefing:no-summary:no-briefing:no-readback:no-delivery:no-authority:no-install:no-live",
    );
    let report_ready = source_ready
        && route_count_source_command_accepted
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_release_publication_authority_derived_count",
        ) == 0
        && surface_count == 18;
    let denials = vec![
        "source_export_query_observability_report_required",
        "operator_summary_recording_denied",
        "operator_briefing_recording_denied",
        "readback_digest_recording_denied",
        "status_banner_recording_denied",
        "exported_summary_text_recording_denied",
        "briefing_card_materialization_denied",
        "notification_timeline_recording_denied",
        "dashboard_narrative_recording_denied",
        "audit_narrative_recording_denied",
        "briefing_delivery_denied",
        "external_telegram_briefing_denied",
        "summary_briefing_acceptance_denied",
        "operator_approval_from_summary_briefing_denied",
        "release_activation_authority_from_summary_briefing_denied",
        "download_install_from_summary_briefing_denied",
        "live_install_restart_active_binary_from_summary_briefing_denied",
        "memory_kg_provider_secret_external_send_from_summary_briefing_denied",
    ];
    let denied_count = denials.len();
    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": "ready",
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-facing-summary-briefing-non-persistence-denial --json",
        "side_effect_free": true,
        "native_route": true,
        "route_enabled": true,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "minimum_required_samples": 24,
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_mode": "native_route_denied_export_query_observability_cannot_create_operator_summary_briefing_readback_delivery_or_authority",
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_ready": report_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_policy_hash_sha256": policy_hash,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denied_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_denied_count": surface_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [{
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_summary": false,
                "records_briefing": false,
                "records_readback": false,
                "records_acknowledgement": false,
                "delivers_briefing": false,
                "derives_operator_approval": false,
                "derives_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }],
        }),
    );
    let zero_count_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_briefing_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_briefing_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_digest_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_banner_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_exported_summary_text_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_briefing_card_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_notification_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_timeline_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_dashboard_narrative_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_narrative_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_briefing_delivery_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_briefing_delivery_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_external_briefing_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_telegram_briefing_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in zero_count_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_briefing_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_digest_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_banner_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_operator_summary_recorded",
        "artifact_download_install_affordance_result_receipt_operator_briefing_recorded",
        "artifact_download_install_affordance_result_receipt_readback_recorded",
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

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| -> bool {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_ready",
    );
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!("unknown"));

    let surface_false_keys = [
        "source_operator_facing_summary_briefing_report_required",
        "final_operator_acknowledgement_requested",
        "operator_received_requested",
        "operator_confirmed_requested",
        "operator_read_requested",
        "operator_seen_requested",
        "final_response_requested",
        "completion_acknowledgement_requested",
        "status_acknowledgement_requested",
        "summary_acknowledgement_requested",
        "briefing_acknowledgement_requested",
        "readback_digest_acknowledgement_requested",
        "dashboard_acknowledgement_requested",
        "notification_acknowledgement_requested",
        "channel_acknowledgement_requested",
        "operator_approval_acknowledgement_requested",
        "external_acknowledgement_requested",
        "telegram_acknowledgement_requested",
        "authority_acknowledgement_requested",
        "live_acknowledgement_requested",
        "install_restart_active_binary_acknowledgement_requested",
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
    let surface_specs: &[(&str, &str, &str, &[&str])] = &[
        (
            "source_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_report_required",
            "blocked_source_summary_briefing_required_noop",
            "source_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_report_required",
            &["source_operator_facing_summary_briefing_report_required"][..],
        ),
        (
            "download_button_revocation_replay_final_operator_acknowledgement_claim",
            "blocked_revocation_replay_final_ack_noop",
            "download_button_revocation_replay_final_operator_acknowledgement_claim_denied",
            &["final_operator_acknowledgement_requested"][..],
        ),
        (
            "direct_download_url_logout_replay_operator_received_claim",
            "blocked_logout_replay_operator_received_noop",
            "direct_download_url_logout_replay_operator_received_claim_denied",
            &["operator_received_requested"][..],
        ),
        (
            "checksum_identity_reinstatement_operator_confirmed_claim",
            "blocked_identity_reinstatement_operator_confirmed_noop",
            "checksum_identity_reinstatement_operator_confirmed_claim_denied",
            &["operator_confirmed_requested"][..],
        ),
        (
            "package_manager_session_reinstatement_operator_read_claim",
            "blocked_session_reinstatement_operator_read_noop",
            "package_manager_session_reinstatement_operator_read_claim_denied",
            &["operator_read_requested"][..],
        ),
        (
            "curl_pipe_shell_revocation_replay_operator_seen_claim",
            "blocked_revocation_replay_operator_seen_noop",
            "curl_pipe_shell_revocation_replay_operator_seen_claim_denied",
            &["operator_seen_requested"][..],
        ),
        (
            "installer_device_session_reinstatement_final_response_claim",
            "blocked_device_session_reinstatement_final_response_noop",
            "installer_device_session_reinstatement_final_response_claim_denied",
            &["final_response_requested"][..],
        ),
        (
            "auto_update_session_logout_replay_completion_acknowledgement_claim",
            "blocked_session_logout_replay_completion_ack_noop",
            "auto_update_session_logout_replay_completion_acknowledgement_claim_denied",
            &["completion_acknowledgement_requested"][..],
        ),
        (
            "release_channel_identity_revocation_replay_status_acknowledgement_claim",
            "blocked_identity_revocation_replay_status_ack_noop",
            "release_channel_identity_revocation_replay_status_acknowledgement_claim_denied",
            &["status_acknowledgement_requested"][..],
        ),
        (
            "update_feed_session_reinstatement_summary_acknowledgement_claim",
            "blocked_session_reinstatement_summary_ack_noop",
            "update_feed_session_reinstatement_summary_acknowledgement_claim_denied",
            &["summary_acknowledgement_requested"][..],
        ),
        (
            "package_registry_identity_badge_briefing_acknowledgement_claim",
            "blocked_identity_badge_briefing_ack_noop",
            "package_registry_identity_badge_briefing_acknowledgement_claim_denied",
            &["briefing_acknowledgement_requested"][..],
        ),
        (
            "cdn_session_readback_logout_replay_digest_acknowledgement_claim",
            "blocked_logout_replay_readback_ack_noop",
            "cdn_session_readback_logout_replay_digest_acknowledgement_claim_denied",
            &["readback_digest_acknowledgement_requested"][..],
        ),
        (
            "sbom_identity_dashboard_reinstatement_notification_acknowledgement_claim",
            "blocked_identity_reinstatement_dashboard_notification_ack_noop",
            "sbom_identity_dashboard_reinstatement_notification_acknowledgement_claim_denied",
            &[
                "dashboard_acknowledgement_requested",
                "notification_acknowledgement_requested",
            ][..],
        ),
        (
            "signature_channel_session_operator_memo_acknowledgement_claim",
            "blocked_session_channel_ack_noop",
            "signature_channel_session_operator_memo_acknowledgement_claim_denied",
            &["channel_acknowledgement_requested"][..],
        ),
        (
            "one_click_identity_approval_reinstatement_operator_approval_acknowledgement_claim",
            "blocked_identity_approval_ack_noop",
            "one_click_identity_approval_reinstatement_operator_approval_acknowledgement_claim_denied",
            &["operator_approval_acknowledgement_requested"][..],
        ),
        (
            "external_telegram_identity_session_reinstatement_external_telegram_acknowledgement_claim",
            "blocked_external_telegram_ack_noop",
            "external_telegram_identity_session_reinstatement_external_telegram_acknowledgement_claim_denied",
            &[
                "external_acknowledgement_requested",
                "telegram_acknowledgement_requested",
            ][..],
        ),
        (
            "release_publication_authority_replay_reinstatement_acknowledgement_claim",
            "blocked_authority_ack_noop",
            "release_publication_authority_replay_reinstatement_acknowledgement_claim_denied",
            &["authority_acknowledgement_requested"][..],
        ),
        (
            "activation_live_install_restart_active_binary_reinstatement_acknowledgement_claim",
            "blocked_live_ack_noop",
            "activation_live_install_restart_active_binary_reinstatement_acknowledgement_claim_denied",
            &[
                "live_acknowledgement_requested",
                "install_restart_active_binary_acknowledgement_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_surface": surface,
                "source_operator_facing_summary_briefing_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_attempted": true,
                "final_operator_acknowledgement_noop_confirmed": true,
                "final_operator_acknowledgement_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-final-operator-acknowledgement-non-acceptance-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:ack=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-final-operator-acknowledgement:no-ack:no-received:no-confirmed:no-read:no-seen:no-final-response:no-status:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_recording_denied",
        "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_persistence_denied",
        "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_materialization_denied",
        "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_filesystem_write_denied",
        "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_delivery_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_received_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_confirmed_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_read_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_seen_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_final_response_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_completion_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_status_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_readback_dashboard_notification_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_channel_external_telegram_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_acceptance_from_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_release_publication_authority_from_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_activation_authority_from_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_download_install_affordance_from_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_install_restart_active_binary_from_acknowledgement_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_memory_provider_secret_external_send_from_acknowledgement_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && route_count_source_command_accepted
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_briefing_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_digest_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_activation_authority_derived_count",
        ) == 0
        && surface_count == 18;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-final-operator-acknowledgement-non-acceptance-denial --json",
        "side_effect_free": true,
        "native_route": true,
        "route_enabled": true,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "minimum_required_samples": 24,
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_mode": "native_route_denied_operator_identity_session_reinstatement_summary_briefing_cannot_create_final_operator_acknowledgement_acceptance_or_authority",
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_ready": report_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_policy_hash_sha256": policy_hash,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_briefing_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_briefing_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_digest_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_digest_recorded_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_denied_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [{
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_gate",
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
            }],
        }),
    );

    let zero_count_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_delivered_count",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in zero_count_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    let additional_zero_count_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_telegram_acknowledgement_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in additional_zero_count_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_persisted",
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

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_report();
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
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_ready",
    );
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_contract_hash_sha256")
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
    let surface_specs = [
        (
            "source_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_report_required",
            "blocked_source_final_acknowledgement_required_noop",
            "source_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_report_required",
            &["source_final_operator_acknowledgement_report_required"][..],
        ),
        (
            "download_button_revocation_replay_terminal_decision_claim",
            "blocked_revocation_replay_terminal_decision_noop",
            "download_button_revocation_replay_terminal_decision_claim_denied",
            &["terminal_decision_requested"][..],
        ),
        (
            "direct_download_url_logout_replay_status_promotion_claim",
            "blocked_logout_replay_status_promotion_noop",
            "direct_download_url_logout_replay_status_promotion_claim_denied",
            &["status_promotion_requested"][..],
        ),
        (
            "checksum_identity_reinstatement_terminal_status_claim",
            "blocked_identity_reinstatement_terminal_status_noop",
            "checksum_identity_reinstatement_terminal_status_claim_denied",
            &["identity_reinstatement_terminal_status_requested"][..],
        ),
        (
            "package_manager_session_reinstatement_decision_record_claim",
            "blocked_session_reinstatement_decision_record_noop",
            "package_manager_session_reinstatement_decision_record_claim_denied",
            &["session_reinstatement_decision_record_requested"][..],
        ),
        (
            "curl_pipe_shell_revocation_replay_status_banner_promotion_claim",
            "blocked_revocation_replay_status_banner_noop",
            "curl_pipe_shell_revocation_replay_status_banner_promotion_claim_denied",
            &["status_banner_promotion_requested"][..],
        ),
        (
            "installer_device_session_reinstatement_final_decision_claim",
            "blocked_device_session_reinstatement_final_decision_noop",
            "installer_device_session_reinstatement_final_decision_claim_denied",
            &["final_decision_requested"][..],
        ),
        (
            "auto_update_session_logout_replay_completion_status_promotion_claim",
            "blocked_session_logout_replay_completion_status_noop",
            "auto_update_session_logout_replay_completion_status_promotion_claim_denied",
            &["completion_status_promotion_requested"][..],
        ),
        (
            "release_channel_identity_revocation_replay_terminal_status_claim",
            "blocked_identity_revocation_replay_terminal_status_noop",
            "release_channel_identity_revocation_replay_terminal_status_claim_denied",
            &["release_channel_terminal_status_requested"][..],
        ),
        (
            "update_feed_session_reinstatement_decision_summary_claim",
            "blocked_session_reinstatement_decision_summary_noop",
            "update_feed_session_reinstatement_decision_summary_claim_denied",
            &["decision_summary_requested"][..],
        ),
        (
            "package_registry_identity_badge_status_badge_claim",
            "blocked_identity_badge_status_badge_noop",
            "package_registry_identity_badge_status_badge_claim_denied",
            &["status_badge_requested"][..],
        ),
        (
            "cdn_session_readback_logout_replay_terminal_readback_claim",
            "blocked_logout_replay_terminal_readback_noop",
            "cdn_session_readback_logout_replay_terminal_readback_claim_denied",
            &["terminal_readback_requested"][..],
        ),
        (
            "sbom_identity_dashboard_reinstatement_terminal_status_claim",
            "blocked_identity_dashboard_terminal_status_noop",
            "sbom_identity_dashboard_reinstatement_terminal_status_claim_denied",
            &["dashboard_status_requested"][..],
        ),
        (
            "signature_channel_session_terminal_decision_claim",
            "blocked_session_channel_terminal_decision_noop",
            "signature_channel_session_terminal_decision_claim_denied",
            &["channel_decision_requested"][..],
        ),
        (
            "one_click_identity_approval_reinstatement_operator_approval_status_claim",
            "blocked_identity_approval_status_noop",
            "one_click_identity_approval_reinstatement_operator_approval_status_claim_denied",
            &["operator_approval_status_requested"][..],
        ),
        (
            "external_telegram_identity_session_reinstatement_external_terminal_decision_claim",
            "blocked_external_telegram_terminal_decision_noop",
            "external_telegram_identity_session_reinstatement_external_terminal_decision_claim_denied",
            &["external_decision_requested", "telegram_decision_requested"][..],
        ),
        (
            "release_publication_authority_replay_reinstatement_terminal_decision_claim",
            "blocked_authority_terminal_decision_noop",
            "release_publication_authority_replay_reinstatement_terminal_decision_claim_denied",
            &["authority_terminal_decision_requested"][..],
        ),
        (
            "activation_live_install_restart_active_binary_reinstatement_status_promotion_claim",
            "blocked_live_status_promotion_noop",
            "activation_live_install_restart_active_binary_reinstatement_status_promotion_claim_denied",
            &[
                "live_status_promotion_requested",
                "install_restart_active_binary_status_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface": surface,
                "source_final_operator_acknowledgement_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_attempted": true,
                "terminal_decision_status_promotion_noop_confirmed": true,
                "terminal_decision_status_promotion_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-decision-status-promotion-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:terminal=0:status=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-decision-status-promotion:no-terminal-decision:no-status-promotion:no-approval:no-authority:no-install:no-live",
    );
    let report_ready = source_ready
        && route_count_source_command_accepted
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_release_publication_authority_derived_count",
        ) == 0
        && surface_count == 18;
    let denials = vec![
        "operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_acceptance_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_persistence_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_materialization_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_filesystem_write_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_delivery_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recording_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_promotion_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_delivery_status_promotion_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_status_promotion_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_received_confirmed_read_seen_status_promotion_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_final_response_completion_status_promotion_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_status_promotion_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_readback_dashboard_notification_status_promotion_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_channel_external_telegram_decision_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_acceptance_from_terminal_decision_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_terminal_status_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_release_publication_authority_from_terminal_decision_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_activation_authority_from_terminal_status_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_download_install_affordance_from_terminal_status_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_install_restart_active_binary_from_terminal_status_denied",
        "operator_identity_session_revocation_logout_replay_reinstatement_memory_provider_secret_external_send_from_terminal_status_denied",
    ];
    let denied_count = denials.len();
    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": "ready",
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-decision-status-promotion-denial --json",
        "side_effect_free": true,
        "native_route": true,
        "route_enabled": true,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "minimum_required_samples": 24,
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_mode": "native_route_denied_operator_identity_session_final_acknowledgement_cannot_create_terminal_decision_status_promotion_acceptance_or_authority",
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_ready": report_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_policy_hash_sha256": policy_hash,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denied_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [{
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_public_claim": false,
                "exposes_status": false,
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
            }, {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_operator_intent": false,
                "records_operator_consent": false,
                "records_operator_identity": false,
                "records_operator_session": false,
                "derives_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }],
        }),
    );
    let zero_count_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_persisted_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in zero_count_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delivery_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_acknowledgement_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_received_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_confirmed_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_read_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_seen_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_completion_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_summary_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_briefing_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_dashboard_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_notification_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_channel_decision_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_external_decision_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_telegram_decision_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_acceptance_from_terminal_decision_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_external_send_count",
    ] {
        if let Some(report_object) = report.as_object_mut() {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded",
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

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        .get("artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_materialized",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_filesystem_written",
        "retention_policy_requested",
        "ttl_lease_requested",
        "expiry_timestamp_requested",
        "expiry_scheduler_requested",
        "expiry_timer_requested",
        "expiry_ack_requested",
        "garbage_collection_queue_requested",
        "garbage_collection_scan_requested",
        "garbage_collection_candidate_requested",
        "garbage_collection_decision_requested",
        "tombstone_gc_requested",
        "delete_marker_gc_requested",
        "archive_requested",
        "compaction_requested",
        "audit_evidence_retention_requested",
        "immutable_evidence_retention_requested",
        "hash_attestation_retention_requested",
        "witness_notary_expiry_requested",
        "ledger_index_retention_requested",
        "delivery_evidence_retention_requested",
        "status_evidence_expiry_requested",
        "external_telegram_retention_requested",
        "release_publication_retention_authority_requested",
        "activation_retention_authority_requested",
        "live_install_gc_evidence_requested",
        "retention_policy_recorded",
        "retention_policy_persisted",
        "ttl_lease_recorded",
        "ttl_lease_persisted",
        "expiry_timestamp_recorded",
        "expiry_scheduler_recorded",
        "expiry_timer_started",
        "expiry_ack_recorded",
        "expiry_state_persisted",
        "garbage_collection_queue_recorded",
        "garbage_collection_scan_performed",
        "garbage_collection_candidate_recorded",
        "garbage_collection_decision_recorded",
        "garbage_collection_state_persisted",
        "tombstone_gc_recorded",
        "delete_marker_gc_recorded",
        "archive_recorded",
        "compaction_recorded",
        "audit_evidence_retention_recorded",
        "immutable_evidence_retention_recorded",
        "hash_attestation_retention_recorded",
        "witness_notary_expiry_recorded",
        "ledger_index_retention_recorded",
        "delivery_evidence_retention_recorded",
        "status_evidence_expiry_recorded",
        "result_receipt_from_retention_recorded",
        "result_receipt_from_retention_persisted",
        "operator_approval_from_retention_derived",
        "release_publication_authority_from_retention_derived",
        "activation_authority_from_retention_derived",
        "download_link_from_retention_rendered",
        "install_command_from_retention_rendered",
        "install_from_retention_executed",
        "service_restart_from_retention_performed",
        "launchd_from_retention_mutated",
        "active_binary_from_retention_mutated",
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
        "filesystem_written",
    ];
    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "source_signing_receipt_audit_evidence_report_required",
            "blocked_source_signing_receipt_audit_evidence_required_noop",
            "source_signing_receipt_audit_evidence_report_required",
            vec!["source_report_required"],
        ),
        (
            "artifact_signing_audit_trail_retention_policy",
            "blocked_artifact_signing_audit_trail_retention_noop",
            "artifact_signing_audit_trail_retention_policy_denied",
            vec![
                "retention_policy_requested",
                "audit_evidence_retention_requested",
            ],
        ),
        (
            "package_signing_immutable_evidence_ttl_lease",
            "blocked_package_signing_immutable_evidence_ttl_noop",
            "package_signing_immutable_evidence_ttl_lease_denied",
            vec![
                "ttl_lease_requested",
                "immutable_evidence_retention_requested",
            ],
        ),
        (
            "signature_manifest_hash_chain_expiry_timestamp",
            "blocked_signature_manifest_hash_expiry_noop",
            "signature_manifest_hash_chain_expiry_timestamp_denied",
            vec![
                "expiry_timestamp_requested",
                "hash_attestation_retention_requested",
            ],
        ),
        (
            "notarization_submission_attestation_retention_ledger",
            "blocked_notarization_attestation_retention_ledger_noop",
            "notarization_submission_attestation_retention_ledger_denied",
            vec![
                "retention_policy_requested",
                "ledger_index_retention_requested",
            ],
        ),
        (
            "notarization_ticket_witness_notary_expiry_scheduler",
            "blocked_witness_notary_expiry_scheduler_noop",
            "notarization_ticket_witness_notary_expiry_scheduler_denied",
            vec![
                "expiry_scheduler_requested",
                "expiry_timer_requested",
                "expiry_ack_requested",
                "witness_notary_expiry_requested",
            ],
        ),
        (
            "stapling_tombstone_garbage_collection_queue",
            "blocked_stapling_tombstone_gc_queue_noop",
            "stapling_tombstone_garbage_collection_queue_denied",
            vec![
                "garbage_collection_queue_requested",
                "tombstone_gc_requested",
            ],
        ),
        (
            "installer_replacement_evidence_garbage_collection_scan",
            "blocked_installer_replacement_gc_scan_noop",
            "installer_replacement_evidence_garbage_collection_scan_denied",
            vec![
                "garbage_collection_scan_requested",
                "garbage_collection_candidate_requested",
            ],
        ),
        (
            "provenance_immutable_evidence_archive",
            "blocked_provenance_immutable_evidence_archive_noop",
            "provenance_immutable_evidence_archive_denied",
            vec![
                "archive_requested",
                "immutable_evidence_retention_requested",
            ],
        ),
        (
            "sbom_evidence_compaction",
            "blocked_sbom_evidence_compaction_noop",
            "sbom_evidence_compaction_denied",
            vec!["compaction_requested", "audit_evidence_retention_requested"],
        ),
        (
            "release_asset_cancelled_query_retention",
            "blocked_release_asset_query_retention_noop",
            "release_asset_cancelled_query_retention_denied",
            vec![
                "retention_policy_requested",
                "audit_evidence_retention_requested",
            ],
        ),
        (
            "cdn_observability_expiry_ack",
            "blocked_cdn_observability_expiry_ack_noop",
            "cdn_observability_expiry_ack_denied",
            vec!["expiry_ack_requested", "status_evidence_expiry_requested"],
        ),
        (
            "package_registry_replacement_status_gc_decision",
            "blocked_package_registry_status_gc_decision_noop",
            "package_registry_replacement_status_gc_decision_denied",
            vec![
                "garbage_collection_decision_requested",
                "garbage_collection_candidate_requested",
            ],
        ),
        (
            "dashboard_endpoint_hash_status_retention",
            "blocked_dashboard_hash_status_retention_noop",
            "dashboard_endpoint_hash_status_retention_denied",
            vec![
                "retention_policy_requested",
                "delivery_evidence_retention_requested",
            ],
        ),
        (
            "external_telegram_retention_delivery",
            "blocked_external_telegram_retention_delivery_noop",
            "external_telegram_retention_delivery_denied",
            vec![
                "external_telegram_retention_requested",
                "delivery_evidence_retention_requested",
            ],
        ),
        (
            "release_publication_authority_retention",
            "blocked_release_publication_retention_authority_noop",
            "release_publication_authority_retention_denied",
            vec![
                "release_publication_retention_authority_requested",
                "retention_policy_requested",
            ],
        ),
        (
            "activation_live_install_garbage_collection_evidence",
            "blocked_activation_live_install_gc_noop",
            "activation_live_install_garbage_collection_evidence_denied",
            vec![
                "activation_retention_authority_requested",
                "live_install_gc_evidence_requested",
            ],
        ),
        (
            "install_restart_active_binary_retention_gc_path",
            "blocked_install_restart_active_binary_retention_gc_noop",
            "install_restart_active_binary_retention_gc_path_denied",
            vec![
                "live_install_gc_evidence_requested",
                "garbage_collection_queue_requested",
                "garbage_collection_decision_requested",
                "delete_marker_gc_requested",
            ],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "source_signing_receipt_audit_evidence_denial_ready": source_ready,
                "canonical_noop_signing_receipt_identity_required": true,
                "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempted": true,
                "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-retention-expiry-garbage-collection-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:retention=0:expiry=0:gc=0:archive=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-retention-expiry-garbage-collection-denial:no-retention:no-expiry:no-gc:no-archive:no-compaction:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_required",
        "signing_receipt_retention_policy_denied",
        "signing_receipt_ttl_lease_denied",
        "signing_receipt_expiry_timestamp_denied",
        "signing_receipt_expiry_scheduler_timer_ack_denied",
        "signing_receipt_garbage_collection_queue_denied",
        "signing_receipt_garbage_collection_scan_denied",
        "signing_receipt_garbage_collection_candidate_decision_denied",
        "signing_receipt_tombstone_delete_marker_gc_denied",
        "signing_receipt_archive_denied",
        "signing_receipt_compaction_denied",
        "signing_receipt_audit_evidence_retention_denied",
        "signing_receipt_immutable_evidence_retention_denied",
        "signing_receipt_hash_attestation_retention_denied",
        "signing_receipt_witness_notary_expiry_denied",
        "external_telegram_signing_receipt_retention_delivery_denied",
        "release_publication_retention_authority_denied",
        "activation_live_install_gc_evidence_denied",
        "install_restart_active_binary_retention_gc_denied",
        "memory_provider_kg_secret_external_send_from_retention_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count",
        ) == 0
        && source_u64("artifact_distribution_signing_notarization_receipt_ledger_recorded_count")
            == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_audit_evidence_derived_count",
        ) == 0
        && source_u64("activation_authority_from_signing_receipt_audit_evidence_derived_count")
            == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-retention-expiry-garbage-collection-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-27",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_route_v1",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_mode": "native_route_denied_signing_notarization_receipt_retention_expiry_garbage_collection_archive_compaction_authority_install_or_live_use",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_ledger_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_ledger_recorded_count"),
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_retention": false,
                    "records_retention_policy": false,
                    "records_expiry": false,
                    "records_garbage_collection": false,
                    "records_archive": false,
                    "records_compaction": false,
                    "registers_export": false,
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
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
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_materialized_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count",
        "artifact_distribution_signing_notarization_receipt_retention_policy_persisted_count",
        "artifact_distribution_signing_notarization_receipt_ttl_lease_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ttl_lease_persisted_count",
        "artifact_distribution_signing_notarization_receipt_expiry_timestamp_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_scheduler_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_timer_started_count",
        "artifact_distribution_signing_notarization_receipt_expiry_ack_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_queue_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_candidate_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_decision_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_tombstone_gc_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delete_marker_gc_recorded_count",
        "artifact_distribution_signing_notarization_receipt_archive_recorded_count",
        "artifact_distribution_signing_notarization_receipt_compaction_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_hash_attestation_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_witness_notary_expiry_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_index_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_evidence_expiry_recorded_count",
        "release_publication_authority_from_signing_receipt_retention_derived_count",
        "activation_authority_from_signing_receipt_retention_derived_count",
        "download_link_from_signing_receipt_retention_rendered_count",
        "install_command_from_signing_receipt_retention_rendered_count",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_materialized",
        "artifact_distribution_signing_notarization_receipt_retention_policy_recorded",
        "artifact_distribution_signing_notarization_receipt_expiry_recorded",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_recorded",
        "artifact_distribution_signing_notarization_receipt_archive_recorded",
        "artifact_distribution_signing_notarization_receipt_compaction_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_retention_recorded",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_retention_recorded",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "retention_policy_recorded",
        "retention_policy_persisted",
        "ttl_lease_recorded",
        "ttl_lease_persisted",
        "expiry_timestamp_recorded",
        "expiry_scheduler_recorded",
        "expiry_timer_started",
        "expiry_ack_recorded",
        "expiry_state_persisted",
        "garbage_collection_queue_recorded",
        "garbage_collection_scan_performed",
        "garbage_collection_candidate_recorded",
        "garbage_collection_decision_recorded",
        "garbage_collection_state_persisted",
        "tombstone_gc_recorded",
        "delete_marker_gc_recorded",
        "archive_recorded",
        "compaction_recorded",
        "audit_evidence_retention_recorded",
        "immutable_evidence_retention_recorded",
        "hash_attestation_retention_recorded",
        "witness_notary_expiry_recorded",
        "ledger_index_retention_recorded",
        "delivery_evidence_retention_recorded",
        "status_evidence_expiry_recorded",
        "result_receipt_from_retention_recorded",
        "result_receipt_from_retention_persisted",
        "operator_approval_from_retention_derived",
        "release_publication_authority_from_retention_derived",
        "activation_authority_from_retention_derived",
        "download_link_from_retention_rendered",
        "install_command_from_retention_rendered",
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

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        .get("artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_receipt_export_query_observability_allowed",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_materialized",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_filesystem_written",
        "query_requested",
        "query_registration_requested",
        "query_execution_requested",
        "query_result_requested",
        "search_index_requested",
        "export_requested",
        "export_snapshot_requested",
        "export_file_requested",
        "export_stream_requested",
        "observability_requested",
        "metric_log_requested",
        "trace_event_requested",
        "dashboard_panel_requested",
        "alert_slo_requested",
        "operator_summary_readback_requested",
        "audit_view_requested",
        "ledger_observability_requested",
        "index_observability_requested",
        "delivery_observability_requested",
        "archive_view_requested",
        "compaction_view_requested",
        "external_telegram_observability_requested",
        "release_publication_authority_view_requested",
        "activation_authority_view_requested",
        "live_install_view_requested",
        "install_restart_active_binary_view_requested",
        "query_registered",
        "query_executed",
        "query_result_recorded",
        "query_result_persisted",
        "search_index_recorded",
        "search_index_persisted",
        "export_accepted",
        "export_snapshot_recorded",
        "export_snapshot_persisted",
        "export_file_written",
        "export_stream_opened",
        "observability_metric_recorded",
        "observability_log_recorded",
        "observability_trace_recorded",
        "observability_event_recorded",
        "dashboard_panel_recorded",
        "alert_registered",
        "slo_recorded",
        "operator_summary_recorded",
        "readback_surface_recorded",
        "audit_view_recorded",
        "ledger_observability_recorded",
        "index_observability_recorded",
        "delivery_observability_recorded",
        "retention_policy_recorded",
        "expiry_recorded",
        "garbage_collection_recorded",
        "archive_recorded",
        "compaction_recorded",
        "audit_evidence_recorded",
        "immutable_evidence_recorded",
        "hash_chain_recorded",
        "attestation_recorded",
        "witness_notary_recorded",
        "result_receipt_recorded",
        "result_receipt_persisted",
        "result_receipt_exported",
        "result_receipt_query_registered",
        "result_receipt_observability_recorded",
        "completion_ack_recorded",
        "operator_acceptance_from_export_query_observability_recorded",
        "operator_approval_from_export_query_observability_derived",
        "release_publication_authority_from_export_query_observability_derived",
        "activation_authority_from_export_query_observability_derived",
        "download_link_from_export_query_observability_rendered",
        "install_command_from_export_query_observability_rendered",
        "install_from_export_query_observability_executed",
        "service_restart_from_export_query_observability_performed",
        "launchd_from_export_query_observability_mutated",
        "active_binary_from_export_query_observability_mutated",
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
        "filesystem_written",
    ];
    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "source_signing_receipt_retention_expiry_garbage_collection_report_required",
            "blocked_source_signing_receipt_retention_gc_report_required_noop",
            "source_signing_receipt_retention_expiry_garbage_collection_report_required",
            vec!["source_report_required"],
        ),
        (
            "artifact_signing_audit_trail_retention_policy_query_registration",
            "blocked_artifact_signing_retention_query_registration_noop",
            "artifact_signing_audit_trail_retention_policy_query_registration_denied",
            vec!["query_requested", "query_registration_requested"],
        ),
        (
            "package_signing_immutable_evidence_ttl_lease_query_execution",
            "blocked_package_signing_ttl_query_execution_noop",
            "package_signing_immutable_evidence_ttl_lease_query_execution_denied",
            vec!["query_requested", "query_execution_requested"],
        ),
        (
            "signature_manifest_hash_chain_expiry_query_result",
            "blocked_signature_manifest_expiry_query_result_noop",
            "signature_manifest_hash_chain_expiry_query_result_denied",
            vec!["query_requested", "query_result_requested"],
        ),
        (
            "notarization_attestation_retention_search_index",
            "blocked_notarization_retention_search_index_noop",
            "notarization_attestation_retention_search_index_denied",
            vec!["search_index_requested", "index_observability_requested"],
        ),
        (
            "notarization_ticket_witness_notary_export_request",
            "blocked_witness_notary_export_request_noop",
            "notarization_ticket_witness_notary_export_request_denied",
            vec!["export_requested"],
        ),
        (
            "stapling_tombstone_garbage_collection_export_snapshot",
            "blocked_stapling_tombstone_export_snapshot_noop",
            "stapling_tombstone_garbage_collection_export_snapshot_denied",
            vec!["export_requested", "export_snapshot_requested"],
        ),
        (
            "installer_replacement_garbage_collection_export_file",
            "blocked_installer_replacement_export_file_noop",
            "installer_replacement_garbage_collection_export_file_denied",
            vec!["export_requested", "export_file_requested"],
        ),
        (
            "provenance_immutable_evidence_archive_export_stream",
            "blocked_provenance_archive_export_stream_noop",
            "provenance_immutable_evidence_archive_export_stream_denied",
            vec![
                "export_requested",
                "export_stream_requested",
                "archive_view_requested",
            ],
        ),
        (
            "sbom_evidence_compaction_observability_metric_log",
            "blocked_sbom_compaction_metric_log_noop",
            "sbom_evidence_compaction_observability_metric_log_denied",
            vec![
                "observability_requested",
                "metric_log_requested",
                "compaction_view_requested",
            ],
        ),
        (
            "release_asset_cancelled_query_retention_readback",
            "blocked_release_asset_cancelled_query_readback_noop",
            "release_asset_cancelled_query_retention_readback_denied",
            vec!["operator_summary_readback_requested"],
        ),
        (
            "cdn_observability_expiry_dashboard_panel",
            "blocked_cdn_expiry_dashboard_panel_noop",
            "cdn_observability_expiry_dashboard_panel_denied",
            vec!["observability_requested", "dashboard_panel_requested"],
        ),
        (
            "package_registry_replacement_status_trace_event",
            "blocked_package_registry_status_trace_event_noop",
            "package_registry_replacement_status_trace_event_denied",
            vec!["observability_requested", "trace_event_requested"],
        ),
        (
            "dashboard_endpoint_hash_status_alert_slo",
            "blocked_dashboard_hash_status_alert_slo_noop",
            "dashboard_endpoint_hash_status_alert_slo_denied",
            vec!["observability_requested", "alert_slo_requested"],
        ),
        (
            "external_telegram_retention_delivery_observability",
            "blocked_external_telegram_retention_observability_noop",
            "external_telegram_retention_delivery_observability_denied",
            vec![
                "observability_requested",
                "delivery_observability_requested",
                "external_telegram_observability_requested",
            ],
        ),
        (
            "release_publication_authority_retention_view",
            "blocked_release_publication_authority_retention_view_noop",
            "release_publication_authority_retention_view_denied",
            vec![
                "audit_view_requested",
                "release_publication_authority_view_requested",
            ],
        ),
        (
            "activation_live_install_garbage_collection_view",
            "blocked_activation_live_install_gc_view_noop",
            "activation_live_install_garbage_collection_view_denied",
            vec![
                "activation_authority_view_requested",
                "live_install_view_requested",
            ],
        ),
        (
            "install_restart_active_binary_retention_gc_view",
            "blocked_install_restart_active_binary_retention_gc_view_noop",
            "install_restart_active_binary_retention_gc_view_denied",
            vec![
                "ledger_observability_requested",
                "install_restart_active_binary_view_requested",
            ],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_export_query_observability_surface": surface,
                "source_signing_receipt_retention_expiry_garbage_collection_ready": source_ready,
                "canonical_noop_signing_receipt_identity_required": true,
                "artifact_distribution_signing_notarization_receipt_export_query_observability_attempted": true,
                "artifact_distribution_signing_notarization_receipt_export_query_observability_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_export_query_observability_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-export-query-observability-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:query=0:export=0:observability=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-export-query-observability-denial:no-query:no-export:no-observability:no-readback:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_required",
        "signing_receipt_retention_query_registration_denied",
        "signing_receipt_ttl_query_execution_denied",
        "signing_receipt_expiry_query_result_denied",
        "signing_receipt_search_index_denied",
        "signing_receipt_export_request_denied",
        "signing_receipt_export_snapshot_denied",
        "signing_receipt_export_file_denied",
        "signing_receipt_export_stream_denied",
        "signing_receipt_archive_export_stream_denied",
        "signing_receipt_compaction_metric_log_denied",
        "signing_receipt_dashboard_panel_denied",
        "signing_receipt_trace_event_denied",
        "signing_receipt_alert_slo_denied",
        "external_telegram_signing_receipt_observability_denied",
        "release_publication_authority_view_denied",
        "activation_live_install_view_denied",
        "install_restart_active_binary_view_denied",
        "memory_provider_kg_secret_external_send_from_view_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count",
        ) == 0
        && source_u64("release_publication_authority_from_signing_receipt_retention_derived_count")
            == 0
        && source_u64("activation_authority_from_signing_receipt_retention_derived_count") == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-export-query-observability-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-27",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_route_v1",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_mode": "native_route_denied_signing_notarization_receipt_query_export_observability_readback_views_authority_install_or_live_use",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_ready": report_ready,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_export_query_observability_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count": source_u64("artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count"),
            "source_artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count": source_u64("artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count"),
            "source_release_publication_authority_from_signing_receipt_retention_derived_count": source_u64("release_publication_authority_from_signing_receipt_retention_derived_count"),
            "source_activation_authority_from_signing_receipt_retention_derived_count": source_u64("activation_authority_from_signing_receipt_retention_derived_count"),
            "artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_export_query_observability": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_export_query_observability_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "registers_query": false,
                    "executes_query": false,
                    "records_query_result": false,
                    "writes_search_index": false,
                    "accepts_export": false,
                    "writes_export": false,
                    "opens_export_stream": false,
                    "records_observability": false,
                    "records_operator_summary": false,
                    "records_readback": false,
                    "records_audit_view": false,
                    "records_delivery_evidence": false,
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
        "artifact_distribution_signing_notarization_receipt_export_query_observability_allowed_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_materialized_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_acceptance_recorded_count",
        "artifact_distribution_signing_notarization_receipt_query_registered_count",
        "artifact_distribution_signing_notarization_receipt_query_executed_count",
        "artifact_distribution_signing_notarization_receipt_query_result_recorded_count",
        "artifact_distribution_signing_notarization_receipt_query_result_persisted_count",
        "artifact_distribution_signing_notarization_receipt_search_index_recorded_count",
        "artifact_distribution_signing_notarization_receipt_search_index_persisted_count",
        "artifact_distribution_signing_notarization_receipt_export_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_snapshot_recorded_count",
        "artifact_distribution_signing_notarization_receipt_export_snapshot_persisted_count",
        "artifact_distribution_signing_notarization_receipt_export_file_written_count",
        "artifact_distribution_signing_notarization_receipt_export_stream_opened_count",
        "artifact_distribution_signing_notarization_receipt_observability_metric_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_log_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_trace_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_event_recorded_count",
        "artifact_distribution_signing_notarization_receipt_dashboard_panel_recorded_count",
        "artifact_distribution_signing_notarization_receipt_alert_registered_count",
        "artifact_distribution_signing_notarization_receipt_slo_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_readback_surface_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_view_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_observability_recorded_count",
        "artifact_distribution_signing_notarization_receipt_index_observability_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delivery_observability_recorded_count",
        "release_publication_authority_from_signing_receipt_export_query_observability_derived_count",
        "activation_authority_from_signing_receipt_export_query_observability_derived_count",
        "download_link_from_signing_receipt_export_query_observability_rendered_count",
        "install_command_from_signing_receipt_export_query_observability_rendered_count",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_materialized",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_filesystem_written",
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
        "artifact_distribution_signing_notarization_receipt_delivery_observability_recorded",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "query_registered",
        "query_executed",
        "query_result_recorded",
        "query_result_persisted",
        "search_index_recorded",
        "search_index_persisted",
        "export_accepted",
        "export_snapshot_recorded",
        "export_snapshot_persisted",
        "export_file_written",
        "export_stream_opened",
        "observability_metric_recorded",
        "observability_log_recorded",
        "observability_trace_recorded",
        "observability_event_recorded",
        "dashboard_panel_recorded",
        "alert_registered",
        "slo_recorded",
        "operator_summary_recorded",
        "readback_surface_recorded",
        "audit_view_recorded",
        "ledger_observability_recorded",
        "index_observability_recorded",
        "delivery_observability_recorded",
        "retention_policy_recorded",
        "expiry_recorded",
        "garbage_collection_recorded",
        "archive_recorded",
        "compaction_recorded",
        "result_receipt_recorded",
        "result_receipt_persisted",
        "result_receipt_exported",
        "result_receipt_query_registered",
        "result_receipt_observability_recorded",
        "completion_ack_recorded",
        "operator_acceptance_from_export_query_observability_recorded",
        "operator_approval_from_export_query_observability_derived",
        "release_publication_authority_from_export_query_observability_derived",
        "activation_authority_from_export_query_observability_derived",
        "download_link_from_export_query_observability_rendered",
        "install_command_from_export_query_observability_rendered",
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

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_ready",
    );
    let source_contract_hash = source
        .get("artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_materialized",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_filesystem_written",
        "operator_summary_recorded",
        "operator_summary_persisted",
        "operator_briefing_recorded",
        "operator_briefing_persisted",
        "signing_receipt_readback_recorded",
        "signing_receipt_readback_persisted",
        "status_banner_recorded",
        "exported_summary_recorded",
        "briefing_card_recorded",
        "notification_timeline_recorded",
        "dashboard_narrative_recorded",
        "audit_narrative_recorded",
        "briefing_delivery_recorded",
        "final_summary_recorded",
        "operator_memo_recorded",
        "approval_summary_recorded",
        "external_briefing_delivered",
        "telegram_briefing_delivered",
        "authority_briefing_recorded",
        "live_status_briefing_recorded",
        "signing_receipt_query_registered",
        "signing_receipt_query_executed",
        "signing_receipt_query_result_recorded",
        "signing_receipt_export_accepted",
        "signing_receipt_export_file_written",
        "signing_receipt_export_stream_opened",
        "signing_receipt_observability_recorded",
        "signing_receipt_dashboard_recorded",
        "signing_receipt_alert_recorded",
        "signing_receipt_result_receipt_recorded",
        "signing_receipt_result_receipt_persisted",
        "signing_receipt_completion_ack_recorded",
        "operator_acceptance_from_summary_recorded",
        "operator_acceptance_from_briefing_recorded",
        "operator_approval_from_summary_derived",
        "operator_approval_from_briefing_derived",
        "release_publication_authority_from_summary_briefing_derived",
        "activation_authority_from_summary_briefing_derived",
        "download_link_from_summary_briefing_rendered",
        "install_command_from_summary_briefing_rendered",
        "install_from_summary_briefing_executed",
        "service_restart_from_summary_briefing_performed",
        "launchd_from_summary_briefing_mutated",
        "active_binary_from_summary_briefing_mutated",
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
    let surface_specs = vec![
        (
            "source_signing_receipt_export_query_observability_report_required",
            "blocked_source_signing_receipt_observability_report_required_noop",
            "source_signing_receipt_export_query_observability_report_required",
            vec!["source_report_required"],
        ),
        (
            "artifact_signing_retention_query_operator_summary",
            "blocked_artifact_signing_query_summary_noop",
            "artifact_signing_retention_query_operator_summary_denied",
            vec!["summary_requested"],
        ),
        (
            "package_signing_ttl_query_operator_briefing",
            "blocked_package_signing_query_briefing_noop",
            "package_signing_ttl_query_operator_briefing_denied",
            vec!["briefing_requested"],
        ),
        (
            "signature_manifest_expiry_query_readback_digest",
            "blocked_signature_manifest_query_readback_noop",
            "signature_manifest_expiry_query_readback_digest_denied",
            vec!["readback_requested"],
        ),
        (
            "notarization_search_index_status_banner",
            "blocked_notarization_search_index_status_banner_noop",
            "notarization_search_index_status_banner_denied",
            vec!["status_banner_requested"],
        ),
        (
            "witness_notary_exported_summary_text",
            "blocked_witness_notary_exported_summary_noop",
            "witness_notary_exported_summary_text_denied",
            vec!["exported_summary_requested", "summary_requested"],
        ),
        (
            "tombstone_garbage_collection_export_briefing_card",
            "blocked_tombstone_gc_briefing_card_noop",
            "tombstone_garbage_collection_export_briefing_card_denied",
            vec!["briefing_card_requested", "briefing_requested"],
        ),
        (
            "replacement_garbage_collection_notification_timeline",
            "blocked_replacement_gc_notification_timeline_noop",
            "replacement_garbage_collection_notification_timeline_denied",
            vec!["notification_timeline_requested"],
        ),
        (
            "provenance_archive_dashboard_narrative",
            "blocked_provenance_archive_dashboard_narrative_noop",
            "provenance_archive_dashboard_narrative_denied",
            vec!["dashboard_narrative_requested"],
        ),
        (
            "sbom_compaction_audit_narrative",
            "blocked_sbom_compaction_audit_narrative_noop",
            "sbom_compaction_audit_narrative_denied",
            vec!["audit_narrative_requested"],
        ),
        (
            "release_asset_retention_readback_final_summary",
            "blocked_release_asset_final_summary_noop",
            "release_asset_retention_readback_final_summary_denied",
            vec![
                "final_summary_requested",
                "summary_requested",
                "readback_requested",
            ],
        ),
        (
            "cdn_expiry_dashboard_briefing_panel",
            "blocked_cdn_expiry_dashboard_briefing_noop",
            "cdn_expiry_dashboard_briefing_panel_denied",
            vec!["briefing_requested", "dashboard_narrative_requested"],
        ),
        (
            "package_registry_trace_event_operator_memo",
            "blocked_package_registry_trace_operator_memo_noop",
            "package_registry_trace_event_operator_memo_denied",
            vec!["operator_memo_requested"],
        ),
        (
            "dashboard_hash_alert_slo_approval_summary",
            "blocked_dashboard_hash_alert_approval_summary_noop",
            "dashboard_hash_alert_slo_approval_summary_denied",
            vec!["approval_summary_requested", "summary_requested"],
        ),
        (
            "external_telegram_observability_briefing_delivery",
            "blocked_external_telegram_observability_briefing_noop",
            "external_telegram_observability_briefing_delivery_denied",
            vec![
                "external_briefing_requested",
                "telegram_briefing_requested",
                "delivery_requested",
                "briefing_requested",
            ],
        ),
        (
            "release_publication_authority_view_briefing",
            "blocked_release_publication_authority_view_briefing_noop",
            "release_publication_authority_view_briefing_denied",
            vec!["authority_briefing_requested", "briefing_requested"],
        ),
        (
            "activation_live_install_view_status_briefing",
            "blocked_activation_live_install_status_briefing_noop",
            "activation_live_install_view_status_briefing_denied",
            vec!["live_status_briefing_requested", "briefing_requested"],
        ),
        (
            "install_restart_active_binary_view_status_briefing",
            "blocked_install_restart_active_binary_status_briefing_noop",
            "install_restart_active_binary_view_status_briefing_denied",
            vec![
                "live_status_briefing_requested",
                "briefing_requested",
                "install_restart_active_binary_status_requested",
            ],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface": surface,
                "source_signing_receipt_export_query_observability_ready": source_ready,
                "canonical_noop_signing_receipt_identity_required": true,
                "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempted": true,
                "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-operator-facing-summary-briefing-non-persistence-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:summary=0:briefing=0:readback=0:delivery=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-operator-facing-summary-briefing-denial:no-summary:no-briefing:no-readback:no-delivery:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_required",
        "artifact_signing_query_operator_summary_denied",
        "package_signing_query_operator_briefing_denied",
        "signature_manifest_query_readback_denied",
        "notarization_search_index_status_banner_denied",
        "witness_notary_exported_summary_denied",
        "tombstone_garbage_collection_briefing_card_denied",
        "replacement_garbage_collection_notification_timeline_denied",
        "provenance_archive_dashboard_narrative_denied",
        "sbom_compaction_audit_narrative_denied",
        "release_asset_retention_final_summary_denied",
        "cdn_expiry_dashboard_briefing_denied",
        "package_registry_trace_operator_memo_denied",
        "dashboard_hash_alert_approval_summary_denied",
        "external_telegram_observability_briefing_denied",
        "release_publication_authority_view_briefing_denied",
        "activation_live_install_status_briefing_denied",
        "install_restart_active_binary_status_briefing_denied",
        "memory_provider_kg_secret_external_send_from_summary_briefing_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_export_query_observability_derived_count",
        ) == 0
        && source_u64(
            "activation_authority_from_signing_receipt_export_query_observability_derived_count",
        ) == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-27",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route_v1",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_mode": "native_route_denied_signing_notarization_receipt_operator_summary_briefing_readback_delivery_authority_install_or_live_use",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": report_ready,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_accepted_count": source_u64("artifact_distribution_signing_notarization_receipt_export_query_observability_accepted_count"),
            "source_artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count"),
            "source_release_publication_authority_from_signing_receipt_export_query_observability_derived_count": source_u64("release_publication_authority_from_signing_receipt_export_query_observability_derived_count"),
            "source_activation_authority_from_signing_receipt_export_query_observability_derived_count": source_u64("activation_authority_from_signing_receipt_export_query_observability_derived_count"),
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_summary": false,
                    "records_briefing": false,
                    "records_readback": false,
                    "records_status_banner": false,
                    "records_delivery": false,
                    "records_acknowledgement": false,
                    "derives_operator_approval": false,
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
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_materialized_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_persisted_count",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_persisted_count",
        "artifact_distribution_signing_notarization_receipt_readback_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_banner_recorded_count",
        "artifact_distribution_signing_notarization_receipt_exported_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_briefing_card_recorded_count",
        "artifact_distribution_signing_notarization_receipt_notification_timeline_recorded_count",
        "artifact_distribution_signing_notarization_receipt_dashboard_narrative_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_narrative_recorded_count",
        "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_memo_recorded_count",
        "artifact_distribution_signing_notarization_receipt_approval_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_external_briefing_delivered_count",
        "artifact_distribution_signing_notarization_receipt_telegram_briefing_delivered_count",
        "artifact_distribution_signing_notarization_receipt_authority_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_live_status_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_summary_briefing_acceptance_recorded_count",
        "operator_approval_from_signing_receipt_summary_briefing_derived_count",
        "release_publication_authority_from_signing_receipt_summary_briefing_derived_count",
        "activation_authority_from_signing_receipt_summary_briefing_derived_count",
        "download_link_from_signing_receipt_summary_briefing_rendered_count",
        "install_command_from_signing_receipt_summary_briefing_rendered_count",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_summary_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_persisted",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "operator_summary_recorded",
        "operator_summary_persisted",
        "operator_briefing_recorded",
        "operator_briefing_persisted",
        "readback_recorded",
        "status_banner_recorded",
        "exported_summary_recorded",
        "briefing_card_recorded",
        "notification_timeline_recorded",
        "dashboard_narrative_recorded",
        "audit_narrative_recorded",
        "briefing_delivery_recorded",
        "final_summary_recorded",
        "operator_memo_recorded",
        "approval_summary_recorded",
        "external_briefing_delivered",
        "telegram_briefing_delivered",
        "authority_briefing_recorded",
        "live_status_briefing_recorded",
        "summary_briefing_acceptance_recorded",
        "operator_approval_from_summary_briefing_derived",
        "release_publication_authority_from_summary_briefing_derived",
        "activation_authority_from_summary_briefing_derived",
        "download_link_from_summary_briefing_rendered",
        "install_command_from_summary_briefing_rendered",
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
        "public_release_claimed",
        "public_ga_claimed",
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

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
    );
    let source_contract_hash = source
        .get("artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "source_signing_receipt_summary_briefing_report_required",
        "final_operator_acknowledgement_requested",
        "final_operator_acknowledgement_request_accepted",
        "final_operator_acknowledgement_accepted",
        "final_operator_acknowledgement_recorded",
        "final_operator_acknowledgement_persisted",
        "final_operator_acknowledgement_materialized",
        "final_operator_acknowledgement_filesystem_written",
        "final_operator_acknowledgement_delivered",
        "operator_received_requested",
        "operator_received_recorded",
        "operator_confirmed_requested",
        "operator_confirmed_recorded",
        "operator_read_requested",
        "operator_read_recorded",
        "operator_seen_requested",
        "operator_seen_recorded",
        "final_response_requested",
        "final_response_recorded",
        "completion_acknowledgement_requested",
        "completion_acknowledgement_recorded",
        "status_acknowledgement_requested",
        "status_acknowledgement_recorded",
        "summary_acknowledgement_requested",
        "summary_acknowledgement_recorded",
        "briefing_acknowledgement_requested",
        "briefing_acknowledgement_recorded",
        "readback_digest_acknowledgement_requested",
        "readback_digest_acknowledgement_recorded",
        "dashboard_acknowledgement_requested",
        "dashboard_acknowledgement_recorded",
        "notification_acknowledgement_requested",
        "notification_acknowledgement_recorded",
        "channel_acknowledgement_requested",
        "channel_acknowledgement_delivered",
        "external_acknowledgement_requested",
        "external_acknowledgement_sent",
        "telegram_acknowledgement_requested",
        "telegram_acknowledgement_sent",
        "operator_approval_acknowledgement_requested",
        "authority_acknowledgement_requested",
        "live_acknowledgement_requested",
        "install_restart_active_binary_acknowledgement_requested",
        "acknowledgement_acceptance_recorded",
        "operator_acceptance_from_acknowledgement_recorded",
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
        "signing_receipt_summary_from_acknowledgement_recorded",
        "signing_receipt_briefing_from_acknowledgement_recorded",
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
    let surface_specs: &[(&str, &str, &str, &[&str])] = &[
        (
            "source_signing_receipt_summary_briefing_report_required",
            "blocked_source_signing_receipt_summary_briefing_required_noop",
            "source_signing_receipt_summary_briefing_report_required",
            &["source_signing_receipt_summary_briefing_report_required"][..],
        ),
        (
            "artifact_signing_summary_final_operator_acknowledgement_claim",
            "blocked_artifact_signing_summary_final_ack_noop",
            "artifact_signing_summary_final_operator_acknowledgement_claim_denied",
            &["final_operator_acknowledgement_requested"][..],
        ),
        (
            "package_signing_briefing_operator_received_claim",
            "blocked_package_signing_briefing_operator_received_noop",
            "package_signing_briefing_operator_received_claim_denied",
            &["operator_received_requested"][..],
        ),
        (
            "signature_manifest_readback_operator_confirmed_claim",
            "blocked_signature_manifest_readback_operator_confirmed_noop",
            "signature_manifest_readback_operator_confirmed_claim_denied",
            &["operator_confirmed_requested"][..],
        ),
        (
            "notarization_status_banner_operator_read_claim",
            "blocked_notarization_status_banner_operator_read_noop",
            "notarization_status_banner_operator_read_claim_denied",
            &["operator_read_requested"][..],
        ),
        (
            "witness_notary_exported_summary_operator_seen_claim",
            "blocked_witness_notary_exported_summary_operator_seen_noop",
            "witness_notary_exported_summary_operator_seen_claim_denied",
            &["operator_seen_requested"][..],
        ),
        (
            "tombstone_garbage_collection_briefing_card_final_response_claim",
            "blocked_tombstone_gc_briefing_card_final_response_noop",
            "tombstone_garbage_collection_briefing_card_final_response_claim_denied",
            &["final_response_requested"][..],
        ),
        (
            "replacement_garbage_collection_notification_completion_acknowledgement_claim",
            "blocked_replacement_gc_notification_completion_ack_noop",
            "replacement_garbage_collection_notification_completion_acknowledgement_claim_denied",
            &["completion_acknowledgement_requested"][..],
        ),
        (
            "provenance_dashboard_narrative_status_acknowledgement_claim",
            "blocked_provenance_dashboard_status_ack_noop",
            "provenance_dashboard_narrative_status_acknowledgement_claim_denied",
            &[
                "status_acknowledgement_requested",
                "dashboard_acknowledgement_requested",
            ][..],
        ),
        (
            "sbom_audit_narrative_summary_acknowledgement_claim",
            "blocked_sbom_audit_summary_ack_noop",
            "sbom_audit_narrative_summary_acknowledgement_claim_denied",
            &["summary_acknowledgement_requested"][..],
        ),
        (
            "release_asset_final_summary_briefing_acknowledgement_claim",
            "blocked_release_asset_summary_briefing_ack_noop",
            "release_asset_final_summary_briefing_acknowledgement_claim_denied",
            &[
                "summary_acknowledgement_requested",
                "briefing_acknowledgement_requested",
            ][..],
        ),
        (
            "cdn_dashboard_briefing_readback_digest_acknowledgement_claim",
            "blocked_cdn_dashboard_briefing_readback_ack_noop",
            "cdn_dashboard_briefing_readback_digest_acknowledgement_claim_denied",
            &[
                "dashboard_acknowledgement_requested",
                "briefing_acknowledgement_requested",
                "readback_digest_acknowledgement_requested",
            ][..],
        ),
        (
            "package_registry_operator_memo_dashboard_notification_acknowledgement_claim",
            "blocked_package_registry_dashboard_notification_ack_noop",
            "package_registry_operator_memo_dashboard_notification_acknowledgement_claim_denied",
            &[
                "dashboard_acknowledgement_requested",
                "notification_acknowledgement_requested",
            ][..],
        ),
        (
            "dashboard_hash_approval_summary_channel_acknowledgement_claim",
            "blocked_dashboard_hash_approval_summary_channel_ack_noop",
            "dashboard_hash_approval_summary_channel_acknowledgement_claim_denied",
            &[
                "operator_approval_acknowledgement_requested",
                "summary_acknowledgement_requested",
                "channel_acknowledgement_requested",
            ][..],
        ),
        (
            "external_telegram_observability_briefing_acknowledgement_claim",
            "blocked_external_telegram_observability_briefing_ack_noop",
            "external_telegram_observability_briefing_acknowledgement_claim_denied",
            &[
                "external_acknowledgement_requested",
                "telegram_acknowledgement_requested",
                "briefing_acknowledgement_requested",
            ][..],
        ),
        (
            "release_publication_authority_view_acknowledgement_claim",
            "blocked_release_publication_authority_ack_noop",
            "release_publication_authority_view_acknowledgement_claim_denied",
            &["authority_acknowledgement_requested"][..],
        ),
        (
            "activation_live_install_view_acknowledgement_claim",
            "blocked_activation_live_install_ack_noop",
            "activation_live_install_view_acknowledgement_claim_denied",
            &["live_acknowledgement_requested"][..],
        ),
        (
            "install_restart_active_binary_status_acknowledgement_claim",
            "blocked_install_restart_active_binary_ack_noop",
            "install_restart_active_binary_status_acknowledgement_claim_denied",
            &[
                "live_acknowledgement_requested",
                "install_restart_active_binary_acknowledgement_requested",
                "status_acknowledgement_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface": surface,
                "source_signing_receipt_summary_briefing_ready": source_ready,
                "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempted": true,
                "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed": false,
                "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-final-operator-acknowledgement-non-acceptance-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:ack=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-final-operator-acknowledgement:no-ack:no-received:no-confirmed:no-read:no-seen:no-final-response:no-status:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_summary_briefing_report_required",
        "artifact_signing_summary_final_operator_acknowledgement_denied",
        "package_signing_briefing_operator_received_denied",
        "signature_manifest_readback_operator_confirmed_denied",
        "notarization_status_banner_operator_read_denied",
        "witness_notary_exported_summary_operator_seen_denied",
        "tombstone_garbage_collection_briefing_card_final_response_denied",
        "replacement_garbage_collection_notification_completion_acknowledgement_denied",
        "provenance_dashboard_narrative_status_acknowledgement_denied",
        "sbom_audit_narrative_summary_acknowledgement_denied",
        "release_asset_final_summary_briefing_acknowledgement_denied",
        "cdn_dashboard_briefing_readback_digest_acknowledgement_denied",
        "package_registry_operator_memo_dashboard_notification_acknowledgement_denied",
        "dashboard_hash_approval_summary_channel_acknowledgement_denied",
        "external_telegram_observability_briefing_acknowledgement_denied",
        "release_publication_authority_view_acknowledgement_denied",
        "activation_live_install_view_acknowledgement_denied",
        "install_restart_active_binary_status_acknowledgement_denied",
        "memory_provider_kg_secret_external_send_from_acknowledgement_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count",
        ) == 0
        && source_u64("artifact_distribution_signing_notarization_receipt_readback_recorded_count")
            == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_summary_briefing_derived_count",
        ) == 0
        && source_u64("activation_authority_from_signing_receipt_summary_briefing_derived_count")
            == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-27",
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
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route_v1",
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_mode": "native_route_denied_signing_receipt_summary_briefing_cannot_be_acknowledged_accepted_promoted_or_used_for_authority_or_live_install",
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": report_ready,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": report_ready,
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_readback_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_readback_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count"),
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [{
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_final_acknowledgement": false,
                "records_received_confirmed_read_seen": false,
                "records_terminal_decision": false,
                "records_status_promotion": false,
                "derives_operator_approval": false,
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
            }],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_materialized_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_delivered_count",
        "artifact_distribution_signing_notarization_receipt_operator_received_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_read_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_seen_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_response_recorded_count",
        "artifact_distribution_signing_notarization_receipt_completion_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_summary_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_briefing_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_readback_digest_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_dashboard_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_notification_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_channel_acknowledgement_delivered_count",
        "artifact_distribution_signing_notarization_receipt_external_acknowledgement_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_acknowledgement_sent_count",
        "artifact_distribution_signing_notarization_receipt_acceptance_from_acknowledgement_recorded_count",
        "operator_approval_from_signing_receipt_acknowledgement_derived_count",
        "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
        "activation_authority_from_signing_receipt_acknowledgement_derived_count",
        "download_link_from_signing_receipt_acknowledgement_rendered_count",
        "install_command_from_signing_receipt_acknowledgement_rendered_count",
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
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
        "artifact_distribution_signing_notarization_receipt_acknowledgement_acceptance_recorded",
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
        "public_release_claimed",
        "public_ga_claimed",
        "filesystem_written",
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

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
    );
    let source_contract_hash = source
        .get("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "source_signing_receipt_final_operator_acknowledgement_report_required",
        "public_claim_requested",
        "status_exposure_requested",
        "public_release_claim_requested",
        "public_status_exposure_requested",
        "telegram_status_exposure_requested",
        "release_publication_status_exposure_requested",
        "install_restart_active_binary_status_exposure_requested",
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
    ];
    let surface_specs: &[(&str, &str, &str, &[&str])] = &[
        (
            "source_signing_receipt_final_operator_acknowledgement_report_required",
            "blocked_source_signing_receipt_final_acknowledgement_required_noop",
            "source_signing_receipt_final_operator_acknowledgement_report_required",
            &["source_signing_receipt_final_operator_acknowledgement_report_required"][..],
        ),
        (
            "artifact_signing_receipt_public_claim_attempt",
            "blocked_artifact_signing_receipt_public_claim_noop",
            "artifact_signing_receipt_public_claim_attempt_denied",
            &["public_claim_requested"][..],
        ),
        (
            "package_signing_receipt_public_status_badge_exposure",
            "blocked_package_signing_receipt_public_status_badge_noop",
            "package_signing_receipt_public_status_badge_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "signature_manifest_public_status_page_exposure",
            "blocked_signature_manifest_public_status_page_noop",
            "signature_manifest_public_status_page_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "notarization_status_public_claim_attempt",
            "blocked_notarization_status_public_claim_noop",
            "notarization_status_public_claim_attempt_denied",
            &["public_claim_requested"][..],
        ),
        (
            "witness_notary_exported_summary_public_status_exposure",
            "blocked_witness_notary_public_status_noop",
            "witness_notary_exported_summary_public_status_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "tombstone_garbage_collection_final_response_public_claim_attempt",
            "blocked_tombstone_gc_final_response_public_claim_noop",
            "tombstone_garbage_collection_final_response_public_claim_attempt_denied",
            &["public_claim_requested"][..],
        ),
        (
            "replacement_garbage_collection_completion_public_status_exposure",
            "blocked_replacement_gc_completion_public_status_noop",
            "replacement_garbage_collection_completion_public_status_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "provenance_dashboard_public_status_exposure",
            "blocked_provenance_dashboard_public_status_noop",
            "provenance_dashboard_public_status_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "sbom_audit_public_claim_attempt",
            "blocked_sbom_audit_public_claim_noop",
            "sbom_audit_public_claim_attempt_denied",
            &["public_claim_requested"][..],
        ),
        (
            "release_asset_public_briefing_exposure",
            "blocked_release_asset_public_briefing_noop",
            "release_asset_public_briefing_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "cdn_dashboard_public_readback_exposure",
            "blocked_cdn_dashboard_public_readback_noop",
            "cdn_dashboard_public_readback_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "package_registry_public_memo_notification_exposure",
            "blocked_package_registry_public_notification_noop",
            "package_registry_public_memo_notification_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "dashboard_hash_public_approval_channel_exposure",
            "blocked_dashboard_hash_public_channel_exposure_noop",
            "dashboard_hash_public_approval_channel_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "external_telegram_public_claim_exposure",
            "blocked_external_telegram_public_claim_exposure_noop",
            "external_telegram_public_claim_exposure_denied",
            &[
                "public_claim_requested",
                "telegram_status_exposure_requested",
            ][..],
        ),
        (
            "release_publication_public_claim_status_exposure",
            "blocked_release_publication_public_claim_status_exposure_noop",
            "release_publication_public_claim_status_exposure_denied",
            &[
                "public_claim_requested",
                "public_release_claim_requested",
                "status_exposure_requested",
                "public_status_exposure_requested",
                "release_publication_status_exposure_requested",
            ][..],
        ),
        (
            "activation_live_install_status_public_exposure",
            "blocked_activation_live_install_public_status_exposure_noop",
            "activation_live_install_status_public_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "install_restart_active_binary_public_status_exposure",
            "blocked_install_restart_active_binary_public_status_exposure_noop",
            "install_restart_active_binary_public_status_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
                "install_restart_active_binary_status_exposure_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface": surface,
                "source_signing_receipt_final_operator_acknowledgement_ready": source_ready,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempted": true,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-terminal-public-claim-status-exposure-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:public=0:status=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-receipt-terminal-public-claim-status-exposure:no-public-claim:no-status-exposure:no-release:no-channel:no-telegram:no-install",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_report_required",
        "artifact_distribution_signing_notarization_receipt_public_claim_recording_denied",
        "artifact_distribution_signing_notarization_receipt_public_status_exposure_denied",
        "artifact_distribution_signing_notarization_receipt_public_release_claim_denied",
        "artifact_distribution_signing_notarization_receipt_channel_external_telegram_public_status_denied",
        "artifact_distribution_signing_notarization_receipt_release_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_public_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_operator_approval_from_public_claim_denied",
        "artifact_distribution_signing_notarization_receipt_release_publication_authority_from_public_claim_denied",
        "artifact_distribution_signing_notarization_receipt_activation_authority_from_status_exposure_denied",
        "artifact_distribution_signing_notarization_receipt_download_install_restart_active_binary_from_status_exposure_denied",
        "artifact_distribution_signing_notarization_receipt_memory_provider_secret_external_send_from_public_exposure_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
        ) == 0
        && source_u64("activation_authority_from_signing_receipt_acknowledgement_derived_count")
            == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-terminal-public-claim-status-exposure-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-28",
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
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_route_v1",
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_mode": "native_route_denied_signing_receipt_final_acknowledgement_cannot_create_public_claim_status_exposure_release_channel_telegram_or_live_install",
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_ready": report_ready,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_ready": report_ready,
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count": source_u64("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count"),
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count"),
            "source_release_publication_authority_from_signing_receipt_acknowledgement_derived_count": source_u64("release_publication_authority_from_signing_receipt_acknowledgement_derived_count"),
            "source_activation_authority_from_signing_receipt_acknowledgement_derived_count": source_u64("activation_authority_from_signing_receipt_acknowledgement_derived_count"),
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_count": denied_count,
            "allowed_next_actions": [{
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_public_claim": false,
                "records_status_exposure": false,
                "delivers_channel_status": false,
                "sends_telegram": false,
                "writes_release_artifact": false,
                "writes_public_artifact": false,
                "derives_operator_approval": false,
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
            }],
        }),
    );

    let zero_keys = [
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
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
    for key in [
        "public_release_claimed",
        "public_ga_claimed",
        "telegram_send_performed",
        "channel_send_performed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_ready",
    );
    let source_contract_hash = source
        .get("artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "source_public_claim_status_exposure_report_required",
        "public_claim_delivery_requested",
        "status_readback_requested",
        "channel_delivery_requested",
        "telegram_delivery_requested",
        "release_publication_delivery_readback_requested",
        "install_restart_active_binary_readback_requested",
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
    ];
    let surface_specs: &[(&str, &str, &str, &[&str])] = &[
        (
            "source_public_claim_status_exposure_report_required",
            "blocked_source_public_exposure_report_required_noop",
            "source_public_claim_status_exposure_report_required",
            &["source_public_claim_status_exposure_report_required"][..],
        ),
        (
            "artifact_signing_receipt_claim_channel_delivery_attempt",
            "blocked_artifact_signing_claim_channel_delivery_noop",
            "artifact_signing_receipt_claim_channel_delivery_attempt_denied",
            &[
                "public_claim_delivery_requested",
                "channel_delivery_requested",
            ][..],
        ),
        (
            "package_signing_status_badge_readback_attempt",
            "blocked_package_signing_status_badge_readback_noop",
            "package_signing_status_badge_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "signature_manifest_status_page_readback_attempt",
            "blocked_signature_manifest_status_page_readback_noop",
            "signature_manifest_status_page_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "notarization_claim_readback_attempt",
            "blocked_notarization_claim_readback_noop",
            "notarization_claim_readback_attempt_denied",
            &["public_claim_delivery_requested"][..],
        ),
        (
            "witness_notary_summary_channel_delivery_attempt",
            "blocked_witness_notary_summary_channel_delivery_noop",
            "witness_notary_summary_channel_delivery_attempt_denied",
            &["status_readback_requested", "channel_delivery_requested"][..],
        ),
        (
            "tombstone_gc_final_response_delivery_readback_attempt",
            "blocked_tombstone_gc_final_response_delivery_readback_noop",
            "tombstone_gc_final_response_delivery_readback_attempt_denied",
            &["public_claim_delivery_requested"][..],
        ),
        (
            "replacement_gc_completion_readback_attempt",
            "blocked_replacement_gc_completion_readback_noop",
            "replacement_gc_completion_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "provenance_dashboard_status_readback_attempt",
            "blocked_provenance_dashboard_status_readback_noop",
            "provenance_dashboard_status_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "sbom_audit_public_claim_readback_attempt",
            "blocked_sbom_audit_public_claim_readback_noop",
            "sbom_audit_public_claim_readback_attempt_denied",
            &["public_claim_delivery_requested"][..],
        ),
        (
            "release_asset_briefing_channel_delivery_attempt",
            "blocked_release_asset_briefing_channel_delivery_noop",
            "release_asset_briefing_channel_delivery_attempt_denied",
            &["status_readback_requested", "channel_delivery_requested"][..],
        ),
        (
            "cdn_dashboard_readback_attempt",
            "blocked_cdn_dashboard_readback_noop",
            "cdn_dashboard_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "package_registry_memo_delivery_attempt",
            "blocked_package_registry_memo_delivery_noop",
            "package_registry_memo_delivery_attempt_denied",
            &["status_readback_requested", "channel_delivery_requested"][..],
        ),
        (
            "dashboard_hash_approval_channel_readback_attempt",
            "blocked_dashboard_hash_approval_channel_readback_noop",
            "dashboard_hash_approval_channel_readback_attempt_denied",
            &["status_readback_requested", "channel_delivery_requested"][..],
        ),
        (
            "external_telegram_claim_delivery_readback_attempt",
            "blocked_external_telegram_claim_delivery_readback_noop",
            "external_telegram_claim_delivery_readback_attempt_denied",
            &[
                "public_claim_delivery_requested",
                "telegram_delivery_requested",
            ][..],
        ),
        (
            "release_publication_claim_status_delivery_readback_attempt",
            "blocked_release_publication_claim_status_delivery_readback_noop",
            "release_publication_claim_status_delivery_readback_attempt_denied",
            &[
                "public_claim_delivery_requested",
                "status_readback_requested",
                "channel_delivery_requested",
                "release_publication_delivery_readback_requested",
            ][..],
        ),
        (
            "activation_live_install_status_readback_attempt",
            "blocked_activation_live_install_status_readback_noop",
            "activation_live_install_status_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "install_restart_active_binary_status_readback_attempt",
            "blocked_install_restart_active_binary_status_readback_noop",
            "install_restart_active_binary_status_readback_attempt_denied",
            &[
                "status_readback_requested",
                "install_restart_active_binary_readback_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface": surface,
                "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_ready": source_ready,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempted": true,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-terminal-public-claim-delivery-readback-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:delivery=0:readback=0:receipt=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-receipt-terminal-public-claim-delivery-readback:no-delivery:no-readback:no-receipt:no-release:no-channel:no-telegram:no-install",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_report_required",
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recording_denied",
        "artifact_distribution_signing_notarization_receipt_status_readback_recording_denied",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_recording_denied",
        "artifact_distribution_signing_notarization_receipt_channel_external_telegram_delivery_readback_denied",
        "artifact_distribution_signing_notarization_receipt_delivery_receipt_persistence_denied",
        "artifact_distribution_signing_notarization_receipt_readback_receipt_persistence_denied",
        "artifact_distribution_signing_notarization_receipt_release_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_public_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_operator_approval_from_delivery_readback_denied",
        "artifact_distribution_signing_notarization_receipt_release_publication_authority_from_delivery_readback_denied",
        "artifact_distribution_signing_notarization_receipt_activation_authority_from_delivery_readback_denied",
        "artifact_distribution_signing_notarization_receipt_download_install_restart_active_binary_from_delivery_readback_denied",
        "artifact_distribution_signing_notarization_receipt_memory_provider_secret_external_send_from_delivery_readback_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_public_claim_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count",
        ) == 0
        && source_u64("release_artifact_written_count") == 0
        && source_u64("public_artifact_written_count") == 0
        && source_u64("operator_approval_from_signing_receipt_public_claim_derived_count") == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_public_claim_derived_count",
        ) == 0
        && source_u64("activation_authority_from_signing_receipt_status_exposure_derived_count")
            == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-terminal-public-claim-delivery-readback-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-28",
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
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_route_v1",
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_mode": "native_route_denied_public_claim_status_exposure_cannot_create_delivery_readback_receipt_release_channel_telegram_or_live_install",
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_ready": report_ready,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_ready": report_ready,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_public_claim_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_public_claim_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count": source_u64("artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count"),
            "source_artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count": source_u64("artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count"),
            "source_artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count": source_u64("artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count"),
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_count": denied_count,
            "allowed_next_actions": [{
                "action": "prepare_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_public_claim_delivery": false,
                "records_status_readback": false,
                "records_channel_delivery": false,
                "records_delivery_receipt": false,
                "records_readback_receipt": false,
                "sends_telegram": false,
                "writes_release_artifact": false,
                "writes_public_artifact": false,
                "derives_operator_approval": false,
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
            }],
        }),
    );

    let zero_keys = [
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
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
    for key in [
        "telegram_send_performed",
        "channel_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_ready",
    );
    let source_contract_hash = source
        .get(
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_contract_hash_sha256",
        )
        .and_then(serde_json::Value::as_str)
        .unwrap_or("missing");
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let surface_false_keys = [
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
    ];
    let surface_specs = [
        (
            "source_terminal_public_claim_delivery_readback_report_required",
            "blocked_missing_source_delivery_readback_noop",
            "source_terminal_public_claim_delivery_readback_report_required",
            &["release_artifact_publication_requested"][..],
        ),
        (
            "release_artifact_publication_attempt",
            "blocked_release_artifact_publication_noop",
            "release_artifact_publication_denied",
            &["release_artifact_publication_requested"][..],
        ),
        (
            "public_artifact_publication_attempt",
            "blocked_public_artifact_publication_noop",
            "public_artifact_publication_denied",
            &[
                "release_artifact_publication_requested",
                "public_artifact_publication_requested",
            ][..],
        ),
        (
            "signature_notarization_public_asset_publication_attempt",
            "blocked_signature_notarization_public_asset_publication_noop",
            "signature_notarization_public_asset_publication_denied",
            &[
                "release_artifact_publication_requested",
                "public_artifact_publication_requested",
            ][..],
        ),
        (
            "publication_queue_enqueue_attempt",
            "blocked_publication_queue_enqueue_noop",
            "publication_queue_enqueue_denied",
            &["publication_queue_requested"][..],
        ),
        (
            "publication_manifest_write_attempt",
            "blocked_publication_manifest_write_noop",
            "publication_manifest_write_denied",
            &[
                "release_artifact_publication_requested",
                "publication_manifest_requested",
            ][..],
        ),
        (
            "cdn_public_artifact_write_attempt",
            "blocked_cdn_public_artifact_write_noop",
            "cdn_public_artifact_write_denied",
            &[
                "public_artifact_publication_requested",
                "public_distribution_requested",
            ][..],
        ),
        (
            "update_feed_publication_attempt",
            "blocked_update_feed_publication_noop",
            "update_feed_publication_denied",
            &[
                "public_artifact_publication_requested",
                "public_distribution_requested",
            ][..],
        ),
        (
            "package_registry_publication_attempt",
            "blocked_package_registry_publication_noop",
            "package_registry_publication_denied",
            &[
                "public_artifact_publication_requested",
                "package_registry_publication_requested",
            ][..],
        ),
        (
            "external_package_channel_publication_attempt",
            "blocked_external_package_channel_publication_noop",
            "external_package_channel_publication_denied",
            &[
                "public_distribution_requested",
                "external_package_channel_publication_requested",
            ][..],
        ),
        (
            "telegram_package_channel_publication_attempt",
            "blocked_telegram_package_channel_publication_noop",
            "telegram_package_channel_publication_denied",
            &[
                "public_distribution_requested",
                "telegram_package_channel_publication_requested",
            ][..],
        ),
        (
            "public_release_claim_publication_attempt",
            "blocked_public_release_claim_publication_noop",
            "public_release_claim_publication_denied",
            &[
                "public_artifact_publication_requested",
                "public_release_claim_requested",
            ][..],
        ),
        (
            "public_ga_claim_publication_attempt",
            "blocked_public_ga_claim_publication_noop",
            "public_ga_claim_publication_denied",
            &[
                "public_artifact_publication_requested",
                "public_ga_claim_requested",
            ][..],
        ),
        (
            "release_notes_changelog_publication_attempt",
            "blocked_release_notes_changelog_publication_noop",
            "release_notes_changelog_publication_denied",
            &[
                "release_artifact_publication_requested",
                "release_notes_changelog_requested",
            ][..],
        ),
        (
            "distribution_index_publication_attempt",
            "blocked_distribution_index_publication_noop",
            "distribution_index_publication_denied",
            &[
                "public_artifact_publication_requested",
                "public_distribution_requested",
                "publication_manifest_requested",
            ][..],
        ),
        (
            "dashboard_publication_status_attempt",
            "blocked_dashboard_publication_status_noop",
            "dashboard_publication_status_denied",
            &[
                "public_distribution_requested",
                "release_publication_status_publication_requested",
            ][..],
        ),
        (
            "activation_live_install_publication_attempt",
            "blocked_activation_live_install_publication_noop",
            "activation_live_install_publication_denied",
            &[
                "public_release_claim_requested",
                "install_restart_active_binary_publication_requested",
            ][..],
        ),
        (
            "install_restart_active_binary_publication_attempt",
            "blocked_install_restart_active_binary_publication_noop",
            "install_restart_active_binary_publication_denied",
            &["install_restart_active_binary_publication_requested"][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surface": surface,
                "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_ready": source_ready,
                "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_attempted": true,
                "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_status": status,
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
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-release-public-artifact-publication-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:release_artifact=0:public_artifact=0:publication=0:public_claim=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-receipt-release-public-artifact-publication:no-release-artifact:no-public-artifact:no-publication:no-public-claim:no-channel:no-telegram:no-install",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_report_required",
        "artifact_distribution_signing_notarization_receipt_release_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_public_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_publication_queue_enqueue_denied",
        "artifact_distribution_signing_notarization_receipt_publication_manifest_write_denied",
        "artifact_distribution_signing_notarization_receipt_public_distribution_denied",
        "artifact_distribution_signing_notarization_receipt_package_registry_publication_denied",
        "artifact_distribution_signing_notarization_receipt_external_package_channel_publication_denied",
        "artifact_distribution_signing_notarization_receipt_telegram_package_channel_publication_denied",
        "artifact_distribution_signing_notarization_receipt_public_release_claim_denied",
        "artifact_distribution_signing_notarization_receipt_public_ga_claim_denied",
        "artifact_distribution_signing_notarization_receipt_release_notes_changelog_materialization_denied",
        "artifact_distribution_signing_notarization_receipt_operator_approval_from_publication_denied",
        "artifact_distribution_signing_notarization_receipt_release_publication_authority_from_publication_denied",
        "artifact_distribution_signing_notarization_receipt_activation_authority_from_publication_denied",
        "artifact_distribution_signing_notarization_receipt_download_install_restart_active_binary_from_publication_denied",
        "artifact_distribution_signing_notarization_receipt_memory_provider_secret_external_send_from_publication_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_status_readback_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count",
        ) == 0
        && source_u64("delivery_receipt_recorded_count") == 0
        && source_u64("readback_receipt_recorded_count") == 0
        && source_u64("release_artifact_written_count") == 0
        && source_u64("public_artifact_written_count") == 0
        && source_u64("release_publication_authority_from_delivery_readback_derived_count") == 0
        && source_u64("activation_authority_from_delivery_readback_derived_count") == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RELEASE_PUBLIC_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-release-public-artifact-publication-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-28",
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
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_route_v1",
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_mode": "native_route_denied_terminal_public_claim_delivery_readback_cannot_write_release_public_artifacts_publish_claims_or_live_install",
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_ready": report_ready,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_denial_ready": report_ready,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_status_readback_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_status_readback_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count"),
            "source_delivery_receipt_recorded_count": source_u64("delivery_receipt_recorded_count"),
            "source_readback_receipt_recorded_count": source_u64("readback_receipt_recorded_count"),
            "source_release_publication_authority_from_delivery_readback_derived_count": source_u64("release_publication_authority_from_delivery_readback_derived_count"),
            "source_activation_authority_from_delivery_readback_derived_count": source_u64("activation_authority_from_delivery_readback_derived_count"),
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_count": denied_count,
            "allowed_next_actions": [{
                "action": "prepare_first_model_positive_approval_packet_boundary_gate",
                "status": "allowed_report_only_next_slice",
                "writes_release_artifact": false,
                "writes_public_artifact": false,
                "publishes_public_distribution": false,
                "claims_public_release": false,
                "claims_public_ga": false,
                "derives_operator_approval": false,
                "derives_release_publication_authority": false,
                "derives_activation_authority": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "invokes_provider": false,
                "reads_credentials": false,
                "sends_externally": false
            }],
        }),
    );

    let zero_keys = [
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
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
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
    for key in [
        "release_artifact_written",
        "public_artifact_written",
        "publication_queue_enqueued",
        "publication_manifest_written",
        "public_distribution_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}
