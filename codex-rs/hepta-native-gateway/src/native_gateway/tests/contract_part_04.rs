#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_endpoint_blocks_payload_exposure()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_REDACTION_PRIVACY_PAYLOAD_EXPOSURE_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
        "operator readiness packet template packet acceptance receipt redaction privacy route json",
    );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_REDACTION_PRIVACY_PAYLOAD_EXPOSURE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_export_query_observability_ready"],
        true
    );
    assert_eq!(value["source_export_query_observability_surface_count"], 16);
    assert_eq!(value["source_query_registered_count"], 0);
    assert_eq!(value["source_export_snapshot_recorded_count"], 0);
    assert_eq!(value["source_observability_metric_recorded_count"], 0);
    assert_eq!(value["source_operator_summary_recorded_count"], 0);
    assert_eq!(value["source_readback_surface_recorded_count"], 0);
    assert_eq!(
        value["source_export_query_observability_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["redaction_privacy_surface_count"], 16);
    assert_eq!(value["redaction_privacy_attempt_count"], 16);
    assert_eq!(value["redacted_payload_preview_recorded_count"], 0);
    assert_eq!(value["payload_hash_preview_recorded_count"], 0);
    assert_eq!(value["payload_diff_recorded_count"], 0);
    assert_eq!(value["readback_text_recorded_count"], 0);
    assert_eq!(value["operator_summary_text_recorded_count"], 0);
    assert_eq!(value["privacy_review_recorded_count"], 0);
    assert_eq!(value["privacy_review_persisted_count"], 0);
    assert_eq!(value["secret_scan_performed_count"], 0);
    assert_eq!(value["pii_scan_performed_count"], 0);
    assert_eq!(value["raw_payload_inspected_count"], 0);
    assert_eq!(value["plaintext_materialized_count"], 0);
    assert_eq!(value["redaction_bypass_allowed_count"], 0);
    assert_eq!(value["hash_to_payload_link_recorded_count"], 0);
    assert_eq!(value["external_redaction_review_performed_count"], 0);
    assert_eq!(value["privacy_acceptance_recorded_count"], 0);
    assert_eq!(value["redaction_privacy_acceptance_recorded_count"], 0);
    assert_eq!(
        value["redaction_privacy_operator_approval_derived_count"],
        0
    );
    assert_eq!(
        value["redaction_privacy_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["redaction_privacy_activation_command_derived_count"],
        0
    );
    assert_eq!(value["redaction_privacy_live_execution_allowed_count"], 0);

    let surfaces = value["redaction_privacy_surfaces"]
        .as_array()
        .expect("packet acceptance receipt redaction/privacy surfaces");
    assert_eq!(surfaces.len(), 16);
    assert_eq!(
        surfaces[0]["redaction_privacy_surface"],
        "packet_receipt_redacted_payload_preview_claim"
    );
    for surface in surfaces {
        assert_eq!(
            surface["redaction_privacy_or_payload_exposure_attempted"],
            true
        );
        assert_eq!(surface["redacted_payload_preview_recorded"], false);
        assert_eq!(surface["payload_hash_preview_recorded"], false);
        assert_eq!(surface["payload_diff_recorded"], false);
        assert_eq!(surface["readback_text_recorded"], false);
        assert_eq!(surface["operator_summary_text_recorded"], false);
        assert_eq!(surface["privacy_review_recorded"], false);
        assert_eq!(surface["secret_scan_performed"], false);
        assert_eq!(surface["pii_scan_performed"], false);
        assert_eq!(surface["raw_payload_inspected"], false);
        assert_eq!(surface["plaintext_materialized"], false);
        assert_eq!(surface["redaction_bypass_allowed"], false);
        assert_eq!(surface["hash_to_payload_link_recorded"], false);
        assert_eq!(surface["external_redaction_review_performed"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(
            surface["redaction_privacy_status"],
            "redaction_privacy_payload_exposure_denied"
        );
    }

    let denied = value["denied_by_packet_receipt_redaction_privacy"]
        .as_array()
        .expect("packet acceptance receipt redaction/privacy denials");
    assert_eq!(denied.len(), 17);
    assert_eq!(
        value["denied_by_packet_receipt_redaction_privacy_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt redaction/privacy next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_redacted_payload_preview_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_payload_hash_preview_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_readback_text_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_operator_summary_text_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_privacy_review_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_secret_scan_performed"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_raw_payload_inspected"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_plaintext_materialized"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_hash_to_payload_link_recorded"],
        false
    );
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["hepta_intelligence_context_attached"], false);
    assert_eq!(value["context_injection_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["external_kg_adapter_read_performed"], false);
    assert_eq!(value["network_call_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_redacted_payload_preview_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_privacy_review_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_secret_scan_performed"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_plaintext_materialized"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_hash_to_payload_link_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_endpoint_blocks_delivery_and_authority()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_OPERATOR_BRIEFING_NON_PERSISTENCE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
        "operator readiness packet template packet acceptance receipt operator briefing route json",
    );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_OPERATOR_BRIEFING_NON_PERSISTENCE_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_redaction_privacy_ready"],
        true
    );
    assert_eq!(value["source_redaction_privacy_surface_count"], 16);
    assert_eq!(value["source_redacted_payload_preview_recorded_count"], 0);
    assert_eq!(value["source_readback_text_recorded_count"], 0);
    assert_eq!(value["source_operator_summary_text_recorded_count"], 0);
    assert_eq!(value["source_privacy_review_recorded_count"], 0);
    assert_eq!(value["source_secret_scan_performed_count"], 0);
    assert_eq!(value["source_raw_payload_inspected_count"], 0);
    assert_eq!(
        value["source_redaction_privacy_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["operator_briefing_surface_count"], 14);
    assert_eq!(value["operator_briefing_attempt_count"], 14);
    assert_eq!(value["briefing_recorded_count"], 0);
    assert_eq!(value["briefing_persisted_count"], 0);
    assert_eq!(value["briefing_materialized_count"], 0);
    assert_eq!(value["briefing_filesystem_written_count"], 0);
    assert_eq!(value["summary_recorded_count"], 0);
    assert_eq!(value["readback_digest_recorded_count"], 0);
    assert_eq!(value["final_note_recorded_count"], 0);
    assert_eq!(value["status_banner_recorded_count"], 0);
    assert_eq!(value["timeline_entry_recorded_count"], 0);
    assert_eq!(value["notification_preview_recorded_count"], 0);
    assert_eq!(value["channel_delivery_performed_count"], 0);
    assert_eq!(value["external_send_performed_count"], 0);
    assert_eq!(value["telegram_send_performed_count"], 0);
    assert_eq!(value["completion_ack_recorded_count"], 0);
    assert_eq!(value["operator_briefing_acceptance_recorded_count"], 0);
    assert_eq!(
        value["operator_briefing_operator_approval_derived_count"],
        0
    );
    assert_eq!(
        value["operator_briefing_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["operator_briefing_activation_command_derived_count"],
        0
    );
    assert_eq!(value["operator_briefing_live_execution_allowed_count"], 0);

    let surfaces = value["operator_briefing_surfaces"]
        .as_array()
        .expect("packet acceptance receipt operator briefing surfaces");
    assert_eq!(surfaces.len(), 14);
    assert_eq!(
        surfaces[0]["briefing_surface"],
        "packet_receipt_operator_briefing_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["briefing_attempted"], true);
        assert_eq!(surface["briefing_recorded"], false);
        assert_eq!(surface["briefing_persisted"], false);
        assert_eq!(surface["briefing_materialized"], false);
        assert_eq!(surface["briefing_filesystem_written"], false);
        assert_eq!(surface["summary_recorded"], false);
        assert_eq!(surface["readback_digest_recorded"], false);
        assert_eq!(surface["final_note_recorded"], false);
        assert_eq!(surface["status_banner_recorded"], false);
        assert_eq!(surface["timeline_entry_recorded"], false);
        assert_eq!(surface["notification_preview_recorded"], false);
        assert_eq!(surface["channel_delivery_performed"], false);
        assert_eq!(surface["external_send_performed"], false);
        assert_eq!(surface["telegram_send_performed"], false);
        assert_eq!(surface["completion_ack_recorded"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(
            surface["briefing_status"],
            "operator_briefing_non_persistence_denied"
        );
    }

    let denied = value["denied_by_packet_receipt_operator_briefing"]
        .as_array()
        .expect("packet acceptance receipt operator briefing denials");
    assert_eq!(denied.len(), 16);
    assert_eq!(
        value["denied_by_packet_receipt_operator_briefing_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt operator briefing next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_final_ack_non_acceptance_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_operator_briefing_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_operator_briefing_persisted"],
        false
    );
    assert_eq!(value["packet_acceptance_receipt_summary_recorded"], false);
    assert_eq!(
        value["packet_acceptance_receipt_readback_digest_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_final_note_recorded"],
        false
    );
    assert_eq!(value["packet_acceptance_receipt_channel_delivered"], false);
    assert_eq!(value["packet_acceptance_receipt_external_sent"], false);
    assert_eq!(value["packet_acceptance_receipt_telegram_sent"], false);
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["hepta_intelligence_context_attached"], false);
    assert_eq!(value["context_injection_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["external_kg_adapter_read_performed"], false);
    assert_eq!(value["network_call_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_operator_briefing_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_operator_briefing_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_channel_delivered"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_telegram_sent"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_endpoint_blocks_acceptance_and_authority()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_FINAL_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt final acknowledgement route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_FINAL_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_operator_briefing_ready"],
        true
    );
    assert_eq!(value["source_operator_briefing_surface_count"], 14);
    assert_eq!(value["source_briefing_recorded_count"], 0);
    assert_eq!(value["source_briefing_persisted_count"], 0);
    assert_eq!(value["source_briefing_materialized_count"], 0);
    assert_eq!(value["source_summary_recorded_count"], 0);
    assert_eq!(value["source_readback_digest_recorded_count"], 0);
    assert_eq!(value["source_final_note_recorded_count"], 0);
    assert_eq!(value["source_channel_delivery_performed_count"], 0);
    assert_eq!(value["source_external_send_performed_count"], 0);
    assert_eq!(value["source_telegram_send_performed_count"], 0);
    assert_eq!(value["source_completion_ack_recorded_count"], 0);
    assert_eq!(
        value["source_operator_briefing_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["final_acknowledgement_surface_count"], 14);
    assert_eq!(value["final_acknowledgement_attempt_count"], 14);
    assert_eq!(value["final_acknowledgement_accepted_count"], 0);
    assert_eq!(value["final_acknowledgement_recorded_count"], 0);
    assert_eq!(value["final_acknowledgement_persisted_count"], 0);
    assert_eq!(value["final_acknowledgement_materialized_count"], 0);
    assert_eq!(value["final_acknowledgement_delivered_count"], 0);
    assert_eq!(value["operator_received_recorded_count"], 0);
    assert_eq!(value["operator_confirmed_recorded_count"], 0);
    assert_eq!(value["operator_read_recorded_count"], 0);
    assert_eq!(value["operator_seen_recorded_count"], 0);
    assert_eq!(value["final_response_recorded_count"], 0);
    assert_eq!(value["completion_ack_recorded_count"], 0);
    assert_eq!(value["status_ack_recorded_count"], 0);
    assert_eq!(value["briefing_ack_recorded_count"], 0);
    assert_eq!(value["readback_ack_recorded_count"], 0);
    assert_eq!(value["channel_ack_delivered_count"], 0);
    assert_eq!(value["external_ack_sent_count"], 0);
    assert_eq!(value["final_acknowledgement_acceptance_recorded_count"], 0);
    assert_eq!(
        value["final_acknowledgement_operator_approval_derived_count"],
        0
    );
    assert_eq!(
        value["final_acknowledgement_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["final_acknowledgement_activation_command_derived_count"],
        0
    );
    assert_eq!(
        value["final_acknowledgement_live_execution_allowed_count"],
        0
    );

    let surfaces = value["final_acknowledgement_surfaces"]
        .as_array()
        .expect("packet acceptance receipt final acknowledgement surfaces");
    assert_eq!(surfaces.len(), 14);
    assert_eq!(
        surfaces[0]["final_acknowledgement_surface"],
        "packet_receipt_final_acknowledgement_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["final_acknowledgement_attempted"], true);
        assert_eq!(surface["final_acknowledgement_accepted"], false);
        assert_eq!(surface["final_acknowledgement_recorded"], false);
        assert_eq!(surface["final_acknowledgement_persisted"], false);
        assert_eq!(surface["final_acknowledgement_materialized"], false);
        assert_eq!(surface["final_acknowledgement_delivered"], false);
        assert_eq!(surface["operator_received_recorded"], false);
        assert_eq!(surface["operator_confirmed_recorded"], false);
        assert_eq!(surface["operator_read_recorded"], false);
        assert_eq!(surface["operator_seen_recorded"], false);
        assert_eq!(surface["final_response_recorded"], false);
        assert_eq!(surface["completion_ack_recorded"], false);
        assert_eq!(surface["status_ack_recorded"], false);
        assert_eq!(surface["briefing_ack_recorded"], false);
        assert_eq!(surface["readback_ack_recorded"], false);
        assert_eq!(surface["channel_ack_delivered"], false);
        assert_eq!(surface["external_ack_sent"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(
            surface["final_acknowledgement_status"],
            "final_acknowledgement_non_acceptance_denied"
        );
    }

    let denied = value["denied_by_packet_receipt_final_acknowledgement"]
        .as_array()
        .expect("packet acceptance receipt final acknowledgement denials");
    assert_eq!(denied.len(), 17);
    assert_eq!(
        value["denied_by_packet_receipt_final_acknowledgement_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt final acknowledgement next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_final_acknowledgement_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_final_acknowledgement_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_final_acknowledgement_persisted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_operator_received_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_operator_confirmed_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_operator_read_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_operator_seen_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_completion_ack_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_channel_ack_delivered"],
        false
    );
    assert_eq!(value["packet_acceptance_receipt_external_ack_sent"], false);
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["hepta_intelligence_context_attached"], false);
    assert_eq!(value["context_injection_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["external_kg_adapter_read_performed"], false);
    assert_eq!(value["network_call_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_final_acknowledgement_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_operator_received_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_operator_seen_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_channel_ack_delivered"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_external_ack_sent"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_endpoint_blocks_status_and_public_claim_promotion()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt terminal decision status route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_final_acknowledgement_ready"],
        true
    );
    assert_eq!(value["source_final_acknowledgement_surface_count"], 14);
    assert_eq!(value["source_final_acknowledgement_recorded_count"], 0);
    assert_eq!(value["source_final_acknowledgement_persisted_count"], 0);
    assert_eq!(
        value["source_final_acknowledgement_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["terminal_decision_status_surface_count"], 14);
    assert_eq!(value["terminal_decision_status_attempt_count"], 14);
    assert_eq!(value["terminal_decision_accepted_count"], 0);
    assert_eq!(value["terminal_decision_recorded_count"], 0);
    assert_eq!(value["terminal_decision_persisted_count"], 0);
    assert_eq!(value["terminal_decision_materialized_count"], 0);
    assert_eq!(value["terminal_status_recorded_count"], 0);
    assert_eq!(value["terminal_status_persisted_count"], 0);
    assert_eq!(value["terminal_status_closed_count"], 0);
    assert_eq!(value["terminal_status_ready_count"], 0);
    assert_eq!(value["terminal_status_accepted_count"], 0);
    assert_eq!(value["terminal_status_approved_count"], 0);
    assert_eq!(value["terminal_status_authoritative_count"], 0);
    assert_eq!(value["terminal_status_live_count"], 0);
    assert_eq!(value["final_state_promoted_count"], 0);
    assert_eq!(value["completion_promoted_count"], 0);
    assert_eq!(value["operator_decision_recorded_count"], 0);
    assert_eq!(value["public_status_claimed_count"], 0);
    assert_eq!(value["release_status_claimed_count"], 0);
    assert_eq!(value["dashboard_status_recorded_count"], 0);
    assert_eq!(value["terminal_decision_acceptance_recorded_count"], 0);
    assert_eq!(
        value["terminal_decision_operator_approval_derived_count"],
        0
    );
    assert_eq!(
        value["terminal_decision_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["terminal_decision_activation_command_derived_count"],
        0
    );
    assert_eq!(value["terminal_decision_live_execution_allowed_count"], 0);

    let surfaces = value["terminal_decision_status_surfaces"]
        .as_array()
        .expect("packet acceptance receipt terminal decision status surfaces");
    assert_eq!(surfaces.len(), 14);
    assert_eq!(
        surfaces[0]["terminal_decision_surface"],
        "packet_receipt_terminal_decision_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["terminal_decision_attempted"], true);
        assert_eq!(surface["terminal_decision_accepted"], false);
        assert_eq!(surface["terminal_decision_recorded"], false);
        assert_eq!(surface["terminal_decision_persisted"], false);
        assert_eq!(surface["terminal_status_recorded"], false);
        assert_eq!(surface["terminal_status_closed"], false);
        assert_eq!(surface["terminal_status_ready"], false);
        assert_eq!(surface["terminal_status_accepted"], false);
        assert_eq!(surface["terminal_status_approved"], false);
        assert_eq!(surface["terminal_status_authoritative"], false);
        assert_eq!(surface["terminal_status_live"], false);
        assert_eq!(surface["final_state_promoted"], false);
        assert_eq!(surface["completion_promoted"], false);
        assert_eq!(surface["operator_decision_recorded"], false);
        assert_eq!(surface["public_status_claimed"], false);
        assert_eq!(surface["release_status_claimed"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(
            surface["terminal_decision_status"],
            "terminal_decision_status_promotion_denied"
        );
    }

    let denied = value["denied_by_packet_receipt_terminal_decision_status"]
        .as_array()
        .expect("packet acceptance receipt terminal decision status denials");
    assert_eq!(denied.len(), 20);
    assert_eq!(
        value["denied_by_packet_receipt_terminal_decision_status_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt terminal decision next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_terminal_decision_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_terminal_status_recorded"],
        false
    );
    assert_eq!(value["packet_acceptance_receipt_status_live"], false);
    assert_eq!(
        value["packet_acceptance_receipt_final_state_promoted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_public_status_claimed"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_status_claimed"],
        false
    );
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["public_release_claimed"], false);
    assert_eq!(value["public_ga_claimed"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_terminal_decision_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_terminal_status_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_public_status_claimed"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_endpoint_blocks_publication_and_public_claims()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_terminal_decision_status_ready"],
        true
    );
    assert_eq!(value["source_terminal_decision_status_surface_count"], 14);
    assert_eq!(value["source_terminal_decision_recorded_count"], 0);
    assert_eq!(value["source_terminal_status_recorded_count"], 0);
    assert_eq!(value["source_terminal_status_live_count"], 0);
    assert_eq!(value["source_public_status_claimed_count"], 0);
    assert_eq!(value["source_release_status_claimed_count"], 0);
    assert_eq!(
        value["source_terminal_decision_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["release_publication_surface_count"], 14);
    assert_eq!(value["release_publication_attempt_count"], 14);
    assert_eq!(value["release_publication_allowed_count"], 0);
    assert_eq!(value["release_publication_recorded_count"], 0);
    assert_eq!(value["release_publication_persisted_count"], 0);
    assert_eq!(value["release_artifact_written_count"], 0);
    assert_eq!(value["public_artifact_written_count"], 0);
    assert_eq!(value["artifact_signature_accepted_count"], 0);
    assert_eq!(value["artifact_notarization_accepted_count"], 0);
    assert_eq!(value["publication_queue_enqueued_count"], 0);
    assert_eq!(value["publication_manifest_written_count"], 0);
    assert_eq!(value["public_distribution_performed_count"], 0);
    assert_eq!(value["channel_delivery_performed_count"], 0);
    assert_eq!(value["external_publication_sent_count"], 0);
    assert_eq!(value["public_version_tag_created_count"], 0);
    assert_eq!(value["release_notes_materialized_count"], 0);
    assert_eq!(value["changelog_materialized_count"], 0);
    assert_eq!(value["public_release_claimed_count"], 0);
    assert_eq!(value["public_ga_claimed_count"], 0);
    assert_eq!(value["terminal_status_release_approval_promoted_count"], 0);
    assert_eq!(
        value["release_publication_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["release_publication_live_execution_allowed_count"], 0);

    let surfaces = value["release_publication_surfaces"]
        .as_array()
        .expect("packet acceptance receipt release publication surfaces");
    assert_eq!(surfaces.len(), 14);
    assert_eq!(
        surfaces[0]["release_publication_surface"],
        "packet_receipt_release_artifact_write_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["release_publication_attempted"], true);
        assert_eq!(surface["release_publication_allowed"], false);
        assert_eq!(surface["release_publication_recorded"], false);
        assert_eq!(surface["release_artifact_written"], false);
        assert_eq!(surface["public_artifact_written"], false);
        assert_eq!(surface["publication_queue_enqueued"], false);
        assert_eq!(surface["publication_manifest_written"], false);
        assert_eq!(surface["public_distribution_performed"], false);
        assert_eq!(surface["channel_delivery_performed"], false);
        assert_eq!(surface["external_send_performed"], false);
        assert_eq!(surface["public_release_claimed"], false);
        assert_eq!(surface["public_ga_claimed"], false);
        assert_eq!(
            surface["terminal_status_promoted_to_release_approval"],
            false
        );
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(
            surface["release_publication_status"],
            "release_publication_denied"
        );
    }

    let denied = value["denied_by_packet_receipt_release_publication"]
        .as_array()
        .expect("packet acceptance receipt release publication denials");
    assert_eq!(denied.len(), 17);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt release publication next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_artifact_written"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_public_distribution_performed"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_public_release_claimed"],
        false
    );
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_artifact_written"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_public_distribution_performed"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_public_release_claimed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_endpoint_blocks_persistence_and_authority()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_ready"],
        true
    );
    assert_eq!(value["source_release_publication_surface_count"], 14);
    assert_eq!(value["source_release_publication_allowed_count"], 0);
    assert_eq!(value["source_release_publication_recorded_count"], 0);
    assert_eq!(value["source_release_artifact_written_count"], 0);
    assert_eq!(value["source_public_artifact_written_count"], 0);
    assert_eq!(value["source_public_distribution_performed_count"], 0);
    assert_eq!(value["source_public_release_claimed_count"], 0);
    assert_eq!(value["source_public_ga_claimed_count"], 0);
    assert_eq!(
        value["source_release_publication_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_surface_count"],
        14
    );
    assert_eq!(
        value["release_publication_result_receipt_attempt_count"],
        14
    );
    assert_eq!(value["release_publication_result_receipt_allowed_count"], 0);
    assert_eq!(
        value["release_publication_result_receipt_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_persisted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_materialized_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_filesystem_written_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_ledger_written_count"],
        0
    );
    assert_eq!(value["release_publication_result_receipt_indexed_count"], 0);
    assert_eq!(
        value["release_publication_result_receipt_enqueued_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_delivered_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_exported_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_query_registered_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_observability_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_hash_bound_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_signature_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_timestamp_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_status_accepted_count"],
        0
    );
    assert_eq!(value["publication_completion_ack_recorded_count"], 0);
    assert_eq!(value["publication_completion_ack_persisted_count"], 0);
    assert_eq!(value["publication_completion_ack_accepted_count"], 0);
    assert_eq!(
        value["release_publication_result_receipt_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_live_execution_allowed_count"],
        0
    );

    let surfaces = value["release_publication_result_receipt_surfaces"]
        .as_array()
        .expect("packet acceptance receipt release publication result receipt surfaces");
    assert_eq!(surfaces.len(), 14);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_surface"],
        "source_release_publication_report_required"
    );
    for surface in surfaces {
        assert_eq!(surface["publication_result_receipt_attempted"], true);
        assert_eq!(surface["publication_result_receipt_allowed"], false);
        assert_eq!(surface["publication_result_receipt_accepted"], false);
        assert_eq!(surface["publication_result_receipt_recorded"], false);
        assert_eq!(surface["publication_result_receipt_persisted"], false);
        assert_eq!(
            surface["publication_result_receipt_filesystem_written"],
            false
        );
        assert_eq!(surface["publication_result_receipt_ledger_written"], false);
        assert_eq!(surface["publication_result_receipt_indexed"], false);
        assert_eq!(surface["publication_result_receipt_enqueued"], false);
        assert_eq!(surface["publication_result_receipt_delivered"], false);
        assert_eq!(surface["publication_result_receipt_exported"], false);
        assert_eq!(
            surface["publication_result_receipt_query_registered"],
            false
        );
        assert_eq!(
            surface["publication_result_receipt_observability_recorded"],
            false
        );
        assert_eq!(surface["publication_result_receipt_hash_bound"], false);
        assert_eq!(
            surface["publication_result_receipt_signature_accepted"],
            false
        );
        assert_eq!(
            surface["publication_result_receipt_timestamp_accepted"],
            false
        );
        assert_eq!(surface["publication_result_receipt_status_accepted"], false);
        assert_eq!(surface["publication_completion_ack_recorded"], false);
        assert_eq!(surface["release_artifact_written"], false);
        assert_eq!(surface["public_artifact_written"], false);
        assert_eq!(surface["public_distribution_performed"], false);
        assert_eq!(surface["channel_delivery_performed"], false);
        assert_eq!(surface["external_send_performed"], false);
        assert_eq!(surface["public_release_claimed"], false);
        assert_eq!(surface["public_ga_claimed"], false);
        assert_eq!(
            surface["terminal_status_promoted_to_release_approval"],
            false
        );
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["activation_performed"], false);
        assert_eq!(surface["memory_store_write_performed"], false);
        assert_eq!(surface["provider_invoked"], false);
        assert_eq!(surface["model_invoked"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["service_restarted"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(surface["publication_result_receipt_noop_confirmed"], true);
        assert_eq!(
            surface["release_publication_result_receipt_status"],
            "release_publication_result_receipt_no_persistence_denied"
        );
    }

    let denied =
        value["denied_by_packet_receipt_release_publication_result_receipt_no_persistence"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt denials");
    assert_eq!(denied.len(), 17);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_no_persistence_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt release publication result receipt next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_persisted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_filesystem_written"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_publication_completion_ack_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_artifact_written"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_public_artifact_written"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_public_distribution_performed"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_public_release_claimed"],
        false
    );
    assert_eq!(value["packet_acceptance_receipt_public_ga_claimed"], false);
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_filesystem_written"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_publication_completion_ack_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_public_release_claimed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_endpoint_blocks_replay_and_authority()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt replay/idempotency route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_surface_count"],
        14
    );
    assert_eq!(
        value["source_release_publication_result_receipt_recorded_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_persisted_count"],
        0
    );
    assert_eq!(value["source_publication_completion_ack_recorded_count"], 0);
    assert_eq!(
        value["source_release_publication_result_receipt_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_replay_surface_count"],
        14
    );
    assert_eq!(
        value["release_publication_result_receipt_replay_attempt_count"],
        14
    );
    assert_eq!(
        value["release_publication_result_receipt_replay_allowed_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_replay_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_replay_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_replay_persisted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_duplicate_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_retry_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_idempotency_key_registered_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_idempotency_cache_written_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_idempotency_cache_hit_promoted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_replay_hash_bound_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_replay_signature_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_query_result_replayed_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_export_snapshot_replayed_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_observability_snapshot_replayed_count"],
        0
    );
    assert_eq!(value["publication_completion_ack_replayed_count"], 0);
    assert_eq!(value["publication_completion_ack_recorded_count"], 0);
    assert_eq!(
        value["release_publication_result_receipt_replay_release_publication_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_replay_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_replay_live_execution_allowed_count"],
        0
    );

    let surfaces = value["release_publication_result_receipt_replay_surfaces"]
        .as_array()
        .expect("packet acceptance receipt release publication result receipt replay surfaces");
    assert_eq!(surfaces.len(), 14);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_replay_surface"],
        "publication_result_receipt_replay"
    );
    for surface in surfaces {
        assert_eq!(surface["result_receipt_replay_attempted"], true);
        assert_eq!(surface["result_receipt_replay_allowed"], false);
        assert_eq!(surface["result_receipt_replay_accepted"], false);
        assert_eq!(surface["result_receipt_replay_recorded"], false);
        assert_eq!(surface["result_receipt_replay_persisted"], false);
        assert_eq!(surface["result_receipt_duplicate_accepted"], false);
        assert_eq!(surface["result_receipt_retry_accepted"], false);
        assert_eq!(surface["idempotency_key_registered"], false);
        assert_eq!(surface["idempotency_cache_written"], false);
        assert_eq!(surface["idempotency_cache_hit_promoted"], false);
        assert_eq!(surface["replay_hash_bound"], false);
        assert_eq!(surface["replay_signature_accepted"], false);
        assert_eq!(surface["replay_timestamp_accepted"], false);
        assert_eq!(surface["replay_status_accepted"], false);
        assert_eq!(surface["query_result_replayed"], false);
        assert_eq!(surface["export_snapshot_replayed"], false);
        assert_eq!(surface["observability_snapshot_replayed"], false);
        assert_eq!(surface["publication_completion_ack_replayed"], false);
        assert_eq!(surface["publication_completion_ack_recorded"], false);
        assert_eq!(surface["release_artifact_written"], false);
        assert_eq!(surface["public_artifact_written"], false);
        assert_eq!(surface["public_distribution_performed"], false);
        assert_eq!(surface["external_send_performed"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["activation_performed"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["service_restarted"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(surface["result_receipt_replay_noop_confirmed"], true);
        assert_eq!(
            surface["release_publication_result_receipt_replay_status"],
            "release_publication_result_receipt_replay_idempotency_denied"
        );
    }

    let denied =
        value["denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt replay denials");
    assert_eq!(denied.len(), 16);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt release publication result receipt replay next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_replayed"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_replay_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_publication_completion_ack_replayed"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_artifact_written"],
        false
    );
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_replay_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_publication_completion_ack_replayed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_endpoint_blocks_ordering_and_authority()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt ordering/monotonicity route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_replay_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_replay_surface_count"],
        14
    );
    assert_eq!(
        value["source_release_publication_result_receipt_replay_recorded_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_idempotency_cache_written_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_ordering_surface_count"],
        14
    );
    assert_eq!(
        value["release_publication_result_receipt_ordering_attempt_count"],
        14
    );
    assert_eq!(
        value["release_publication_result_receipt_ordering_allowed_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_ordering_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_sequence_cursor_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_sequence_cursor_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_monotonicity_state_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_duplicate_sequence_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_stale_sequence_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_future_sequence_gap_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_latest_wins_overwrite_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_query_ordering_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_export_ordering_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_observability_ordering_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_completion_ack_ordering_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_ordering_release_publication_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_ordering_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_ordering_live_execution_allowed_count"],
        0
    );

    let surfaces = value["release_publication_result_receipt_ordering_surfaces"]
        .as_array()
        .expect("packet acceptance receipt release publication result receipt ordering surfaces");
    assert_eq!(surfaces.len(), 14);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_ordering_surface"],
        "publication_result_receipt_duplicate_sequence_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["ordering_attempted"], true);
        assert_eq!(surface["ordering_allowed"], false);
        assert_eq!(surface["ordering_recorded"], false);
        assert_eq!(surface["ordering_persisted"], false);
        assert_eq!(surface["sequence_cursor_accepted"], false);
        assert_eq!(surface["sequence_cursor_recorded"], false);
        assert_eq!(surface["monotonicity_state_recorded"], false);
        assert_eq!(surface["duplicate_sequence_accepted"], false);
        assert_eq!(surface["stale_sequence_accepted"], false);
        assert_eq!(surface["future_sequence_gap_accepted"], false);
        assert_eq!(surface["same_sequence_hash_override_accepted"], false);
        assert_eq!(surface["latest_wins_overwrite_accepted"], false);
        assert_eq!(surface["query_ordering_accepted"], false);
        assert_eq!(surface["export_ordering_accepted"], false);
        assert_eq!(surface["observability_ordering_accepted"], false);
        assert_eq!(surface["completion_ack_ordering_accepted"], false);
        assert_eq!(surface["publication_completion_ack_recorded"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["activation_performed"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["service_restarted"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(surface["ordering_noop_confirmed"], true);
        assert_eq!(
            surface["release_publication_result_receipt_ordering_status"],
            "release_publication_result_receipt_ordering_monotonicity_denied"
        );
    }

    let denied =
        value["denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity"]
            .as_array()
            .expect(
                "packet acceptance receipt release publication result receipt ordering denials",
            );
    assert_eq!(denied.len(), 25);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"].as_array().expect(
        "packet acceptance receipt release publication result receipt ordering next actions",
    );
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_latest_wins_overwrite_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_completion_ack_ordering_accepted"],
        false
    );
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_completion_ack_ordering_accepted"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_endpoint_blocks_cancellation_and_authority()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt cancellation/supersession route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_ordering_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_ordering_surface_count"],
        14
    );
    assert_eq!(
        value["source_release_publication_result_receipt_ordering_recorded_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_sequence_cursor_recorded_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_monotonicity_state_recorded_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_ordering_release_publication_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_cancellation_supersession_surface_count"],
        14
    );
    assert_eq!(
        value["release_publication_result_receipt_cancellation_supersession_attempt_count"],
        14
    );
    assert_eq!(
        value["release_publication_result_receipt_cancellation_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_cancellation_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_revocation_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_withdrawal_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_supersession_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_supersession_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_replacement_receipt_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_replacement_receipt_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_tombstone_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_delete_marker_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_latest_replacement_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_ack_replacement_accepted_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_query_replacement_registered_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_export_replacement_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_observability_replacement_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_cancellation_supersession_live_execution_allowed_count"],
        0
    );

    let surfaces = value["release_publication_result_receipt_cancellation_supersession_surfaces"]
        .as_array()
        .expect(
            "packet acceptance receipt release publication result receipt cancellation surfaces",
        );
    assert_eq!(surfaces.len(), 14);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_cancellation_surface"],
        "publication_result_receipt_cancel_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["cancellation_supersession_attempted"], true);
        assert_eq!(surface["cancellation_accepted"], false);
        assert_eq!(surface["cancellation_recorded"], false);
        assert_eq!(surface["revocation_accepted"], false);
        assert_eq!(surface["withdrawal_accepted"], false);
        assert_eq!(surface["supersession_accepted"], false);
        assert_eq!(surface["supersession_recorded"], false);
        assert_eq!(surface["replacement_receipt_accepted"], false);
        assert_eq!(surface["replacement_receipt_recorded"], false);
        assert_eq!(surface["tombstone_recorded"], false);
        assert_eq!(surface["delete_marker_recorded"], false);
        assert_eq!(surface["latest_replacement_accepted"], false);
        assert_eq!(surface["ack_replacement_accepted"], false);
        assert_eq!(surface["query_replacement_registered"], false);
        assert_eq!(surface["export_replacement_recorded"], false);
        assert_eq!(surface["observability_replacement_recorded"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["service_restarted"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(surface["cancellation_supersession_noop_confirmed"], true);
        assert_eq!(
            surface["release_publication_result_receipt_cancellation_supersession_status"],
            "release_publication_result_receipt_cancellation_supersession_denied"
        );
    }

    let denied =
            value["denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession"]
                .as_array()
                .expect(
                    "packet acceptance receipt release publication result receipt cancellation denials",
                );
    assert_eq!(denied.len(), 24);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"].as_array().expect(
        "packet acceptance receipt release publication result receipt cancellation next actions",
    );
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_cancellation_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_supersession_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted"],
        false
    );
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_cancellation_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_supersession_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_endpoint_blocks_audit_evidence_and_authority()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt audit-trail immutable-evidence route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-audit-trail-immutable-evidence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_cancellation_supersession_surface_count"],
        14
    );
    assert_eq!(
        value["source_release_publication_result_receipt_cancellation_supersession_attempt_count"],
        14
    );
    assert_eq!(
        value["source_release_publication_result_receipt_cancellation_recorded_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_supersession_recorded_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_replacement_receipt_recorded_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_tombstone_recorded_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_latest_replacement_accepted_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count"],
        0
    );
    assert_eq!(
        value["source_release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_audit_evidence_surface_count"],
        16
    );
    assert_eq!(
        value["release_publication_result_receipt_audit_evidence_attempt_count"],
        16
    );
    for key in [
        "release_publication_result_receipt_audit_trail_accepted_count",
        "release_publication_result_receipt_audit_trail_recorded_count",
        "release_publication_result_receipt_audit_trail_persisted_count",
        "release_publication_result_receipt_audit_trail_materialized_count",
        "release_publication_result_receipt_immutable_evidence_accepted_count",
        "release_publication_result_receipt_immutable_evidence_recorded_count",
        "release_publication_result_receipt_immutable_evidence_persisted_count",
        "release_publication_result_receipt_immutable_evidence_materialized_count",
        "release_publication_result_receipt_hash_chain_recorded_count",
        "release_publication_result_receipt_merkle_root_recorded_count",
        "release_publication_result_receipt_attestation_recorded_count",
        "release_publication_result_receipt_witness_recorded_count",
        "release_publication_result_receipt_notary_recorded_count",
        "release_publication_result_receipt_ledger_evidence_recorded_count",
        "release_publication_result_receipt_index_evidence_recorded_count",
        "release_publication_result_receipt_delivery_evidence_recorded_count",
        "release_publication_result_receipt_export_evidence_recorded_count",
        "release_publication_result_receipt_query_evidence_registered_count",
        "release_publication_result_receipt_observability_evidence_recorded_count",
        "release_publication_result_receipt_readback_evidence_recorded_count",
        "release_publication_result_receipt_publication_completion_ack_recorded_count",
        "release_publication_result_receipt_audit_evidence_acceptance_recorded_count",
        "release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count",
        "release_publication_result_receipt_audit_evidence_activation_authority_derived_count",
        "release_publication_result_receipt_audit_evidence_activation_command_derived_count",
        "release_publication_result_receipt_audit_evidence_live_execution_allowed_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_audit_evidence_surfaces"]
        .as_array()
        .expect(
            "packet acceptance receipt release publication result receipt audit evidence surfaces",
        );
    assert_eq!(surfaces.len(), 16);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_audit_evidence_surface"],
        "publication_result_receipt_audit_trail_append_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["audit_or_evidence_attempted"], true);
        assert_eq!(surface["audit_trail_accepted"], false);
        assert_eq!(surface["audit_trail_recorded"], false);
        assert_eq!(surface["audit_trail_persisted"], false);
        assert_eq!(surface["audit_trail_materialized"], false);
        assert_eq!(surface["immutable_evidence_accepted"], false);
        assert_eq!(surface["immutable_evidence_recorded"], false);
        assert_eq!(surface["immutable_evidence_persisted"], false);
        assert_eq!(surface["immutable_evidence_materialized"], false);
        assert_eq!(surface["hash_chain_recorded"], false);
        assert_eq!(surface["merkle_root_recorded"], false);
        assert_eq!(surface["attestation_recorded"], false);
        assert_eq!(surface["witness_recorded"], false);
        assert_eq!(surface["notary_recorded"], false);
        assert_eq!(surface["ledger_evidence_recorded"], false);
        assert_eq!(surface["index_evidence_recorded"], false);
        assert_eq!(surface["delivery_evidence_recorded"], false);
        assert_eq!(surface["export_evidence_recorded"], false);
        assert_eq!(surface["query_evidence_registered"], false);
        assert_eq!(surface["observability_evidence_recorded"], false);
        assert_eq!(surface["readback_evidence_recorded"], false);
        assert_eq!(surface["publication_completion_ack_recorded"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["service_restarted"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(surface["audit_evidence_noop_confirmed"], true);
        assert_eq!(
            surface["release_publication_result_receipt_audit_evidence_status"],
            "release_publication_result_receipt_audit_trail_immutable_evidence_denied"
        );
    }

    let denied =
            value["denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence"]
                .as_array()
                .expect(
                    "packet acceptance receipt release publication result receipt audit evidence denials",
                );
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"].as_array().expect(
        "packet acceptance receipt release publication result receipt audit evidence next actions",
    );
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_merkle_root_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_ledger_evidence_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_publication_completion_ack_recorded"],
        false
    );
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["external_send_performed"], false);
    let side_effects = value["side_effects"]
        .as_object()
        .expect("audit evidence side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_first_model_positive_approval_packet_boundary_endpoint_blocks_approval_and_invocation_authority()
 {
    let value = hepta_first_model_positive_approval_packet_boundary_report();

    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_FIRST_MODEL_POSITIVE_APPROVAL_PACKET_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-first-model-positive-approval-packet-boundary --json"
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(
        value["implemented_route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], serde_json::json!(0));
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["first_model_positive_approval_packet_boundary_route_enabled"],
        true
    );
    assert_eq!(
        value["first_model_positive_approval_packet_boundary_ready"],
        true
    );
    assert_eq!(
        value["first_model_positive_approval_packet_boundary_status"],
        "blocked"
    );
    assert_eq!(value["source_artifact_publication_denial_ready"], true);
    assert_eq!(value["source_first_model_terminal_decision_ready"], true);
    assert_eq!(value["positive_approval_packet_item_count"], 12);
    assert_eq!(value["accepted_positive_approval_packet_item_count"], 0);
    assert_eq!(
        value["denied_by_first_model_positive_approval_packet_boundary_count"],
        15
    );
    assert_eq!(value["positive_approval_packet_recorded"], false);
    assert_eq!(value["positive_approval_packet_persisted"], false);
    assert_eq!(value["positive_approval_packet_accepted"], false);
    assert_eq!(value["fresh_operator_approval_artifact_present"], false);
    assert_eq!(value["single_use_nonce_consumed"], false);
    assert_eq!(value["operator_identity_session_bound"], false);
    assert_eq!(value["explicit_invocation_command_accepted"], false);
    for key in [
        "operator_approval_recorded",
        "approval_authority_derived",
        "activation_authority_derived",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let items = value["positive_approval_packet_items"]
        .as_array()
        .expect("positive approval packet items");
    assert_eq!(items.len(), 12);
    assert!(
        items
            .iter()
            .all(|item| item["accepted"].as_bool() == Some(false))
    );
    let denied = value["denied_by_first_model_positive_approval_packet_boundary"]
        .as_array()
        .expect("positive approval packet denials");
    assert_eq!(denied.len(), 15);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_minimal_memory_canary_scoped_operator_packet"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_positive_approval_packet"],
        false
    );
    assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
    assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    let side_effects = value["side_effects"]
        .as_object()
        .expect("positive approval packet boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_scoped_memory_canary_durable_receipt_boundary_endpoint_blocks_durable_memory_mutation() {
    let body = route_contract_body(HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT);

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("scoped memory canary durable receipt json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-scoped-memory-canary-durable-receipt-boundary --json"
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
        value["scoped_memory_canary_durable_receipt_boundary_route_enabled"],
        true
    );
    assert_eq!(
        value["scoped_memory_canary_durable_receipt_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_memory_canary_durable_receipt_boundary_status"],
        "blocked_report_only"
    );
    assert_eq!(
        value["source_first_model_positive_approval_packet_boundary_ready"],
        true
    );
    assert_eq!(value["source_minimal_memory_canary_ready"], true);
    assert_eq!(value["durable_receipt_candidate_count"], 12);
    assert_eq!(value["accepted_durable_receipt_candidate_count"], 0);
    assert_eq!(
        value["denied_by_scoped_memory_canary_durable_receipt_boundary_count"],
        16
    );
    assert_eq!(value["durable_receipt_preview_generated"], true);
    assert_eq!(value["durable_receipt_recorded"], false);
    assert_eq!(value["durable_receipt_persisted"], false);
    assert_eq!(value["durable_receipt_accepted"], false);
    assert_eq!(value["fresh_durable_memory_write_command_required"], true);
    assert_eq!(value["fresh_durable_memory_write_command_present"], false);
    assert_eq!(value["fresh_durable_memory_write_command_accepted"], false);
    assert_eq!(value["accepted_scoped_memory_write_command"], false);
    assert!(value["source_memory_canary_idempotency_receipt_hash_sha256"].is_string());
    assert!(value["source_memory_canary_post_rollback_store_hash_sha256"].is_string());

    let candidates = value["durable_receipt_candidates"]
        .as_array()
        .expect("durable receipt candidates");
    assert_eq!(candidates.len(), 12);
    assert!(
        candidates
            .iter()
            .all(|candidate| candidate["accepted"].as_bool() == Some(false))
    );
    let denied = value["denied_by_scoped_memory_canary_durable_receipt_boundary"]
        .as_array()
        .expect("durable receipt boundary denials");
    assert_eq!(denied.len(), 16);
    let audit_steps = value["audit_steps"]
        .as_array()
        .expect("durable receipt boundary audit steps");
    assert_eq!(audit_steps.len(), 5);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "hepta_bounded_intelligence_context_handoff_prompt_preview_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["uses_scoped_memory_canary_durable_receipt_boundary"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["uses_durable_receipt_hash_only"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_durable_receipt"],
        false
    );
    for key in [
        "memory_write_receipt_recorded",
        "memory_write_receipt_persisted",
        "memory_receipt_ledger_recorded",
        "memory_receipt_index_written",
        "operator_approval_recorded",
        "approval_authority_derived",
        "activation_authority_derived",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "rollback_executed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "live_kg_write_performed",
        "kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("durable receipt boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_endpoint_blocks_lifecycle_mutation()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt retention expiry garbage collection route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_audit_evidence_surface_count"],
        16
    );
    assert_eq!(
        value["source_release_publication_result_receipt_audit_evidence_attempt_count"],
        16
    );
    for key in [
        "source_release_publication_result_receipt_audit_trail_recorded_count",
        "source_release_publication_result_receipt_immutable_evidence_recorded_count",
        "source_release_publication_result_receipt_hash_chain_recorded_count",
        "source_release_publication_result_receipt_ledger_evidence_recorded_count",
        "source_release_publication_result_receipt_readback_evidence_recorded_count",
        "source_release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_audit_evidence_activation_authority_derived_count",
        "release_publication_result_receipt_retention_policy_accepted_count",
        "release_publication_result_receipt_retention_policy_recorded_count",
        "release_publication_result_receipt_retention_policy_persisted_count",
        "release_publication_result_receipt_retention_index_recorded_count",
        "release_publication_result_receipt_retention_ledger_recorded_count",
        "release_publication_result_receipt_ttl_update_recorded_count",
        "release_publication_result_receipt_ttl_extension_recorded_count",
        "release_publication_result_receipt_expiry_accepted_count",
        "release_publication_result_receipt_expiry_recorded_count",
        "release_publication_result_receipt_expiry_scheduler_registered_count",
        "release_publication_result_receipt_expiry_timer_started_count",
        "release_publication_result_receipt_expiry_ack_recorded_count",
        "release_publication_result_receipt_garbage_collection_accepted_count",
        "release_publication_result_receipt_garbage_collection_scan_performed_count",
        "release_publication_result_receipt_garbage_collection_candidate_recorded_count",
        "release_publication_result_receipt_garbage_collection_decision_recorded_count",
        "release_publication_result_receipt_delete_marker_recorded_count",
        "release_publication_result_receipt_delete_performed_count",
        "release_publication_result_receipt_tombstone_recorded_count",
        "release_publication_result_receipt_sweep_performed_count",
        "release_publication_result_receipt_archive_written_count",
        "release_publication_result_receipt_compaction_performed_count",
        "release_publication_result_receipt_compaction_artifact_written_count",
        "release_publication_result_receipt_ledger_retention_recorded_count",
        "release_publication_result_receipt_index_retention_recorded_count",
        "release_publication_result_receipt_delivery_retention_recorded_count",
        "release_publication_result_receipt_retention_acceptance_recorded_count",
        "release_publication_result_receipt_retention_release_publication_authority_derived_count",
        "release_publication_result_receipt_retention_activation_authority_derived_count",
        "release_publication_result_receipt_retention_activation_command_derived_count",
        "release_publication_result_receipt_retention_live_execution_allowed_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_retention_surfaces"]
        .as_array()
        .expect("packet acceptance receipt release publication result receipt retention surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_retention_surface"],
        "publication_result_receipt_retention_policy_claim"
    );
    for surface in surfaces {
        assert_eq!(
            surface["retention_expiry_or_garbage_collection_attempted"],
            true
        );
        assert_eq!(surface["retention_policy_recorded"], false);
        assert_eq!(surface["retention_index_recorded"], false);
        assert_eq!(surface["retention_ledger_recorded"], false);
        assert_eq!(surface["ttl_update_recorded"], false);
        assert_eq!(surface["ttl_extension_recorded"], false);
        assert_eq!(surface["expiry_recorded"], false);
        assert_eq!(surface["expiry_scheduler_registered"], false);
        assert_eq!(surface["expiry_timer_started"], false);
        assert_eq!(surface["garbage_collection_scan_performed"], false);
        assert_eq!(surface["garbage_collection_candidate_recorded"], false);
        assert_eq!(surface["garbage_collection_decision_recorded"], false);
        assert_eq!(surface["delete_marker_recorded"], false);
        assert_eq!(surface["delete_performed"], false);
        assert_eq!(surface["tombstone_recorded"], false);
        assert_eq!(surface["sweep_performed"], false);
        assert_eq!(surface["archive_written"], false);
        assert_eq!(surface["compaction_performed"], false);
        assert_eq!(surface["compaction_artifact_written"], false);
        assert_eq!(surface["audit_trail_recorded"], false);
        assert_eq!(surface["immutable_evidence_recorded"], false);
        assert_eq!(surface["hash_chain_recorded"], false);
        assert_eq!(surface["readback_evidence_recorded"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["service_restarted"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(
            surface["retention_expiry_garbage_collection_noop_confirmed"],
            true
        );
        assert_eq!(
            surface["release_publication_result_receipt_retention_status"],
            "release_publication_result_receipt_retention_expiry_garbage_collection_denied"
        );
    }

    let denied =
            value["denied_by_packet_receipt_release_publication_result_receipt_retention_expiry_garbage_collection"]
                .as_array()
                .expect("packet acceptance receipt release publication result receipt retention denials");
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"].as_array().expect(
        "packet acceptance receipt release publication result receipt retention next actions",
    );
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_scheduler_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_delete_marker_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delete_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_archive_written",
        "packet_acceptance_receipt_release_publication_result_receipt_compaction_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "credential_read",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("retention expiry garbage collection side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_endpoint_blocks_view_materialization()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt export query observability route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_retention_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_retention_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_retention_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_retention_policy_recorded_count",
        "source_release_publication_result_receipt_expiry_recorded_count",
        "source_release_publication_result_receipt_garbage_collection_scan_performed_count",
        "source_release_publication_result_receipt_delete_performed_count",
        "source_release_publication_result_receipt_archive_written_count",
        "source_release_publication_result_receipt_compaction_artifact_written_count",
        "source_release_publication_result_receipt_retention_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_retention_activation_authority_derived_count",
        "release_publication_result_receipt_query_registered_count",
        "release_publication_result_receipt_query_executed_count",
        "release_publication_result_receipt_query_result_recorded_count",
        "release_publication_result_receipt_query_result_persisted_count",
        "release_publication_result_receipt_search_index_recorded_count",
        "release_publication_result_receipt_search_index_persisted_count",
        "release_publication_result_receipt_export_requested_count",
        "release_publication_result_receipt_export_accepted_count",
        "release_publication_result_receipt_export_snapshot_recorded_count",
        "release_publication_result_receipt_export_snapshot_persisted_count",
        "release_publication_result_receipt_export_file_written_count",
        "release_publication_result_receipt_export_stream_opened_count",
        "release_publication_result_receipt_observability_metric_recorded_count",
        "release_publication_result_receipt_observability_log_recorded_count",
        "release_publication_result_receipt_observability_trace_recorded_count",
        "release_publication_result_receipt_observability_event_recorded_count",
        "release_publication_result_receipt_dashboard_panel_recorded_count",
        "release_publication_result_receipt_alert_registered_count",
        "release_publication_result_receipt_slo_recorded_count",
        "release_publication_result_receipt_operator_summary_recorded_count",
        "release_publication_result_receipt_readback_surface_recorded_count",
        "release_publication_result_receipt_audit_view_recorded_count",
        "release_publication_result_receipt_ledger_observability_recorded_count",
        "release_publication_result_receipt_index_observability_recorded_count",
        "release_publication_result_receipt_delivery_observability_recorded_count",
        "release_publication_result_receipt_export_query_observability_acceptance_recorded_count",
        "release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count",
        "release_publication_result_receipt_export_query_observability_activation_authority_derived_count",
        "release_publication_result_receipt_export_query_observability_activation_command_derived_count",
        "release_publication_result_receipt_export_query_observability_live_execution_allowed_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_export_query_observability_surfaces"]
            .as_array()
            .expect(
                "packet acceptance receipt release publication result receipt export query observability surfaces",
            );
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_export_query_observability_surface"],
        "publication_result_receipt_query_registration_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["export_query_or_observability_attempted"], true);
        assert_eq!(surface["query_registered"], false);
        assert_eq!(surface["query_executed"], false);
        assert_eq!(surface["query_result_recorded"], false);
        assert_eq!(surface["search_index_recorded"], false);
        assert_eq!(surface["export_requested"], false);
        assert_eq!(surface["export_snapshot_recorded"], false);
        assert_eq!(surface["export_file_written"], false);
        assert_eq!(surface["export_stream_opened"], false);
        assert_eq!(surface["observability_metric_recorded"], false);
        assert_eq!(surface["observability_log_recorded"], false);
        assert_eq!(surface["observability_trace_recorded"], false);
        assert_eq!(surface["observability_event_recorded"], false);
        assert_eq!(surface["dashboard_panel_recorded"], false);
        assert_eq!(surface["alert_registered"], false);
        assert_eq!(surface["slo_recorded"], false);
        assert_eq!(surface["operator_summary_recorded"], false);
        assert_eq!(surface["readback_surface_recorded"], false);
        assert_eq!(surface["audit_view_recorded"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["service_restarted"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(surface["export_query_observability_noop_confirmed"], true);
        assert_eq!(
            surface["release_publication_result_receipt_export_query_observability_status"],
            "release_publication_result_receipt_export_query_observability_denied"
        );
    }

    let denied =
            value["denied_by_packet_receipt_release_publication_result_receipt_export_query_observability"]
                .as_array()
                .expect("packet acceptance receipt release publication result receipt export denials");
    assert_eq!(denied.len(), 29);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_export_query_observability_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt release publication result receipt export next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_query_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_search_index_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_export_requested",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_export_file_written",
        "packet_acceptance_receipt_release_publication_result_receipt_export_stream_opened",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_metric_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_event_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_panel_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_alert_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_slo_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_surface_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_view_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "credential_read",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("export query observability side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_endpoint_blocks_delivery()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt operator-facing summary briefing route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_export_query_observability_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_export_query_observability_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_operator_facing_summary_briefing_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_operator_facing_summary_briefing_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_operator_summary_recorded_count",
        "source_release_publication_result_receipt_readback_surface_recorded_count",
        "source_release_publication_result_receipt_audit_view_recorded_count",
        "source_release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_export_query_observability_activation_authority_derived_count",
        "release_publication_result_receipt_operator_summary_allowed_count",
        "release_publication_result_receipt_operator_summary_request_accepted_count",
        "release_publication_result_receipt_operator_summary_recorded_count",
        "release_publication_result_receipt_operator_summary_persisted_count",
        "release_publication_result_receipt_operator_summary_materialized_count",
        "release_publication_result_receipt_operator_summary_filesystem_written_count",
        "release_publication_result_receipt_operator_summary_delivered_count",
        "release_publication_result_receipt_operator_briefing_allowed_count",
        "release_publication_result_receipt_operator_briefing_request_accepted_count",
        "release_publication_result_receipt_operator_briefing_recorded_count",
        "release_publication_result_receipt_operator_briefing_persisted_count",
        "release_publication_result_receipt_operator_briefing_materialized_count",
        "release_publication_result_receipt_operator_briefing_filesystem_written_count",
        "release_publication_result_receipt_operator_briefing_delivered_count",
        "release_publication_result_receipt_readback_digest_recorded_count",
        "release_publication_result_receipt_readback_digest_persisted_count",
        "release_publication_result_receipt_final_note_recorded_count",
        "release_publication_result_receipt_final_note_persisted_count",
        "release_publication_result_receipt_status_banner_recorded_count",
        "release_publication_result_receipt_dashboard_annotation_recorded_count",
        "release_publication_result_receipt_notification_preview_recorded_count",
        "release_publication_result_receipt_timeline_entry_recorded_count",
        "release_publication_result_receipt_audit_narrative_recorded_count",
        "release_publication_result_receipt_privacy_review_narrative_recorded_count",
        "release_publication_result_receipt_alert_explanation_recorded_count",
        "release_publication_result_receipt_slo_report_recorded_count",
        "release_publication_result_receipt_operator_summary_briefing_channel_delivery_count",
        "release_publication_result_receipt_operator_summary_briefing_external_send_count",
        "release_publication_result_receipt_operator_summary_briefing_telegram_send_count",
        "release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count",
        "release_publication_result_receipt_operator_summary_briefing_acceptance_recorded_count",
        "release_publication_result_receipt_operator_summary_briefing_operator_approval_derived_count",
        "release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count",
        "release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count",
        "release_publication_result_receipt_operator_summary_briefing_activation_command_derived_count",
        "release_publication_result_receipt_operator_summary_briefing_live_execution_allowed_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces =
            value["release_publication_result_receipt_operator_facing_summary_briefing_surfaces"]
                .as_array()
                .expect("packet acceptance receipt release publication result receipt summary briefing surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_operator_facing_summary_briefing_surface"],
        "publication_result_receipt_operator_summary_request_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["operator_facing_text_attempted"], true);
        assert_eq!(surface["operator_summary_recorded"], false);
        assert_eq!(surface["operator_summary_persisted"], false);
        assert_eq!(surface["operator_summary_materialized"], false);
        assert_eq!(surface["operator_summary_filesystem_written"], false);
        assert_eq!(surface["operator_summary_delivered"], false);
        assert_eq!(surface["operator_briefing_recorded"], false);
        assert_eq!(surface["operator_briefing_persisted"], false);
        assert_eq!(surface["operator_briefing_materialized"], false);
        assert_eq!(surface["operator_briefing_filesystem_written"], false);
        assert_eq!(surface["operator_briefing_delivered"], false);
        assert_eq!(surface["readback_digest_recorded"], false);
        assert_eq!(surface["final_note_recorded"], false);
        assert_eq!(surface["status_banner_recorded"], false);
        assert_eq!(surface["dashboard_annotation_recorded"], false);
        assert_eq!(surface["notification_preview_recorded"], false);
        assert_eq!(surface["timeline_entry_recorded"], false);
        assert_eq!(surface["audit_narrative_recorded"], false);
        assert_eq!(surface["privacy_review_narrative_recorded"], false);
        assert_eq!(surface["alert_explanation_recorded"], false);
        assert_eq!(surface["slo_report_recorded"], false);
        assert_eq!(surface["channel_delivery_performed"], false);
        assert_eq!(surface["telegram_send_performed"], false);
        assert_eq!(surface["external_send_performed"], false);
        assert_eq!(surface["completion_ack_recorded"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["service_restarted"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(
            surface["operator_facing_summary_briefing_noop_confirmed"],
            true
        );
    }

    let denied =
            value["denied_by_packet_receipt_release_publication_result_receipt_operator_facing_summary_briefing"]
                .as_array()
                .expect("packet acceptance receipt release publication result receipt summary briefing denials");
    assert_eq!(denied.len(), 32);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_operator_facing_summary_briefing_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"].as_array().expect(
            "packet acceptance receipt release publication result receipt summary briefing next actions",
        );
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded",
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
    let side_effects = value["side_effects"]
        .as_object()
        .expect("operator-facing summary briefing side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_endpoint_blocks_acknowledgement_and_authority()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt final operator acknowledgement route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_operator_summary_briefing_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_final_operator_acknowledgement_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_final_operator_acknowledgement_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_operator_summary_recorded_count",
        "source_release_publication_result_receipt_operator_briefing_recorded_count",
        "source_release_publication_result_receipt_readback_digest_recorded_count",
        "source_release_publication_result_receipt_final_note_recorded_count",
        "source_release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count",
        "source_release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count",
        "release_publication_result_receipt_final_operator_acknowledgement_allowed_count",
        "release_publication_result_receipt_final_operator_acknowledgement_request_accepted_count",
        "release_publication_result_receipt_final_operator_acknowledgement_accepted_count",
        "release_publication_result_receipt_final_operator_acknowledgement_recorded_count",
        "release_publication_result_receipt_final_operator_acknowledgement_persisted_count",
        "release_publication_result_receipt_final_operator_acknowledgement_materialized_count",
        "release_publication_result_receipt_final_operator_acknowledgement_filesystem_written_count",
        "release_publication_result_receipt_final_operator_acknowledgement_delivered_count",
        "release_publication_result_receipt_operator_received_recorded_count",
        "release_publication_result_receipt_operator_confirmed_recorded_count",
        "release_publication_result_receipt_operator_read_recorded_count",
        "release_publication_result_receipt_operator_seen_recorded_count",
        "release_publication_result_receipt_final_response_recorded_count",
        "release_publication_result_receipt_completion_ack_recorded_count",
        "release_publication_result_receipt_status_ack_recorded_count",
        "release_publication_result_receipt_summary_ack_recorded_count",
        "release_publication_result_receipt_briefing_ack_recorded_count",
        "release_publication_result_receipt_readback_digest_ack_recorded_count",
        "release_publication_result_receipt_dashboard_ack_recorded_count",
        "release_publication_result_receipt_notification_ack_recorded_count",
        "release_publication_result_receipt_channel_ack_delivered_count",
        "release_publication_result_receipt_external_ack_sent_count",
        "release_publication_result_receipt_telegram_ack_sent_count",
        "release_publication_result_receipt_final_operator_acknowledgement_acceptance_recorded_count",
        "release_publication_result_receipt_final_operator_acknowledgement_operator_approval_derived_count",
        "release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count",
        "release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count",
        "release_publication_result_receipt_final_operator_acknowledgement_activation_command_derived_count",
        "release_publication_result_receipt_final_operator_acknowledgement_live_execution_allowed_count",
        "release_publication_result_receipt_final_operator_acknowledgement_install_executed_count",
        "release_publication_result_receipt_final_operator_acknowledgement_service_restarted_count",
        "release_publication_result_receipt_final_operator_acknowledgement_active_binary_mutated_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces =
            value["release_publication_result_receipt_final_operator_acknowledgement_surfaces"]
                .as_array()
                .expect("packet acceptance receipt release publication result receipt final acknowledgement surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_final_operator_acknowledgement_surface"],
        "publication_result_receipt_final_operator_acknowledgement_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["final_operator_acknowledgement_attempted"], true);
        assert_eq!(surface["final_operator_acknowledgement_accepted"], false);
        assert_eq!(surface["final_operator_acknowledgement_recorded"], false);
        assert_eq!(surface["final_operator_acknowledgement_persisted"], false);
        assert_eq!(
            surface["final_operator_acknowledgement_materialized"],
            false
        );
        assert_eq!(
            surface["final_operator_acknowledgement_filesystem_written"],
            false
        );
        assert_eq!(surface["final_operator_acknowledgement_delivered"], false);
        assert_eq!(surface["operator_received_recorded"], false);
        assert_eq!(surface["operator_confirmed_recorded"], false);
        assert_eq!(surface["operator_read_recorded"], false);
        assert_eq!(surface["operator_seen_recorded"], false);
        assert_eq!(surface["final_response_recorded"], false);
        assert_eq!(surface["completion_ack_recorded"], false);
        assert_eq!(surface["status_ack_recorded"], false);
        assert_eq!(surface["summary_ack_recorded"], false);
        assert_eq!(surface["briefing_ack_recorded"], false);
        assert_eq!(surface["readback_digest_ack_recorded"], false);
        assert_eq!(surface["dashboard_ack_recorded"], false);
        assert_eq!(surface["notification_ack_recorded"], false);
        assert_eq!(surface["channel_ack_delivered"], false);
        assert_eq!(surface["external_ack_sent"], false);
        assert_eq!(surface["telegram_ack_sent"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["service_restarted"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(
            surface["final_operator_acknowledgement_noop_confirmed"],
            true
        );
    }

    let denied =
            value["denied_by_packet_receipt_release_publication_result_receipt_final_operator_acknowledgement"]
                .as_array()
                .expect("packet acceptance receipt release publication result receipt final acknowledgement denials");
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_final_operator_acknowledgement_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_gate"
    );
    for key in [
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
        "packet_acceptance_receipt_release_publication_result_receipt_channel_ack_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_ack_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_ack_sent",
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
    let side_effects = value["side_effects"]
        .as_object()
        .expect("final operator acknowledgement side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_endpoint_blocks_status_and_authority()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal decision status route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-decision-status-promotion-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_final_operator_acknowledgement_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_decision_status_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_decision_status_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_final_operator_acknowledgement_accepted_count",
        "source_release_publication_result_receipt_final_operator_acknowledgement_recorded_count",
        "source_release_publication_result_receipt_final_operator_acknowledgement_persisted_count",
        "source_release_publication_result_receipt_final_operator_acknowledgement_materialized_count",
        "source_release_publication_result_receipt_final_operator_acknowledgement_delivered_count",
        "source_release_publication_result_receipt_operator_received_recorded_count",
        "source_release_publication_result_receipt_operator_confirmed_recorded_count",
        "source_release_publication_result_receipt_completion_ack_recorded_count",
        "source_release_publication_result_receipt_status_ack_recorded_count",
        "source_release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_decision_allowed_count",
        "release_publication_result_receipt_terminal_decision_request_accepted_count",
        "release_publication_result_receipt_terminal_decision_accepted_count",
        "release_publication_result_receipt_terminal_decision_recorded_count",
        "release_publication_result_receipt_terminal_decision_persisted_count",
        "release_publication_result_receipt_terminal_decision_materialized_count",
        "release_publication_result_receipt_terminal_decision_filesystem_written_count",
        "release_publication_result_receipt_terminal_decision_delivered_count",
        "release_publication_result_receipt_terminal_status_recorded_count",
        "release_publication_result_receipt_terminal_status_persisted_count",
        "release_publication_result_receipt_terminal_status_closed_count",
        "release_publication_result_receipt_status_ready_count",
        "release_publication_result_receipt_status_accepted_count",
        "release_publication_result_receipt_status_approved_count",
        "release_publication_result_receipt_status_authoritative_count",
        "release_publication_result_receipt_status_live_count",
        "release_publication_result_receipt_final_state_promoted_count",
        "release_publication_result_receipt_completion_promoted_count",
        "release_publication_result_receipt_operator_decision_recorded_count",
        "release_publication_result_receipt_public_status_claimed_count",
        "release_publication_result_receipt_release_status_claimed_count",
        "release_publication_result_receipt_publication_status_claimed_count",
        "release_publication_result_receipt_dashboard_status_recorded_count",
        "release_publication_result_receipt_channel_status_delivered_count",
        "release_publication_result_receipt_external_status_sent_count",
        "release_publication_result_receipt_telegram_status_sent_count",
        "release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_decision_status_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_decision_status_install_executed_count",
        "release_publication_result_receipt_terminal_decision_status_service_restarted_count",
        "release_publication_result_receipt_terminal_decision_status_active_binary_mutated_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_decision_status_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal decision status surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_decision_status_surface"],
        "publication_result_receipt_terminal_decision_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["terminal_decision_attempted"], true);
        assert_eq!(surface["terminal_decision_allowed"], false);
        assert_eq!(surface["terminal_decision_accepted"], false);
        assert_eq!(surface["terminal_decision_recorded"], false);
        assert_eq!(surface["terminal_decision_persisted"], false);
        assert_eq!(surface["terminal_decision_materialized"], false);
        assert_eq!(surface["terminal_decision_filesystem_written"], false);
        assert_eq!(surface["terminal_decision_delivered"], false);
        assert_eq!(surface["terminal_status_recorded"], false);
        assert_eq!(surface["terminal_status_persisted"], false);
        assert_eq!(surface["terminal_status_closed"], false);
        assert_eq!(surface["terminal_status_ready"], false);
        assert_eq!(surface["terminal_status_accepted"], false);
        assert_eq!(surface["terminal_status_approved"], false);
        assert_eq!(surface["terminal_status_authoritative"], false);
        assert_eq!(surface["terminal_status_live"], false);
        assert_eq!(surface["final_state_promoted"], false);
        assert_eq!(surface["completion_promoted"], false);
        assert_eq!(surface["operator_decision_recorded"], false);
        assert_eq!(surface["public_status_claimed"], false);
        assert_eq!(surface["release_status_claimed"], false);
        assert_eq!(surface["publication_status_claimed"], false);
        assert_eq!(surface["channel_status_delivered"], false);
        assert_eq!(surface["external_status_sent"], false);
        assert_eq!(surface["telegram_status_sent"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["terminal_decision_status_noop_confirmed"], true);
    }

    let denied =
            value["denied_by_packet_receipt_release_publication_result_receipt_terminal_decision_status"]
                .as_array()
                .expect("packet acceptance receipt release publication result receipt terminal decision status denials");
    assert_eq!(denied.len(), 32);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_decision_status_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_closed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_live",
        "packet_acceptance_receipt_release_publication_result_receipt_final_state_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_public_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_publication_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent",
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
    let side_effects = value["side_effects"]
        .as_object()
        .expect("release publication result receipt terminal decision status side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_endpoint_blocks_public_exposure_and_authority()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal public claim status exposure route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_decision_status_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_decision_accepted_count",
        "source_release_publication_result_receipt_terminal_decision_recorded_count",
        "source_release_publication_result_receipt_terminal_status_recorded_count",
        "source_release_publication_result_receipt_public_status_claimed_count",
        "source_release_publication_result_receipt_release_status_claimed_count",
        "source_release_publication_result_receipt_publication_status_claimed_count",
        "source_release_publication_result_receipt_dashboard_status_recorded_count",
        "source_release_publication_result_receipt_channel_status_delivered_count",
        "source_release_publication_result_receipt_external_status_sent_count",
        "source_release_publication_result_receipt_telegram_status_sent_count",
        "source_release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_allowed_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_request_accepted_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_persisted_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_materialized_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_filesystem_written_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_delivered_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposed_count",
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
        "release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_install_executed_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_service_restarted_count",
        "release_publication_result_receipt_terminal_public_claim_status_exposure_active_binary_mutated_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_public_claim_status_exposure_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal public claim status exposure surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_public_claim_status_exposure_surface"],
        "publication_result_receipt_public_claim_status_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["public_claim_status_exposure_attempted"], true);
        assert_eq!(surface["public_claim_status_exposure_allowed"], false);
        assert_eq!(surface["public_claim_status_exposure_accepted"], false);
        assert_eq!(surface["public_claim_status_exposure_recorded"], false);
        assert_eq!(surface["public_claim_status_exposure_persisted"], false);
        assert_eq!(surface["public_claim_status_exposed"], false);
        assert_eq!(surface["public_status_claimed"], false);
        assert_eq!(surface["public_release_claimed"], false);
        assert_eq!(surface["public_ga_claimed"], false);
        assert_eq!(surface["dashboard_status_exposed"], false);
        assert_eq!(surface["status_endpoint_exposed"], false);
        assert_eq!(surface["query_status_exposed"], false);
        assert_eq!(surface["export_status_exposed"], false);
        assert_eq!(surface["observability_status_exposed"], false);
        assert_eq!(surface["channel_status_delivered"], false);
        assert_eq!(surface["external_status_sent"], false);
        assert_eq!(surface["telegram_status_sent"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["public_claim_status_exposure_noop_confirmed"], true);
    }

    let denied =
            value["denied_by_packet_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure"]
                .as_array()
                .expect("packet acceptance receipt release publication result receipt terminal public claim status exposure denials");
    assert_eq!(denied.len(), 34);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_status_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_status_live",
        "packet_acceptance_receipt_release_publication_result_receipt_public_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_publication_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted",
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
        "release publication result receipt terminal public claim status exposure side effects",
    );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_endpoint_blocks_distribution_and_artifact_status()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_QUEUE_ARTIFACT_AVAILABILITY_STATUS_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution queue artifact availability route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_QUEUE_ARTIFACT_AVAILABILITY_STATUS_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_artifact_status_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count",
        "source_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count",
        "source_release_publication_result_receipt_terminal_public_claim_status_exposed_count",
        "source_release_publication_result_receipt_artifact_availability_status_exposed_count",
        "source_release_publication_result_receipt_distribution_queue_status_exposed_count",
        "source_release_publication_result_receipt_channel_status_delivered_count",
        "source_release_publication_result_receipt_external_status_sent_count",
        "source_release_publication_result_receipt_telegram_status_sent_count",
        "source_release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_allowed_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_request_accepted_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_materialized_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count",
        "release_publication_result_receipt_distribution_queue_status_exposed_count",
        "release_publication_result_receipt_distribution_queue_enqueued_count",
        "release_publication_result_receipt_distribution_worker_dispatched_count",
        "release_publication_result_receipt_artifact_availability_status_exposed_count",
        "release_publication_result_receipt_artifact_manifest_entry_exposed_count",
        "release_publication_result_receipt_artifact_download_url_exposed_count",
        "release_publication_result_receipt_artifact_checksum_exposed_count",
        "release_publication_result_receipt_artifact_signature_notarization_exposed_count",
        "release_publication_result_receipt_package_index_status_exposed_count",
        "release_publication_result_receipt_update_feed_status_exposed_count",
        "release_publication_result_receipt_cdn_mirror_status_exposed_count",
        "release_publication_result_receipt_release_channel_status_exposed_count",
        "release_publication_result_receipt_public_bucket_listing_status_exposed_count",
        "release_publication_result_receipt_status_endpoint_artifact_ready_exposed_count",
        "release_publication_result_receipt_dashboard_artifact_available_badge_exposed_count",
        "release_publication_result_receipt_channel_status_delivered_count",
        "release_publication_result_receipt_external_status_sent_count",
        "release_publication_result_receipt_telegram_status_sent_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_activation_command_derived_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_service_restarted_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_release_artifact_written_count",
        "release_publication_result_receipt_terminal_distribution_artifact_status_public_artifact_written_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_artifact_status_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution artifact status surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_artifact_status_surface"],
        "publication_result_receipt_distribution_queue_ready_status"
    );
    for surface in surfaces {
        assert_eq!(
            surface["terminal_distribution_artifact_status_attempted"],
            true
        );
        assert_eq!(
            surface["terminal_distribution_artifact_status_allowed"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_artifact_status_request_accepted"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_artifact_status_accepted"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_artifact_status_recorded"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_artifact_status_persisted"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_artifact_status_exposed"],
            false
        );
        assert_eq!(surface["distribution_queue_status_exposed"], false);
        assert_eq!(surface["distribution_queue_enqueued"], false);
        assert_eq!(surface["distribution_worker_dispatched"], false);
        assert_eq!(surface["artifact_availability_status_exposed"], false);
        assert_eq!(surface["artifact_manifest_entry_exposed"], false);
        assert_eq!(surface["artifact_download_url_exposed"], false);
        assert_eq!(surface["package_index_status_exposed"], false);
        assert_eq!(surface["update_feed_status_exposed"], false);
        assert_eq!(surface["cdn_mirror_status_exposed"], false);
        assert_eq!(surface["release_channel_status_exposed"], false);
        assert_eq!(surface["channel_status_delivered"], false);
        assert_eq!(surface["external_status_sent"], false);
        assert_eq!(surface["telegram_status_sent"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["release_artifact_written"], false);
        assert_eq!(surface["public_artifact_written"], false);
        assert_eq!(
            surface["terminal_distribution_artifact_status_noop_confirmed"],
            true
        );
    }

    let denied =
            value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_artifact_status"]
                .as_array()
                .expect("packet acceptance receipt release publication result receipt terminal distribution artifact denials");
    assert_eq!(denied.len(), 35);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed",
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
        "packet_acceptance_receipt_release_publication_result_receipt_package_index_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_update_feed_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_cdn_mirror_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_channel_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_artifact_ready_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_artifact_available_badge_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent",
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
        "release publication result receipt terminal distribution artifact status side effects",
    );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_endpoint_blocks_external_delivery_receipts()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_EXTERNAL_DELIVERY_NON_PERSISTENCE_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_EXTERNAL_DELIVERY_NON_PERSISTENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_artifact_status_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count",
        "source_release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count",
        "source_release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count",
        "source_release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count",
        "source_release_publication_result_receipt_distribution_queue_enqueued_count",
        "source_release_publication_result_receipt_distribution_worker_dispatched_count",
        "source_release_publication_result_receipt_artifact_download_url_exposed_count",
        "source_release_publication_result_receipt_channel_status_delivered_count",
        "source_release_publication_result_receipt_external_status_sent_count",
        "source_release_publication_result_receipt_telegram_status_sent_count",
        "source_release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_request_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_queued_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count",
        "release_publication_result_receipt_status_endpoint_delivery_receipt_exposed_count",
        "release_publication_result_receipt_dashboard_delivery_receipt_exposed_count",
        "release_publication_result_receipt_delivery_confirmation_recorded_count",
        "release_publication_result_receipt_delivery_ack_recorded_count",
        "release_publication_result_receipt_receipt_echo_delivered_count",
        "release_publication_result_receipt_downstream_consumer_notified_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_command_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_service_restarted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_artifact_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_public_artifact_written_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_surface"],
        "publication_result_receipt_distribution_delivery_receipt_creation"
    );
    for surface in surfaces {
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_attempted"],
            true
        );
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_allowed"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_accepted"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_recorded"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_persisted"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_ledger_written"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_index_written"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_externally_sent"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_channel_sent"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_webhook_sent"],
            false
        );
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_telegram_sent"],
            false
        );
        assert_eq!(surface["delivery_confirmation_recorded"], false);
        assert_eq!(surface["delivery_ack_recorded"], false);
        assert_eq!(surface["receipt_echo_delivered"], false);
        assert_eq!(surface["downstream_consumer_notified"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["release_artifact_written"], false);
        assert_eq!(surface["public_artifact_written"], false);
        assert_eq!(
            surface["terminal_distribution_delivery_receipt_noop_confirmed"],
            true
        );
    }

    let denied =
            value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt"]
                .as_array()
                .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt denials");
    assert_eq!(denied.len(), 36);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_queued",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_delivery_receipt_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_delivery_receipt_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_confirmation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_receipt_echo_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_downstream_consumer_notified",
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
        "release publication result receipt terminal distribution delivery receipt side effects",
    );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}
