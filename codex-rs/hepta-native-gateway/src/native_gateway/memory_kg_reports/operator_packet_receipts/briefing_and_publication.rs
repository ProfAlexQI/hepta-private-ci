
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_report();
    let source_view_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_export_query_observability_surface_count = source
        .get("export_query_observability_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_query_registered_count = source
        .get("query_registered_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_export_snapshot_recorded_count = source
        .get("export_snapshot_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_observability_metric_recorded_count = source
        .get("observability_metric_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_summary_recorded_count = source
        .get("operator_summary_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_readback_surface_recorded_count = source
        .get("readback_surface_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_export_query_observability_activation_authority_derived_count = source
        .get("export_query_observability_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_export_query_observability_contract_hash_sha256 = source
        .get("export_query_observability_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let redaction_privacy_surfaces = vec![
        "packet_receipt_redacted_payload_preview_claim",
        "packet_receipt_payload_hash_preview_claim",
        "packet_receipt_payload_diff_claim",
        "packet_receipt_readback_text_claim",
        "packet_receipt_operator_summary_text_claim",
        "packet_receipt_privacy_review_claim",
        "packet_receipt_secret_scan_claim",
        "packet_receipt_pii_scan_claim",
        "packet_receipt_raw_payload_inspection_claim",
        "packet_receipt_plaintext_materialization_claim",
        "packet_receipt_redaction_bypass_claim",
        "packet_receipt_hash_to_payload_link_claim",
        "packet_receipt_external_redaction_review_claim",
        "packet_receipt_privacy_acceptance_claim",
        "packet_receipt_authority_from_redaction_claim",
        "packet_receipt_live_from_privacy_claim",
    ]
    .into_iter()
    .map(|redaction_privacy_surface| {
        serde_json::json!({
            "redaction_privacy_surface": redaction_privacy_surface,
            "redaction_privacy_or_payload_exposure_attempted": true,
            "redacted_payload_preview_recorded": false,
            "payload_hash_preview_recorded": false,
            "payload_diff_recorded": false,
            "readback_text_recorded": false,
            "operator_summary_text_recorded": false,
            "privacy_review_recorded": false,
            "privacy_review_persisted": false,
            "secret_scan_performed": false,
            "pii_scan_performed": false,
            "raw_payload_inspected": false,
            "plaintext_materialized": false,
            "redaction_bypass_allowed": false,
            "hash_to_payload_link_recorded": false,
            "external_redaction_review_performed": false,
            "privacy_acceptance_recorded": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "redaction_privacy_status": "redaction_privacy_payload_exposure_denied"
        })
    })
    .collect::<Vec<_>>();
    let redaction_privacy_surface_count = redaction_privacy_surfaces.len();
    let redaction_privacy_payload_exposure_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial:native:source={source_view_report_sha256}:surfaces={redaction_privacy_surface_count}:route_count={}:redaction=0:privacy=0:payload=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_redaction_privacy = vec![
        "operator_readiness_packet_template_packet_receipt_redacted_payload_preview_denied",
        "operator_readiness_packet_template_packet_receipt_payload_hash_preview_denied",
        "operator_readiness_packet_template_packet_receipt_payload_diff_denied",
        "operator_readiness_packet_template_packet_receipt_readback_text_denied",
        "operator_readiness_packet_template_packet_receipt_operator_summary_text_denied",
        "operator_readiness_packet_template_packet_receipt_privacy_review_recording_denied",
        "operator_readiness_packet_template_packet_receipt_privacy_review_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_secret_scan_denied",
        "operator_readiness_packet_template_packet_receipt_pii_scan_denied",
        "operator_readiness_packet_template_packet_receipt_raw_payload_inspection_denied",
        "operator_readiness_packet_template_packet_receipt_plaintext_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_redaction_bypass_denied",
        "operator_readiness_packet_template_packet_receipt_hash_to_payload_link_denied",
        "operator_readiness_packet_template_packet_receipt_external_redaction_review_denied",
        "operator_readiness_packet_template_packet_receipt_privacy_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_redaction_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_privacy_denied",
    ];
    let denied_by_packet_receipt_redaction_privacy_count =
        denied_by_packet_receipt_redaction_privacy.len();
    let report_ready = source_ready
        && source_export_query_observability_surface_count == 16
        && source_query_registered_count == 0
        && source_export_snapshot_recorded_count == 0
        && source_observability_metric_recorded_count == 0
        && source_operator_summary_recorded_count == 0
        && source_readback_surface_recorded_count == 0
        && source_export_query_observability_activation_authority_derived_count == 0
        && redaction_privacy_surface_count == 16
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_REDACTION_PRIVACY_PAYLOAD_EXPOSURE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "receipt_redaction_privacy_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_v1",
        "receipt_redaction_privacy_mode": "native_route_non_persistent_receipts_cannot_expose_payload_or_create_privacy_authority",
        "source_packet_acceptance_receipt_export_query_observability_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_export_query_observability_ready": source_ready,
        "source_view_report_sha256": source_view_report_sha256,
        "source_export_query_observability_contract_hash_sha256": source_export_query_observability_contract_hash_sha256,
        "redaction_privacy_payload_exposure_contract_hash_sha256": redaction_privacy_payload_exposure_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready": report_ready,
            "source_export_query_observability_surface_count": source_export_query_observability_surface_count,
            "source_query_registered_count": source_query_registered_count,
            "source_export_snapshot_recorded_count": source_export_snapshot_recorded_count,
            "source_observability_metric_recorded_count": source_observability_metric_recorded_count,
            "source_operator_summary_recorded_count": source_operator_summary_recorded_count,
            "source_readback_surface_recorded_count": source_readback_surface_recorded_count,
            "source_export_query_observability_activation_authority_derived_count": source_export_query_observability_activation_authority_derived_count,
            "redaction_privacy_surface_count": redaction_privacy_surface_count,
            "redaction_privacy_attempt_count": redaction_privacy_surface_count,
            "redacted_payload_preview_recorded_count": 0,
            "payload_hash_preview_recorded_count": 0,
            "payload_diff_recorded_count": 0,
            "readback_text_recorded_count": 0,
            "operator_summary_text_recorded_count": 0,
            "privacy_review_recorded_count": 0,
            "privacy_review_persisted_count": 0,
            "secret_scan_performed_count": 0,
            "pii_scan_performed_count": 0,
            "raw_payload_inspected_count": 0,
            "plaintext_materialized_count": 0,
            "redaction_bypass_allowed_count": 0,
            "hash_to_payload_link_recorded_count": 0,
            "external_redaction_review_performed_count": 0,
            "privacy_acceptance_recorded_count": 0,
            "redaction_privacy_acceptance_recorded_count": 0,
            "redaction_privacy_operator_approval_derived_count": 0,
            "redaction_privacy_activation_authority_derived_count": 0,
            "redaction_privacy_activation_command_derived_count": 0,
            "redaction_privacy_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "redaction_privacy_surfaces": redaction_privacy_surfaces,
            "denied_by_packet_receipt_redaction_privacy": denied_by_packet_receipt_redaction_privacy,
            "denied_by_packet_receipt_redaction_privacy_count": denied_by_packet_receipt_redaction_privacy_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_gate",
                    "status": "allowed_report_only_next_slice",
                    "exposes_payload": false,
                    "records_privacy_review": false,
                    "performs_secret_scan": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_query_registered": false,
            "packet_acceptance_receipt_export_file_written": false,
            "packet_acceptance_receipt_observability_metric_recorded": false,
            "packet_acceptance_receipt_redacted_payload_preview_recorded": false,
            "packet_acceptance_receipt_payload_hash_preview_recorded": false,
            "packet_acceptance_receipt_payload_diff_recorded": false,
            "packet_acceptance_receipt_readback_text_recorded": false,
            "packet_acceptance_receipt_operator_summary_text_recorded": false,
            "packet_acceptance_receipt_privacy_review_recorded": false,
            "packet_acceptance_receipt_secret_scan_performed": false,
            "packet_acceptance_receipt_pii_scan_performed": false,
            "packet_acceptance_receipt_raw_payload_inspected": false,
            "packet_acceptance_receipt_plaintext_materialized": false,
            "packet_acceptance_receipt_redaction_bypass_allowed": false,
            "packet_acceptance_receipt_hash_to_payload_link_recorded": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({});
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_redacted_payload_preview_recorded": false,
            "packet_acceptance_receipt_payload_hash_preview_recorded": false,
            "packet_acceptance_receipt_payload_diff_recorded": false,
            "packet_acceptance_receipt_readback_text_recorded": false,
            "packet_acceptance_receipt_operator_summary_text_recorded": false,
            "packet_acceptance_receipt_privacy_review_recorded": false,
            "packet_acceptance_receipt_privacy_review_persisted": false,
            "packet_acceptance_receipt_secret_scan_performed": false,
            "packet_acceptance_receipt_pii_scan_performed": false,
            "packet_acceptance_receipt_raw_payload_inspected": false,
            "packet_acceptance_receipt_plaintext_materialized": false,
            "packet_acceptance_receipt_redaction_bypass_allowed": false,
            "packet_acceptance_receipt_hash_to_payload_link_recorded": false,
            "packet_acceptance_receipt_external_redaction_review_performed": false,
            "packet_acceptance_receipt_privacy_acceptance_recorded": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_query_registered": false,
            "packet_acceptance_receipt_export_file_written": false,
            "packet_acceptance_receipt_observability_metric_recorded": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_report();
    let source_view_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_redaction_privacy_surface_count = source
        .get("redaction_privacy_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_redacted_payload_preview_recorded_count = source
        .get("redacted_payload_preview_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_payload_hash_preview_recorded_count = source
        .get("payload_hash_preview_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_readback_text_recorded_count = source
        .get("readback_text_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_summary_text_recorded_count = source
        .get("operator_summary_text_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_privacy_review_recorded_count = source
        .get("privacy_review_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_secret_scan_performed_count = source
        .get("secret_scan_performed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_raw_payload_inspected_count = source
        .get("raw_payload_inspected_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_redaction_privacy_activation_authority_derived_count = source
        .get("redaction_privacy_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_redaction_privacy_payload_exposure_contract_hash_sha256 = source
        .get("redaction_privacy_payload_exposure_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let operator_briefing_surfaces = vec![
        "packet_receipt_operator_briefing_claim",
        "packet_receipt_operator_facing_summary_claim",
        "packet_receipt_readback_digest_claim",
        "packet_receipt_final_note_claim",
        "packet_receipt_status_banner_claim",
        "packet_receipt_timeline_entry_claim",
        "packet_receipt_notification_preview_claim",
        "packet_receipt_channel_delivery_claim",
        "packet_receipt_external_send_claim",
        "packet_receipt_telegram_briefing_claim",
        "packet_receipt_completion_briefing_claim",
        "packet_receipt_acceptance_briefing_claim",
        "packet_receipt_authority_briefing_claim",
        "packet_receipt_live_briefing_claim",
    ]
    .into_iter()
    .map(|briefing_surface| {
        serde_json::json!({
            "briefing_surface": briefing_surface,
            "briefing_attempted": true,
            "briefing_recorded": false,
            "briefing_persisted": false,
            "briefing_materialized": false,
            "briefing_filesystem_written": false,
            "summary_recorded": false,
            "readback_digest_recorded": false,
            "final_note_recorded": false,
            "status_banner_recorded": false,
            "timeline_entry_recorded": false,
            "notification_preview_recorded": false,
            "channel_delivery_performed": false,
            "external_send_performed": false,
            "telegram_send_performed": false,
            "completion_ack_recorded": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "briefing_status": "operator_briefing_non_persistence_denied"
        })
    })
    .collect::<Vec<_>>();
    let operator_briefing_surface_count = operator_briefing_surfaces.len();
    let operator_briefing_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence:native:source={source_view_report_sha256}:surfaces={operator_briefing_surface_count}:route_count={}:briefing=0:summary=0:send=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_operator_briefing = vec![
        "operator_readiness_packet_template_packet_receipt_operator_briefing_recording_denied",
        "operator_readiness_packet_template_packet_receipt_operator_briefing_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_operator_briefing_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_summary_recording_denied",
        "operator_readiness_packet_template_packet_receipt_readback_digest_denied",
        "operator_readiness_packet_template_packet_receipt_final_note_denied",
        "operator_readiness_packet_template_packet_receipt_status_banner_denied",
        "operator_readiness_packet_template_packet_receipt_timeline_entry_denied",
        "operator_readiness_packet_template_packet_receipt_notification_preview_denied",
        "operator_readiness_packet_template_packet_receipt_channel_delivery_denied",
        "operator_readiness_packet_template_packet_receipt_external_send_denied",
        "operator_readiness_packet_template_packet_receipt_telegram_send_denied",
        "operator_readiness_packet_template_packet_receipt_completion_ack_briefing_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_briefing_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_briefing_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_briefing_denied",
    ];
    let denied_by_packet_receipt_operator_briefing_count =
        denied_by_packet_receipt_operator_briefing.len();
    let report_ready = source_ready
        && source_redaction_privacy_surface_count == 16
        && source_redacted_payload_preview_recorded_count == 0
        && source_payload_hash_preview_recorded_count == 0
        && source_readback_text_recorded_count == 0
        && source_operator_summary_text_recorded_count == 0
        && source_privacy_review_recorded_count == 0
        && source_secret_scan_performed_count == 0
        && source_raw_payload_inspected_count == 0
        && source_redaction_privacy_activation_authority_derived_count == 0
        && operator_briefing_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_OPERATOR_BRIEFING_NON_PERSISTENCE_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "receipt_operator_briefing_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_v1",
        "receipt_operator_briefing_mode": "native_route_non_persistent_receipts_cannot_create_operator_briefing_acceptance_or_authority",
        "source_packet_acceptance_receipt_redaction_privacy_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_redaction_privacy_ready": source_ready,
        "source_view_report_sha256": source_view_report_sha256,
        "source_redaction_privacy_payload_exposure_contract_hash_sha256": source_redaction_privacy_payload_exposure_contract_hash_sha256,
        "operator_briefing_contract_hash_sha256": operator_briefing_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready": report_ready,
            "source_redaction_privacy_surface_count": source_redaction_privacy_surface_count,
            "source_redacted_payload_preview_recorded_count": source_redacted_payload_preview_recorded_count,
            "source_payload_hash_preview_recorded_count": source_payload_hash_preview_recorded_count,
            "source_readback_text_recorded_count": source_readback_text_recorded_count,
            "source_operator_summary_text_recorded_count": source_operator_summary_text_recorded_count,
            "source_privacy_review_recorded_count": source_privacy_review_recorded_count,
            "source_secret_scan_performed_count": source_secret_scan_performed_count,
            "source_raw_payload_inspected_count": source_raw_payload_inspected_count,
            "source_redaction_privacy_activation_authority_derived_count": source_redaction_privacy_activation_authority_derived_count,
            "operator_briefing_surface_count": operator_briefing_surface_count,
            "operator_briefing_attempt_count": operator_briefing_surface_count,
            "briefing_recorded_count": 0,
            "briefing_persisted_count": 0,
            "briefing_materialized_count": 0,
            "briefing_filesystem_written_count": 0,
            "summary_recorded_count": 0,
            "readback_digest_recorded_count": 0,
            "final_note_recorded_count": 0,
            "status_banner_recorded_count": 0,
            "timeline_entry_recorded_count": 0,
            "notification_preview_recorded_count": 0,
            "channel_delivery_performed_count": 0,
            "external_send_performed_count": 0,
            "telegram_send_performed_count": 0,
            "completion_ack_recorded_count": 0,
            "operator_briefing_acceptance_recorded_count": 0,
            "operator_briefing_operator_approval_derived_count": 0,
            "operator_briefing_activation_authority_derived_count": 0,
            "operator_briefing_activation_command_derived_count": 0,
            "operator_briefing_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_briefing_surfaces": operator_briefing_surfaces,
            "denied_by_packet_receipt_operator_briefing": denied_by_packet_receipt_operator_briefing,
            "denied_by_packet_receipt_operator_briefing_count": denied_by_packet_receipt_operator_briefing_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_final_ack_non_acceptance_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_briefing": false,
                    "persists_briefing": false,
                    "sends_externally": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_redacted_payload_preview_recorded": false,
            "packet_acceptance_receipt_readback_text_recorded": false,
            "packet_acceptance_receipt_operator_summary_text_recorded": false,
            "packet_acceptance_receipt_operator_briefing_recorded": false,
            "packet_acceptance_receipt_operator_briefing_persisted": false,
            "packet_acceptance_receipt_summary_recorded": false,
            "packet_acceptance_receipt_readback_digest_recorded": false,
            "packet_acceptance_receipt_final_note_recorded": false,
            "packet_acceptance_receipt_status_banner_recorded": false,
            "packet_acceptance_receipt_timeline_entry_recorded": false,
            "packet_acceptance_receipt_notification_preview_recorded": false,
            "packet_acceptance_receipt_channel_delivered": false,
            "packet_acceptance_receipt_external_sent": false,
            "packet_acceptance_receipt_telegram_sent": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({});
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_operator_briefing_recorded": false,
            "packet_acceptance_receipt_operator_briefing_persisted": false,
            "packet_acceptance_receipt_operator_briefing_materialized": false,
            "packet_acceptance_receipt_operator_briefing_filesystem_written": false,
            "packet_acceptance_receipt_summary_recorded": false,
            "packet_acceptance_receipt_readback_digest_recorded": false,
            "packet_acceptance_receipt_final_note_recorded": false,
            "packet_acceptance_receipt_status_banner_recorded": false,
            "packet_acceptance_receipt_timeline_entry_recorded": false,
            "packet_acceptance_receipt_notification_preview_recorded": false,
            "packet_acceptance_receipt_channel_delivered": false,
            "packet_acceptance_receipt_external_sent": false,
            "packet_acceptance_receipt_telegram_sent": false,
            "packet_acceptance_receipt_completion_ack_recorded": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_redacted_payload_preview_recorded": false,
            "packet_acceptance_receipt_readback_text_recorded": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_report();
    let source_view_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_operator_briefing_surface_count = source
        .get("operator_briefing_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_briefing_recorded_count = source
        .get("briefing_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_briefing_persisted_count = source
        .get("briefing_persisted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_briefing_materialized_count = source
        .get("briefing_materialized_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_summary_recorded_count = source
        .get("summary_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_readback_digest_recorded_count = source
        .get("readback_digest_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_note_recorded_count = source
        .get("final_note_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_channel_delivery_performed_count = source
        .get("channel_delivery_performed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_external_send_performed_count = source
        .get("external_send_performed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_telegram_send_performed_count = source
        .get("telegram_send_performed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_completion_ack_recorded_count = source
        .get("completion_ack_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_briefing_activation_authority_derived_count = source
        .get("operator_briefing_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_briefing_contract_hash_sha256 = source
        .get("operator_briefing_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let final_acknowledgement_surfaces = vec![
        "packet_receipt_final_acknowledgement_claim",
        "packet_receipt_operator_received_claim",
        "packet_receipt_operator_confirmed_claim",
        "packet_receipt_operator_read_claim",
        "packet_receipt_operator_seen_claim",
        "packet_receipt_final_response_claim",
        "packet_receipt_completion_acknowledgement_claim",
        "packet_receipt_status_acknowledgement_claim",
        "packet_receipt_briefing_acknowledgement_claim",
        "packet_receipt_readback_acknowledgement_claim",
        "packet_receipt_channel_acknowledgement_claim",
        "packet_receipt_external_acknowledgement_claim",
        "packet_receipt_authority_acknowledgement_claim",
        "packet_receipt_live_acknowledgement_claim",
    ]
    .into_iter()
    .map(|final_acknowledgement_surface| {
        serde_json::json!({
            "final_acknowledgement_surface": final_acknowledgement_surface,
            "final_acknowledgement_attempted": true,
            "final_acknowledgement_accepted": false,
            "final_acknowledgement_recorded": false,
            "final_acknowledgement_persisted": false,
            "final_acknowledgement_materialized": false,
            "final_acknowledgement_delivered": false,
            "operator_received_recorded": false,
            "operator_confirmed_recorded": false,
            "operator_read_recorded": false,
            "operator_seen_recorded": false,
            "final_response_recorded": false,
            "completion_ack_recorded": false,
            "status_ack_recorded": false,
            "briefing_ack_recorded": false,
            "readback_ack_recorded": false,
            "channel_ack_delivered": false,
            "external_ack_sent": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "final_acknowledgement_status": "final_acknowledgement_non_acceptance_denied"
        })
    })
    .collect::<Vec<_>>();
    let final_acknowledgement_surface_count = final_acknowledgement_surfaces.len();
    let final_acknowledgement_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance:native:source={source_view_report_sha256}:surfaces={final_acknowledgement_surface_count}:route_count={}:ack=0:acceptance=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_final_acknowledgement = vec![
        "operator_readiness_packet_template_packet_receipt_final_acknowledgement_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_final_acknowledgement_recording_denied",
        "operator_readiness_packet_template_packet_receipt_final_acknowledgement_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_final_acknowledgement_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_final_acknowledgement_delivery_denied",
        "operator_readiness_packet_template_packet_receipt_operator_received_recording_denied",
        "operator_readiness_packet_template_packet_receipt_operator_confirmed_recording_denied",
        "operator_readiness_packet_template_packet_receipt_operator_read_recording_denied",
        "operator_readiness_packet_template_packet_receipt_completion_ack_recording_denied",
        "operator_readiness_packet_template_packet_receipt_status_ack_recording_denied",
        "operator_readiness_packet_template_packet_receipt_briefing_ack_recording_denied",
        "operator_readiness_packet_template_packet_receipt_readback_ack_recording_denied",
        "operator_readiness_packet_template_packet_receipt_channel_ack_delivery_denied",
        "operator_readiness_packet_template_packet_receipt_external_ack_send_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_final_acknowledgement_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_final_acknowledgement_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_final_acknowledgement_denied",
    ];
    let denied_by_packet_receipt_final_acknowledgement_count =
        denied_by_packet_receipt_final_acknowledgement.len();
    let report_ready = source_ready
        && source_operator_briefing_surface_count == 14
        && source_briefing_recorded_count == 0
        && source_briefing_persisted_count == 0
        && source_briefing_materialized_count == 0
        && source_summary_recorded_count == 0
        && source_readback_digest_recorded_count == 0
        && source_final_note_recorded_count == 0
        && source_channel_delivery_performed_count == 0
        && source_external_send_performed_count == 0
        && source_telegram_send_performed_count == 0
        && source_completion_ack_recorded_count == 0
        && source_operator_briefing_activation_authority_derived_count == 0
        && final_acknowledgement_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_FINAL_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_final_acknowledgement_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_v1",
        "receipt_final_acknowledgement_mode": "native_route_non_persistent_receipt_briefings_cannot_become_operator_acceptance_or_authority",
        "source_packet_acceptance_receipt_operator_briefing_route": source["gate"].clone(),
        "source_packet_acceptance_receipt_operator_briefing_ready": source_ready,
        "source_view_report_sha256": source_view_report_sha256,
        "source_operator_briefing_contract_hash_sha256": source_operator_briefing_contract_hash_sha256,
        "final_acknowledgement_contract_hash_sha256": final_acknowledgement_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready": report_ready,
            "source_operator_briefing_surface_count": source_operator_briefing_surface_count,
            "source_briefing_recorded_count": source_briefing_recorded_count,
            "source_briefing_persisted_count": source_briefing_persisted_count,
            "source_briefing_materialized_count": source_briefing_materialized_count,
            "source_summary_recorded_count": source_summary_recorded_count,
            "source_readback_digest_recorded_count": source_readback_digest_recorded_count,
            "source_final_note_recorded_count": source_final_note_recorded_count,
            "source_channel_delivery_performed_count": source_channel_delivery_performed_count,
            "source_external_send_performed_count": source_external_send_performed_count,
            "source_telegram_send_performed_count": source_telegram_send_performed_count,
            "source_completion_ack_recorded_count": source_completion_ack_recorded_count,
            "source_operator_briefing_activation_authority_derived_count": source_operator_briefing_activation_authority_derived_count,
            "final_acknowledgement_surface_count": final_acknowledgement_surface_count,
            "final_acknowledgement_attempt_count": final_acknowledgement_surface_count,
            "final_acknowledgement_accepted_count": 0,
            "final_acknowledgement_recorded_count": 0,
            "final_acknowledgement_persisted_count": 0,
            "final_acknowledgement_materialized_count": 0,
            "final_acknowledgement_delivered_count": 0,
            "operator_received_recorded_count": 0,
            "operator_confirmed_recorded_count": 0,
            "operator_read_recorded_count": 0,
            "operator_seen_recorded_count": 0,
            "final_response_recorded_count": 0,
            "completion_ack_recorded_count": 0,
            "status_ack_recorded_count": 0,
            "briefing_ack_recorded_count": 0,
            "readback_ack_recorded_count": 0,
            "channel_ack_delivered_count": 0,
            "external_ack_sent_count": 0,
            "final_acknowledgement_acceptance_recorded_count": 0,
            "final_acknowledgement_operator_approval_derived_count": 0,
            "final_acknowledgement_activation_authority_derived_count": 0,
            "final_acknowledgement_activation_command_derived_count": 0,
            "final_acknowledgement_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_acknowledgement_surfaces": final_acknowledgement_surfaces,
            "denied_by_packet_receipt_final_acknowledgement": denied_by_packet_receipt_final_acknowledgement,
            "denied_by_packet_receipt_final_acknowledgement_count": denied_by_packet_receipt_final_acknowledgement_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_final_acknowledgement": false,
                    "persists_final_acknowledgement": false,
                    "sends_externally": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_operator_briefing_recorded": false,
            "packet_acceptance_receipt_operator_briefing_persisted": false,
            "packet_acceptance_receipt_summary_recorded": false,
            "packet_acceptance_receipt_readback_digest_recorded": false,
            "packet_acceptance_receipt_final_note_recorded": false,
            "packet_acceptance_receipt_final_acknowledgement_accepted": false,
            "packet_acceptance_receipt_final_acknowledgement_recorded": false,
            "packet_acceptance_receipt_final_acknowledgement_persisted": false,
            "packet_acceptance_receipt_final_acknowledgement_materialized": false,
            "packet_acceptance_receipt_final_acknowledgement_delivered": false,
            "packet_acceptance_receipt_operator_received_recorded": false,
            "packet_acceptance_receipt_operator_confirmed_recorded": false,
            "packet_acceptance_receipt_operator_read_recorded": false,
            "packet_acceptance_receipt_operator_seen_recorded": false,
            "packet_acceptance_receipt_final_response_recorded": false,
            "packet_acceptance_receipt_completion_ack_recorded": false,
            "packet_acceptance_receipt_status_ack_recorded": false,
            "packet_acceptance_receipt_briefing_ack_recorded": false,
            "packet_acceptance_receipt_readback_ack_recorded": false,
            "packet_acceptance_receipt_channel_ack_delivered": false,
            "packet_acceptance_receipt_external_ack_sent": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({});
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_final_acknowledgement_accepted": false,
            "packet_acceptance_receipt_final_acknowledgement_recorded": false,
            "packet_acceptance_receipt_final_acknowledgement_persisted": false,
            "packet_acceptance_receipt_final_acknowledgement_materialized": false,
            "packet_acceptance_receipt_final_acknowledgement_delivered": false,
            "packet_acceptance_receipt_operator_received_recorded": false,
            "packet_acceptance_receipt_operator_confirmed_recorded": false,
            "packet_acceptance_receipt_operator_read_recorded": false,
            "packet_acceptance_receipt_operator_seen_recorded": false,
            "packet_acceptance_receipt_final_response_recorded": false,
            "packet_acceptance_receipt_completion_ack_recorded": false,
            "packet_acceptance_receipt_status_ack_recorded": false,
            "packet_acceptance_receipt_briefing_ack_recorded": false,
            "packet_acceptance_receipt_readback_ack_recorded": false,
            "packet_acceptance_receipt_channel_ack_delivered": false,
            "packet_acceptance_receipt_external_ack_sent": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_operator_briefing_recorded": false,
            "packet_acceptance_receipt_operator_briefing_persisted": false,
            "packet_acceptance_receipt_summary_recorded": false,
            "packet_acceptance_receipt_readback_digest_recorded": false,
            "packet_acceptance_receipt_final_note_recorded": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_report();
    let source_view_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_final_acknowledgement_surface_count = source
        .get("final_acknowledgement_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_accepted_count = source
        .get("final_acknowledgement_accepted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_recorded_count = source
        .get("final_acknowledgement_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_persisted_count = source
        .get("final_acknowledgement_persisted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_materialized_count = source
        .get("final_acknowledgement_materialized_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_delivered_count = source
        .get("final_acknowledgement_delivered_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_received_recorded_count = source
        .get("operator_received_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_confirmed_recorded_count = source
        .get("operator_confirmed_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_read_recorded_count = source
        .get("operator_read_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_seen_recorded_count = source
        .get("operator_seen_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_completion_ack_recorded_count = source
        .get("completion_ack_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_status_ack_recorded_count = source
        .get("status_ack_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_external_ack_sent_count = source
        .get("external_ack_sent_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_activation_authority_derived_count = source
        .get("final_acknowledgement_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_contract_hash_sha256 = source
        .get("final_acknowledgement_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let terminal_decision_status_surfaces = vec![
        "packet_receipt_terminal_decision_claim",
        "packet_receipt_terminal_status_closed_claim",
        "packet_receipt_final_state_promotion_claim",
        "packet_receipt_completion_promotion_claim",
        "packet_receipt_status_ready_claim",
        "packet_receipt_status_accepted_claim",
        "packet_receipt_status_approved_claim",
        "packet_receipt_status_authoritative_claim",
        "packet_receipt_status_live_claim",
        "packet_receipt_operator_decision_claim",
        "packet_receipt_public_status_claim",
        "packet_receipt_release_status_claim",
        "packet_receipt_dashboard_status_claim",
        "packet_receipt_live_execution_decision_claim",
    ]
    .into_iter()
    .map(|terminal_decision_surface| {
        serde_json::json!({
            "terminal_decision_surface": terminal_decision_surface,
            "terminal_decision_attempted": true,
            "terminal_decision_accepted": false,
            "terminal_decision_recorded": false,
            "terminal_decision_persisted": false,
            "terminal_decision_materialized": false,
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
            "dashboard_status_recorded": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "terminal_decision_status": "terminal_decision_status_promotion_denied"
        })
    })
    .collect::<Vec<_>>();
    let terminal_decision_status_surface_count = terminal_decision_status_surfaces.len();
    let terminal_decision_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial:native:source={source_view_report_sha256}:surfaces={terminal_decision_status_surface_count}:route_count={}:terminal=0:status=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_terminal_decision_status = vec![
        "operator_readiness_packet_template_packet_receipt_terminal_decision_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_decision_recording_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_decision_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_decision_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_status_recording_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_status_closed_denied",
        "operator_readiness_packet_template_packet_receipt_status_ready_denied",
        "operator_readiness_packet_template_packet_receipt_status_accepted_denied",
        "operator_readiness_packet_template_packet_receipt_status_approved_denied",
        "operator_readiness_packet_template_packet_receipt_status_authoritative_denied",
        "operator_readiness_packet_template_packet_receipt_status_live_denied",
        "operator_readiness_packet_template_packet_receipt_final_state_promotion_denied",
        "operator_readiness_packet_template_packet_receipt_completion_promotion_denied",
        "operator_readiness_packet_template_packet_receipt_operator_decision_recording_denied",
        "operator_readiness_packet_template_packet_receipt_public_status_claim_denied",
        "operator_readiness_packet_template_packet_receipt_release_status_claim_denied",
        "operator_readiness_packet_template_packet_receipt_dashboard_status_recording_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_terminal_status_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_terminal_status_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_terminal_status_denied",
    ];
    let denied_by_packet_receipt_terminal_decision_status_count =
        denied_by_packet_receipt_terminal_decision_status.len();
    let report_ready = source_ready
        && source_final_acknowledgement_surface_count == 14
        && source_final_acknowledgement_accepted_count == 0
        && source_final_acknowledgement_recorded_count == 0
        && source_final_acknowledgement_persisted_count == 0
        && source_final_acknowledgement_materialized_count == 0
        && source_final_acknowledgement_delivered_count == 0
        && source_operator_received_recorded_count == 0
        && source_operator_confirmed_recorded_count == 0
        && source_operator_read_recorded_count == 0
        && source_operator_seen_recorded_count == 0
        && source_completion_ack_recorded_count == 0
        && source_status_ack_recorded_count == 0
        && source_external_ack_sent_count == 0
        && source_final_acknowledgement_activation_authority_derived_count == 0
        && terminal_decision_status_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_terminal_decision_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_v1",
        "receipt_terminal_decision_mode": "native_route_final_acknowledgements_cannot_become_terminal_decisions_status_public_claims_or_authority",
        "source_packet_acceptance_receipt_final_acknowledgement_route": source["gate"].clone(),
        "source_packet_acceptance_receipt_final_acknowledgement_ready": source_ready,
        "source_view_report_sha256": source_view_report_sha256,
        "source_final_acknowledgement_contract_hash_sha256": source_final_acknowledgement_contract_hash_sha256,
        "terminal_decision_contract_hash_sha256": terminal_decision_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_ready": report_ready,
            "source_final_acknowledgement_surface_count": source_final_acknowledgement_surface_count,
            "source_final_acknowledgement_accepted_count": source_final_acknowledgement_accepted_count,
            "source_final_acknowledgement_recorded_count": source_final_acknowledgement_recorded_count,
            "source_final_acknowledgement_persisted_count": source_final_acknowledgement_persisted_count,
            "source_final_acknowledgement_materialized_count": source_final_acknowledgement_materialized_count,
            "source_final_acknowledgement_delivered_count": source_final_acknowledgement_delivered_count,
            "source_operator_received_recorded_count": source_operator_received_recorded_count,
            "source_operator_confirmed_recorded_count": source_operator_confirmed_recorded_count,
            "source_operator_read_recorded_count": source_operator_read_recorded_count,
            "source_operator_seen_recorded_count": source_operator_seen_recorded_count,
            "source_completion_ack_recorded_count": source_completion_ack_recorded_count,
            "source_status_ack_recorded_count": source_status_ack_recorded_count,
            "source_external_ack_sent_count": source_external_ack_sent_count,
            "source_final_acknowledgement_activation_authority_derived_count": source_final_acknowledgement_activation_authority_derived_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "terminal_decision_status_surface_count": terminal_decision_status_surface_count,
            "terminal_decision_status_attempt_count": terminal_decision_status_surface_count,
            "terminal_decision_accepted_count": 0,
            "terminal_decision_recorded_count": 0,
            "terminal_decision_persisted_count": 0,
            "terminal_decision_materialized_count": 0,
            "terminal_decision_delivered_count": 0,
            "terminal_status_recorded_count": 0,
            "terminal_status_persisted_count": 0,
            "terminal_status_closed_count": 0,
            "terminal_status_ready_count": 0,
            "terminal_status_accepted_count": 0,
            "terminal_status_approved_count": 0,
            "terminal_status_authoritative_count": 0,
            "terminal_status_live_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_state_promoted_count": 0,
            "completion_promoted_count": 0,
            "operator_decision_recorded_count": 0,
            "public_status_claimed_count": 0,
            "release_status_claimed_count": 0,
            "dashboard_status_recorded_count": 0,
            "terminal_decision_acceptance_recorded_count": 0,
            "terminal_decision_operator_approval_derived_count": 0,
            "terminal_decision_activation_authority_derived_count": 0,
            "terminal_decision_activation_command_derived_count": 0,
            "terminal_decision_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "terminal_decision_status_surfaces": terminal_decision_status_surfaces,
            "denied_by_packet_receipt_terminal_decision_status": denied_by_packet_receipt_terminal_decision_status,
            "denied_by_packet_receipt_terminal_decision_status_count": denied_by_packet_receipt_terminal_decision_status_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_terminal_decision": false,
                    "promotes_status": false,
                    "claims_public_status": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_final_acknowledgement_accepted": false,
            "packet_acceptance_receipt_final_acknowledgement_recorded": false,
            "packet_acceptance_receipt_terminal_decision_accepted": false,
            "packet_acceptance_receipt_terminal_decision_recorded": false,
            "packet_acceptance_receipt_terminal_decision_persisted": false,
            "packet_acceptance_receipt_terminal_decision_materialized": false,
            "packet_acceptance_receipt_terminal_status_recorded": false,
            "packet_acceptance_receipt_terminal_status_persisted": false,
            "packet_acceptance_receipt_terminal_status_closed": false,
            "packet_acceptance_receipt_status_ready": false,
            "packet_acceptance_receipt_status_accepted": false,
            "packet_acceptance_receipt_status_approved": false,
            "packet_acceptance_receipt_status_authoritative": false,
            "packet_acceptance_receipt_status_live": false,
            "packet_acceptance_receipt_final_state_promoted": false,
            "packet_acceptance_receipt_completion_promoted": false,
            "packet_acceptance_receipt_operator_decision_recorded": false,
            "packet_acceptance_receipt_public_status_claimed": false,
            "packet_acceptance_receipt_release_status_claimed": false,
            "packet_acceptance_receipt_dashboard_status_recorded": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({});
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_terminal_decision_accepted": false,
            "packet_acceptance_receipt_terminal_decision_recorded": false,
            "packet_acceptance_receipt_terminal_decision_persisted": false,
            "packet_acceptance_receipt_terminal_decision_materialized": false,
            "packet_acceptance_receipt_terminal_status_recorded": false,
            "packet_acceptance_receipt_terminal_status_persisted": false,
            "packet_acceptance_receipt_terminal_status_closed": false,
            "packet_acceptance_receipt_status_ready": false,
            "packet_acceptance_receipt_status_accepted": false,
            "packet_acceptance_receipt_status_approved": false,
            "packet_acceptance_receipt_status_authoritative": false,
            "packet_acceptance_receipt_status_live": false,
            "packet_acceptance_receipt_final_state_promoted": false,
            "packet_acceptance_receipt_completion_promoted": false,
            "packet_acceptance_receipt_operator_decision_recorded": false,
            "packet_acceptance_receipt_public_status_claimed": false,
            "packet_acceptance_receipt_release_status_claimed": false,
            "packet_acceptance_receipt_dashboard_status_recorded": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_report();
    let source_view_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_terminal_decision_status_surface_count = source
        .get("terminal_decision_status_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_decision_recorded_count = source
        .get("terminal_decision_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_status_recorded_count = source
        .get("terminal_status_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_status_live_count = source
        .get("terminal_status_live_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_public_status_claimed_count = source
        .get("public_status_claimed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_release_status_claimed_count = source
        .get("release_status_claimed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_decision_activation_authority_derived_count = source
        .get("terminal_decision_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_decision_live_execution_allowed_count = source
        .get("terminal_decision_live_execution_allowed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_decision_contract_hash_sha256 = source
        .get("terminal_decision_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let release_publication_surfaces = vec![
        "packet_receipt_release_artifact_write_claim",
        "packet_receipt_public_artifact_write_claim",
        "packet_receipt_artifact_signature_claim",
        "packet_receipt_artifact_notarization_claim",
        "packet_receipt_publication_queue_claim",
        "packet_receipt_publication_manifest_claim",
        "packet_receipt_public_distribution_claim",
        "packet_receipt_channel_delivery_publication_claim",
        "packet_receipt_public_version_tag_claim",
        "packet_receipt_release_notes_materialization_claim",
        "packet_receipt_changelog_materialization_claim",
        "packet_receipt_public_release_claim",
        "packet_receipt_public_ga_claim",
        "packet_receipt_terminal_status_release_approval_claim",
    ]
    .into_iter()
    .map(|release_publication_surface| {
        serde_json::json!({
            "release_publication_surface": release_publication_surface,
            "release_publication_attempted": true,
            "release_publication_allowed": false,
            "release_publication_accepted": false,
            "release_publication_recorded": false,
            "release_publication_persisted": false,
            "release_publication_materialized": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "artifact_signature_accepted": false,
            "artifact_notarization_accepted": false,
            "publication_queue_enqueued": false,
            "publication_manifest_written": false,
            "public_distribution_performed": false,
            "channel_delivery_performed": false,
            "external_send_performed": false,
            "public_version_tag_created": false,
            "release_notes_materialized": false,
            "changelog_materialized": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "terminal_status_promoted_to_release_approval": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "release_publication_status": "release_publication_denied"
        })
    })
    .collect::<Vec<_>>();
    let release_publication_surface_count = release_publication_surfaces.len();
    let release_publication_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial:native:source={source_view_report_sha256}:surfaces={release_publication_surface_count}:route_count={}:release=0:artifact=0:public=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_release_publication = vec![
        "operator_readiness_packet_template_packet_receipt_release_artifact_write_denied",
        "operator_readiness_packet_template_packet_receipt_public_artifact_write_denied",
        "operator_readiness_packet_template_packet_receipt_artifact_signature_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_artifact_notarization_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_publication_queue_enqueue_denied",
        "operator_readiness_packet_template_packet_receipt_publication_manifest_write_denied",
        "operator_readiness_packet_template_packet_receipt_public_distribution_denied",
        "operator_readiness_packet_template_packet_receipt_channel_delivery_publication_denied",
        "operator_readiness_packet_template_packet_receipt_public_version_tag_denied",
        "operator_readiness_packet_template_packet_receipt_release_notes_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_changelog_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_public_release_claim_denied",
        "operator_readiness_packet_template_packet_receipt_public_ga_claim_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_status_as_release_approval_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_release_publication_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_release_publication_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_release_publication_denied",
    ];
    let denied_by_packet_receipt_release_publication_count =
        denied_by_packet_receipt_release_publication.len();
    let report_ready = source_ready
        && source_terminal_decision_status_surface_count == 14
        && source_terminal_decision_recorded_count == 0
        && source_terminal_status_recorded_count == 0
        && source_terminal_status_live_count == 0
        && source_public_status_claimed_count == 0
        && source_release_status_claimed_count == 0
        && source_terminal_decision_activation_authority_derived_count == 0
        && source_terminal_decision_live_execution_allowed_count == 0
        && release_publication_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_v1",
        "receipt_release_publication_mode": "native_route_terminal_receipt_status_cannot_become_release_publication_public_artifacts_or_public_claims",
        "source_packet_acceptance_receipt_terminal_decision_status_route": source["gate"].clone(),
        "source_packet_acceptance_receipt_terminal_decision_status_ready": source_ready,
        "source_view_report_sha256": source_view_report_sha256,
        "source_terminal_decision_contract_hash_sha256": source_terminal_decision_contract_hash_sha256,
        "release_publication_contract_hash_sha256": release_publication_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_ready": report_ready,
            "source_terminal_decision_status_surface_count": source_terminal_decision_status_surface_count,
            "source_terminal_decision_recorded_count": source_terminal_decision_recorded_count,
            "source_terminal_status_recorded_count": source_terminal_status_recorded_count,
            "source_terminal_status_live_count": source_terminal_status_live_count,
            "source_public_status_claimed_count": source_public_status_claimed_count,
            "source_release_status_claimed_count": source_release_status_claimed_count,
            "source_terminal_decision_activation_authority_derived_count": source_terminal_decision_activation_authority_derived_count,
            "source_terminal_decision_live_execution_allowed_count": source_terminal_decision_live_execution_allowed_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_surface_count": release_publication_surface_count,
            "release_publication_attempt_count": release_publication_surface_count,
            "release_publication_allowed_count": 0,
            "release_publication_accepted_count": 0,
            "release_publication_recorded_count": 0,
            "release_publication_persisted_count": 0,
            "release_publication_materialized_count": 0,
            "release_artifact_written_count": 0,
            "public_artifact_written_count": 0,
            "artifact_signature_accepted_count": 0,
            "artifact_notarization_accepted_count": 0,
            "publication_queue_enqueued_count": 0,
            "publication_manifest_written_count": 0,
            "public_distribution_performed_count": 0,
            "channel_delivery_performed_count": 0,
            "external_publication_sent_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "public_version_tag_created_count": 0,
            "release_notes_materialized_count": 0,
            "changelog_materialized_count": 0,
            "public_release_claimed_count": 0,
            "public_ga_claimed_count": 0,
            "terminal_status_release_approval_promoted_count": 0,
            "release_publication_acceptance_recorded_count": 0,
            "release_publication_operator_approval_derived_count": 0,
            "release_publication_activation_authority_derived_count": 0,
            "release_publication_activation_command_derived_count": 0,
            "release_publication_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_surfaces": release_publication_surfaces,
            "denied_by_packet_receipt_release_publication": denied_by_packet_receipt_release_publication,
            "denied_by_packet_receipt_release_publication_count": denied_by_packet_receipt_release_publication_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_release_publication": false,
                    "writes_release_artifact": false,
                    "writes_public_artifact": false,
                    "enqueues_publication": false,
                    "claims_public_release": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_terminal_decision_recorded": false,
            "packet_acceptance_receipt_terminal_status_recorded": false,
            "packet_acceptance_receipt_public_status_claimed": false,
            "packet_acceptance_receipt_release_status_claimed": false,
            "packet_acceptance_receipt_release_publication_allowed": false,
            "packet_acceptance_receipt_release_publication_accepted": false,
            "packet_acceptance_receipt_release_publication_recorded": false,
            "packet_acceptance_receipt_release_publication_persisted": false,
            "packet_acceptance_receipt_release_publication_materialized": false,
            "packet_acceptance_receipt_release_artifact_written": false,
            "packet_acceptance_receipt_public_artifact_written": false,
            "packet_acceptance_receipt_artifact_signature_accepted": false,
            "packet_acceptance_receipt_artifact_notarization_accepted": false,
            "packet_acceptance_receipt_publication_queue_enqueued": false,
            "packet_acceptance_receipt_publication_manifest_written": false,
            "packet_acceptance_receipt_public_distribution_performed": false,
            "packet_acceptance_receipt_channel_delivery_performed": false,
            "packet_acceptance_receipt_external_publication_sent": false,
            "packet_acceptance_receipt_public_version_tag_created": false,
            "packet_acceptance_receipt_release_notes_materialized": false,
            "packet_acceptance_receipt_changelog_materialized": false,
            "packet_acceptance_receipt_public_release_claimed": false,
            "packet_acceptance_receipt_public_ga_claimed": false,
            "packet_acceptance_receipt_terminal_status_promoted_to_release_approval": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );

    let mut side_effects = serde_json::json!({});
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_release_publication_allowed": false,
            "packet_acceptance_receipt_release_publication_accepted": false,
            "packet_acceptance_receipt_release_publication_recorded": false,
            "packet_acceptance_receipt_release_publication_persisted": false,
            "packet_acceptance_receipt_release_publication_materialized": false,
            "packet_acceptance_receipt_release_artifact_written": false,
            "packet_acceptance_receipt_public_artifact_written": false,
            "packet_acceptance_receipt_artifact_signature_accepted": false,
            "packet_acceptance_receipt_artifact_notarization_accepted": false,
            "packet_acceptance_receipt_publication_queue_enqueued": false,
            "packet_acceptance_receipt_publication_manifest_written": false,
            "packet_acceptance_receipt_public_distribution_performed": false,
            "packet_acceptance_receipt_channel_delivery_performed": false,
            "packet_acceptance_receipt_external_publication_sent": false,
            "packet_acceptance_receipt_public_version_tag_created": false,
            "packet_acceptance_receipt_release_notes_materialized": false,
            "packet_acceptance_receipt_changelog_materialized": false,
            "packet_acceptance_receipt_public_release_claimed": false,
            "packet_acceptance_receipt_public_ga_claimed": false,
            "packet_acceptance_receipt_terminal_status_promoted_to_release_approval": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_release_publication_contract_hash_sha256 = source
        .get("release_publication_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let release_publication_result_receipt_surfaces = vec![
        "source_release_publication_report_required",
        "publication_result_receipt_recording_denied",
        "publication_result_receipt_persistence_denied",
        "publication_result_receipt_materialization_denied",
        "publication_result_receipt_filesystem_write_denied",
        "publication_result_receipt_ledger_index_denied",
        "publication_result_receipt_enqueue_delivery_denied",
        "publication_result_receipt_export_query_denied",
        "publication_result_receipt_observability_denied",
        "publication_result_receipt_hash_binding_denied",
        "publication_result_receipt_signature_timestamp_status_denied",
        "publication_completion_ack_denied",
        "publication_result_receipt_release_publication_authority_denied",
        "publication_result_receipt_activation_live_install_restart_active_binary_denied",
    ]
    .into_iter()
    .map(|release_publication_result_receipt_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_surface".to_string(),
            serde_json::json!(release_publication_result_receipt_surface),
        );
        surface.insert(
            "source_release_publication_report_present".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "source_release_publication_denial_ready".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "publication_result_receipt_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "publication_result_receipt_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_status".to_string(),
            serde_json::json!("release_publication_result_receipt_no_persistence_denied"),
        );
        for key in [
            "publication_result_receipt_allowed",
            "publication_result_receipt_accepted",
            "publication_result_receipt_recorded",
            "publication_result_receipt_persisted",
            "publication_result_receipt_materialized",
            "publication_result_receipt_filesystem_written",
            "publication_result_receipt_ledger_written",
            "publication_result_receipt_indexed",
            "publication_result_receipt_enqueued",
            "publication_result_receipt_delivered",
            "publication_result_receipt_exported",
            "publication_result_receipt_query_registered",
            "publication_result_receipt_observability_recorded",
            "publication_result_receipt_hash_bound",
            "publication_result_receipt_signature_accepted",
            "publication_result_receipt_timestamp_accepted",
            "publication_result_receipt_status_accepted",
            "publication_completion_ack_recorded",
            "publication_completion_ack_persisted",
            "publication_completion_ack_accepted",
            "release_publication_recorded",
            "release_publication_persisted",
            "release_publication_materialized",
            "release_artifact_written",
            "public_artifact_written",
            "artifact_signature_accepted",
            "artifact_notarization_accepted",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "public_release_claimed",
            "public_ga_claimed",
            "terminal_status_promoted_to_release_approval",
            "acceptance_recorded",
            "operator_approval_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let release_publication_result_receipt_surface_count =
        release_publication_result_receipt_surfaces.len();
    let release_publication_result_receipt_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence:native:source={source_report_sha256}:surfaces={release_publication_result_receipt_surface_count}:route_count={}:record=0:persist=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_release_publication_result_receipt_no_persistence = vec![
        "operator_readiness_packet_template_packet_receipt_source_release_publication_report_required",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_filesystem_write_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_ledger_write_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_index_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_enqueue_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_delivery_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_export_query_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_observability_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_hash_binding_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_signature_timestamp_status_denied",
        "operator_readiness_packet_template_packet_receipt_publication_completion_ack_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_authority_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_install_restart_active_binary_denied",
    ];
    let denied_by_packet_receipt_release_publication_result_receipt_no_persistence_count =
        denied_by_packet_receipt_release_publication_result_receipt_no_persistence.len();
    let report_ready = source_ready
        && source_u64("release_publication_surface_count") == 14
        && source_u64("release_publication_attempt_count") == 14
        && source_u64("release_publication_allowed_count") == 0
        && source_u64("release_publication_accepted_count") == 0
        && source_u64("release_publication_recorded_count") == 0
        && source_u64("release_publication_persisted_count") == 0
        && source_u64("release_artifact_written_count") == 0
        && source_u64("public_artifact_written_count") == 0
        && source_u64("public_distribution_performed_count") == 0
        && source_u64("public_release_claimed_count") == 0
        && source_u64("public_ga_claimed_count") == 0
        && source_u64("release_publication_activation_authority_derived_count") == 0
        && release_publication_result_receipt_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_v1",
        "receipt_release_publication_result_receipt_mode": "native_route_denied_release_publication_attempt_cannot_persist_result_receipt_or_derive_authority",
        "source_packet_acceptance_receipt_release_publication_route": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_report_sha256": source_report_sha256,
        "source_release_publication_contract_hash_sha256": source_release_publication_contract_hash_sha256,
        "release_publication_result_receipt_contract_hash_sha256": release_publication_result_receipt_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready": report_ready,
            "source_release_publication_surface_count": source_u64("release_publication_surface_count"),
            "source_release_publication_attempt_count": source_u64("release_publication_attempt_count"),
            "source_release_publication_allowed_count": source_u64("release_publication_allowed_count"),
            "source_release_publication_accepted_count": source_u64("release_publication_accepted_count"),
            "source_release_publication_recorded_count": source_u64("release_publication_recorded_count"),
            "source_release_publication_persisted_count": source_u64("release_publication_persisted_count"),
            "source_release_artifact_written_count": source_u64("release_artifact_written_count"),
            "source_public_artifact_written_count": source_u64("public_artifact_written_count"),
            "source_public_distribution_performed_count": source_u64("public_distribution_performed_count"),
            "source_public_release_claimed_count": source_u64("public_release_claimed_count"),
            "source_public_ga_claimed_count": source_u64("public_ga_claimed_count"),
            "source_release_publication_activation_authority_derived_count": source_u64("release_publication_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_surface_count": release_publication_result_receipt_surface_count,
            "release_publication_result_receipt_attempt_count": release_publication_result_receipt_surface_count,
            "release_publication_result_receipt_allowed_count": 0,
            "release_publication_result_receipt_accepted_count": 0,
            "release_publication_result_receipt_recorded_count": 0,
            "release_publication_result_receipt_persisted_count": 0,
            "release_publication_result_receipt_materialized_count": 0,
            "release_publication_result_receipt_filesystem_written_count": 0,
            "release_publication_result_receipt_ledger_written_count": 0,
            "release_publication_result_receipt_indexed_count": 0,
            "release_publication_result_receipt_enqueued_count": 0,
            "release_publication_result_receipt_delivered_count": 0,
            "release_publication_result_receipt_exported_count": 0,
            "release_publication_result_receipt_query_registered_count": 0,
            "release_publication_result_receipt_observability_recorded_count": 0,
            "release_publication_result_receipt_hash_bound_count": 0,
            "release_publication_result_receipt_signature_accepted_count": 0,
            "release_publication_result_receipt_timestamp_accepted_count": 0,
            "release_publication_result_receipt_status_accepted_count": 0,
            "publication_completion_ack_recorded_count": 0,
            "publication_completion_ack_persisted_count": 0,
            "publication_completion_ack_accepted_count": 0,
            "release_publication_result_receipt_acceptance_recorded_count": 0,
            "release_publication_result_receipt_operator_approval_derived_count": 0,
            "release_publication_result_receipt_activation_authority_derived_count": 0,
            "release_publication_result_receipt_activation_command_derived_count": 0,
            "release_publication_result_receipt_live_execution_allowed_count": 0,
            "release_publication_result_receipt_surfaces": release_publication_result_receipt_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_no_persistence": denied_by_packet_receipt_release_publication_result_receipt_no_persistence,
            "denied_by_packet_receipt_release_publication_result_receipt_no_persistence_count": denied_by_packet_receipt_release_publication_result_receipt_no_persistence_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_release_publication_result_receipt": false,
                    "persists_release_publication_result_receipt": false,
                    "writes_release_artifact": false,
                    "writes_public_artifact": false,
                    "enqueues_publication": false,
                    "records_publication_completion_ack": false,
                    "claims_public_release": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );
    let release_publication_result_receipt_false_keys = [
        "packet_acceptance_receipt_release_publication_allowed",
        "packet_acceptance_receipt_release_publication_accepted",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_publication_persisted",
        "packet_acceptance_receipt_release_publication_materialized",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_artifact_signature_accepted",
        "packet_acceptance_receipt_artifact_notarization_accepted",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "packet_acceptance_receipt_terminal_status_promoted_to_release_approval",
        "packet_acceptance_receipt_release_publication_result_receipt_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_ledger_written",
        "packet_acceptance_receipt_release_publication_result_receipt_indexed",
        "packet_acceptance_receipt_release_publication_result_receipt_enqueued",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_bound",
        "packet_acceptance_receipt_release_publication_result_receipt_signature_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_timestamp_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_status_accepted",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_publication_completion_ack_persisted",
        "packet_acceptance_receipt_publication_completion_ack_accepted",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
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
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_release_publication_result_receipt_persisted": false,
            "packet_acceptance_receipt_publication_completion_ack_recorded": false,
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in release_publication_result_receipt_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in release_publication_result_receipt_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_release_publication_result_receipt_contract_hash_sha256 = source
        .get("release_publication_result_receipt_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let release_publication_result_receipt_replay_surfaces = vec![
        "publication_result_receipt_replay",
        "publication_result_receipt_duplicate_replay",
        "publication_result_receipt_retry_replay",
        "publication_result_receipt_idempotency_key_registration",
        "publication_result_receipt_idempotency_cache_write",
        "publication_result_receipt_cache_hit_promotion",
        "publication_result_receipt_hash_replay_binding",
        "publication_result_receipt_signature_timestamp_status_replay",
        "publication_result_receipt_query_result_replay",
        "publication_result_receipt_export_snapshot_replay",
        "publication_result_receipt_observability_snapshot_replay",
        "publication_completion_ack_replay",
        "publication_result_receipt_release_publication_authority_replay",
        "publication_result_receipt_activation_live_install_restart_active_binary_replay",
    ]
    .into_iter()
    .map(|release_publication_result_receipt_replay_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_replay_surface".to_string(),
            serde_json::json!(release_publication_result_receipt_replay_surface),
        );
        surface.insert(
            "source_release_publication_result_receipt_present".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "source_release_publication_result_receipt_no_persistence_ready".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "result_receipt_replay_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "result_receipt_replay_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_replay_status".to_string(),
            serde_json::json!("release_publication_result_receipt_replay_idempotency_denied"),
        );
        for key in [
            "result_receipt_replay_allowed",
            "result_receipt_replay_accepted",
            "result_receipt_replay_recorded",
            "result_receipt_replay_persisted",
            "result_receipt_replay_materialized",
            "result_receipt_duplicate_accepted",
            "result_receipt_retry_accepted",
            "idempotency_key_registered",
            "idempotency_cache_written",
            "idempotency_cache_hit_promoted",
            "replay_hash_bound",
            "replay_signature_accepted",
            "replay_timestamp_accepted",
            "replay_status_accepted",
            "query_result_replayed",
            "export_snapshot_replayed",
            "observability_snapshot_replayed",
            "publication_completion_ack_replayed",
            "publication_completion_ack_recorded",
            "release_publication_recorded",
            "release_publication_persisted",
            "release_publication_materialized",
            "release_artifact_written",
            "public_artifact_written",
            "artifact_signature_accepted",
            "artifact_notarization_accepted",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "public_release_claimed",
            "public_ga_claimed",
            "terminal_status_promoted_to_release_approval",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let release_publication_result_receipt_replay_surface_count =
        release_publication_result_receipt_replay_surfaces.len();
    let release_publication_result_receipt_replay_idempotency_contract_hash_sha256 =
        sha256_text_value(&format!(
            "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial:native:source={source_report_sha256}:surfaces={release_publication_result_receipt_replay_surface_count}:route_count={}:replay=0:idempotency=0:authority=0:live=0",
            route_matrix.route_count
        ));
    let denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replay_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replay_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replay_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_duplicate_replay_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retry_replay_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_idempotency_key_registration_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_idempotency_cache_write_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_cache_hit_promotion_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_hash_replay_binding_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_signature_timestamp_status_replay_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_export_observability_replay_denied",
        "operator_readiness_packet_template_packet_receipt_publication_completion_ack_replay_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_replay_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_authority_replay_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_replay_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_replay_denied",
    ];
    let denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency_count =
        denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency.len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_surface_count") == 14
        && source_u64("release_publication_result_receipt_attempt_count") == 14
        && source_u64("release_publication_result_receipt_recorded_count") == 0
        && source_u64("release_publication_result_receipt_persisted_count") == 0
        && source_u64("release_publication_result_receipt_materialized_count") == 0
        && source_u64("release_publication_result_receipt_delivered_count") == 0
        && source_u64("release_publication_result_receipt_exported_count") == 0
        && source_u64("release_publication_result_receipt_query_registered_count") == 0
        && source_u64("release_publication_result_receipt_observability_recorded_count") == 0
        && source_u64("publication_completion_ack_recorded_count") == 0
        && source_u64("release_publication_result_receipt_activation_authority_derived_count") == 0
        && release_publication_result_receipt_replay_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_replay_idempotency_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_v1",
        "receipt_release_publication_result_receipt_replay_idempotency_mode": "native_route_denied_release_publication_result_receipt_cannot_replay_cache_or_derive_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_route": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_contract_hash_sha256": source_release_publication_result_receipt_contract_hash_sha256,
        "release_publication_result_receipt_replay_idempotency_contract_hash_sha256": release_publication_result_receipt_replay_idempotency_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_ready": report_ready,
            "source_release_publication_result_receipt_surface_count": source_u64("release_publication_result_receipt_surface_count"),
            "source_release_publication_result_receipt_attempt_count": source_u64("release_publication_result_receipt_attempt_count"),
            "source_release_publication_result_receipt_recorded_count": source_u64("release_publication_result_receipt_recorded_count"),
            "source_release_publication_result_receipt_persisted_count": source_u64("release_publication_result_receipt_persisted_count"),
            "source_release_publication_result_receipt_materialized_count": source_u64("release_publication_result_receipt_materialized_count"),
            "source_release_publication_result_receipt_delivered_count": source_u64("release_publication_result_receipt_delivered_count"),
            "source_release_publication_result_receipt_exported_count": source_u64("release_publication_result_receipt_exported_count"),
            "source_release_publication_result_receipt_query_registered_count": source_u64("release_publication_result_receipt_query_registered_count"),
            "source_release_publication_result_receipt_observability_recorded_count": source_u64("release_publication_result_receipt_observability_recorded_count"),
            "source_publication_completion_ack_recorded_count": source_u64("publication_completion_ack_recorded_count"),
            "source_release_publication_result_receipt_activation_authority_derived_count": source_u64("release_publication_result_receipt_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_replay_surface_count": release_publication_result_receipt_replay_surface_count,
            "release_publication_result_receipt_replay_attempt_count": release_publication_result_receipt_replay_surface_count,
            "release_publication_result_receipt_replay_allowed_count": 0,
            "release_publication_result_receipt_replay_accepted_count": 0,
            "release_publication_result_receipt_replay_recorded_count": 0,
            "release_publication_result_receipt_replay_persisted_count": 0,
            "release_publication_result_receipt_replay_materialized_count": 0,
            "release_publication_result_receipt_duplicate_accepted_count": 0,
            "release_publication_result_receipt_retry_accepted_count": 0,
            "release_publication_result_receipt_idempotency_key_registered_count": 0,
            "release_publication_result_receipt_idempotency_cache_written_count": 0,
            "release_publication_result_receipt_idempotency_cache_hit_promoted_count": 0,
            "release_publication_result_receipt_replay_hash_bound_count": 0,
            "release_publication_result_receipt_replay_signature_accepted_count": 0,
            "release_publication_result_receipt_replay_timestamp_accepted_count": 0,
            "release_publication_result_receipt_replay_status_accepted_count": 0,
            "release_publication_result_receipt_query_result_replayed_count": 0,
            "release_publication_result_receipt_export_snapshot_replayed_count": 0,
            "release_publication_result_receipt_observability_snapshot_replayed_count": 0,
            "publication_completion_ack_replayed_count": 0,
            "publication_completion_ack_recorded_count": 0,
            "release_publication_result_receipt_replay_acceptance_recorded_count": 0,
            "release_publication_result_receipt_replay_operator_approval_derived_count": 0,
            "release_publication_result_receipt_replay_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_replay_activation_authority_derived_count": 0,
            "release_publication_result_receipt_replay_activation_command_derived_count": 0,
            "release_publication_result_receipt_replay_live_execution_allowed_count": 0,
            "release_publication_result_receipt_replay_surfaces": release_publication_result_receipt_replay_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency": denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency,
            "denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency_count": denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_release_publication_result_receipt_replay": false,
                    "registers_idempotency_key": false,
                    "writes_idempotency_cache": false,
                    "promotes_cache_hit": false,
                    "records_publication_completion_ack": false,
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
    let release_publication_result_receipt_replay_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_duplicate_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_retry_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_hash_bound",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_signature_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_timestamp_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_snapshot_replayed",
        "packet_acceptance_receipt_publication_completion_ack_replayed",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_ledger_written",
        "packet_acceptance_receipt_release_publication_result_receipt_indexed",
        "packet_acceptance_receipt_release_publication_result_receipt_enqueued",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_allowed",
        "packet_acceptance_receipt_release_publication_accepted",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "packet_acceptance_receipt_terminal_status_promoted_to_release_approval",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
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
        for key in release_publication_result_receipt_replay_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in release_publication_result_receipt_replay_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_ready")
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

    let release_publication_result_receipt_ordering_surfaces = vec![
        "publication_result_receipt_duplicate_sequence_claim",
        "publication_result_receipt_stale_sequence_claim",
        "publication_result_receipt_late_arrival_claim",
        "publication_result_receipt_future_sequence_gap_claim",
        "publication_result_receipt_timestamp_rollback_claim",
        "publication_result_receipt_epoch_rollback_claim",
        "publication_result_receipt_same_sequence_different_hash_claim",
        "publication_result_receipt_latest_wins_overwrite_claim",
        "publication_result_receipt_query_ordering_claim",
        "publication_result_receipt_export_ordering_claim",
        "publication_result_receipt_observability_ordering_claim",
        "publication_result_receipt_completion_ack_ordering_claim",
        "publication_result_receipt_release_publication_authority_ordering_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_ordering_claim",
    ]
    .into_iter()
    .map(|release_publication_result_receipt_ordering_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_ordering_surface".to_string(),
            serde_json::json!(release_publication_result_receipt_ordering_surface),
        );
        surface.insert(
            "source_release_publication_result_receipt_replay_present".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "source_release_publication_result_receipt_replay_idempotency_ready".to_string(),
            serde_json::json!(source_ready),
        );
        surface.insert("ordering_attempted".to_string(), serde_json::json!(true));
        surface.insert(
            "ordering_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_ordering_status".to_string(),
            serde_json::json!("release_publication_result_receipt_ordering_monotonicity_denied"),
        );
        for key in [
            "ordering_allowed",
            "ordering_recorded",
            "ordering_persisted",
            "ordering_materialized",
            "sequence_cursor_accepted",
            "sequence_cursor_recorded",
            "sequence_cursor_persisted",
            "monotonicity_state_recorded",
            "monotonicity_state_persisted",
            "duplicate_sequence_accepted",
            "stale_sequence_accepted",
            "late_arrival_accepted",
            "future_sequence_gap_accepted",
            "timestamp_rollback_accepted",
            "epoch_rollback_accepted",
            "same_sequence_hash_override_accepted",
            "latest_wins_overwrite_accepted",
            "query_ordering_accepted",
            "export_ordering_accepted",
            "observability_ordering_accepted",
            "completion_ack_ordering_accepted",
            "publication_completion_ack_recorded",
            "result_receipt_replay_recorded",
            "idempotency_key_registered",
            "idempotency_cache_written",
            "idempotency_cache_hit_promoted",
            "release_publication_recorded",
            "release_artifact_written",
            "public_artifact_written",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "public_release_claimed",
            "public_ga_claimed",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let release_publication_result_receipt_ordering_surface_count =
        release_publication_result_receipt_ordering_surfaces.len();
    let release_publication_result_receipt_ordering_monotonicity_contract_hash_sha256 =
        sha256_text_value(&format!(
            "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial:native:source={source_report_sha256}:surfaces={release_publication_result_receipt_ordering_surface_count}:route_count={}:ordering=0:monotonicity=0:authority=0:live=0",
            route_matrix.route_count
        ));
    let denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ordering_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ordering_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ordering_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_sequence_cursor_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_sequence_cursor_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_sequence_cursor_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_monotonicity_state_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_monotonicity_state_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_duplicate_sequence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_stale_sequence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_late_arrival_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_future_sequence_gap_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_timestamp_rollback_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_epoch_rollback_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_same_sequence_hash_override_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_latest_wins_overwrite_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_completion_ack_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_from_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_ordering_denied",
    ];
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_replay_surface_count") == 14
        && source_u64("release_publication_result_receipt_replay_attempt_count") == 14
        && source_u64("release_publication_result_receipt_replay_recorded_count") == 0
        && source_u64("release_publication_result_receipt_replay_persisted_count") == 0
        && source_u64("release_publication_result_receipt_idempotency_key_registered_count") == 0
        && source_u64("release_publication_result_receipt_idempotency_cache_written_count") == 0
        && source_u64("release_publication_result_receipt_idempotency_cache_hit_promoted_count")
            == 0
        && source_u64("release_publication_result_receipt_query_result_replayed_count") == 0
        && source_u64("release_publication_result_receipt_export_snapshot_replayed_count") == 0
        && source_u64("release_publication_result_receipt_observability_snapshot_replayed_count")
            == 0
        && source_u64("publication_completion_ack_replayed_count") == 0
        && source_u64(
            "release_publication_result_receipt_replay_activation_authority_derived_count",
        ) == 0
        && release_publication_result_receipt_ordering_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_ordering_monotonicity_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_v1",
        "receipt_release_publication_result_receipt_ordering_monotonicity_mode": "native_route_denied_release_publication_result_receipt_cannot_use_ordering_or_monotonicity_as_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_replay_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_replay_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_replay_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_replay_idempotency_contract_hash_sha256": source["release_publication_result_receipt_replay_idempotency_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_ordering_monotonicity_contract_hash_sha256": release_publication_result_receipt_ordering_monotonicity_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_ready": report_ready,
            "source_release_publication_result_receipt_replay_surface_count": source_u64("release_publication_result_receipt_replay_surface_count"),
            "source_release_publication_result_receipt_replay_attempt_count": source_u64("release_publication_result_receipt_replay_attempt_count"),
            "source_release_publication_result_receipt_replay_recorded_count": source_u64("release_publication_result_receipt_replay_recorded_count"),
            "source_release_publication_result_receipt_replay_persisted_count": source_u64("release_publication_result_receipt_replay_persisted_count"),
            "source_release_publication_result_receipt_idempotency_key_registered_count": source_u64("release_publication_result_receipt_idempotency_key_registered_count"),
            "source_release_publication_result_receipt_idempotency_cache_written_count": source_u64("release_publication_result_receipt_idempotency_cache_written_count"),
            "source_release_publication_result_receipt_idempotency_cache_hit_promoted_count": source_u64("release_publication_result_receipt_idempotency_cache_hit_promoted_count"),
            "source_release_publication_result_receipt_query_result_replayed_count": source_u64("release_publication_result_receipt_query_result_replayed_count"),
            "source_release_publication_result_receipt_export_snapshot_replayed_count": source_u64("release_publication_result_receipt_export_snapshot_replayed_count"),
            "source_release_publication_result_receipt_observability_snapshot_replayed_count": source_u64("release_publication_result_receipt_observability_snapshot_replayed_count"),
            "source_publication_completion_ack_replayed_count": source_u64("publication_completion_ack_replayed_count"),
            "source_release_publication_result_receipt_replay_activation_authority_derived_count": source_u64("release_publication_result_receipt_replay_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_ordering_surface_count": release_publication_result_receipt_ordering_surface_count,
            "release_publication_result_receipt_ordering_attempt_count": release_publication_result_receipt_ordering_surface_count,
            "release_publication_result_receipt_ordering_allowed_count": 0,
            "release_publication_result_receipt_ordering_recorded_count": 0,
            "release_publication_result_receipt_ordering_persisted_count": 0,
            "release_publication_result_receipt_ordering_materialized_count": 0,
            "release_publication_result_receipt_sequence_cursor_accepted_count": 0,
            "release_publication_result_receipt_sequence_cursor_recorded_count": 0,
            "release_publication_result_receipt_sequence_cursor_persisted_count": 0,
            "release_publication_result_receipt_monotonicity_state_recorded_count": 0,
            "release_publication_result_receipt_monotonicity_state_persisted_count": 0,
            "release_publication_result_receipt_duplicate_sequence_accepted_count": 0,
            "release_publication_result_receipt_stale_sequence_accepted_count": 0,
            "release_publication_result_receipt_late_arrival_accepted_count": 0,
            "release_publication_result_receipt_future_sequence_gap_accepted_count": 0,
            "release_publication_result_receipt_timestamp_rollback_accepted_count": 0,
            "release_publication_result_receipt_epoch_rollback_accepted_count": 0,
            "release_publication_result_receipt_same_sequence_hash_override_accepted_count": 0,
            "release_publication_result_receipt_latest_wins_overwrite_accepted_count": 0,
            "release_publication_result_receipt_query_ordering_accepted_count": 0,
            "release_publication_result_receipt_export_ordering_accepted_count": 0,
            "release_publication_result_receipt_observability_ordering_accepted_count": 0,
            "release_publication_result_receipt_completion_ack_ordering_accepted_count": 0,
            "publication_completion_ack_recorded_count": 0,
            "release_publication_result_receipt_ordering_acceptance_recorded_count": 0,
            "release_publication_result_receipt_ordering_operator_approval_derived_count": 0,
            "release_publication_result_receipt_ordering_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_ordering_activation_authority_derived_count": 0,
            "release_publication_result_receipt_ordering_activation_command_derived_count": 0,
            "release_publication_result_receipt_ordering_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_ordering_surfaces": release_publication_result_receipt_ordering_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity": denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity,
            "denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity_count": denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity.len(),
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_release_publication_result_receipt_ordering": false,
                    "records_sequence_cursor": false,
                    "records_monotonicity_state": false,
                    "accepts_latest_wins": false,
                    "records_publication_completion_ack": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let release_publication_result_receipt_ordering_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_duplicate_sequence_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_stale_sequence_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_late_arrival_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_future_sequence_gap_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_timestamp_rollback_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_epoch_rollback_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_same_sequence_hash_override_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_latest_wins_overwrite_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_query_ordering_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_export_ordering_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_ordering_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_completion_ack_ordering_accepted",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_duplicate_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_retry_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_snapshot_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "packet_acceptance_receipt_terminal_status_promoted_to_release_approval",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
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
        for key in release_publication_result_receipt_ordering_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in release_publication_result_receipt_ordering_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_ready")
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

    let release_publication_result_receipt_cancellation_supersession_surfaces = vec![
        "publication_result_receipt_cancel_claim",
        "publication_result_receipt_revoke_claim",
        "publication_result_receipt_withdraw_claim",
        "publication_result_receipt_supersede_claim",
        "publication_result_receipt_replacement_receipt_claim",
        "publication_result_receipt_tombstone_claim",
        "publication_result_receipt_delete_marker_claim",
        "publication_result_receipt_latest_replacement_claim",
        "publication_result_receipt_ack_replacement_claim",
        "publication_result_receipt_query_replacement_claim",
        "publication_result_receipt_export_replacement_claim",
        "publication_result_receipt_observability_replacement_claim",
        "publication_result_receipt_release_publication_authority_replacement_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_replacement_claim",
    ]
    .into_iter()
    .map(|release_publication_result_receipt_cancellation_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_cancellation_surface".to_string(),
            serde_json::json!(release_publication_result_receipt_cancellation_surface),
        );
        surface.insert(
            "source_release_publication_result_receipt_ordering_present".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "source_release_publication_result_receipt_ordering_monotonicity_ready".to_string(),
            serde_json::json!(source_ready),
        );
        surface.insert(
            "cancellation_supersession_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "cancellation_supersession_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_cancellation_supersession_status".to_string(),
            serde_json::json!("release_publication_result_receipt_cancellation_supersession_denied"),
        );
        for key in [
            "cancellation_accepted",
            "cancellation_recorded",
            "cancellation_persisted",
            "revocation_accepted",
            "withdrawal_accepted",
            "supersession_accepted",
            "supersession_recorded",
            "supersession_persisted",
            "replacement_receipt_accepted",
            "replacement_receipt_recorded",
            "replacement_receipt_persisted",
            "tombstone_recorded",
            "tombstone_persisted",
            "delete_marker_recorded",
            "latest_replacement_accepted",
            "ack_replacement_accepted",
            "query_replacement_registered",
            "export_replacement_recorded",
            "observability_replacement_recorded",
            "publication_completion_ack_recorded",
            "result_receipt_ordering_recorded",
            "sequence_cursor_recorded",
            "monotonicity_state_recorded",
            "result_receipt_replay_recorded",
            "idempotency_key_registered",
            "idempotency_cache_written",
            "idempotency_cache_hit_promoted",
            "release_publication_recorded",
            "release_artifact_written",
            "public_artifact_written",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "public_release_claimed",
            "public_ga_claimed",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let release_publication_result_receipt_cancellation_supersession_surface_count =
        release_publication_result_receipt_cancellation_supersession_surfaces.len();
    let release_publication_result_receipt_cancellation_supersession_contract_hash_sha256 =
        sha256_text_value(&format!(
            "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial:native:source={source_report_sha256}:surfaces={release_publication_result_receipt_cancellation_supersession_surface_count}:route_count={}:cancellation=0:supersession=0:replacement=0:authority=0:live=0",
            route_matrix.route_count
        ));
    let denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_cancellation_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_cancellation_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_cancellation_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_revocation_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_withdrawal_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_supersession_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_supersession_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_supersession_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replacement_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replacement_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replacement_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_tombstone_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_tombstone_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_delete_marker_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_latest_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ack_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_cancellation_supersession_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_cancellation_supersession_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_from_cancellation_supersession_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_cancellation_supersession_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_cancellation_supersession_denied",
    ];
    let denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession_count =
        denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession.len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_ordering_surface_count") == 14
        && source_u64("release_publication_result_receipt_ordering_attempt_count") == 14
        && source_u64("release_publication_result_receipt_ordering_recorded_count") == 0
        && source_u64("release_publication_result_receipt_ordering_persisted_count") == 0
        && source_u64("release_publication_result_receipt_sequence_cursor_recorded_count") == 0
        && source_u64("release_publication_result_receipt_monotonicity_state_recorded_count") == 0
        && source_u64("release_publication_result_receipt_ordering_acceptance_recorded_count") == 0
        && source_u64(
            "release_publication_result_receipt_ordering_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_ordering_activation_authority_derived_count",
        ) == 0
        && release_publication_result_receipt_cancellation_supersession_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_cancellation_supersession_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_v1",
        "receipt_release_publication_result_receipt_cancellation_supersession_mode": "native_route_denied_release_publication_result_receipt_cannot_use_cancellation_supersession_or_replacement_as_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_ordering_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_ordering_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_ordering_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_ordering_monotonicity_contract_hash_sha256": source["release_publication_result_receipt_ordering_monotonicity_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_cancellation_supersession_contract_hash_sha256": release_publication_result_receipt_cancellation_supersession_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_ready": report_ready,
            "source_release_publication_result_receipt_ordering_surface_count": source_u64("release_publication_result_receipt_ordering_surface_count"),
            "source_release_publication_result_receipt_ordering_attempt_count": source_u64("release_publication_result_receipt_ordering_attempt_count"),
            "source_release_publication_result_receipt_ordering_recorded_count": source_u64("release_publication_result_receipt_ordering_recorded_count"),
            "source_release_publication_result_receipt_ordering_persisted_count": source_u64("release_publication_result_receipt_ordering_persisted_count"),
            "source_release_publication_result_receipt_sequence_cursor_recorded_count": source_u64("release_publication_result_receipt_sequence_cursor_recorded_count"),
            "source_release_publication_result_receipt_monotonicity_state_recorded_count": source_u64("release_publication_result_receipt_monotonicity_state_recorded_count"),
            "source_release_publication_result_receipt_ordering_acceptance_recorded_count": source_u64("release_publication_result_receipt_ordering_acceptance_recorded_count"),
            "source_release_publication_result_receipt_ordering_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_ordering_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_ordering_activation_authority_derived_count": source_u64("release_publication_result_receipt_ordering_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_cancellation_supersession_surface_count": release_publication_result_receipt_cancellation_supersession_surface_count,
            "release_publication_result_receipt_cancellation_supersession_attempt_count": release_publication_result_receipt_cancellation_supersession_surface_count,
            "release_publication_result_receipt_cancellation_accepted_count": 0,
            "release_publication_result_receipt_cancellation_recorded_count": 0,
            "release_publication_result_receipt_cancellation_persisted_count": 0,
            "release_publication_result_receipt_revocation_accepted_count": 0,
            "release_publication_result_receipt_withdrawal_accepted_count": 0,
            "release_publication_result_receipt_supersession_accepted_count": 0,
            "release_publication_result_receipt_supersession_recorded_count": 0,
            "release_publication_result_receipt_supersession_persisted_count": 0,
            "release_publication_result_receipt_replacement_receipt_accepted_count": 0,
            "release_publication_result_receipt_replacement_receipt_recorded_count": 0,
            "release_publication_result_receipt_replacement_receipt_persisted_count": 0,
            "release_publication_result_receipt_tombstone_recorded_count": 0,
            "release_publication_result_receipt_tombstone_persisted_count": 0,
            "release_publication_result_receipt_delete_marker_recorded_count": 0,
            "release_publication_result_receipt_latest_replacement_accepted_count": 0,
            "release_publication_result_receipt_ack_replacement_accepted_count": 0,
            "release_publication_result_receipt_query_replacement_registered_count": 0,
            "release_publication_result_receipt_export_replacement_recorded_count": 0,
            "release_publication_result_receipt_observability_replacement_recorded_count": 0,
            "release_publication_result_receipt_cancellation_supersession_acceptance_recorded_count": 0,
            "release_publication_result_receipt_cancellation_supersession_operator_approval_derived_count": 0,
            "release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count": 0,
            "release_publication_result_receipt_cancellation_supersession_activation_command_derived_count": 0,
            "release_publication_result_receipt_cancellation_supersession_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_cancellation_supersession_surfaces": release_publication_result_receipt_cancellation_supersession_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession": denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession,
            "denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession_count": denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_release_publication_result_receipt_cancellation": false,
                    "records_release_publication_result_receipt_supersession": false,
                    "accepts_replacement_receipt": false,
                    "records_tombstone": false,
                    "records_delete_marker": false,
                    "accepts_latest_replacement": false,
                    "records_publication_completion_ack": false,
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

    let release_publication_result_receipt_cancellation_supersession_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_revocation_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_withdrawal_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_supersession_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_supersession_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_supersession_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_delete_marker_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_ack_replacement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_query_replacement_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_export_replacement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_replacement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_latest_wins_overwrite_accepted",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_duplicate_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_retry_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_snapshot_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "packet_acceptance_receipt_terminal_status_promoted_to_release_approval",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
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
        for key in release_publication_result_receipt_cancellation_supersession_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in release_publication_result_receipt_cancellation_supersession_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_ready")
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

    let release_publication_result_receipt_audit_evidence_surfaces = vec![
        "publication_result_receipt_audit_trail_append_claim",
        "publication_result_receipt_immutable_evidence_claim",
        "publication_result_receipt_hash_chain_claim",
        "publication_result_receipt_merkle_root_claim",
        "publication_result_receipt_attestation_claim",
        "publication_result_receipt_witness_claim",
        "publication_result_receipt_notary_claim",
        "publication_result_receipt_ledger_evidence_claim",
        "publication_result_receipt_index_evidence_claim",
        "publication_result_receipt_delivery_evidence_claim",
        "publication_result_receipt_export_evidence_claim",
        "publication_result_receipt_query_evidence_claim",
        "publication_result_receipt_observability_evidence_claim",
        "publication_result_receipt_readback_evidence_claim",
        "publication_result_receipt_release_publication_authority_evidence_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_evidence_claim",
    ]
    .into_iter()
    .map(
        |release_publication_result_receipt_audit_evidence_surface| {
            let mut surface = serde_json::Map::new();
            surface.insert(
                "release_publication_result_receipt_audit_evidence_surface".to_string(),
                serde_json::json!(release_publication_result_receipt_audit_evidence_surface),
            );
            surface.insert(
                "source_release_publication_result_receipt_cancellation_supersession_ready"
                    .to_string(),
                serde_json::json!(source_ready),
            );
            surface.insert(
                "audit_or_evidence_attempted".to_string(),
                serde_json::json!(true),
            );
            surface.insert(
                "audit_evidence_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            surface.insert(
                "release_publication_result_receipt_audit_evidence_status".to_string(),
                serde_json::json!(
                    "release_publication_result_receipt_audit_trail_immutable_evidence_denied"
                ),
            );
            for key in [
                "audit_trail_accepted",
                "audit_trail_recorded",
                "audit_trail_persisted",
                "audit_trail_materialized",
                "immutable_evidence_accepted",
                "immutable_evidence_recorded",
                "immutable_evidence_persisted",
                "immutable_evidence_materialized",
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
                "publication_completion_ack_recorded",
                "cancellation_recorded",
                "supersession_recorded",
                "replacement_receipt_recorded",
                "tombstone_recorded",
                "result_receipt_ordering_recorded",
                "result_receipt_replay_recorded",
                "release_publication_recorded",
                "release_artifact_written",
                "public_artifact_written",
                "public_distribution_performed",
                "channel_delivery_performed",
                "external_send_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "acceptance_recorded",
                "operator_approval_derived",
                "release_publication_authority_derived",
                "activation_authority_derived",
                "activation_command_derived",
                "live_execution_allowed",
                "activation_performed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "provider_invoked",
                "model_invoked",
                "credential_read",
                "secret_file_read",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "active_binary_mutated",
            ] {
                surface.insert(key.to_string(), serde_json::json!(false));
            }
            serde_json::Value::Object(surface)
        },
    )
    .collect::<Vec<_>>();
    let release_publication_result_receipt_audit_evidence_surface_count =
        release_publication_result_receipt_audit_evidence_surfaces.len();
    let release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256 =
        sha256_text_value(&format!(
            "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-audit-trail-immutable-evidence-denial:native:source={source_report_sha256}:surfaces={release_publication_result_receipt_audit_evidence_surface_count}:route_count={}:audit=0:evidence=0:hashchain=0:authority=0:live=0",
            route_matrix.route_count
        ));
    let denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_trail_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_trail_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_trail_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_trail_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_immutable_evidence_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_immutable_evidence_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_immutable_evidence_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_immutable_evidence_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_hash_chain_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_hash_chain_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_merkle_root_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_merkle_root_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_attestation_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_witness_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_notary_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ledger_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_index_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_delivery_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_readback_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_completion_ack_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_audit_evidence_denied",
    ];
    let denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_count =
        denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence
            .len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_cancellation_supersession_surface_count")
            == 14
        && source_u64("release_publication_result_receipt_cancellation_supersession_attempt_count")
            == 14
        && source_u64("release_publication_result_receipt_cancellation_recorded_count") == 0
        && source_u64("release_publication_result_receipt_supersession_recorded_count") == 0
        && source_u64("release_publication_result_receipt_replacement_receipt_recorded_count") == 0
        && source_u64("release_publication_result_receipt_tombstone_recorded_count") == 0
        && source_u64("release_publication_result_receipt_latest_replacement_accepted_count") == 0
        && source_u64(
            "release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count",
        ) == 0
        && release_publication_result_receipt_audit_evidence_surface_count == 16
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-audit-trail-immutable-evidence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_audit_trail_immutable_evidence_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_v1",
        "receipt_release_publication_result_receipt_audit_trail_immutable_evidence_mode": "native_route_denied_release_publication_result_receipt_cannot_become_audit_trail_immutable_evidence_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_cancellation_supersession_contract_hash_sha256": source["release_publication_result_receipt_cancellation_supersession_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256": release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_ready": report_ready,
            "source_release_publication_result_receipt_cancellation_supersession_surface_count": source_u64("release_publication_result_receipt_cancellation_supersession_surface_count"),
            "source_release_publication_result_receipt_cancellation_supersession_attempt_count": source_u64("release_publication_result_receipt_cancellation_supersession_attempt_count"),
            "source_release_publication_result_receipt_cancellation_recorded_count": source_u64("release_publication_result_receipt_cancellation_recorded_count"),
            "source_release_publication_result_receipt_supersession_recorded_count": source_u64("release_publication_result_receipt_supersession_recorded_count"),
            "source_release_publication_result_receipt_replacement_receipt_recorded_count": source_u64("release_publication_result_receipt_replacement_receipt_recorded_count"),
            "source_release_publication_result_receipt_tombstone_recorded_count": source_u64("release_publication_result_receipt_tombstone_recorded_count"),
            "source_release_publication_result_receipt_latest_replacement_accepted_count": source_u64("release_publication_result_receipt_latest_replacement_accepted_count"),
            "source_release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count": source_u64("release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_audit_evidence_surface_count": release_publication_result_receipt_audit_evidence_surface_count,
            "release_publication_result_receipt_audit_evidence_attempt_count": release_publication_result_receipt_audit_evidence_surface_count,
            "release_publication_result_receipt_audit_trail_accepted_count": 0,
            "release_publication_result_receipt_audit_trail_recorded_count": 0,
            "release_publication_result_receipt_audit_trail_persisted_count": 0,
            "release_publication_result_receipt_audit_trail_materialized_count": 0,
            "release_publication_result_receipt_immutable_evidence_accepted_count": 0,
            "release_publication_result_receipt_immutable_evidence_recorded_count": 0,
            "release_publication_result_receipt_immutable_evidence_persisted_count": 0,
            "release_publication_result_receipt_immutable_evidence_materialized_count": 0,
            "release_publication_result_receipt_hash_chain_recorded_count": 0,
            "release_publication_result_receipt_hash_chain_persisted_count": 0,
            "release_publication_result_receipt_merkle_root_recorded_count": 0,
            "release_publication_result_receipt_merkle_root_persisted_count": 0,
            "release_publication_result_receipt_attestation_recorded_count": 0,
            "release_publication_result_receipt_attestation_persisted_count": 0,
            "release_publication_result_receipt_witness_recorded_count": 0,
            "release_publication_result_receipt_witness_persisted_count": 0,
            "release_publication_result_receipt_notary_recorded_count": 0,
            "release_publication_result_receipt_notary_persisted_count": 0,
            "release_publication_result_receipt_ledger_evidence_recorded_count": 0,
            "release_publication_result_receipt_ledger_evidence_persisted_count": 0,
            "release_publication_result_receipt_index_evidence_recorded_count": 0,
            "release_publication_result_receipt_index_evidence_persisted_count": 0,
            "release_publication_result_receipt_delivery_evidence_recorded_count": 0,
            "release_publication_result_receipt_delivery_evidence_persisted_count": 0,
            "release_publication_result_receipt_export_evidence_recorded_count": 0,
            "release_publication_result_receipt_query_evidence_registered_count": 0,
            "release_publication_result_receipt_observability_evidence_recorded_count": 0,
            "release_publication_result_receipt_readback_evidence_recorded_count": 0,
            "release_publication_result_receipt_publication_completion_ack_recorded_count": 0,
            "release_publication_result_receipt_audit_evidence_acceptance_recorded_count": 0,
            "release_publication_result_receipt_audit_evidence_operator_approval_derived_count": 0,
            "release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_audit_evidence_activation_authority_derived_count": 0,
            "release_publication_result_receipt_audit_evidence_activation_command_derived_count": 0,
            "release_publication_result_receipt_audit_evidence_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_audit_evidence_surfaces": release_publication_result_receipt_audit_evidence_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence": denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence,
            "denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_count": denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_audit_trail": false,
                    "accepts_immutable_evidence": false,
                    "records_hash_chain": false,
                    "records_ledger_evidence": false,
                    "records_publication_completion_ack": false,
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

    let release_publication_result_receipt_audit_evidence_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_chain_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_merkle_root_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_merkle_root_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_attestation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_attestation_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_witness_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_witness_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_notary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_notary_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_ledger_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ledger_evidence_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_index_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_index_evidence_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_evidence_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_export_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_query_evidence_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_supersession_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "packet_acceptance_receipt_terminal_status_promoted_to_release_approval",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
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
        for key in release_publication_result_receipt_audit_evidence_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in release_publication_result_receipt_audit_evidence_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_ready")
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

    let retention_surfaces = vec![
        "publication_result_receipt_retention_policy_claim",
        "publication_result_receipt_retention_index_claim",
        "publication_result_receipt_retention_ledger_claim",
        "publication_result_receipt_ttl_update_claim",
        "publication_result_receipt_ttl_extension_claim",
        "publication_result_receipt_expiry_schedule_claim",
        "publication_result_receipt_expiry_timer_claim",
        "publication_result_receipt_expiry_ack_claim",
        "publication_result_receipt_garbage_collection_scan_claim",
        "publication_result_receipt_garbage_collection_candidate_claim",
        "publication_result_receipt_garbage_collection_decision_claim",
        "publication_result_receipt_delete_claim",
        "publication_result_receipt_tombstone_claim",
        "publication_result_receipt_sweep_claim",
        "publication_result_receipt_archive_claim",
        "publication_result_receipt_compaction_claim",
        "publication_result_receipt_release_publication_authority_retention_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_retention_claim",
    ]
    .into_iter()
    .map(|retention_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_retention_surface".to_string(),
            serde_json::json!(retention_surface),
        );
        surface.insert(
            "source_release_publication_result_receipt_audit_evidence_ready".to_string(),
            serde_json::json!(source_ready),
        );
        surface.insert(
            "retention_expiry_or_garbage_collection_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "retention_expiry_garbage_collection_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_retention_status".to_string(),
            serde_json::json!(
                "release_publication_result_receipt_retention_expiry_garbage_collection_denied"
            ),
        );
        for key in [
            "retention_policy_accepted",
            "retention_policy_recorded",
            "retention_policy_persisted",
            "retention_policy_materialized",
            "retention_index_recorded",
            "retention_index_persisted",
            "retention_ledger_recorded",
            "retention_ledger_persisted",
            "ttl_update_accepted",
            "ttl_update_recorded",
            "ttl_update_persisted",
            "ttl_extension_accepted",
            "ttl_extension_recorded",
            "ttl_extension_persisted",
            "expiry_accepted",
            "expiry_recorded",
            "expiry_persisted",
            "expiry_scheduler_registered",
            "expiry_timer_started",
            "expiry_ack_recorded",
            "garbage_collection_accepted",
            "garbage_collection_scan_performed",
            "garbage_collection_candidate_recorded",
            "garbage_collection_decision_recorded",
            "garbage_collection_persisted",
            "delete_accepted",
            "delete_marker_recorded",
            "delete_performed",
            "tombstone_recorded",
            "tombstone_persisted",
            "sweep_performed",
            "archive_written",
            "archive_persisted",
            "compaction_performed",
            "compaction_artifact_written",
            "compaction_artifact_persisted",
            "ledger_retention_recorded",
            "ledger_retention_persisted",
            "index_retention_recorded",
            "index_retention_persisted",
            "delivery_retention_recorded",
            "delivery_retention_persisted",
            "audit_trail_recorded",
            "immutable_evidence_recorded",
            "hash_chain_recorded",
            "readback_evidence_recorded",
            "publication_completion_ack_recorded",
            "release_publication_recorded",
            "release_artifact_written",
            "public_artifact_written",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let retention_surface_count = retention_surfaces.len();
    let retention_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial:native:source={source_report_sha256}:surfaces={retention_surface_count}:route_count={}:retention=0:expiry=0:gc=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denials = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_policy_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_policy_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_policy_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_index_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_ledger_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ttl_update_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ttl_extension_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_scheduler_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_timer_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_ack_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_scan_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_candidate_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_decision_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_delete_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_tombstone_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_sweep_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_archive_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_compaction_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ledger_index_delivery_retention_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_completion_ack_from_retention_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_retention_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_retention_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_from_retention_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_retention_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_retention_denied",
    ];
    let denials_count = denials.len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_audit_evidence_surface_count") == 16
        && source_u64("release_publication_result_receipt_audit_evidence_attempt_count") == 16
        && source_u64("release_publication_result_receipt_audit_trail_recorded_count") == 0
        && source_u64("release_publication_result_receipt_immutable_evidence_recorded_count") == 0
        && source_u64("release_publication_result_receipt_hash_chain_recorded_count") == 0
        && source_u64("release_publication_result_receipt_ledger_evidence_recorded_count") == 0
        && source_u64("release_publication_result_receipt_readback_evidence_recorded_count") == 0
        && source_u64(
            "release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_audit_evidence_activation_authority_derived_count",
        ) == 0
        && retention_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_retention_expiry_garbage_collection_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_v1",
        "receipt_release_publication_result_receipt_retention_expiry_garbage_collection_mode": "native_route_denied_release_publication_result_receipt_cannot_create_retention_expiry_garbage_collection_state_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256": source["release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256": retention_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_ready": report_ready,
            "source_release_publication_result_receipt_audit_evidence_surface_count": source_u64("release_publication_result_receipt_audit_evidence_surface_count"),
            "source_release_publication_result_receipt_audit_evidence_attempt_count": source_u64("release_publication_result_receipt_audit_evidence_attempt_count"),
            "source_release_publication_result_receipt_audit_trail_recorded_count": source_u64("release_publication_result_receipt_audit_trail_recorded_count"),
            "source_release_publication_result_receipt_immutable_evidence_recorded_count": source_u64("release_publication_result_receipt_immutable_evidence_recorded_count"),
            "source_release_publication_result_receipt_hash_chain_recorded_count": source_u64("release_publication_result_receipt_hash_chain_recorded_count"),
            "source_release_publication_result_receipt_ledger_evidence_recorded_count": source_u64("release_publication_result_receipt_ledger_evidence_recorded_count"),
            "source_release_publication_result_receipt_readback_evidence_recorded_count": source_u64("release_publication_result_receipt_readback_evidence_recorded_count"),
            "source_release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_audit_evidence_activation_authority_derived_count": source_u64("release_publication_result_receipt_audit_evidence_activation_authority_derived_count"),
            "release_publication_result_receipt_retention_surface_count": retention_surface_count,
            "release_publication_result_receipt_retention_attempt_count": retention_surface_count,
            "release_publication_result_receipt_retention_policy_accepted_count": 0,
            "release_publication_result_receipt_retention_policy_recorded_count": 0,
            "release_publication_result_receipt_retention_policy_persisted_count": 0,
            "release_publication_result_receipt_retention_policy_materialized_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_retention_index_recorded_count": 0,
            "release_publication_result_receipt_retention_index_persisted_count": 0,
            "release_publication_result_receipt_retention_ledger_recorded_count": 0,
            "release_publication_result_receipt_retention_ledger_persisted_count": 0,
            "release_publication_result_receipt_ttl_update_accepted_count": 0,
            "release_publication_result_receipt_ttl_update_recorded_count": 0,
            "release_publication_result_receipt_ttl_update_persisted_count": 0,
            "release_publication_result_receipt_ttl_extension_accepted_count": 0,
            "release_publication_result_receipt_ttl_extension_recorded_count": 0,
            "release_publication_result_receipt_ttl_extension_persisted_count": 0,
            "release_publication_result_receipt_expiry_accepted_count": 0,
            "release_publication_result_receipt_expiry_recorded_count": 0,
            "release_publication_result_receipt_expiry_persisted_count": 0,
            "release_publication_result_receipt_expiry_scheduler_registered_count": 0,
            "release_publication_result_receipt_expiry_timer_started_count": 0,
            "release_publication_result_receipt_expiry_ack_recorded_count": 0,
            "release_publication_result_receipt_garbage_collection_accepted_count": 0,
            "release_publication_result_receipt_garbage_collection_scan_performed_count": 0,
            "release_publication_result_receipt_garbage_collection_candidate_recorded_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_garbage_collection_decision_recorded_count": 0,
            "release_publication_result_receipt_garbage_collection_persisted_count": 0,
            "release_publication_result_receipt_delete_accepted_count": 0,
            "release_publication_result_receipt_delete_marker_recorded_count": 0,
            "release_publication_result_receipt_delete_performed_count": 0,
            "release_publication_result_receipt_tombstone_recorded_count": 0,
            "release_publication_result_receipt_tombstone_persisted_count": 0,
            "release_publication_result_receipt_sweep_performed_count": 0,
            "release_publication_result_receipt_archive_written_count": 0,
            "release_publication_result_receipt_archive_persisted_count": 0,
            "release_publication_result_receipt_compaction_performed_count": 0,
            "release_publication_result_receipt_compaction_artifact_written_count": 0,
            "release_publication_result_receipt_compaction_artifact_persisted_count": 0,
            "release_publication_result_receipt_ledger_retention_recorded_count": 0,
            "release_publication_result_receipt_index_retention_recorded_count": 0,
            "release_publication_result_receipt_delivery_retention_recorded_count": 0,
            "release_publication_result_receipt_retention_acceptance_recorded_count": 0,
            "release_publication_result_receipt_retention_operator_approval_derived_count": 0,
            "release_publication_result_receipt_retention_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_retention_activation_authority_derived_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_retention_activation_command_derived_count": 0,
            "release_publication_result_receipt_retention_live_execution_allowed_count": 0,
            "release_publication_result_receipt_retention_surfaces": retention_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_retention_expiry_garbage_collection": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_count": denials_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_retention_policy": false,
                    "records_expiry": false,
                    "performs_garbage_collection": false,
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
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

    let retention_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_retention_policy_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_policy_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_ledger_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ttl_update_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ttl_extension_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_scheduler_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_timer_started",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_candidate_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delete_marker_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delete_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_sweep_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_archive_written",
        "packet_acceptance_receipt_release_publication_result_receipt_compaction_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_compaction_artifact_written",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_supersession_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_lifecycle_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
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
        for key in retention_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in retention_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_ready")
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

    let export_query_observability_surfaces = vec![
        "publication_result_receipt_query_registration_claim",
        "publication_result_receipt_query_execution_claim",
        "publication_result_receipt_query_result_claim",
        "publication_result_receipt_search_index_claim",
        "publication_result_receipt_export_request_claim",
        "publication_result_receipt_export_snapshot_claim",
        "publication_result_receipt_export_file_claim",
        "publication_result_receipt_export_stream_claim",
        "publication_result_receipt_observability_metric_claim",
        "publication_result_receipt_observability_log_claim",
        "publication_result_receipt_observability_trace_claim",
        "publication_result_receipt_observability_event_claim",
        "publication_result_receipt_dashboard_panel_claim",
        "publication_result_receipt_alert_slo_claim",
        "publication_result_receipt_operator_summary_readback_claim",
        "publication_result_receipt_audit_view_evidence_claim",
        "publication_result_receipt_release_publication_authority_view_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_view_claim",
    ]
    .into_iter()
    .map(|export_query_observability_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_export_query_observability_surface".to_string(),
            serde_json::json!(export_query_observability_surface),
        );
        surface.insert(
            "source_release_publication_result_receipt_retention_ready".to_string(),
            serde_json::json!(source_ready),
        );
        surface.insert(
            "export_query_or_observability_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "export_query_observability_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_export_query_observability_status".to_string(),
            serde_json::json!(
                "release_publication_result_receipt_export_query_observability_denied"
            ),
        );
        for key in [
            "query_registered",
            "query_executed",
            "query_result_recorded",
            "query_result_persisted",
            "search_index_recorded",
            "search_index_persisted",
            "export_requested",
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
            "publication_completion_ack_recorded",
            "release_publication_recorded",
            "release_artifact_written",
            "public_artifact_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let export_query_observability_surface_count = export_query_observability_surfaces.len();
    let export_query_observability_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial:native:source={source_report_sha256}:surfaces={export_query_observability_surface_count}:route_count={}:query=0:export=0:observability=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denials = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_registration_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_execution_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_result_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_result_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_search_index_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_search_index_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_request_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_snapshot_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_snapshot_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_file_write_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_stream_open_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_metric_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_log_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_trace_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_event_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_dashboard_panel_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_alert_registration_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_slo_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_operator_summary_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_readback_surface_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_view_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ledger_index_delivery_observability_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_completion_ack_from_view_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_view_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_result_receipt_view_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_from_result_receipt_view_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_result_receipt_view_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_result_receipt_view_denied",
    ];
    let denials_count = denials.len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_retention_surface_count") == 18
        && source_u64("release_publication_result_receipt_retention_attempt_count") == 18
        && source_u64("release_publication_result_receipt_retention_policy_recorded_count") == 0
        && source_u64("release_publication_result_receipt_expiry_recorded_count") == 0
        && source_u64("release_publication_result_receipt_garbage_collection_scan_performed_count")
            == 0
        && source_u64("release_publication_result_receipt_delete_performed_count") == 0
        && source_u64("release_publication_result_receipt_archive_written_count") == 0
        && source_u64("release_publication_result_receipt_compaction_artifact_written_count") == 0
        && source_u64(
            "release_publication_result_receipt_retention_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_retention_activation_authority_derived_count",
        ) == 0
        && export_query_observability_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_export_query_observability_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_v1",
        "receipt_release_publication_result_receipt_export_query_observability_mode": "native_route_denied_release_publication_result_receipt_cannot_create_export_query_observability_views_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_retention_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_retention_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_retention_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_retention_contract_hash_sha256": source["release_publication_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_export_query_observability_contract_hash_sha256": export_query_observability_contract_hash_sha256,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_retention_surface_count": source_u64("release_publication_result_receipt_retention_surface_count"),
            "source_release_publication_result_receipt_retention_attempt_count": source_u64("release_publication_result_receipt_retention_attempt_count"),
            "source_release_publication_result_receipt_retention_policy_recorded_count": source_u64("release_publication_result_receipt_retention_policy_recorded_count"),
            "source_release_publication_result_receipt_expiry_recorded_count": source_u64("release_publication_result_receipt_expiry_recorded_count"),
            "source_release_publication_result_receipt_garbage_collection_scan_performed_count": source_u64("release_publication_result_receipt_garbage_collection_scan_performed_count"),
            "source_release_publication_result_receipt_delete_performed_count": source_u64("release_publication_result_receipt_delete_performed_count"),
            "source_release_publication_result_receipt_archive_written_count": source_u64("release_publication_result_receipt_archive_written_count"),
            "source_release_publication_result_receipt_compaction_artifact_written_count": source_u64("release_publication_result_receipt_compaction_artifact_written_count"),
            "source_release_publication_result_receipt_retention_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_retention_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_retention_activation_authority_derived_count": source_u64("release_publication_result_receipt_retention_activation_authority_derived_count"),
            "release_publication_result_receipt_export_query_observability_surface_count": export_query_observability_surface_count,
            "release_publication_result_receipt_export_query_observability_attempt_count": export_query_observability_surface_count,
            "release_publication_result_receipt_query_registered_count": 0,
            "release_publication_result_receipt_query_executed_count": 0,
            "release_publication_result_receipt_query_result_recorded_count": 0,
            "release_publication_result_receipt_query_result_persisted_count": 0,
            "release_publication_result_receipt_search_index_recorded_count": 0,
            "release_publication_result_receipt_search_index_persisted_count": 0,
            "release_publication_result_receipt_export_requested_count": 0,
            "release_publication_result_receipt_export_accepted_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_export_snapshot_recorded_count": 0,
            "release_publication_result_receipt_export_snapshot_persisted_count": 0,
            "release_publication_result_receipt_export_file_written_count": 0,
            "release_publication_result_receipt_export_stream_opened_count": 0,
            "release_publication_result_receipt_observability_metric_recorded_count": 0,
            "release_publication_result_receipt_observability_log_recorded_count": 0,
            "release_publication_result_receipt_observability_trace_recorded_count": 0,
            "release_publication_result_receipt_observability_event_recorded_count": 0,
            "release_publication_result_receipt_dashboard_panel_recorded_count": 0,
            "release_publication_result_receipt_alert_registered_count": 0,
            "release_publication_result_receipt_slo_recorded_count": 0,
            "release_publication_result_receipt_operator_summary_recorded_count": 0,
            "release_publication_result_receipt_readback_surface_recorded_count": 0,
            "release_publication_result_receipt_audit_view_recorded_count": 0,
            "release_publication_result_receipt_ledger_observability_recorded_count": 0,
            "release_publication_result_receipt_index_observability_recorded_count": 0,
            "release_publication_result_receipt_delivery_observability_recorded_count": 0,
            "release_publication_result_receipt_export_query_observability_acceptance_recorded_count": 0,
            "release_publication_result_receipt_export_query_observability_operator_approval_derived_count": 0,
            "release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_export_query_observability_activation_authority_derived_count": 0,
            "release_publication_result_receipt_export_query_observability_activation_command_derived_count": 0,
            "release_publication_result_receipt_export_query_observability_live_execution_allowed_count": 0,
            "release_publication_result_receipt_export_query_observability_surfaces": export_query_observability_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_export_query_observability": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_export_query_observability_count": denials_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "records_summary": false,
                    "records_briefing": false,
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

    let export_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_query_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_search_index_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_search_index_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_export_requested",
        "packet_acceptance_receipt_release_publication_result_receipt_export_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_export_file_written",
        "packet_acceptance_receipt_release_publication_result_receipt_export_stream_opened",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_metric_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_log_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_trace_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_event_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_panel_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_alert_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_slo_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_surface_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_view_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ledger_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_index_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_ledger_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_archive_written",
        "packet_acceptance_receipt_release_publication_result_receipt_compaction_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
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
        for key in export_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in export_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}
