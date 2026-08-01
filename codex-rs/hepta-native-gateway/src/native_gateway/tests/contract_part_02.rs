#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_materialization_lane_endpoint_enables_payload_shape_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator-approved KG prompt payload materialization lane json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_kg_prompt_payload_materialization_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_ENDPOINT
        );
    assert_eq!(
            value["kg_prompt_preview_read_only_adapter_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(
        value["kg_prompt_preview_read_only_adapter_lane_ready"],
        true
    );
    assert_eq!(
        value["kg_prompt_preview_read_only_adapter_lane_status"],
        "ready"
    );
    assert_eq!(value["operator_authorization_received"], true);
    assert_eq!(
        value["operator_authorization_scope"],
        "kg_prompt_payload_materialization_lane_no_report_payload_no_kg_live_write_provider_model_channel_or_public_release"
    );
    assert_eq!(value["operator_approved_activation_lane_present"], true);
    assert_eq!(value["operator_approved_activation_lane_effective"], true);
    assert_eq!(value["memory_durable_mutation_lane_enabled"], true);
    assert_eq!(value["memory_store_write_path_enabled"], true);
    assert_eq!(value["memory_store_mutation_enabled"], true);
    assert_eq!(value["live_memory_write_allowed_by_lane"], true);
    assert_eq!(value["live_memory_write_performed_by_report_route"], false);
    assert_eq!(
        value["hepta_intelligence_context_attachment_lane_enabled"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_context_attachment_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_context_attached_by_report_route"],
        false
    );
    assert_eq!(value["bounded_prompt_preview_lane_enabled"], true);
    assert_eq!(value["bounded_prompt_preview_allowed_by_lane"], true);
    assert_eq!(value["prompt_preview_rendered_by_report_route"], false);
    assert_eq!(value["prompt_preview_requires_explicit_command"], true);
    assert_eq!(value["prompt_payload_materialized_by_report_route"], false);
    assert_eq!(value["kg_prompt_preview_lane_enabled"], true);
    assert_eq!(value["kg_prompt_preview_allowed_by_lane"], true);
    assert_eq!(value["kg_prompt_preview_rendered_by_report_route"], false);
    assert_eq!(value["kg_external_adapter_read_lane_enabled"], true);
    assert_eq!(value["kg_external_adapter_read_allowed_by_lane"], true);
    assert_eq!(
        value["kg_external_adapter_read_performed_by_report_route"],
        false
    );
    assert_eq!(value["kg_external_adapter_requires_explicit_command"], true);
    assert_eq!(
        value["kg_external_adapter_credential_reference_required"],
        true
    );
    assert_eq!(
        value["kg_external_adapter_credential_read_allowed_by_lane"],
        false
    );
    assert_eq!(
        value["kg_external_adapter_credential_read_performed_by_report_route"],
        false
    );
    assert_eq!(value["supported_kg_adapter_count"], 3);
    assert_eq!(
        value["kg_prompt_payload_materialization_lane_enabled"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_materialization_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_materialized_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_shape_requires_explicit_command"],
        true
    );
    assert_eq!(value["kg_prompt_payload_redaction_required"], true);
    assert_eq!(
        value["kg_prompt_payload_raw_text_exposed_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_hash_preview_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_hash_preview_rendered_by_report_route"],
        false
    );
    assert_eq!(value["context_handoff_acceptance_required"], true);
    assert_eq!(value["context_attachment_requires_explicit_command"], true);
    assert_eq!(value["context_injection_allowed_by_lane"], false);
    assert_eq!(value["context_injection_performed_by_report_route"], false);
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["kg_live_write_allowed_by_lane"], false);
    assert_eq!(value["kg_live_write_performed_by_report_route"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_allowed_by_lane"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 4);
    assert_eq!(value["enablement_lane_count"], 7);
    assert_eq!(value["ready_enablement_lane_count"], 7);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked KG prompt payload materialization lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"materialize_prompt_payload_from_report_route"));
    assert!(blocked.contains(&"expose_raw_prompt_payload_from_report_route"));
    assert!(blocked.contains(&"read_kg_adapter_from_report_route"));
    assert!(blocked.contains(&"read_auth_secret_or_credential"));
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(
        value["side_effects"]["hepta_intelligence_context_attached"],
        false
    );
    assert_eq!(value["side_effects"]["prompt_preview_rendered"], false);
    assert_eq!(value["side_effects"]["prompt_payload_materialized"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_acceptance_receipt_lane_endpoint_enables_redacted_receipt_shape_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator-approved KG prompt payload acceptance receipt lane json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-acceptance-receipt-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_kg_prompt_payload_acceptance_receipt_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_ENDPOINT
        );
    assert_eq!(
            value["kg_prompt_payload_materialization_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["kg_prompt_payload_materialization_lane_ready"], true);
    assert_eq!(
        value["kg_prompt_payload_materialization_lane_status"],
        "ready"
    );
    assert_eq!(value["operator_authorization_received"], true);
    assert_eq!(
        value["operator_authorization_scope"],
        "kg_prompt_payload_acceptance_receipt_lane_no_report_receipt_persistence_no_kg_live_write_provider_model_channel_or_public_release"
    );
    assert_eq!(value["memory_store_mutation_enabled"], true);
    assert_eq!(
        value["hepta_intelligence_context_attachment_lane_enabled"],
        true
    );
    assert_eq!(value["kg_prompt_preview_lane_enabled"], true);
    assert_eq!(value["kg_external_adapter_read_lane_enabled"], true);
    assert_eq!(value["supported_kg_adapter_count"], 3);
    assert_eq!(
        value["kg_prompt_payload_materialization_lane_enabled"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_materialization_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_materialized_by_report_route"],
        false
    );
    assert_eq!(value["kg_prompt_payload_redaction_required"], true);
    assert_eq!(
        value["kg_prompt_payload_raw_text_exposed_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_lane_enabled"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_redaction_required"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_redaction_proof_required"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_hash_binding_required"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_raw_payload_allowed"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_filesystem_written_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_ledger_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_promotes_activation_authority"],
        false
    );
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 5);
    assert_eq!(value["enablement_lane_count"], 8);
    assert_eq!(value["ready_enablement_lane_count"], 8);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked KG prompt payload acceptance receipt lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"record_prompt_payload_acceptance_receipt_from_report_route"));
    assert!(blocked.contains(&"persist_prompt_payload_acceptance_receipt_from_report_route"));
    assert!(blocked.contains(&"accept_prompt_payload_acceptance_receipt_from_report_route"));
    assert!(blocked.contains(&"write_prompt_payload_acceptance_receipt_filesystem_artifact"));
    assert!(blocked.contains(&"record_prompt_payload_acceptance_receipt_ledger_entry"));
    assert!(blocked.contains(&"promote_receipt_to_activation_authority"));
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["prompt_payload_materialized"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_readback_audit_receipt_lane_endpoint_enables_redacted_readback_audit_shape_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator-approved KG prompt payload readback audit receipt lane json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-readback-audit-receipt-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_kg_prompt_payload_readback_audit_receipt_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT
        );
    assert_eq!(
            value["kg_prompt_payload_acceptance_receipt_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_acceptance_receipt_lane_ready"], true);
    assert_eq!(
        value["operator_authorization_scope"],
        "kg_prompt_payload_readback_audit_receipt_lane_no_report_receipt_render_record_persist_accept_no_kg_live_write_provider_model_channel_or_public_release"
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_lane_enabled"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_promotes_activation_authority"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_lane_enabled"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_requires_acceptance_receipt"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_redaction_required"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_redaction_proof_required"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_hash_binding_required"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_raw_payload_allowed"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_rendered_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_filesystem_written_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_ledger_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_promotes_activation_authority"],
        false
    );
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 6);
    assert_eq!(value["enablement_lane_count"], 9);
    assert_eq!(value["ready_enablement_lane_count"], 9);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked KG prompt payload readback audit receipt lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"render_prompt_payload_readback_audit_receipt_from_report_route"));
    assert!(blocked.contains(&"record_prompt_payload_readback_audit_receipt_from_report_route"));
    assert!(blocked.contains(&"persist_prompt_payload_readback_audit_receipt_from_report_route"));
    assert!(blocked.contains(&"accept_prompt_payload_readback_audit_receipt_from_report_route"));
    assert!(blocked.contains(&"promote_readback_audit_receipt_to_activation_authority"));
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(value["side_effects"]["prompt_payload_materialized"], false);
    assert_eq!(
        value["side_effects"]["prompt_payload_readback_audit_receipt_rendered"],
        false
    );
    assert_eq!(
        value["side_effects"]["prompt_payload_readback_audit_receipt_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["prompt_payload_readback_audit_receipt_persisted"],
        false
    );
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_acceptance_lane_endpoint_enables_redacted_context_handoff_acceptance_shape_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator-approved context handoff acceptance lane json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_context_handoff_acceptance_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_ENDPOINT
        );
    assert_eq!(
            value["kg_prompt_payload_readback_audit_receipt_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_readback_audit_receipt_lane_ready"], true);
    assert_eq!(
        value["operator_authorization_scope"],
        "context_handoff_acceptance_lane_no_report_context_attach_inject_record_persist_accept_no_kg_live_write_provider_model_channel_or_public_release"
    );
    assert_eq!(value["memory_durable_mutation_lane_enabled"], true);
    assert_eq!(
        value["hepta_intelligence_context_attachment_lane_enabled"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_context_attached_by_report_route"],
        false
    );
    assert_eq!(value["bounded_prompt_preview_lane_enabled"], true);
    assert_eq!(value["prompt_preview_rendered_by_report_route"], false);
    assert_eq!(value["kg_prompt_preview_lane_enabled"], true);
    assert_eq!(value["kg_external_adapter_read_lane_enabled"], true);
    assert_eq!(
        value["kg_external_adapter_read_performed_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_materialization_lane_enabled"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_materialized_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_lane_enabled"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_acceptance_receipt_promotes_activation_authority"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_lane_enabled"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_requires_acceptance_receipt"],
        true
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_rendered_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["kg_prompt_payload_readback_audit_receipt_promotes_activation_authority"],
        false
    );
    assert_eq!(value["context_handoff_acceptance_required"], true);
    assert_eq!(value["context_handoff_acceptance_lane_enabled"], true);
    assert_eq!(value["context_handoff_acceptance_allowed_by_lane"], true);
    assert_eq!(
        value["context_handoff_acceptance_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["context_handoff_acceptance_requires_readback_audit_receipt"],
        true
    );
    assert_eq!(value["context_handoff_acceptance_redaction_required"], true);
    assert_eq!(
        value["context_handoff_acceptance_scope_binding_required"],
        true
    );
    assert_eq!(
        value["context_handoff_acceptance_operator_identity_binding_required"],
        true
    );
    assert_eq!(
        value["context_handoff_acceptance_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_acceptance_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_acceptance_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_acceptance_filesystem_written_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_acceptance_ledger_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_acceptance_promotes_activation_authority"],
        false
    );
    assert_eq!(value["context_attachment_requires_explicit_command"], true);
    assert_eq!(value["context_attachment_performed_by_report_route"], false);
    assert_eq!(value["context_injection_allowed_by_lane"], false);
    assert_eq!(value["context_injection_performed_by_report_route"], false);
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 7);
    assert_eq!(value["enablement_lane_count"], 10);
    assert_eq!(value["ready_enablement_lane_count"], 10);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked context handoff acceptance lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"attach_context_from_report_route"));
    assert!(blocked.contains(&"inject_context_into_provider_prompt"));
    assert!(blocked.contains(&"record_context_handoff_acceptance_from_report_route"));
    assert!(blocked.contains(&"persist_context_handoff_acceptance_from_report_route"));
    assert!(blocked.contains(&"accept_context_handoff_from_report_route"));
    assert!(blocked.contains(&"promote_context_handoff_acceptance_to_activation_authority"));
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["hepta_intelligence_context_attached"],
        false
    );
    assert_eq!(value["side_effects"]["context_attached"], false);
    assert_eq!(value["side_effects"]["prompt_preview_rendered"], false);
    assert_eq!(value["side_effects"]["prompt_payload_materialized"], false);
    assert_eq!(
        value["side_effects"]["context_handoff_acceptance_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["context_handoff_acceptance_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["context_handoff_acceptance_accepted"],
        false
    );
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["context_injected"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_receipt_audit_lane_endpoint_enables_receipt_audit_shape_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator-approved context handoff receipt audit lane json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_context_handoff_receipt_audit_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_ENDPOINT
        );
    assert_eq!(
            value["context_handoff_acceptance_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_context_handoff_acceptance_lane_ready"], true);
    assert_eq!(
        value["operator_authorization_source"],
        "telegram_direct_operator_highest_authorization_2026_06_13_08_01_56_asia_shanghai"
    );
    assert_eq!(
        value["operator_authorization_scope"],
        "context_handoff_receipt_audit_lane_no_report_context_attach_inject_render_record_persist_accept_no_kg_live_write_provider_model_channel_or_public_release"
    );
    assert_eq!(value["context_handoff_acceptance_lane_enabled"], true);
    assert_eq!(value["context_handoff_acceptance_allowed_by_lane"], true);
    assert_eq!(
        value["context_handoff_acceptance_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_acceptance_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_acceptance_accepted_by_report_route"],
        false
    );
    assert_eq!(value["context_handoff_receipt_audit_lane_enabled"], true);
    assert_eq!(value["context_handoff_receipt_audit_allowed_by_lane"], true);
    assert_eq!(
        value["context_handoff_receipt_audit_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["context_handoff_receipt_audit_requires_context_handoff_acceptance"],
        true
    );
    assert_eq!(
        value["context_handoff_receipt_audit_redaction_required"],
        true
    );
    assert_eq!(
        value["context_handoff_receipt_audit_redaction_proof_required"],
        true
    );
    assert_eq!(
        value["context_handoff_receipt_audit_scope_binding_required"],
        true
    );
    assert_eq!(
        value["context_handoff_receipt_audit_operator_identity_binding_required"],
        true
    );
    assert_eq!(
        value["context_handoff_receipt_audit_hash_binding_required"],
        true
    );
    assert_eq!(
        value["context_handoff_receipt_audit_raw_context_allowed"],
        false
    );
    assert_eq!(
        value["context_handoff_receipt_audit_rendered_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_receipt_audit_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_receipt_audit_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_receipt_audit_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_receipt_audit_filesystem_written_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_receipt_audit_ledger_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_receipt_audit_promotes_activation_authority"],
        false
    );
    assert_eq!(value["context_attachment_performed_by_report_route"], false);
    assert_eq!(value["context_injection_allowed_by_lane"], false);
    assert_eq!(value["context_injection_performed_by_report_route"], false);
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 8);
    assert_eq!(value["enablement_lane_count"], 11);
    assert_eq!(value["ready_enablement_lane_count"], 11);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked context handoff receipt audit lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"render_context_handoff_receipt_audit_from_report_route"));
    assert!(blocked.contains(&"record_context_handoff_receipt_audit_from_report_route"));
    assert!(blocked.contains(&"persist_context_handoff_receipt_audit_from_report_route"));
    assert!(blocked.contains(&"accept_context_handoff_receipt_audit_from_report_route"));
    assert!(blocked.contains(&"promote_context_handoff_receipt_audit_to_activation_authority"));
    assert!(blocked.contains(&"inject_context_into_provider_prompt"));
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(value["side_effects"]["context_attached"], false);
    assert_eq!(
        value["side_effects"]["context_handoff_receipt_audit_rendered"],
        false
    );
    assert_eq!(
        value["side_effects"]["context_handoff_receipt_audit_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["context_handoff_receipt_audit_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["context_handoff_receipt_audit_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["context_handoff_receipt_audit_ledger_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["context_injected"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_precondition_lane_endpoint_enables_precondition_shape_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator-approved bounded provider-router injection precondition lane json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_bounded_provider_router_injection_precondition_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_ENDPOINT
        );
    assert_eq!(
            value["context_handoff_receipt_audit_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_context_handoff_receipt_audit_lane_ready"],
        true
    );
    assert_eq!(
        value["operator_authorization_source"],
        "telegram_direct_operator_highest_authorization_2026_06_13_08_01_56_asia_shanghai"
    );
    assert_eq!(
        value["operator_authorization_scope"],
        "bounded_provider_router_injection_precondition_lane_no_report_context_inject_prompt_mutation_record_persist_accept_no_kg_live_write_provider_model_channel_or_public_release"
    );
    assert_eq!(value["context_handoff_receipt_audit_lane_enabled"], true);
    assert_eq!(value["context_handoff_receipt_audit_allowed_by_lane"], true);
    assert_eq!(
        value["context_handoff_receipt_audit_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_receipt_audit_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["context_handoff_receipt_audit_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_lane_enabled"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_requires_context_handoff_receipt_audit"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_redaction_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_redaction_proof_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_scope_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_operator_identity_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_hash_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_provider_router_target_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_budget_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_dry_run_only"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_raw_context_allowed"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_rendered_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_filesystem_written_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_ledger_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_promotes_activation_authority"],
        false
    );
    assert_eq!(
        value["provider_router_prompt_mutated_by_report_route"],
        false
    );
    assert_eq!(
        value["provider_router_context_packet_materialized_by_report_route"],
        false
    );
    assert_eq!(value["context_attachment_performed_by_report_route"], false);
    assert_eq!(value["context_injection_allowed_by_lane"], false);
    assert_eq!(value["context_injection_performed_by_report_route"], false);
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 9);
    assert_eq!(value["enablement_lane_count"], 12);
    assert_eq!(value["ready_enablement_lane_count"], 12);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked bounded provider-router injection precondition lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"inject_context_into_provider_prompt"));
    assert!(blocked.contains(&"mutate_provider_router_prompt_from_report_route"));
    assert!(blocked.contains(&"record_provider_router_injection_precondition_from_report_route"));
    assert!(blocked.contains(&"persist_provider_router_injection_precondition_from_report_route"));
    assert!(blocked.contains(&"accept_provider_router_injection_precondition_from_report_route"));
    assert!(
        blocked.contains(&"promote_provider_router_injection_precondition_to_activation_authority")
    );
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(value["side_effects"]["context_attached"], false);
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_precondition_rendered"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_precondition_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_precondition_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_precondition_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_precondition_ledger_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["provider_router_prompt_mutated"],
        false
    );
    assert_eq!(
        value["side_effects"]["provider_router_context_packet_materialized"],
        false
    );
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["context_injected"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_lane_endpoint_enables_envelope_shape_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator-approved bounded provider-router injection dry-run envelope lane json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_ENDPOINT
        );
    assert_eq!(
            value["bounded_provider_router_injection_precondition_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_bounded_provider_router_injection_precondition_lane_ready"],
        true
    );
    assert_eq!(
        value["operator_authorization_source"],
        "telegram_direct_operator_highest_authorization_2026_06_13_11_12_08_asia_shanghai"
    );
    assert_eq!(
        value["operator_authorization_scope"],
        "bounded_provider_router_injection_dry_run_envelope_lane_no_report_envelope_construct_render_record_persist_accept_execute_no_context_inject_prompt_mutation_kg_live_write_provider_model_channel_or_public_release"
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_lane_enabled"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_precondition_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_lane_enabled"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_requires_bounded_provider_router_injection_precondition"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_redaction_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_redaction_proof_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_scope_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_operator_identity_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_hash_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_provider_router_target_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_budget_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_shape_locked"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_dry_run_only"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_raw_context_allowed"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_constructed_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_rendered_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_executed_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_filesystem_written_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_ledger_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_promotes_activation_authority"],
        false
    );
    assert_eq!(
        value["provider_router_injection_execution_allowed_by_lane"],
        false
    );
    assert_eq!(
        value["provider_router_prompt_mutated_by_report_route"],
        false
    );
    assert_eq!(
        value["provider_router_context_packet_materialized_by_report_route"],
        false
    );
    assert_eq!(value["context_attachment_performed_by_report_route"], false);
    assert_eq!(value["context_injection_allowed_by_lane"], false);
    assert_eq!(value["context_injection_performed_by_report_route"], false);
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 10);
    assert_eq!(value["enablement_lane_count"], 13);
    assert_eq!(value["ready_enablement_lane_count"], 13);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked bounded provider-router injection dry-run envelope lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(
        blocked.contains(&"construct_provider_router_injection_dry_run_envelope_from_report_route")
    );
    assert!(
        blocked.contains(&"execute_provider_router_injection_dry_run_envelope_from_report_route")
    );
    assert!(blocked.contains(&"inject_context_into_provider_prompt"));
    assert!(blocked.contains(&"mutate_provider_router_prompt_from_report_route"));
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_constructed"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_rendered"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_executed"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_filesystem_written"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_ledger_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["provider_router_prompt_mutated"],
        false
    );
    assert_eq!(
        value["side_effects"]["provider_router_context_packet_materialized"],
        false
    );
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["context_injected"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_endpoint_enables_receipt_shape_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator-approved bounded provider-router injection dry-run envelope readback audit receipt lane json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT
        );
    assert_eq!(
            value["bounded_provider_router_injection_dry_run_envelope_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_bounded_provider_router_injection_dry_run_envelope_lane_ready"],
        true
    );
    assert_eq!(
        value["operator_authorization_source"],
        "telegram_direct_operator_highest_authorization_2026_06_13_11_12_08_asia_shanghai"
    );
    assert_eq!(
        value["operator_authorization_scope"],
        "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_no_report_receipt_render_record_persist_accept_no_envelope_construct_execute_no_context_inject_prompt_mutation_kg_live_write_provider_model_channel_or_public_release"
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_lane_enabled"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_requires_bounded_provider_router_injection_precondition"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_dry_run_only"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_raw_context_allowed"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_constructed_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_executed_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_enabled"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_requires_bounded_provider_router_injection_dry_run_envelope"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_redaction_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_redaction_proof_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_scope_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_operator_identity_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_hash_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_provider_router_target_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_budget_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_envelope_shape_binding_required"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_raw_context_allowed"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_rendered_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_filesystem_written_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_ledger_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_promotes_activation_authority"],
        false
    );
    assert_eq!(
        value["provider_router_injection_execution_allowed_by_lane"],
        false
    );
    assert_eq!(
        value["provider_router_prompt_mutated_by_report_route"],
        false
    );
    assert_eq!(
        value["provider_router_context_packet_materialized_by_report_route"],
        false
    );
    assert_eq!(value["context_attachment_performed_by_report_route"], false);
    assert_eq!(value["context_injection_allowed_by_lane"], false);
    assert_eq!(value["context_injection_performed_by_report_route"], false);
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 11);
    assert_eq!(value["enablement_lane_count"], 14);
    assert_eq!(value["ready_enablement_lane_count"], 14);

    let blocked = value["blocked_actions"]
            .as_array()
            .expect("blocked bounded provider-router injection dry-run envelope readback audit receipt lane actions")
            .iter()
            .filter_map(|item| item.as_str())
            .collect::<Vec<_>>();
    assert!(
        blocked.contains(&"construct_provider_router_injection_dry_run_envelope_from_report_route")
    );
    assert!(
        blocked.contains(&"execute_provider_router_injection_dry_run_envelope_from_report_route")
    );
    assert!(
            blocked.contains(
                &"render_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route"
            )
        );
    assert!(
            blocked.contains(
                &"record_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route"
            )
        );
    assert!(
            blocked.contains(
                &"promote_provider_router_injection_dry_run_envelope_readback_audit_receipt_to_activation_authority"
            )
        );
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_constructed"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_executed"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_rendered"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_filesystem_written"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_ledger_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["provider_router_prompt_mutated"],
        false
    );
    assert_eq!(
        value["side_effects"]["provider_router_context_packet_materialized"],
        false
    );
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["context_injected"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_endpoint_enables_acknowledgement_shape_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator-approved bounded provider-router injection dry-run envelope readback audit receipt acknowledgement no-op handoff lane json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-acknowledgement-no-op-handoff-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_ENDPOINT
        );
    assert_eq!(
            value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_ready"],
        true
    );
    assert_eq!(
        value["operator_authorization_source"],
        "telegram_direct_operator_highest_authorization_2026_06_13_14_01_56_asia_shanghai"
    );
    assert_eq!(
        value["operator_authorization_scope"],
        "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_no_report_acknowledge_handoff_record_persist_accept_no_envelope_construct_execute_no_context_inject_prompt_mutation_kg_live_write_provider_model_channel_or_public_release"
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_enabled"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_rendered_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_enabled"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_readback_audit_receipt_lane"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_acknowledgement_shape_binding"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_no_op_handoff_boundary"],
        true
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_raw_context_allowed"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_acknowledged_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_handoff_performed_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_persisted_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_accepted_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_filesystem_written_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_ledger_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_promotes_activation_authority"],
        false
    );
    assert_eq!(
        value["provider_router_injection_execution_allowed_by_lane"],
        false
    );
    assert_eq!(
        value["provider_router_prompt_mutated_by_report_route"],
        false
    );
    assert_eq!(
        value["provider_router_context_packet_materialized_by_report_route"],
        false
    );
    assert_eq!(value["context_attachment_performed_by_report_route"], false);
    assert_eq!(value["context_injection_allowed_by_lane"], false);
    assert_eq!(value["context_injection_performed_by_report_route"], false);
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 12);
    assert_eq!(value["enablement_lane_count"], 15);
    assert_eq!(value["ready_enablement_lane_count"], 15);

    let blocked = value["blocked_actions"]
            .as_array()
            .expect("blocked bounded provider-router injection dry-run envelope readback audit receipt acknowledgement no-op handoff lane actions")
            .iter()
            .filter_map(|item| item.as_str())
            .collect::<Vec<_>>();
    assert!(
            blocked.contains(
                &"acknowledge_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route"
            )
        );
    assert!(
            blocked.contains(
                &"perform_provider_router_injection_dry_run_envelope_readback_audit_receipt_no_op_handoff_from_report_route"
            )
        );
    assert!(
            blocked.contains(
                &"promote_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_to_activation_authority"
            )
        );
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledged"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_no_op_handoff_performed"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_filesystem_written"],
        false
    );
    assert_eq!(
        value["side_effects"]["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_ledger_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_endpoint_reports_noop_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness single-budget dispatch dry-run no-op receipt json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_ENDPOINT
        );
    assert_eq!(
            value["bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(
        value["source_acknowledgement_no_op_handoff_lane_ready"],
        true
    );
    assert_eq!(
        value["operator_authorization_source"],
        "telegram_direct_operator_highest_authorization_2026_06_13_16_27_10_asia_shanghai"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status"],
        "blocked"
    );
    assert_eq!(value["source_receipt_hash_preview_count"], 2);
    assert_eq!(value["source_receipt_accepted_count"], 0);
    assert_eq!(value["source_acceptance_skeleton_declared_count"], 2);
    assert_eq!(
        value["source_acceptance_skeleton_operator_input_supplied_count"],
        0
    );
    assert_eq!(
        value["source_controlled_request_dispatch_budget_declared"],
        1
    );
    assert_eq!(
        value["source_controlled_request_dispatch_budget_accepted"],
        false
    );
    assert_eq!(
        value["source_controlled_request_dispatch_budget_consumed"],
        0
    );
    assert_eq!(value["dispatch_dry_run_noop_receipt_count"], 1);
    assert_eq!(value["dispatch_dry_run_shape_declared_count"], 1);
    assert_eq!(value["dispatch_intent_shape_declared_count"], 1);
    assert_eq!(value["single_budget_shape_declared_count"], 1);
    assert_eq!(value["single_budget_declared"], 1);
    assert_eq!(value["single_budget_accepted"], false);
    assert_eq!(value["single_budget_consumed"], 0);
    assert_eq!(value["single_budget_remaining"], 0);
    assert_eq!(value["dispatch_authority_accepted_count"], 0);
    assert_eq!(value["dispatch_preconditions_satisfied_count"], 0);
    assert_eq!(value["controlled_request_dispatch_ready_count"], 0);
    assert_eq!(value["controlled_request_dispatch_allowed_count"], 0);
    assert_eq!(value["controlled_request_dispatched_count"], 0);
    assert_eq!(value["controlled_request_execution_allowed_count"], 0);
    assert_eq!(value["controlled_request_executed_count"], 0);
    assert_eq!(value["noop_receipt_shape_declared_count"], 1);
    assert_eq!(value["noop_receipt_recorded_count"], 0);
    assert_eq!(value["noop_receipt_persisted_count"], 0);
    assert_eq!(value["noop_receipt_delivered_count"], 0);
    assert_eq!(value["noop_receipt_accepted_count"], 0);
    assert_eq!(value["noop_receipt_materialized_count"], 0);
    assert_eq!(value["request_payload_materialized_count"], 0);
    assert_eq!(value["request_payload_file_written_count"], 0);
    assert_eq!(value["raw_payload_inspected_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(
        value["dispatch_dry_run_noop_receipt_negative_fixture_count"],
        7
    );
    assert_eq!(
        value["dispatch_dry_run_noop_receipt_blocked_negative_fixture_count"],
        7
    );
    assert_eq!(
        value["dispatch_dry_run_noop_receipt_allowed_negative_fixture_count"],
        0
    );
    assert_eq!(value["denied_by_dispatch_dry_run_noop_receipt_count"], 16);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 13);
    assert_eq!(value["enablement_lane_count"], 16);
    assert_eq!(value["ready_enablement_lane_count"], 16);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked canary single-budget dispatch dry-run actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"dispatch_operator_canary_controlled_request_from_report_route"));
    assert!(
        blocked
            .contains(&"persist_operator_canary_controlled_request_noop_receipt_from_report_route")
    );
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(value["side_effects"]["single_budget_accepted"], false);
    assert_eq!(value["side_effects"]["single_budget_consumed"], false);
    assert_eq!(value["side_effects"]["dispatch_performed"], false);
    assert_eq!(value["side_effects"]["execution_performed"], false);
    assert_eq!(value["side_effects"]["noop_receipt_recorded"], false);
    assert_eq!(value["side_effects"]["noop_receipt_persisted"], false);
    assert_eq!(value["side_effects"]["noop_receipt_accepted"], false);
    assert_eq!(value["side_effects"]["request_payload_materialized"], false);
    assert_eq!(value["side_effects"]["raw_payload_inspected"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_endpoint_reports_noop_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_READBACK_INDEX_NO_PERSISTENCE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review/readback index no-persistence json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_READBACK_INDEX_NO_PERSISTENCE_ENDPOINT
        );
    assert_eq!(
            value["source_single_budget_dispatch_dry_run_noop_receipt_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_single_budget_dispatch_dry_run_noop_receipt_route_ready"],
        true
    );
    assert_eq!(
        value["operator_authorization_source"],
        "telegram_direct_operator_highest_authorization_2026_06_13_16_27_10_asia_shanghai"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status"],
        "blocked"
    );
    assert_eq!(value["source_single_budget_declared"], 1);
    assert_eq!(value["source_single_budget_accepted"], false);
    assert_eq!(value["source_single_budget_consumed"], 0);
    assert_eq!(value["source_controlled_request_dispatched_count"], 0);
    assert_eq!(value["source_controlled_request_executed_count"], 0);
    assert_eq!(value["source_noop_receipt_persisted_count"], 0);
    assert_eq!(value["source_noop_receipt_accepted_count"], 0);
    assert_eq!(value["operator_review_readback_index_section_count"], 8);
    assert_eq!(value["operator_review_required_count"], 8);
    assert_eq!(value["operator_review_supplied_count"], 0);
    assert_eq!(value["operator_review_recorded_count"], 0);
    assert_eq!(value["operator_review_persisted_count"], 0);
    assert_eq!(value["operator_review_delivered_count"], 0);
    assert_eq!(value["operator_review_accepted_count"], 0);
    assert_eq!(value["readback_index_declared_count"], 1);
    assert_eq!(value["readback_index_bound_to_payload_hash_count"], 1);
    assert_eq!(
        value["readback_index_bound_to_readback_receipt_hash_count"],
        1
    );
    assert_eq!(value["readback_index_bound_to_audit_receipt_hash_count"], 1);
    assert_eq!(value["readback_index_bound_to_noop_receipt_hash_count"], 1);
    assert_eq!(value["readback_index_recorded_count"], 0);
    assert_eq!(value["readback_index_persisted_count"], 0);
    assert_eq!(value["readback_index_materialized_count"], 0);
    assert_eq!(value["operator_review_index_recorded"], false);
    assert_eq!(value["operator_review_index_persisted"], false);
    assert_eq!(value["operator_review_index_channel_delivered"], false);
    assert_eq!(value["review_authorizes_dispatch_count"], 0);
    assert_eq!(value["review_authorizes_execution_count"], 0);
    assert_eq!(value["review_authorizes_live_count"], 0);
    assert_eq!(value["dispatch_allowed_count"], 0);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_allowed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(
        value["operator_review_readback_index_negative_fixture_count"],
        8
    );
    assert_eq!(
        value["operator_review_readback_index_blocked_negative_fixture_count"],
        8
    );
    assert_eq!(
        value["operator_review_readback_index_allowed_negative_fixture_count"],
        0
    );
    assert_eq!(value["denied_by_operator_review_readback_index_count"], 17);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 14);
    assert_eq!(value["enablement_lane_count"], 17);
    assert_eq!(value["ready_enablement_lane_count"], 17);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked canary operator-review/readback index actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(
        &"accept_operator_canary_controlled_request_harness_operator_review_from_report_route"
    ));
    assert!(blocked.contains(
        &"persist_operator_canary_controlled_request_harness_operator_review_readback_index"
    ));
    assert!(blocked.contains(&"dispatch_operator_canary_controlled_request_from_operator_review"));
    assert!(blocked.contains(&"invoke_provider_or_model_from_operator_review"));
    assert!(blocked.contains(&"telegram_or_channel_delivery_from_operator_review"));
    assert_eq!(value["side_effects"]["operator_review_recorded"], false);
    assert_eq!(value["side_effects"]["operator_review_persisted"], false);
    assert_eq!(value["side_effects"]["operator_review_delivered"], false);
    assert_eq!(value["side_effects"]["operator_review_accepted"], false);
    assert_eq!(
        value["side_effects"]["operator_review_index_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_review_index_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_review_index_materialized"],
        false
    );
    assert_eq!(value["side_effects"]["readback_index_persisted"], false);
    assert_eq!(value["side_effects"]["dispatch_performed"], false);
    assert_eq!(value["side_effects"]["execution_performed"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_endpoint_reports_noop_only()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement non-acceptance json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_readback_index_no_persistence_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_READBACK_INDEX_NO_PERSISTENCE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_readback_index_no_persistence_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status"],
        "blocked"
    );
    assert_eq!(value["source_operator_review_required_count"], 8);
    assert_eq!(value["source_operator_review_accepted_count"], 0);
    assert_eq!(value["source_readback_index_declared_count"], 1);
    assert_eq!(value["source_readback_index_persisted_count"], 0);
    assert_eq!(value["source_review_authorizes_dispatch_count"], 0);
    assert_eq!(value["source_review_authorizes_execution_count"], 0);
    assert_eq!(value["source_review_authorizes_live_count"], 0);
    assert_eq!(value["operator_review_acknowledgement_fixture_count"], 8);
    assert_eq!(
        value["operator_review_acknowledgement_requested_fixture_count"],
        8
    );
    assert_eq!(
        value["blocked_operator_review_acknowledgement_fixture_count"],
        8
    );
    assert_eq!(
        value["noop_operator_review_acknowledgement_fixture_count"],
        8
    );
    assert_eq!(
        value["allowed_operator_review_acknowledgement_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_operator_review_acknowledgement_fixture_count"],
        0
    );
    assert_eq!(value["operator_review_acknowledgement_performed_count"], 0);
    assert_eq!(value["operator_review_acknowledgement_allowed"], false);
    assert_eq!(value["operator_review_acknowledgement_accepted"], false);
    assert_eq!(value["operator_review_acknowledgement_recorded"], false);
    assert_eq!(value["operator_review_acknowledgement_persisted"], false);
    assert_eq!(value["operator_review_acknowledgement_materialized"], false);
    assert_eq!(
        value["operator_review_acknowledgement_filesystem_written"],
        false
    );
    assert_eq!(value["operator_review_acknowledgement_delivered"], false);
    assert_eq!(
        value["operator_review_acknowledgement_identity_accepted"],
        false
    );
    assert_eq!(
        value["operator_review_acknowledgement_signature_accepted"],
        false
    );
    assert_eq!(
        value["operator_review_acknowledgement_final_state_promoted"],
        false
    );
    assert_eq!(
        value["operator_review_acknowledgement_completion_promoted"],
        false
    );
    assert_eq!(
        value["operator_review_acknowledgement_authorizes_dispatch_count"],
        0
    );
    assert_eq!(
        value["operator_review_acknowledgement_authorizes_execution_count"],
        0
    );
    assert_eq!(
        value["operator_review_acknowledgement_authorizes_live_count"],
        0
    );
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["operator_identity_accepted"], false);
    assert_eq!(value["readback_index_recorded_count"], 0);
    assert_eq!(value["readback_index_persisted_count"], 0);
    assert_eq!(value["readback_index_materialized_count"], 0);
    assert_eq!(value["readback_index_filesystem_written_count"], 0);
    assert_eq!(value["dispatch_allowed_count"], 0);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_allowed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(
        value["denied_by_operator_review_acknowledgement_non_acceptance_count"],
        19
    );
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 15);
    assert_eq!(value["enablement_lane_count"], 18);
    assert_eq!(value["ready_enablement_lane_count"], 18);

    let fixtures = value["operator_review_acknowledgement_fixtures"]
        .as_array()
        .expect("operator review acknowledgement fixtures");
    assert_eq!(fixtures.len(), 8);
    for fixture in fixtures {
        assert_eq!(fixture["acknowledgement_requested"], true);
        assert_eq!(fixture["acknowledgement_status"], "blocked_noop");
        assert_eq!(fixture["acknowledgement_performed"], false);
        assert_eq!(fixture["acknowledgement_accepted"], false);
        assert_eq!(fixture["acknowledgement_recorded"], false);
        assert_eq!(fixture["acknowledgement_persisted"], false);
        assert_eq!(fixture["acknowledgement_materialized"], false);
        assert_eq!(fixture["acknowledgement_filesystem_written"], false);
        assert_eq!(fixture["acknowledgement_delivered"], false);
        assert_eq!(fixture["dispatch_performed"], false);
        assert_eq!(fixture["execution_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["channel_send_performed"], false);
    }

    let denied = value["denied_by_operator_review_acknowledgement_non_acceptance"]
        .as_array()
        .expect("denied operator review acknowledgement actions");
    assert_eq!(denied.len(), 19);
    assert_eq!(
        value["side_effects"]["operator_review_acknowledgement_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_review_acknowledgement_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_review_acknowledgement_delivered"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_review_acknowledgement_accepted"],
        false
    );
    assert_eq!(value["side_effects"]["operator_approval_recorded"], false);
    assert_eq!(value["side_effects"]["operator_identity_accepted"], false);
    assert_eq!(value["side_effects"]["readback_index_persisted"], false);
    assert_eq!(value["side_effects"]["dispatch_performed"], false);
    assert_eq!(value["side_effects"]["execution_performed"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_endpoint_blocks_activation_requests()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_REQUEST_DENIAL_MATRIX_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation request denial matrix json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_REQUEST_DENIAL_MATRIX_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_non_acceptance_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_non_acceptance_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_status"],
        "blocked"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_request_denial_matrix_v1"
    );
    assert_eq!(
        value["source_operator_review_acknowledgement_fixture_count"],
        8
    );
    assert_eq!(
        value["source_operator_review_acknowledgement_accepted_count"],
        0
    );
    assert_eq!(
        value["source_operator_review_acknowledgement_performed_count"],
        0
    );
    assert_eq!(
        value["source_operator_review_acknowledgement_authorizes_dispatch_count"],
        0
    );
    assert_eq!(
        value["source_operator_review_acknowledgement_authorizes_execution_count"],
        0
    );
    assert_eq!(
        value["source_operator_review_acknowledgement_authorizes_live_count"],
        0
    );
    assert_eq!(value["activation_request_denial_fixture_count"], 9);
    assert_eq!(value["activation_request_requested_fixture_count"], 9);
    assert_eq!(value["blocked_activation_request_fixture_count"], 9);
    assert_eq!(value["noop_activation_request_fixture_count"], 9);
    assert_eq!(value["allowed_activation_request_fixture_count"], 0);
    assert_eq!(value["accepted_activation_request_fixture_count"], 0);
    assert_eq!(value["activation_request_performed_count"], 0);
    assert_eq!(value["activation_request_allowed"], false);
    assert_eq!(value["activation_request_accepted"], false);
    assert_eq!(value["activation_request_recorded"], false);
    assert_eq!(value["activation_request_persisted"], false);
    assert_eq!(value["activation_request_materialized"], false);
    assert_eq!(value["activation_request_filesystem_written"], false);
    assert_eq!(value["activation_request_delivered"], false);
    assert_eq!(value["activation_request_executed"], false);
    assert_eq!(value["activation_nonce_generated"], false);
    assert_eq!(value["activation_identity_accepted"], false);
    assert_eq!(value["activation_scope_accepted"], false);
    assert_eq!(value["activation_final_state_promoted"], false);
    assert_eq!(value["operator_review_acknowledgement_accepted"], false);
    assert_eq!(value["operator_review_acknowledgement_recorded"], false);
    assert_eq!(value["operator_review_acknowledgement_persisted"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["operator_identity_accepted"], false);
    assert_eq!(value["dispatch_allowed_count"], 0);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_allowed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["install_performed_count"], 0);
    assert_eq!(value["service_restarted_count"], 0);
    assert_eq!(value["active_binary_mutated_count"], 0);
    assert_eq!(value["upstream_fetch_performed_count"], 0);
    assert_eq!(value["upstream_merge_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(
        value["denied_by_operator_review_acknowledgement_activation_request_denial_matrix_count"],
        26
    );
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 16);
    assert_eq!(value["enablement_lane_count"], 19);
    assert_eq!(value["ready_enablement_lane_count"], 19);

    let fixtures = value["activation_request_denial_fixtures"]
        .as_array()
        .expect("activation request denial fixtures");
    assert_eq!(fixtures.len(), 9);
    for fixture in fixtures {
        assert_eq!(fixture["activation_request_requested"], true);
        assert_eq!(fixture["activation_request_status"], "blocked_noop");
        assert_eq!(fixture["activation_request_allowed"], false);
        assert_eq!(fixture["activation_request_accepted"], false);
        assert_eq!(fixture["activation_request_recorded"], false);
        assert_eq!(fixture["activation_request_persisted"], false);
        assert_eq!(fixture["activation_request_materialized"], false);
        assert_eq!(fixture["activation_request_filesystem_written"], false);
        assert_eq!(fixture["activation_request_delivered"], false);
        assert_eq!(fixture["activation_request_executed"], false);
        assert_eq!(fixture["activation_nonce_generated"], false);
        assert_eq!(fixture["activation_identity_accepted"], false);
        assert_eq!(fixture["activation_scope_accepted"], false);
        assert_eq!(fixture["activation_final_state_promoted"], false);
        assert_eq!(fixture["dispatch_allowed"], false);
        assert_eq!(fixture["dispatch_performed"], false);
        assert_eq!(fixture["execution_allowed"], false);
        assert_eq!(fixture["execution_performed"], false);
        assert_eq!(fixture["context_injection_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["install_performed"], false);
        assert_eq!(fixture["service_restarted"], false);
        assert_eq!(fixture["active_binary_mutated"], false);
        assert_eq!(fixture["upstream_fetch_performed"], false);
        assert_eq!(fixture["upstream_merge_performed"], false);
    }

    let denied =
        value["denied_by_operator_review_acknowledgement_activation_request_denial_matrix"]
            .as_array()
            .expect("denied activation request actions");
    assert_eq!(denied.len(), 26);
    assert_eq!(value["side_effects"]["activation_request_recorded"], false);
    assert_eq!(value["side_effects"]["activation_request_persisted"], false);
    assert_eq!(value["side_effects"]["activation_request_executed"], false);
    assert_eq!(
        value["side_effects"]["activation_final_state_promoted"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_review_acknowledgement_accepted"],
        false
    );
    assert_eq!(value["side_effects"]["operator_approval_recorded"], false);
    assert_eq!(value["side_effects"]["dispatch_performed"], false);
    assert_eq!(value["side_effects"]["execution_performed"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["install_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["upstream_fetch_performed"], false);
    assert_eq!(value["side_effects"]["upstream_merge_performed"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_endpoint_blocks_activation_commands()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_NOOP_HANDOFF_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command no-op handoff json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-noop-handoff --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_NOOP_HANDOFF_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_request_denial_matrix_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_REQUEST_DENIAL_MATRIX_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_request_denial_matrix_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_status"],
        "blocked"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_noop_handoff_v1"
    );
    assert_eq!(value["source_activation_request_denial_fixture_count"], 9);
    assert_eq!(value["source_blocked_activation_request_fixture_count"], 9);
    assert_eq!(value["source_noop_activation_request_fixture_count"], 9);
    assert_eq!(value["source_accepted_activation_request_fixture_count"], 0);
    assert_eq!(value["source_activation_request_performed_count"], 0);
    assert_eq!(value["activation_command_surface_count"], 13);
    assert_eq!(value["activation_command_surface_ready_count"], 13);
    assert_eq!(
        value["activation_command_side_effect_free_surface_count"],
        13
    );
    assert_eq!(value["activation_command_fixture_count"], 10);
    assert_eq!(value["activation_command_requested_fixture_count"], 10);
    assert_eq!(value["blocked_activation_command_fixture_count"], 10);
    assert_eq!(value["noop_activation_command_fixture_count"], 10);
    assert_eq!(value["allowed_activation_command_fixture_count"], 0);
    assert_eq!(value["accepted_activation_command_fixture_count"], 0);
    assert_eq!(value["activation_command_performed_count"], 0);
    assert_eq!(value["activation_command_dispatch_performed_count"], 0);
    assert_eq!(value["activation_command_shape_registered"], false);
    assert_eq!(value["activation_command_allowed"], false);
    assert_eq!(value["activation_command_accepted"], false);
    assert_eq!(value["activation_command_enabled"], false);
    assert_eq!(value["activation_command_invoked"], false);
    assert_eq!(value["activation_command_dispatched"], false);
    assert_eq!(value["activation_command_noop_decision_recorded"], false);
    assert_eq!(value["activation_command_noop_decision_persisted"], false);
    assert_eq!(value["activation_command_handoff_recorded"], false);
    assert_eq!(value["activation_command_handoff_persisted"], false);
    assert_eq!(value["activation_command_result_receipt_recorded"], false);
    assert_eq!(value["activation_command_result_receipt_persisted"], false);
    assert_eq!(value["activation_command_result_receipt_accepted"], false);
    assert_eq!(value["activation_request_accepted"], false);
    assert_eq!(value["activation_request_recorded"], false);
    assert_eq!(value["activation_request_persisted"], false);
    assert_eq!(value["activation_request_executed"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["install_performed_count"], 0);
    assert_eq!(value["service_restarted_count"], 0);
    assert_eq!(value["active_binary_mutated_count"], 0);
    assert_eq!(value["upstream_fetch_performed_count"], 0);
    assert_eq!(value["upstream_merge_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(
        value["denied_by_operator_review_acknowledgement_activation_command_noop_handoff_count"],
        57
    );
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 17);
    assert_eq!(value["enablement_lane_count"], 20);
    assert_eq!(value["ready_enablement_lane_count"], 20);

    let fixtures = value["activation_command_fixtures"]
        .as_array()
        .expect("activation command no-op handoff fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert_eq!(fixture["activation_command_requested"], true);
        assert_eq!(fixture["activation_command_allowed"], false);
        assert_eq!(fixture["activation_command_accepted"], false);
        assert_eq!(fixture["activation_command_enabled"], false);
        assert_eq!(fixture["activation_command_invoked"], false);
        assert_eq!(fixture["activation_command_dispatched"], false);
        assert_eq!(fixture["activation_command_dispatch_performed"], false);
        assert_eq!(fixture["activation_command_handoff_recorded"], false);
        assert_eq!(fixture["activation_command_result_receipt_recorded"], false);
        assert_eq!(fixture["activation_request_accepted"], false);
        assert_eq!(fixture["activation_request_executed"], false);
        assert_eq!(fixture["dispatch_performed"], false);
        assert_eq!(fixture["execution_performed"], false);
        assert_eq!(fixture["context_injection_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["external_kg_adapter_read_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["install_performed"], false);
        assert_eq!(fixture["service_restarted"], false);
        assert_eq!(fixture["active_binary_mutated"], false);
        assert_eq!(fixture["upstream_fetch_performed"], false);
        assert_eq!(fixture["upstream_merge_performed"], false);
        assert_eq!(fixture["activation_command_noop_confirmed"], true);
    }

    let denied = value["denied_by_operator_review_acknowledgement_activation_command_noop_handoff"]
        .as_array()
        .expect("denied activation command no-op handoff actions");
    assert_eq!(denied.len(), 57);
    assert_eq!(
        value["side_effects"]["activation_command_registered"],
        false
    );
    assert_eq!(value["side_effects"]["activation_command_enabled"], false);
    assert_eq!(value["side_effects"]["activation_command_invoked"], false);
    assert_eq!(
        value["side_effects"]["activation_command_dispatched"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_handoff_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["activation_request_recorded"], false);
    assert_eq!(value["side_effects"]["activation_request_executed"], false);
    assert_eq!(value["side_effects"]["dispatch_performed"], false);
    assert_eq!(value["side_effects"]["execution_performed"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["install_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["upstream_fetch_performed"], false);
    assert_eq!(value["side_effects"]["upstream_merge_performed"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_endpoint_blocks_receipts()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt no-persistence json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_noop_handoff_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_NOOP_HANDOFF_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_noop_handoff_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status"],
        "blocked"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_v1"
    );
    assert_eq!(value["source_activation_command_fixture_count"], 10);
    assert_eq!(value["source_accepted_activation_command_fixture_count"], 0);
    assert_eq!(value["source_activation_command_performed_count"], 0);
    assert_eq!(value["activation_command_result_receipt_surface_count"], 14);
    assert_eq!(
        value["activation_command_result_receipt_surface_ready_count"],
        14
    );
    assert_eq!(
        value["activation_command_result_receipt_side_effect_free_surface_count"],
        14
    );
    assert_eq!(value["activation_command_result_receipt_fixture_count"], 10);
    assert_eq!(
        value["activation_command_result_receipt_requested_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_fixture_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_shape_registered"],
        false
    );
    assert_eq!(value["activation_command_result_receipt_allowed"], false);
    assert_eq!(
        value["activation_command_result_receipt_schema_accepted"],
        false
    );
    assert_eq!(value["activation_command_result_receipt_recorded"], false);
    assert_eq!(value["activation_command_result_receipt_persisted"], false);
    assert_eq!(value["activation_command_result_receipt_accepted"], false);
    assert_eq!(
        value["activation_command_result_receipt_materialized"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_filesystem_written"],
        false
    );
    assert_eq!(value["activation_command_result_receipt_exported"], false);
    assert_eq!(
        value["activation_command_result_receipt_query_registered"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_recorded"],
        false
    );
    assert_eq!(value["activation_command_completion_ack_recorded"], false);
    assert_eq!(value["activation_command_completion_ack_accepted"], false);
    assert_eq!(value["operator_approval_from_receipt_accepted"], false);
    assert_eq!(value["activation_from_receipt_allowed"], false);
    assert_eq!(value["activation_command_enabled"], false);
    assert_eq!(value["activation_command_invoked"], false);
    assert_eq!(value["activation_command_dispatched"], false);
    assert_eq!(value["activation_command_handoff_recorded"], false);
    assert_eq!(value["activation_request_accepted"], false);
    assert_eq!(value["activation_request_recorded"], false);
    assert_eq!(value["activation_request_executed"], false);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["install_performed_count"], 0);
    assert_eq!(value["service_restarted_count"], 0);
    assert_eq!(value["active_binary_mutated_count"], 0);
    assert_eq!(value["upstream_fetch_performed_count"], 0);
    assert_eq!(value["upstream_merge_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 18);
    assert_eq!(value["enablement_lane_count"], 21);
    assert_eq!(value["ready_enablement_lane_count"], 21);

    let fixtures = value["activation_command_result_receipt_fixtures"]
        .as_array()
        .expect("activation command result receipt no-persistence fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert_eq!(fixture["activation_command_result_receipt_requested"], true);
        assert_eq!(fixture["activation_command_result_receipt_allowed"], false);
        assert_eq!(fixture["activation_command_result_receipt_recorded"], false);
        assert_eq!(
            fixture["activation_command_result_receipt_persisted"],
            false
        );
        assert_eq!(fixture["activation_command_result_receipt_accepted"], false);
        assert_eq!(
            fixture["activation_command_result_receipt_materialized"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_filesystem_written"],
            false
        );
        assert_eq!(fixture["activation_command_result_receipt_exported"], false);
        assert_eq!(
            fixture["activation_command_result_receipt_query_registered"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_observability_recorded"],
            false
        );
        assert_eq!(fixture["activation_command_completion_ack_recorded"], false);
        assert_eq!(fixture["operator_approval_from_receipt_accepted"], false);
        assert_eq!(fixture["activation_from_receipt_allowed"], false);
        assert_eq!(fixture["activation_command_enabled"], false);
        assert_eq!(fixture["activation_command_invoked"], false);
        assert_eq!(fixture["activation_command_dispatched"], false);
        assert_eq!(fixture["activation_request_accepted"], false);
        assert_eq!(fixture["activation_request_executed"], false);
        assert_eq!(fixture["dispatch_performed"], false);
        assert_eq!(fixture["execution_performed"], false);
        assert_eq!(fixture["context_injection_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["external_kg_adapter_read_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["install_performed"], false);
        assert_eq!(fixture["service_restarted"], false);
        assert_eq!(fixture["active_binary_mutated"], false);
        assert_eq!(fixture["upstream_fetch_performed"], false);
        assert_eq!(fixture["upstream_merge_performed"], false);
        assert_eq!(
            fixture["activation_command_result_receipt_non_authority_confirmed"],
            true
        );
    }

    let denied = value
            ["denied_by_operator_review_acknowledgement_activation_command_result_receipt_no_persistence"]
            .as_array()
            .expect("denied activation command result receipt no-persistence actions");
    assert!(denied.len() >= 90);
    assert_eq!(
        value["denied_by_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_completion_ack_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["activation_command_enabled"], false);
    assert_eq!(value["side_effects"]["activation_command_invoked"], false);
    assert_eq!(
        value["side_effects"]["activation_command_dispatched"],
        false
    );
    assert_eq!(value["side_effects"]["activation_request_recorded"], false);
    assert_eq!(value["side_effects"]["activation_request_executed"], false);
    assert_eq!(value["side_effects"]["dispatch_performed"], false);
    assert_eq!(value["side_effects"]["execution_performed"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["install_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["upstream_fetch_performed"], false);
    assert_eq!(value["side_effects"]["upstream_merge_performed"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_endpoint_blocks_replay()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt replay idempotency denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status"],
        "blocked"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_v1"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_activation_command_result_receipt_fixture_count"],
        0
    );
    assert_eq!(
        value["source_activation_command_result_receipt_performed_count"],
        0
    );
    assert_eq!(value["replay_idempotency_surface_count"], 14);
    assert_eq!(value["replay_idempotency_surface_ready_count"], 14);
    assert_eq!(
        value["replay_idempotency_side_effect_free_surface_count"],
        14
    );
    assert_eq!(value["replay_idempotency_fixture_count"], 10);
    assert_eq!(value["blocked_replay_idempotency_fixture_count"], 10);
    assert_eq!(value["noop_replay_idempotency_fixture_count"], 10);
    assert_eq!(value["allowed_replay_idempotency_fixture_count"], 0);
    assert_eq!(value["accepted_replay_idempotency_fixture_count"], 0);
    assert_eq!(value["replay_idempotency_denied_count"], 10);
    assert_eq!(value["replay_idempotency_performed_count"], 0);
    assert_eq!(value["duplicate_result_receipt_accepted_count"], 0);
    assert_eq!(value["idempotency_state_recorded_count"], 0);
    assert_eq!(
        value["activation_command_result_receipt_replay_allowed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_replay_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_replay_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_replay_performed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_duplicate_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_idempotency_key_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_idempotency_state_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_idempotency_state_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_replay_nonce_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_cross_scope_reuse_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_status_upgrade_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_completed_status_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_ack_replay_accepted"],
        false
    );
    assert_eq!(value["operator_approval_from_replay_accepted"], false);
    assert_eq!(value["activation_from_replay_allowed"], false);
    assert_eq!(value["activation_from_receipt_allowed"], false);
    assert_eq!(value["activation_command_enabled"], false);
    assert_eq!(value["activation_command_invoked"], false);
    assert_eq!(value["activation_command_dispatched"], false);
    assert_eq!(value["activation_request_accepted"], false);
    assert_eq!(value["activation_request_recorded"], false);
    assert_eq!(value["activation_request_executed"], false);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["install_performed_count"], 0);
    assert_eq!(value["service_restarted_count"], 0);
    assert_eq!(value["active_binary_mutated_count"], 0);
    assert_eq!(value["upstream_fetch_performed_count"], 0);
    assert_eq!(value["upstream_merge_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 19);
    assert_eq!(value["enablement_lane_count"], 22);
    assert_eq!(value["ready_enablement_lane_count"], 22);

    let fixtures = value["replay_idempotency_fixtures"]
        .as_array()
        .expect("activation command result receipt replay idempotency denial fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert_eq!(
            fixture["activation_command_result_receipt_replay_allowed"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_replay_recorded"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_replay_persisted"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_duplicate_accepted"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_idempotency_state_recorded"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_cross_scope_reuse_accepted"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_status_upgrade_accepted"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_ack_replay_accepted"],
            false
        );
        assert_eq!(fixture["operator_approval_from_replay_accepted"], false);
        assert_eq!(fixture["activation_from_replay_allowed"], false);
        assert_eq!(fixture["activation_command_enabled"], false);
        assert_eq!(fixture["activation_command_invoked"], false);
        assert_eq!(fixture["activation_command_dispatched"], false);
        assert_eq!(fixture["activation_request_accepted"], false);
        assert_eq!(fixture["activation_request_executed"], false);
        assert_eq!(fixture["dispatch_performed"], false);
        assert_eq!(fixture["execution_performed"], false);
        assert_eq!(fixture["context_injection_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["external_kg_adapter_read_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["install_performed"], false);
        assert_eq!(fixture["service_restarted"], false);
        assert_eq!(fixture["active_binary_mutated"], false);
        assert_eq!(fixture["upstream_fetch_performed"], false);
        assert_eq!(fixture["upstream_merge_performed"], false);
        assert_eq!(fixture["receipt_noop_confirmed"], true);
    }

    let denied = value
            ["denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency"]
            .as_array()
            .expect("denied activation command result receipt replay idempotency actions");
    assert!(denied.len() >= 120);
    assert_eq!(
        value["denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_replay_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_replay_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_duplicate_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_idempotency_state_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_ack_replay_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_from_replay_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_command_enabled"], false);
    assert_eq!(value["side_effects"]["activation_command_invoked"], false);
    assert_eq!(
        value["side_effects"]["activation_command_dispatched"],
        false
    );
    assert_eq!(value["side_effects"]["activation_request_recorded"], false);
    assert_eq!(value["side_effects"]["activation_request_executed"], false);
    assert_eq!(value["side_effects"]["dispatch_performed"], false);
    assert_eq!(value["side_effects"]["execution_performed"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["install_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["upstream_fetch_performed"], false);
    assert_eq!(value["side_effects"]["upstream_merge_performed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_endpoint_blocks_ordering()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt ordering monotonicity denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status"],
        "blocked"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_v1"
    );
    assert_eq!(value["source_replay_idempotency_fixture_count"], 10);
    assert_eq!(value["source_accepted_replay_idempotency_fixture_count"], 0);
    assert_eq!(value["ordering_monotonicity_surface_count"], 14);
    assert_eq!(value["ordering_monotonicity_surface_ready_count"], 14);
    assert_eq!(
        value["ordering_monotonicity_side_effect_free_surface_count"],
        14
    );
    assert_eq!(value["ordering_monotonicity_fixture_count"], 10);
    assert_eq!(value["blocked_ordering_monotonicity_fixture_count"], 10);
    assert_eq!(value["noop_ordering_monotonicity_fixture_count"], 10);
    assert_eq!(value["allowed_ordering_monotonicity_fixture_count"], 0);
    assert_eq!(value["accepted_ordering_monotonicity_fixture_count"], 0);
    assert_eq!(value["ordering_monotonicity_denied_count"], 10);
    assert_eq!(value["ordering_monotonicity_performed_count"], 0);
    assert_eq!(value["sequence_cursor_accepted_count"], 0);
    assert_eq!(value["sequence_cursor_recorded_count"], 0);
    assert_eq!(value["monotonicity_state_recorded_count"], 0);
    assert_eq!(value["monotonicity_state_persisted_count"], 0);
    assert_eq!(
        value["activation_command_result_receipt_ordering_allowed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_ordering_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_ordering_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_sequence_cursor_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_monotonicity_state_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_monotonicity_state_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_out_of_order_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_stale_sequence_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_future_sequence_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_latest_wins_overwrite_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_ledger_ordering_bypass_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_provider_ordering_bypass_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_memory_kg_ordering_bypass_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_external_public_install_ordering_bypass_accepted"],
        false
    );
    assert_eq!(value["operator_approval_from_ordering_accepted"], false);
    assert_eq!(value["activation_from_ordering_allowed"], false);
    assert_eq!(value["activation_from_replay_allowed"], false);
    assert_eq!(value["activation_from_receipt_allowed"], false);
    assert_eq!(value["activation_command_enabled"], false);
    assert_eq!(value["activation_command_invoked"], false);
    assert_eq!(value["activation_command_dispatched"], false);
    assert_eq!(value["activation_request_accepted"], false);
    assert_eq!(value["activation_request_recorded"], false);
    assert_eq!(value["activation_request_executed"], false);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["install_performed_count"], 0);
    assert_eq!(value["service_restarted_count"], 0);
    assert_eq!(value["active_binary_mutated_count"], 0);
    assert_eq!(value["upstream_fetch_performed_count"], 0);
    assert_eq!(value["upstream_merge_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 20);
    assert_eq!(value["enablement_lane_count"], 23);
    assert_eq!(value["ready_enablement_lane_count"], 23);

    let fixtures = value["ordering_monotonicity_fixtures"]
        .as_array()
        .expect("activation command result receipt ordering monotonicity denial fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert_eq!(
            fixture["activation_command_result_receipt_ordering_allowed"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_ordering_recorded"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_sequence_cursor_recorded"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_monotonicity_state_recorded"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_out_of_order_accepted"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_latest_wins_overwrite_accepted"],
            false
        );
        assert_eq!(fixture["operator_approval_from_ordering_accepted"], false);
        assert_eq!(fixture["activation_from_ordering_allowed"], false);
        assert_eq!(fixture["activation_command_enabled"], false);
        assert_eq!(fixture["activation_command_invoked"], false);
        assert_eq!(fixture["activation_command_dispatched"], false);
        assert_eq!(fixture["activation_request_accepted"], false);
        assert_eq!(fixture["activation_request_executed"], false);
        assert_eq!(fixture["dispatch_performed"], false);
        assert_eq!(fixture["execution_performed"], false);
        assert_eq!(fixture["context_injection_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["external_kg_adapter_read_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["install_performed"], false);
        assert_eq!(fixture["service_restarted"], false);
        assert_eq!(fixture["active_binary_mutated"], false);
        assert_eq!(fixture["upstream_fetch_performed"], false);
        assert_eq!(fixture["upstream_merge_performed"], false);
        assert_eq!(fixture["receipt_noop_confirmed"], true);
    }

    let denied = value
            ["denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity"]
            .as_array()
            .expect("denied activation command result receipt ordering monotonicity actions");
    assert!(denied.len() >= 145);
    assert_eq!(
        value["denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_ordering_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_sequence_cursor_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_monotonicity_state_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_latest_wins_overwrite_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_from_ordering_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_command_enabled"], false);
    assert_eq!(value["side_effects"]["activation_command_invoked"], false);
    assert_eq!(
        value["side_effects"]["activation_command_dispatched"],
        false
    );
    assert_eq!(value["side_effects"]["activation_request_recorded"], false);
    assert_eq!(value["side_effects"]["activation_request_executed"], false);
    assert_eq!(value["side_effects"]["dispatch_performed"], false);
    assert_eq!(value["side_effects"]["execution_performed"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["install_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["upstream_fetch_performed"], false);
    assert_eq!(value["side_effects"]["upstream_merge_performed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_endpoint_blocks_cancellation()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt cancellation supersession denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status"],
        "blocked"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_v1"
    );
    assert_eq!(value["source_ordering_monotonicity_fixture_count"], 10);
    assert_eq!(
        value["source_accepted_ordering_monotonicity_fixture_count"],
        0
    );
    assert_eq!(value["source_ordering_monotonicity_performed_count"], 0);
    assert_eq!(value["source_sequence_cursor_recorded_count"], 0);
    assert_eq!(value["source_monotonicity_state_recorded_count"], 0);
    assert_eq!(value["cancellation_supersession_surface_count"], 14);
    assert_eq!(value["cancellation_supersession_surface_ready_count"], 14);
    assert_eq!(
        value["cancellation_supersession_side_effect_free_surface_count"],
        14
    );
    assert_eq!(value["cancellation_supersession_fixture_count"], 10);
    assert_eq!(value["blocked_cancellation_supersession_fixture_count"], 10);
    assert_eq!(value["noop_cancellation_supersession_fixture_count"], 10);
    assert_eq!(value["allowed_cancellation_supersession_fixture_count"], 0);
    assert_eq!(value["accepted_cancellation_supersession_fixture_count"], 0);
    assert_eq!(value["cancellation_fixture_count"], 6);
    assert_eq!(value["supersession_fixture_count"], 5);
    assert_eq!(value["cancellation_denied_count"], 6);
    assert_eq!(value["supersession_denied_count"], 5);
    assert_eq!(value["cancellation_performed_count"], 0);
    assert_eq!(value["supersession_performed_count"], 0);
    assert_eq!(value["replacement_receipt_accepted_count"], 0);
    assert_eq!(value["replacement_receipt_recorded_count"], 0);
    assert_eq!(value["replacement_receipt_persisted_count"], 0);
    assert_eq!(value["tombstone_recorded_count"], 0);
    assert_eq!(value["delete_marker_recorded_count"], 0);
    assert_eq!(
        value["activation_command_result_receipt_cancellation_allowed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_cancellation_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_cancellation_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_cancellation_request_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_supersession_allowed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_supersession_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_supersession_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_supersession_request_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_replacement_receipt_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_replacement_receipt_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_replacement_receipt_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_tombstone_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_delete_marker_recorded"],
        false
    );
    assert_eq!(value["operator_approval_from_cancellation_accepted"], false);
    assert_eq!(value["operator_approval_from_supersession_accepted"], false);
    assert_eq!(value["activation_from_cancellation_allowed"], false);
    assert_eq!(value["activation_from_supersession_allowed"], false);
    assert_eq!(value["activation_from_ordering_allowed"], false);
    assert_eq!(value["activation_from_replay_allowed"], false);
    assert_eq!(value["activation_from_receipt_allowed"], false);
    assert_eq!(value["activation_command_enabled"], false);
    assert_eq!(value["activation_command_invoked"], false);
    assert_eq!(value["activation_command_dispatched"], false);
    assert_eq!(value["activation_request_accepted"], false);
    assert_eq!(value["activation_request_recorded"], false);
    assert_eq!(value["activation_request_executed"], false);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["install_performed_count"], 0);
    assert_eq!(value["service_restarted_count"], 0);
    assert_eq!(value["active_binary_mutated_count"], 0);
    assert_eq!(value["upstream_fetch_performed_count"], 0);
    assert_eq!(value["upstream_merge_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 21);
    assert_eq!(value["enablement_lane_count"], 24);
    assert_eq!(value["ready_enablement_lane_count"], 24);

    let fixtures = value["cancellation_supersession_fixtures"]
        .as_array()
        .expect("activation command result receipt cancellation supersession denial fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert_eq!(
            fixture["activation_command_result_receipt_cancellation_allowed"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_cancellation_recorded"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_supersession_allowed"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_supersession_recorded"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_replacement_receipt_accepted"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_replacement_receipt_recorded"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_tombstone_recorded"],
            false
        );
        assert_eq!(
            fixture["activation_command_result_receipt_delete_marker_recorded"],
            false
        );
        assert_eq!(
            fixture["operator_approval_from_cancellation_accepted"],
            false
        );
        assert_eq!(
            fixture["operator_approval_from_supersession_accepted"],
            false
        );
        assert_eq!(fixture["activation_from_cancellation_allowed"], false);
        assert_eq!(fixture["activation_from_supersession_allowed"], false);
        assert_eq!(fixture["activation_command_enabled"], false);
        assert_eq!(fixture["activation_command_invoked"], false);
        assert_eq!(fixture["activation_command_dispatched"], false);
        assert_eq!(fixture["activation_request_accepted"], false);
        assert_eq!(fixture["activation_request_executed"], false);
        assert_eq!(fixture["dispatch_performed"], false);
        assert_eq!(fixture["execution_performed"], false);
        assert_eq!(fixture["context_injection_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["external_kg_adapter_read_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["install_performed"], false);
        assert_eq!(fixture["service_restarted"], false);
        assert_eq!(fixture["active_binary_mutated"], false);
        assert_eq!(fixture["upstream_fetch_performed"], false);
        assert_eq!(fixture["upstream_merge_performed"], false);
        assert_eq!(fixture["receipt_noop_confirmed"], true);
    }

    let denied = value
            ["denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession"]
            .as_array()
            .expect("denied activation command result receipt cancellation supersession actions");
    assert!(denied.len() >= 160);
    assert_eq!(
        value["denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_cancellation_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_supersession_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_replacement_receipt_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_tombstone_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_from_cancellation_allowed"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_from_supersession_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_command_enabled"], false);
    assert_eq!(value["side_effects"]["activation_command_invoked"], false);
    assert_eq!(
        value["side_effects"]["activation_command_dispatched"],
        false
    );
    assert_eq!(value["side_effects"]["activation_request_recorded"], false);
    assert_eq!(value["side_effects"]["activation_request_executed"], false);
    assert_eq!(value["side_effects"]["dispatch_performed"], false);
    assert_eq!(value["side_effects"]["execution_performed"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["install_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["upstream_fetch_performed"], false);
    assert_eq!(value["side_effects"]["upstream_merge_performed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_endpoint_blocks_audit_evidence()
 {
    let body = route_contract_body(HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt audit trail immutable evidence denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status"],
        "blocked"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_v1"
    );
    assert_eq!(value["source_cancellation_supersession_fixture_count"], 10);
    assert_eq!(
        value["source_accepted_cancellation_supersession_fixture_count"],
        0
    );
    assert_eq!(value["source_cancellation_performed_count"], 0);
    assert_eq!(value["source_supersession_performed_count"], 0);
    assert_eq!(value["source_replacement_receipt_recorded_count"], 0);
    assert_eq!(value["audit_trail_immutable_evidence_surface_count"], 12);
    assert_eq!(
        value["audit_trail_immutable_evidence_surface_ready_count"],
        12
    );
    assert_eq!(
        value["audit_trail_immutable_evidence_side_effect_free_surface_count"],
        12
    );
    assert_eq!(value["audit_trail_immutable_evidence_fixture_count"], 10);
    assert_eq!(
        value["blocked_audit_trail_immutable_evidence_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_audit_trail_immutable_evidence_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_audit_trail_immutable_evidence_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_audit_trail_immutable_evidence_fixture_count"],
        0
    );
    assert_eq!(value["audit_trail_performed_count"], 0);
    assert_eq!(value["immutable_evidence_performed_count"], 0);
    assert_eq!(value["hash_chain_recorded_count"], 0);
    assert_eq!(value["merkle_root_recorded_count"], 0);
    assert_eq!(value["attestation_recorded_count"], 0);
    assert_eq!(value["witness_recorded_count"], 0);
    assert_eq!(value["notary_recorded_count"], 0);
    assert_eq!(value["ledger_evidence_recorded_count"], 0);
    assert_eq!(
        value["activation_command_result_receipt_audit_trail_allowed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_audit_trail_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_audit_trail_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_immutable_evidence_allowed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_immutable_evidence_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_immutable_evidence_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_hash_chain_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_attestation_recorded"],
        false
    );
    assert_eq!(value["activation_command_result_receipt_recorded"], false);
    assert_eq!(value["activation_command_result_receipt_persisted"], false);
    assert_eq!(value["activation_command_result_receipt_accepted"], false);
    assert_eq!(value["operator_approval_from_audit_trail_accepted"], false);
    assert_eq!(
        value["operator_approval_from_immutable_evidence_accepted"],
        false
    );
    assert_eq!(value["activation_from_audit_trail_allowed"], false);
    assert_eq!(value["activation_from_immutable_evidence_allowed"], false);
    assert_eq!(value["activation_command_enabled"], false);
    assert_eq!(value["activation_command_invoked"], false);
    assert_eq!(value["activation_command_dispatched"], false);
    assert_eq!(value["activation_request_accepted"], false);
    assert_eq!(value["activation_request_executed"], false);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["install_performed_count"], 0);
    assert_eq!(value["service_restarted_count"], 0);
    assert_eq!(value["active_binary_mutated_count"], 0);
    assert_eq!(value["upstream_fetch_performed_count"], 0);
    assert_eq!(value["upstream_merge_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 22);
    assert_eq!(value["enablement_lane_count"], 25);
    assert_eq!(value["ready_enablement_lane_count"], 25);

    let fixtures = value["audit_trail_immutable_evidence_fixtures"]
        .as_array()
        .expect("activation command result receipt audit trail immutable evidence denial fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert!(
            fixture["audit_evidence_status"]
                .as_str()
                .expect("audit evidence fixture status")
                .starts_with("blocked")
        );
        assert_eq!(fixture["audit_trail_recorded"], false);
        assert_eq!(fixture["audit_trail_persisted"], false);
        assert_eq!(fixture["immutable_evidence_recorded"], false);
        assert_eq!(fixture["immutable_evidence_persisted"], false);
        assert_eq!(fixture["hash_chain_recorded"], false);
        assert_eq!(fixture["attestation_recorded"], false);
        assert_eq!(fixture["activation_command_result_receipt_accepted"], false);
        assert_eq!(
            fixture["operator_approval_from_audit_trail_accepted"],
            false
        );
        assert_eq!(
            fixture["operator_approval_from_immutable_evidence_accepted"],
            false
        );
        assert_eq!(fixture["activation_from_audit_trail_allowed"], false);
        assert_eq!(fixture["activation_from_immutable_evidence_allowed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["external_kg_adapter_read_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["receipt_noop_confirmed"], true);
    }

    let denied = value
            ["denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence"]
            .as_array()
            .expect("denied activation command result receipt audit trail immutable evidence actions");
    assert!(denied.len() >= 180);
    assert_eq!(
        value["denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_audit_trail_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_immutable_evidence_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_hash_chain_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_attestation_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["install_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["upstream_fetch_performed"], false);
    assert_eq!(value["side_effects"]["upstream_merge_performed"], false);
}
