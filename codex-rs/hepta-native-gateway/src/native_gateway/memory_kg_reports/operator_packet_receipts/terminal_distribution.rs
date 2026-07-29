fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready")
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

    let operator_summary_briefing_surfaces = vec![
        "publication_result_receipt_operator_summary_request_claim",
        "publication_result_receipt_operator_briefing_request_claim",
        "publication_result_receipt_readback_digest_claim",
        "publication_result_receipt_final_note_claim",
        "publication_result_receipt_status_banner_claim",
        "publication_result_receipt_dashboard_annotation_claim",
        "publication_result_receipt_notification_preview_claim",
        "publication_result_receipt_timeline_entry_claim",
        "publication_result_receipt_audit_narrative_claim",
        "publication_result_receipt_privacy_review_narrative_claim",
        "publication_result_receipt_alert_explanation_claim",
        "publication_result_receipt_slo_report_claim",
        "publication_result_receipt_channel_delivery_summary_claim",
        "publication_result_receipt_external_send_summary_claim",
        "publication_result_receipt_telegram_briefing_claim",
        "publication_result_receipt_completion_ack_from_summary_claim",
        "publication_result_receipt_release_publication_authority_summary_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_summary_claim",
    ]
    .into_iter()
    .map(|summary_briefing_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_operator_facing_summary_briefing_surface"
                .to_string(),
            serde_json::json!(summary_briefing_surface),
        );
        surface.insert(
            "source_export_query_observability_ready".to_string(),
            serde_json::json!(source_ready),
        );
        surface.insert(
            "operator_facing_text_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "operator_facing_summary_briefing_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "operator_summary_briefing_status".to_string(),
            serde_json::json!(
                "release_publication_result_receipt_operator_summary_briefing_denied"
            ),
        );
        for key in [
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
            "dashboard_annotation_recorded",
            "notification_preview_recorded",
            "timeline_entry_recorded",
            "audit_narrative_recorded",
            "privacy_review_narrative_recorded",
            "alert_explanation_recorded",
            "slo_report_recorded",
            "channel_delivery_performed",
            "telegram_send_performed",
            "external_send_performed",
            "query_registered",
            "export_snapshot_recorded",
            "observability_event_recorded",
            "completion_ack_recorded",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
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
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let operator_summary_briefing_surface_count = operator_summary_briefing_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial:native:source={source_report_sha256}:surfaces={operator_summary_briefing_surface_count}:route_count={}:summary=0:briefing=0:delivery=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-operator-facing-summary-briefing-non-persistence:no-summary:no-briefing:no-readback:no-final-note:no-dashboard:no-delivery:no-authority:no-live",
    );
    let denials = vec![
        "source_export_query_observability_report_required",
        "operator_summary_request_acceptance_denied",
        "operator_briefing_request_acceptance_denied",
        "readback_digest_recording_denied",
        "final_note_recording_denied",
        "status_banner_recording_denied",
        "dashboard_annotation_recording_denied",
        "notification_preview_recording_denied",
        "timeline_entry_recording_denied",
        "audit_narrative_recording_denied",
        "privacy_review_narrative_recording_denied",
        "alert_explanation_recording_denied",
        "slo_report_recording_denied",
        "operator_summary_persistence_denied",
        "operator_briefing_persistence_denied",
        "readback_digest_persistence_denied",
        "final_note_persistence_denied",
        "operator_summary_materialization_denied",
        "operator_briefing_materialization_denied",
        "operator_summary_filesystem_write_denied",
        "operator_briefing_filesystem_write_denied",
        "operator_summary_delivery_denied",
        "operator_briefing_delivery_denied",
        "channel_delivery_denied",
        "external_send_denied",
        "telegram_send_denied",
        "completion_ack_from_summary_briefing_denied",
        "acceptance_from_summary_briefing_denied",
        "release_publication_authority_from_summary_briefing_denied",
        "activation_live_from_summary_briefing_denied",
        "install_restart_active_binary_from_summary_briefing_denied",
        "memory_provider_kg_from_summary_briefing_denied",
    ];
    let denials_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_export_query_observability_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_export_query_observability_attempt_count",
        ) == 18
        && source_u64("release_publication_result_receipt_operator_summary_recorded_count") == 0
        && source_u64("release_publication_result_receipt_readback_surface_recorded_count") == 0
        && source_u64("release_publication_result_receipt_audit_view_recorded_count") == 0
        && source_u64(
            "release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_export_query_observability_activation_authority_derived_count",
        ) == 0
        && operator_summary_briefing_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_operator_facing_summary_briefing_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
        "receipt_release_publication_result_receipt_operator_facing_summary_briefing_mode": "native_route_denied_release_publication_result_receipt_cannot_create_operator_facing_summary_briefing_readback_delivery_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_export_query_observability_contract_hash_sha256": source["release_publication_result_receipt_export_query_observability_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_operator_summary_briefing_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_operator_summary_briefing_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_export_query_observability_surface_count": source_u64("release_publication_result_receipt_export_query_observability_surface_count"),
            "source_release_publication_result_receipt_export_query_observability_attempt_count": source_u64("release_publication_result_receipt_export_query_observability_attempt_count"),
            "source_release_publication_result_receipt_operator_summary_recorded_count": source_u64("release_publication_result_receipt_operator_summary_recorded_count"),
            "source_release_publication_result_receipt_readback_surface_recorded_count": source_u64("release_publication_result_receipt_readback_surface_recorded_count"),
            "source_release_publication_result_receipt_audit_view_recorded_count": source_u64("release_publication_result_receipt_audit_view_recorded_count"),
            "source_release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_export_query_observability_activation_authority_derived_count": source_u64("release_publication_result_receipt_export_query_observability_activation_authority_derived_count"),
            "release_publication_result_receipt_operator_facing_summary_briefing_surface_count": operator_summary_briefing_surface_count,
            "release_publication_result_receipt_operator_facing_summary_briefing_attempt_count": operator_summary_briefing_surface_count,
            "release_publication_result_receipt_operator_summary_allowed_count": 0,
            "release_publication_result_receipt_operator_summary_request_accepted_count": 0,
            "release_publication_result_receipt_operator_summary_recorded_count": 0,
            "release_publication_result_receipt_operator_summary_persisted_count": 0,
            "release_publication_result_receipt_operator_summary_materialized_count": 0,
            "release_publication_result_receipt_operator_summary_filesystem_written_count": 0,
            "release_publication_result_receipt_operator_summary_delivered_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_operator_briefing_allowed_count": 0,
            "release_publication_result_receipt_operator_briefing_request_accepted_count": 0,
            "release_publication_result_receipt_operator_briefing_recorded_count": 0,
            "release_publication_result_receipt_operator_briefing_persisted_count": 0,
            "release_publication_result_receipt_operator_briefing_materialized_count": 0,
            "release_publication_result_receipt_operator_briefing_filesystem_written_count": 0,
            "release_publication_result_receipt_operator_briefing_delivered_count": 0,
            "release_publication_result_receipt_readback_digest_recorded_count": 0,
            "release_publication_result_receipt_readback_digest_persisted_count": 0,
            "release_publication_result_receipt_final_note_recorded_count": 0,
            "release_publication_result_receipt_final_note_persisted_count": 0,
            "release_publication_result_receipt_status_banner_recorded_count": 0,
            "release_publication_result_receipt_dashboard_annotation_recorded_count": 0,
            "release_publication_result_receipt_notification_preview_recorded_count": 0,
            "release_publication_result_receipt_timeline_entry_recorded_count": 0,
            "release_publication_result_receipt_audit_narrative_recorded_count": 0,
            "release_publication_result_receipt_privacy_review_narrative_recorded_count": 0,
            "release_publication_result_receipt_alert_explanation_recorded_count": 0,
            "release_publication_result_receipt_slo_report_recorded_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_operator_summary_briefing_channel_delivery_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_external_send_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_telegram_send_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_acceptance_recorded_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_operator_approval_derived_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_activation_command_derived_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_live_execution_allowed_count": 0,
            "release_publication_result_receipt_operator_facing_summary_briefing_surfaces": operator_summary_briefing_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_operator_facing_summary_briefing": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_operator_facing_summary_briefing_count": denials_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_summary": false,
                    "persists_summary": false,
                    "records_briefing": false,
                    "persists_briefing": false,
                    "delivers_briefing": false,
                    "records_acknowledgement": false,
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

    let summary_briefing_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_digest_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_note_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_status_banner_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_annotation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_notification_preview_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_timeline_entry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_narrative_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_privacy_review_narrative_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_alert_explanation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_slo_report_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_send_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_external_send_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
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
        for key in summary_briefing_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in summary_briefing_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_contract_hash = source
        .get("release_publication_result_receipt_operator_summary_briefing_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let final_acknowledgement_surfaces = vec![
        "publication_result_receipt_final_operator_acknowledgement_claim",
        "publication_result_receipt_operator_received_claim",
        "publication_result_receipt_operator_confirmed_claim",
        "publication_result_receipt_operator_read_claim",
        "publication_result_receipt_operator_seen_claim",
        "publication_result_receipt_final_response_claim",
        "publication_result_receipt_completion_acknowledgement_claim",
        "publication_result_receipt_status_acknowledgement_claim",
        "publication_result_receipt_summary_acknowledgement_claim",
        "publication_result_receipt_briefing_acknowledgement_claim",
        "publication_result_receipt_readback_digest_acknowledgement_claim",
        "publication_result_receipt_dashboard_notification_acknowledgement_claim",
        "publication_result_receipt_channel_acknowledgement_claim",
        "publication_result_receipt_external_acknowledgement_claim",
        "publication_result_receipt_telegram_acknowledgement_claim",
        "publication_result_receipt_release_publication_authority_acknowledgement_claim",
        "publication_result_receipt_activation_live_acknowledgement_claim",
        "publication_result_receipt_install_restart_active_binary_acknowledgement_claim",
    ]
    .into_iter()
    .map(|surface| {
        let mut surface_report = serde_json::json!({
            "release_publication_result_receipt_final_operator_acknowledgement_surface": surface,
            "source_operator_summary_briefing_ready": true,
            "final_operator_acknowledgement_attempted": true,
            "final_operator_acknowledgement_allowed": false,
            "final_operator_acknowledgement_request_accepted": false,
            "final_operator_acknowledgement_accepted": false,
            "final_operator_acknowledgement_recorded": false,
            "final_operator_acknowledgement_persisted": false,
            "final_operator_acknowledgement_materialized": false,
            "final_operator_acknowledgement_filesystem_written": false,
            "final_operator_acknowledgement_delivered": false,
            "operator_received_recorded": false,
            "operator_confirmed_recorded": false,
            "operator_read_recorded": false,
            "operator_seen_recorded": false,
            "final_response_recorded": false,
            "completion_ack_recorded": false,
            "status_ack_recorded": false,
            "summary_ack_recorded": false,
            "briefing_ack_recorded": false,
            "readback_digest_ack_recorded": false,
        });
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
            "dashboard_ack_recorded": false,
            "notification_ack_recorded": false,
            "channel_ack_delivered": false,
            "external_ack_sent": false,
            "telegram_ack_sent": false,
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
            }),
        );
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "live_kg_write_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_read": false,
            "secret_file_read": false,
            "final_operator_acknowledgement_noop_confirmed": true,
            "final_operator_acknowledgement_status": "blocked_final_operator_acknowledgement_noop"
            }),
        );
        surface_report
    })
    .collect::<Vec<_>>();
    let final_acknowledgement_surface_count = final_acknowledgement_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-final-operator-acknowledgement-non-acceptance-denial:native:source={source_report_sha256}:surfaces={final_acknowledgement_surface_count}:route_count={}:ack=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-final-operator-acknowledgement-non-acceptance:no-final-ack:no-received:no-confirmed:no-read:no-delivery:no-authority:no-live",
    );
    let denials = vec![
        "source_operator_summary_briefing_report_required",
        "final_operator_acknowledgement_request_acceptance_denied",
        "final_operator_acknowledgement_acceptance_denied",
        "final_operator_acknowledgement_recording_denied",
        "final_operator_acknowledgement_persistence_denied",
        "final_operator_acknowledgement_materialization_denied",
        "final_operator_acknowledgement_filesystem_write_denied",
        "final_operator_acknowledgement_delivery_denied",
        "operator_received_recording_denied",
        "operator_confirmed_recording_denied",
        "operator_read_recording_denied",
        "operator_seen_recording_denied",
        "final_response_recording_denied",
        "completion_acknowledgement_recording_denied",
        "status_acknowledgement_recording_denied",
        "summary_acknowledgement_recording_denied",
        "briefing_acknowledgement_recording_denied",
        "readback_digest_acknowledgement_recording_denied",
        "dashboard_notification_acknowledgement_recording_denied",
        "channel_acknowledgement_delivery_denied",
        "external_acknowledgement_send_denied",
        "telegram_acknowledgement_send_denied",
        "acceptance_from_final_operator_acknowledgement_denied",
        "operator_approval_from_final_operator_acknowledgement_denied",
        "release_publication_authority_from_final_operator_acknowledgement_denied",
        "activation_live_from_final_operator_acknowledgement_denied",
        "install_restart_active_binary_from_final_operator_acknowledgement_denied",
        "memory_provider_kg_from_final_operator_acknowledgement_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_operator_facing_summary_briefing_surface_count",
        ) == 18
        && source_u64("release_publication_result_receipt_operator_summary_recorded_count") == 0
        && source_u64("release_publication_result_receipt_operator_briefing_recorded_count") == 0
        && source_u64("release_publication_result_receipt_readback_digest_recorded_count") == 0
        && source_u64("release_publication_result_receipt_final_note_recorded_count") == 0
        && source_u64(
            "release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count",
        ) == 0
        && final_acknowledgement_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_final_operator_acknowledgement_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
        "receipt_release_publication_result_receipt_final_operator_acknowledgement_mode": "native_route_denied_release_publication_result_receipt_cannot_become_final_operator_acknowledgement_acceptance_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_operator_summary_briefing_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_final_operator_acknowledgement_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_final_operator_acknowledgement_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_operator_summary_briefing_surface_count": source_u64("release_publication_result_receipt_operator_facing_summary_briefing_surface_count"),
            "source_release_publication_result_receipt_operator_summary_briefing_attempt_count": source_u64("release_publication_result_receipt_operator_facing_summary_briefing_attempt_count"),
            "source_release_publication_result_receipt_operator_summary_recorded_count": source_u64("release_publication_result_receipt_operator_summary_recorded_count"),
            "source_release_publication_result_receipt_operator_briefing_recorded_count": source_u64("release_publication_result_receipt_operator_briefing_recorded_count"),
            "source_release_publication_result_receipt_readback_digest_recorded_count": source_u64("release_publication_result_receipt_readback_digest_recorded_count"),
            "source_release_publication_result_receipt_final_note_recorded_count": source_u64("release_publication_result_receipt_final_note_recorded_count"),
            "source_release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count": source_u64("release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count"),
            "source_release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count": source_u64("release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count"),
            "release_publication_result_receipt_final_operator_acknowledgement_surface_count": final_acknowledgement_surface_count,
            "release_publication_result_receipt_final_operator_acknowledgement_attempt_count": final_acknowledgement_surface_count,
            "release_publication_result_receipt_final_operator_acknowledgement_allowed_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_request_accepted_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_accepted_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_recorded_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_persisted_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_materialized_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_filesystem_written_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_delivered_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_operator_received_recorded_count": 0,
            "release_publication_result_receipt_operator_confirmed_recorded_count": 0,
            "release_publication_result_receipt_operator_read_recorded_count": 0,
            "release_publication_result_receipt_operator_seen_recorded_count": 0,
            "release_publication_result_receipt_final_response_recorded_count": 0,
            "release_publication_result_receipt_completion_ack_recorded_count": 0,
            "release_publication_result_receipt_status_ack_recorded_count": 0,
            "release_publication_result_receipt_summary_ack_recorded_count": 0,
            "release_publication_result_receipt_briefing_ack_recorded_count": 0,
            "release_publication_result_receipt_readback_digest_ack_recorded_count": 0,
            "release_publication_result_receipt_dashboard_ack_recorded_count": 0,
            "release_publication_result_receipt_notification_ack_recorded_count": 0,
            "release_publication_result_receipt_channel_ack_delivered_count": 0,
            "release_publication_result_receipt_external_ack_sent_count": 0,
            "release_publication_result_receipt_telegram_ack_sent_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_acceptance_recorded_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_operator_approval_derived_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_activation_command_derived_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_live_execution_allowed_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_install_executed_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_service_restarted_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_active_binary_mutated_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_final_operator_acknowledgement_surfaces": final_acknowledgement_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_final_operator_acknowledgement": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_final_operator_acknowledgement_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_acknowledgement": false,
                    "persists_acknowledgement": false,
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

    let final_ack_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_received_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_confirmed_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_read_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_seen_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_response_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_status_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_summary_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_briefing_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_digest_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_notification_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_ack_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_ack_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_ack_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_send_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_external_send_performed",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
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
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
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
        for key in final_ack_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in final_ack_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_contract_hash = source
        .get("release_publication_result_receipt_final_operator_acknowledgement_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let terminal_decision_status_surfaces = [
        "publication_result_receipt_terminal_decision_claim",
        "publication_result_receipt_terminal_status_closed_claim",
        "publication_result_receipt_final_state_promotion_claim",
        "publication_result_receipt_completion_promotion_claim",
        "publication_result_receipt_status_ready_claim",
        "publication_result_receipt_status_accepted_claim",
        "publication_result_receipt_status_approved_claim",
        "publication_result_receipt_status_authoritative_claim",
        "publication_result_receipt_status_live_claim",
        "publication_result_receipt_operator_decision_claim",
        "publication_result_receipt_public_status_claim",
        "publication_result_receipt_release_status_claim",
        "publication_result_receipt_publication_status_claim",
        "publication_result_receipt_dashboard_status_claim",
        "publication_result_receipt_channel_external_telegram_status_claim",
        "publication_result_receipt_release_publication_authority_status_claim",
        "publication_result_receipt_activation_live_status_claim",
        "publication_result_receipt_install_restart_active_binary_status_claim",
    ]
    .into_iter()
    .map(|surface| {
        let mut surface_report = serde_json::json!({
            "release_publication_result_receipt_terminal_decision_status_surface": surface,
            "source_final_operator_acknowledgement_ready": true,
            "terminal_decision_attempted": true,
            "terminal_decision_allowed": false,
            "terminal_decision_request_accepted": false,
            "terminal_decision_accepted": false,
            "terminal_decision_recorded": false,
            "terminal_decision_persisted": false,
            "terminal_decision_materialized": false,
            "terminal_decision_filesystem_written": false,
            "terminal_decision_delivered": false,
            "terminal_status_recorded": false,
            "terminal_status_persisted": false,
            "terminal_status_closed": false,
            "terminal_status_ready": false,
            "terminal_status_accepted": false,
            "terminal_status_approved": false,
            "terminal_status_authoritative": false,
            "terminal_status_live": false,
            "final_state_promoted": false,
            "completion_promoted": false,
            "operator_decision_recorded": false,
            "public_status_claimed": false,
            "release_status_claimed": false,
            "publication_status_claimed": false,
            "dashboard_status_recorded": false,
            "channel_status_delivered": false,
        });
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
                "external_status_sent": false,
                "telegram_status_sent": false,
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
                "terminal_decision_status_noop_confirmed": true,
                "terminal_decision_status": "terminal_decision_status_promotion_denied"
            }),
        );
        surface_report
    })
    .collect::<Vec<_>>();
    let terminal_decision_status_surface_count = terminal_decision_status_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-decision-status-promotion-denial:native:source={source_report_sha256}:surfaces={terminal_decision_status_surface_count}:route_count={}:terminal=0:status=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-decision-status-promotion-denial:no-terminal-decision:no-status:no-release-authority:no-live:no-active-binary",
    );
    let denials = vec![
        "source_final_operator_acknowledgement_report_required",
        "terminal_decision_request_acceptance_denied",
        "terminal_decision_acceptance_denied",
        "terminal_decision_recording_denied",
        "terminal_decision_persistence_denied",
        "terminal_decision_materialization_denied",
        "terminal_decision_filesystem_write_denied",
        "terminal_decision_delivery_denied",
        "terminal_status_recording_denied",
        "terminal_status_persistence_denied",
        "terminal_status_closed_denied",
        "status_ready_denied",
        "status_accepted_denied",
        "status_approved_denied",
        "status_authoritative_denied",
        "status_live_denied",
        "final_state_promotion_denied",
        "completion_promotion_denied",
        "operator_decision_recording_denied",
        "public_status_claim_denied",
        "release_status_claim_denied",
        "publication_status_claim_denied",
        "dashboard_status_recording_denied",
        "channel_status_delivery_denied",
        "external_status_send_denied",
        "telegram_status_send_denied",
        "acceptance_from_terminal_status_denied",
        "operator_approval_from_terminal_status_denied",
        "release_publication_authority_from_terminal_status_denied",
        "activation_live_from_terminal_status_denied",
        "install_restart_active_binary_from_terminal_status_denied",
        "memory_provider_kg_from_terminal_status_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_persisted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_materialized_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_delivered_count",
        ) == 0
        && source_u64("release_publication_result_receipt_operator_received_recorded_count") == 0
        && source_u64("release_publication_result_receipt_operator_confirmed_recorded_count") == 0
        && source_u64("release_publication_result_receipt_completion_ack_recorded_count") == 0
        && source_u64("release_publication_result_receipt_status_ack_recorded_count") == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count",
        ) == 0
        && terminal_decision_status_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-decision-status-promotion-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-17",
        "receipt_release_publication_result_receipt_terminal_decision_status_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_v1",
        "receipt_release_publication_result_receipt_terminal_decision_status_mode": "native_route_non_accepted_final_operator_acknowledgement_cannot_become_terminal_decision_status_release_publication_or_activation_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_final_operator_acknowledgement_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_decision_status_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_decision_status_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_final_operator_acknowledgement_surface_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_surface_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_attempt_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_attempt_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_accepted_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_accepted_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_recorded_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_recorded_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_persisted_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_persisted_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_materialized_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_materialized_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_delivered_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_delivered_count"),
            "source_release_publication_result_receipt_operator_received_recorded_count": source_u64("release_publication_result_receipt_operator_received_recorded_count"),
            "source_release_publication_result_receipt_operator_confirmed_recorded_count": source_u64("release_publication_result_receipt_operator_confirmed_recorded_count"),
            "source_release_publication_result_receipt_completion_ack_recorded_count": source_u64("release_publication_result_receipt_completion_ack_recorded_count"),
            "source_release_publication_result_receipt_status_ack_recorded_count": source_u64("release_publication_result_receipt_status_ack_recorded_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_decision_status_surface_count": terminal_decision_status_surface_count,
            "release_publication_result_receipt_terminal_decision_status_attempt_count": terminal_decision_status_surface_count,
            "release_publication_result_receipt_terminal_decision_allowed_count": 0,
            "release_publication_result_receipt_terminal_decision_request_accepted_count": 0,
            "release_publication_result_receipt_terminal_decision_accepted_count": 0,
            "release_publication_result_receipt_terminal_decision_recorded_count": 0,
            "release_publication_result_receipt_terminal_decision_persisted_count": 0,
            "release_publication_result_receipt_terminal_decision_materialized_count": 0,
            "release_publication_result_receipt_terminal_decision_filesystem_written_count": 0,
            "release_publication_result_receipt_terminal_decision_delivered_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_status_recorded_count": 0,
            "release_publication_result_receipt_terminal_status_persisted_count": 0,
            "release_publication_result_receipt_terminal_status_closed_count": 0,
            "release_publication_result_receipt_status_ready_count": 0,
            "release_publication_result_receipt_status_accepted_count": 0,
            "release_publication_result_receipt_status_approved_count": 0,
            "release_publication_result_receipt_status_authoritative_count": 0,
            "release_publication_result_receipt_status_live_count": 0,
            "release_publication_result_receipt_final_state_promoted_count": 0,
            "release_publication_result_receipt_completion_promoted_count": 0,
            "release_publication_result_receipt_operator_decision_recorded_count": 0,
            "release_publication_result_receipt_public_status_claimed_count": 0,
            "release_publication_result_receipt_release_status_claimed_count": 0,
            "release_publication_result_receipt_publication_status_claimed_count": 0,
            "release_publication_result_receipt_dashboard_status_recorded_count": 0,
            "release_publication_result_receipt_channel_status_delivered_count": 0,
            "release_publication_result_receipt_external_status_sent_count": 0,
            "release_publication_result_receipt_telegram_status_sent_count": 0,
            "release_publication_result_receipt_terminal_decision_status_acceptance_recorded_count": 0,
            "release_publication_result_receipt_terminal_decision_status_operator_approval_derived_count": 0,
            "release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_decision_status_activation_command_derived_count": 0,
            "release_publication_result_receipt_terminal_decision_status_live_execution_allowed_count": 0,
            "release_publication_result_receipt_terminal_decision_status_install_executed_count": 0,
            "release_publication_result_receipt_terminal_decision_status_service_restarted_count": 0,
            "release_publication_result_receipt_terminal_decision_status_active_binary_mutated_count": 0,
            "release_publication_result_receipt_terminal_decision_status_surfaces": terminal_decision_status_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_decision_status": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_decision_status_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_terminal_decision": false,
                    "promotes_status": false,
                    "claims_public_status": false,
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

    let terminal_decision_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_received_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_confirmed_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_read_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_seen_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_response_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_status_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_closed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_ready",
        "packet_acceptance_receipt_release_publication_result_receipt_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_status_approved",
        "packet_acceptance_receipt_release_publication_result_receipt_status_authoritative",
        "packet_acceptance_receipt_release_publication_result_receipt_status_live",
        "packet_acceptance_receipt_release_publication_result_receipt_final_state_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_completion_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_public_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_publication_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent",
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
        for key in terminal_decision_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in terminal_decision_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report_sha256 = sha256_text_value(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-decision-status-promotion-denial:native-source-summary:v1:surfaces=18:terminal=0:status=0:authority=0:live=0",
    );
    let source_ready = true;
    let source_u64 = |key: &str| match key {
        "release_publication_result_receipt_terminal_decision_status_surface_count" => 18,
        "release_publication_result_receipt_terminal_decision_status_attempt_count" => 18,
        "release_publication_result_receipt_terminal_decision_accepted_count"
        | "release_publication_result_receipt_terminal_decision_recorded_count"
        | "release_publication_result_receipt_terminal_status_recorded_count"
        | "release_publication_result_receipt_public_status_claimed_count"
        | "release_publication_result_receipt_release_status_claimed_count"
        | "release_publication_result_receipt_publication_status_claimed_count"
        | "release_publication_result_receipt_dashboard_status_recorded_count"
        | "release_publication_result_receipt_channel_status_delivered_count"
        | "release_publication_result_receipt_external_status_sent_count"
        | "release_publication_result_receipt_telegram_status_sent_count"
        | "release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count"
        | "release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count" => {
            0
        }
        _ => 0,
    };
    let source_contract_hash = serde_json::json!(sha256_text_value(
        "release-publication-result-receipt-terminal-decision-status-promotion-denial:native-source-summary:no-terminal-decision:no-status:no-release-authority:no-live:no-active-binary",
    ));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let terminal_public_claim_status_exposure_surfaces = [
        "publication_result_receipt_public_claim_status_claim",
        "publication_result_receipt_release_claim_status_claim",
        "publication_result_receipt_publication_claim_status_claim",
        "publication_result_receipt_ga_stable_claim_status_claim",
        "publication_result_receipt_dashboard_public_badge_status_claim",
        "publication_result_receipt_status_endpoint_claim",
        "publication_result_receipt_query_status_claim",
        "publication_result_receipt_export_status_claim",
        "publication_result_receipt_observability_status_claim",
        "publication_result_receipt_release_notes_status_claim",
        "publication_result_receipt_changelog_status_claim",
        "publication_result_receipt_version_tag_status_claim",
        "publication_result_receipt_artifact_availability_status_claim",
        "publication_result_receipt_distribution_queue_status_claim",
        "publication_result_receipt_channel_external_telegram_public_status_claim",
        "publication_result_receipt_release_publication_authority_public_status_claim",
        "publication_result_receipt_activation_live_public_status_claim",
        "publication_result_receipt_install_restart_active_binary_public_status_claim",
    ]
    .into_iter()
    .map(|surface| {
        let mut surface_report = serde_json::json!({
            "release_publication_result_receipt_terminal_public_claim_status_exposure_surface": surface,
            "source_terminal_decision_status_ready": true,
            "public_claim_status_exposure_attempted": true,
            "public_claim_status_exposure_allowed": false,
            "public_claim_status_exposure_request_accepted": false,
            "public_claim_status_exposure_accepted": false,
            "public_claim_status_exposure_recorded": false,
            "public_claim_status_exposure_persisted": false,
            "public_claim_status_exposure_materialized": false,
            "public_claim_status_exposure_filesystem_written": false,
            "public_claim_status_exposure_delivered": false,
            "public_claim_status_exposed": false,
            "public_status_claimed": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_status_exposed": false,
            "publication_status_exposed": false,
            "dashboard_status_exposed": false,
            "public_badge_exposed": false,
            "status_endpoint_exposed": false,
            "query_status_exposed": false,
            "export_status_exposed": false,
            "observability_status_exposed": false,
            "release_notes_status_exposed": false,
            "changelog_status_exposed": false,
            "version_tag_status_exposed": false,
            "artifact_availability_status_exposed": false,
            "distribution_queue_status_exposed": false,
            "channel_status_delivered": false,
        });
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
                "external_status_sent": false,
                "telegram_status_sent": false,
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
                "public_claim_status_exposure_noop_confirmed": true,
                "public_claim_status_exposure_status": "public_claim_status_exposure_denied"
            }),
        );
        surface_report
    })
    .collect::<Vec<_>>();
    let terminal_public_claim_status_exposure_surface_count =
        terminal_public_claim_status_exposure_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial:native:source={source_report_sha256}:surfaces={terminal_public_claim_status_exposure_surface_count}:route_count={}:public=0:status=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-public-claim-status-exposure-denial:no-public-claim:no-dashboard:no-channel:no-telegram:no-ga:no-live",
    );
    let denials = vec![
        "source_terminal_decision_status_report_required",
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
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_terminal_decision_status_surface_count")
            == 18
        && source_u64("release_publication_result_receipt_terminal_decision_status_attempt_count")
            == 18
        && source_u64("release_publication_result_receipt_terminal_decision_accepted_count") == 0
        && source_u64("release_publication_result_receipt_terminal_decision_recorded_count") == 0
        && source_u64("release_publication_result_receipt_terminal_status_recorded_count") == 0
        && source_u64("release_publication_result_receipt_public_status_claimed_count") == 0
        && source_u64("release_publication_result_receipt_release_status_claimed_count") == 0
        && source_u64("release_publication_result_receipt_publication_status_claimed_count") == 0
        && source_u64("release_publication_result_receipt_dashboard_status_recorded_count") == 0
        && source_u64("release_publication_result_receipt_channel_status_delivered_count") == 0
        && source_u64("release_publication_result_receipt_external_status_sent_count") == 0
        && source_u64("release_publication_result_receipt_telegram_status_sent_count") == 0
        && source_u64(
            "release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count",
        ) == 0
        && terminal_public_claim_status_exposure_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-17",
        "receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_v1",
        "receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_mode": "native_route_denied_terminal_status_cannot_be_exposed_as_public_release_publication_or_activation_status",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_decision_status_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_public_claim_status_exposure_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_public_claim_status_exposure_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_decision_status_surface_count": source_u64("release_publication_result_receipt_terminal_decision_status_surface_count"),
            "source_release_publication_result_receipt_terminal_decision_status_attempt_count": source_u64("release_publication_result_receipt_terminal_decision_status_attempt_count"),
            "source_release_publication_result_receipt_terminal_decision_accepted_count": source_u64("release_publication_result_receipt_terminal_decision_accepted_count"),
            "source_release_publication_result_receipt_terminal_decision_recorded_count": source_u64("release_publication_result_receipt_terminal_decision_recorded_count"),
            "source_release_publication_result_receipt_terminal_status_recorded_count": source_u64("release_publication_result_receipt_terminal_status_recorded_count"),
            "source_release_publication_result_receipt_public_status_claimed_count": source_u64("release_publication_result_receipt_public_status_claimed_count"),
            "source_release_publication_result_receipt_release_status_claimed_count": source_u64("release_publication_result_receipt_release_status_claimed_count"),
            "source_release_publication_result_receipt_publication_status_claimed_count": source_u64("release_publication_result_receipt_publication_status_claimed_count"),
            "source_release_publication_result_receipt_dashboard_status_recorded_count": source_u64("release_publication_result_receipt_dashboard_status_recorded_count"),
            "source_release_publication_result_receipt_channel_status_delivered_count": source_u64("release_publication_result_receipt_channel_status_delivered_count"),
            "source_release_publication_result_receipt_external_status_sent_count": source_u64("release_publication_result_receipt_external_status_sent_count"),
            "source_release_publication_result_receipt_telegram_status_sent_count": source_u64("release_publication_result_receipt_telegram_status_sent_count"),
            "source_release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count": terminal_public_claim_status_exposure_surface_count,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count": terminal_public_claim_status_exposure_surface_count,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_allowed_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_request_accepted_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_persisted_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_materialized_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_filesystem_written_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_delivered_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposed_count": 0,
            "release_publication_result_receipt_public_status_claimed_count": 0,
            "release_publication_result_receipt_public_release_claimed_count": 0,
            "release_publication_result_receipt_public_ga_claimed_count": 0,
            "release_publication_result_receipt_release_status_exposed_count": 0,
            "release_publication_result_receipt_publication_status_exposed_count": 0,
            "release_publication_result_receipt_dashboard_status_exposed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_public_badge_exposed_count": 0,
            "release_publication_result_receipt_status_endpoint_exposed_count": 0,
            "release_publication_result_receipt_query_status_exposed_count": 0,
            "release_publication_result_receipt_export_status_exposed_count": 0,
            "release_publication_result_receipt_observability_status_exposed_count": 0,
            "release_publication_result_receipt_release_notes_status_exposed_count": 0,
            "release_publication_result_receipt_changelog_status_exposed_count": 0,
            "release_publication_result_receipt_version_tag_status_exposed_count": 0,
            "release_publication_result_receipt_artifact_availability_status_exposed_count": 0,
            "release_publication_result_receipt_distribution_queue_status_exposed_count": 0,
            "release_publication_result_receipt_channel_status_delivered_count": 0,
            "release_publication_result_receipt_external_status_sent_count": 0,
            "release_publication_result_receipt_telegram_status_sent_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_acceptance_recorded_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_operator_approval_derived_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_activation_command_derived_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_live_execution_allowed_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_install_executed_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_service_restarted_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_active_binary_mutated_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_surfaces": terminal_public_claim_status_exposure_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_status_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "exposes_public_status": false,
                    "claims_public_release": false,
                    "claims_public_ga": false,
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

    let public_claim_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_closed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_ready",
        "packet_acceptance_receipt_release_publication_result_receipt_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_status_approved",
        "packet_acceptance_receipt_release_publication_result_receipt_status_authoritative",
        "packet_acceptance_receipt_release_publication_result_receipt_status_live",
        "packet_acceptance_receipt_release_publication_result_receipt_public_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_publication_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_public_badge_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_query_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_notes_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_changelog_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_version_tag_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed",
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
        for key in public_claim_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in public_claim_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report_sha256 = sha256_text_value(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial:native-source-summary:v1:surfaces=18:public=0:status=0:authority=0:live=0",
    );
    let source_ready = true;
    let source_u64 = |key: &str| match key {
        "release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count" => {
            18
        }
        "release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count" => {
            18
        }
        "release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count"
        | "release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count"
        | "release_publication_result_receipt_terminal_public_claim_status_exposed_count"
        | "release_publication_result_receipt_artifact_availability_status_exposed_count"
        | "release_publication_result_receipt_distribution_queue_status_exposed_count"
        | "release_publication_result_receipt_channel_status_delivered_count"
        | "release_publication_result_receipt_external_status_sent_count"
        | "release_publication_result_receipt_telegram_status_sent_count"
        | "release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count"
        | "release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count" => {
            0
        }
        _ => 0,
    };
    let source_contract_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-public-claim-status-exposure-denial:native-source-summary:no-public-claim:no-dashboard:no-channel:no-telegram:no-ga:no-live",
    );
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let terminal_distribution_artifact_status_surfaces = [
        "publication_result_receipt_distribution_queue_ready_status",
        "publication_result_receipt_distribution_queue_enqueued_status",
        "publication_result_receipt_distribution_worker_dispatch_status",
        "publication_result_receipt_artifact_availability_ready_status",
        "publication_result_receipt_artifact_manifest_entry_status",
        "publication_result_receipt_artifact_download_url_status",
        "publication_result_receipt_artifact_checksum_status",
        "publication_result_receipt_artifact_signature_notarization_status",
        "publication_result_receipt_package_index_status",
        "publication_result_receipt_update_feed_status",
        "publication_result_receipt_cdn_mirror_status",
        "publication_result_receipt_release_channel_status",
        "publication_result_receipt_public_bucket_listing_status",
        "publication_result_receipt_status_endpoint_artifact_ready_status",
        "publication_result_receipt_dashboard_artifact_available_badge_status",
        "publication_result_receipt_channel_external_telegram_distribution_status",
        "publication_result_receipt_release_publication_authority_distribution_status",
        "publication_result_receipt_activation_live_install_restart_active_binary_distribution_status",
    ]
    .into_iter()
    .map(|surface| {
        let mut surface_report = serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_artifact_status_surface": surface,
            "source_terminal_public_claim_status_exposure_ready": true,
            "terminal_distribution_artifact_status_attempted": true,
            "terminal_distribution_artifact_status_allowed": false,
            "terminal_distribution_artifact_status_request_accepted": false,
            "terminal_distribution_artifact_status_accepted": false,
            "terminal_distribution_artifact_status_recorded": false,
            "terminal_distribution_artifact_status_persisted": false,
            "terminal_distribution_artifact_status_materialized": false,
            "terminal_distribution_artifact_status_filesystem_written": false,
            "terminal_distribution_artifact_status_delivered": false,
            "terminal_distribution_artifact_status_exposed": false,
            "distribution_queue_status_exposed": false,
            "distribution_queue_enqueued": false,
            "distribution_worker_dispatched": false,
            "artifact_availability_status_exposed": false,
            "artifact_manifest_entry_exposed": false,
            "artifact_download_url_exposed": false,
            "artifact_checksum_exposed": false,
            "artifact_signature_notarization_exposed": false,
            "package_index_status_exposed": false,
            "update_feed_status_exposed": false,
            "cdn_mirror_status_exposed": false,
            "release_channel_status_exposed": false,
            "public_bucket_listing_status_exposed": false,
            "status_endpoint_artifact_ready_exposed": false,
            "dashboard_artifact_available_badge_exposed": false,
            "channel_status_delivered": false,
        });
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
                "external_status_sent": false,
                "telegram_status_sent": false,
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
                "provider_invoked": false,
                "model_invoked": false,
                "credential_read": false,
                "secret_file_read": false,
                "terminal_distribution_artifact_status_noop_confirmed": true,
                "terminal_distribution_artifact_status": "terminal_distribution_artifact_status_denied"
            }),
        );
        surface_report
    })
    .collect::<Vec<_>>();
    let terminal_distribution_artifact_status_surface_count =
        terminal_distribution_artifact_status_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial:native:source={source_report_sha256}:surfaces={terminal_distribution_artifact_status_surface_count}:route_count={}:distribution=0:artifact=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-artifact-status-denial:no-distribution-queue:no-artifact-availability:no-download-url:no-feed:no-channel:no-active-binary",
    );
    let denials = vec![
        "source_terminal_public_claim_status_exposure_report_required",
        "distribution_artifact_status_request_acceptance_denied",
        "distribution_artifact_status_acceptance_denied",
        "distribution_artifact_status_recording_denied",
        "distribution_artifact_status_persistence_denied",
        "distribution_artifact_status_materialization_denied",
        "distribution_artifact_status_filesystem_write_denied",
        "distribution_artifact_status_delivery_denied",
        "distribution_artifact_status_exposure_denied",
        "distribution_queue_status_exposure_denied",
        "distribution_queue_enqueue_denied",
        "distribution_worker_dispatch_denied",
        "artifact_availability_status_exposure_denied",
        "artifact_manifest_entry_exposure_denied",
        "artifact_download_url_exposure_denied",
        "artifact_checksum_exposure_denied",
        "artifact_signature_notarization_status_denied",
        "package_index_status_exposure_denied",
        "update_feed_status_exposure_denied",
        "cdn_mirror_status_exposure_denied",
        "release_channel_status_exposure_denied",
        "public_bucket_listing_status_exposure_denied",
        "status_endpoint_artifact_ready_exposure_denied",
        "dashboard_artifact_available_badge_exposure_denied",
        "channel_status_delivery_denied",
        "external_status_send_denied",
        "telegram_status_send_denied",
        "release_artifact_write_denied",
        "public_artifact_write_denied",
        "acceptance_from_distribution_status_denied",
        "operator_approval_from_distribution_status_denied",
        "release_publication_authority_from_distribution_status_denied",
        "activation_live_from_distribution_status_denied",
        "install_restart_active_binary_from_distribution_status_denied",
        "memory_provider_kg_from_distribution_status_denied",
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
            "release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposed_count",
        ) == 0
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
        && terminal_distribution_artifact_status_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_QUEUE_ARTIFACT_AVAILABILITY_STATUS_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-17",
        "receipt_release_publication_result_receipt_terminal_distribution_artifact_status_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_artifact_status_mode": "native_route_denied_terminal_public_status_cannot_become_distribution_queue_or_artifact_availability_status",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_public_claim_status_exposure_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_artifact_status_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_artifact_status_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposed_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposed_count"),
            "source_release_publication_result_receipt_artifact_availability_status_exposed_count": source_u64("release_publication_result_receipt_artifact_availability_status_exposed_count"),
            "source_release_publication_result_receipt_distribution_queue_status_exposed_count": source_u64("release_publication_result_receipt_distribution_queue_status_exposed_count"),
            "source_release_publication_result_receipt_channel_status_delivered_count": source_u64("release_publication_result_receipt_channel_status_delivered_count"),
            "source_release_publication_result_receipt_external_status_sent_count": source_u64("release_publication_result_receipt_external_status_sent_count"),
            "source_release_publication_result_receipt_telegram_status_sent_count": source_u64("release_publication_result_receipt_telegram_status_sent_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_distribution_artifact_status_surface_count": terminal_distribution_artifact_status_surface_count,
            "release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count": terminal_distribution_artifact_status_surface_count,
            "release_publication_result_receipt_terminal_distribution_artifact_status_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_request_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_materialized_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count": 0,
            "release_publication_result_receipt_distribution_queue_status_exposed_count": 0,
            "release_publication_result_receipt_distribution_queue_enqueued_count": 0,
            "release_publication_result_receipt_distribution_worker_dispatched_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_artifact_availability_status_exposed_count": 0,
            "release_publication_result_receipt_artifact_manifest_entry_exposed_count": 0,
            "release_publication_result_receipt_artifact_download_url_exposed_count": 0,
            "release_publication_result_receipt_artifact_checksum_exposed_count": 0,
            "release_publication_result_receipt_artifact_signature_notarization_exposed_count": 0,
            "release_publication_result_receipt_package_index_status_exposed_count": 0,
            "release_publication_result_receipt_update_feed_status_exposed_count": 0,
            "release_publication_result_receipt_cdn_mirror_status_exposed_count": 0,
            "release_publication_result_receipt_release_channel_status_exposed_count": 0,
            "release_publication_result_receipt_public_bucket_listing_status_exposed_count": 0,
            "release_publication_result_receipt_status_endpoint_artifact_ready_exposed_count": 0,
            "release_publication_result_receipt_dashboard_artifact_available_badge_exposed_count": 0,
            "release_publication_result_receipt_channel_status_delivered_count": 0,
            "release_publication_result_receipt_external_status_sent_count": 0,
            "release_publication_result_receipt_telegram_status_sent_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_acceptance_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_operator_approval_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_activation_command_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_live_execution_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_install_executed_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_service_restarted_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_active_binary_mutated_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_release_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_public_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_surfaces": terminal_distribution_artifact_status_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_artifact_status": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "exposes_distribution_queue": false,
                    "exposes_artifact_availability": false,
                    "writes_release_artifact": false,
                    "writes_public_artifact": false,
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

    let distribution_artifact_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_enqueued",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_worker_dispatched",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_manifest_entry_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_download_url_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_checksum_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_signature_notarization_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_package_index_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_update_feed_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_cdn_mirror_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_channel_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_public_bucket_listing_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_artifact_ready_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_artifact_available_badge_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent",
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
        for key in distribution_artifact_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in distribution_artifact_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}
