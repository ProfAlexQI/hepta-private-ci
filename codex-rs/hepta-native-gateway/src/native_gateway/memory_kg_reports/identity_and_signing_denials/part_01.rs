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
