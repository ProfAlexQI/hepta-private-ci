fn hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_materialization_lane_report()
-> HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedKgPromptPayloadMaterializationLaneResponse
{
    let route_matrix = control_ui_route_parity_report();
    let preview_adapter_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_preview_read_only_adapter_lane_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && preview_adapter_lane.status == "ready"
        && preview_adapter_lane.operator_approved_activation_lane_present
        && preview_adapter_lane.operator_approved_activation_lane_effective
        && preview_adapter_lane.memory_durable_mutation_lane_enabled
        && preview_adapter_lane.memory_store_write_path_enabled
        && preview_adapter_lane.memory_store_mutation_enabled
        && preview_adapter_lane.live_memory_write_allowed_by_lane
        && !preview_adapter_lane.live_memory_write_performed_by_report_route
        && preview_adapter_lane.hepta_intelligence_context_attachment_lane_enabled
        && preview_adapter_lane.hepta_intelligence_context_attachment_allowed_by_lane
        && !preview_adapter_lane.hepta_intelligence_context_attached_by_report_route
        && preview_adapter_lane.bounded_prompt_preview_lane_enabled
        && preview_adapter_lane.bounded_prompt_preview_allowed_by_lane
        && !preview_adapter_lane.prompt_preview_rendered_by_report_route
        && !preview_adapter_lane.prompt_payload_materialized_by_report_route
        && preview_adapter_lane.prompt_preview_requires_explicit_command
        && preview_adapter_lane.kg_prompt_preview_lane_enabled
        && preview_adapter_lane.kg_prompt_preview_allowed_by_lane
        && !preview_adapter_lane.kg_prompt_preview_rendered_by_report_route
        && preview_adapter_lane.kg_external_adapter_read_lane_enabled
        && preview_adapter_lane.kg_external_adapter_read_allowed_by_lane
        && !preview_adapter_lane.kg_external_adapter_read_performed_by_report_route
        && preview_adapter_lane.kg_external_adapter_requires_explicit_command
        && preview_adapter_lane.kg_external_adapter_credential_reference_required
        && !preview_adapter_lane.kg_external_adapter_credential_read_allowed_by_lane
        && !preview_adapter_lane.kg_external_adapter_credential_read_performed_by_report_route
        && preview_adapter_lane.context_handoff_acceptance_required
        && preview_adapter_lane.context_attachment_requires_explicit_command
        && !preview_adapter_lane.context_injection_allowed_by_lane
        && !preview_adapter_lane.context_injection_performed_by_report_route
        && !preview_adapter_lane.kg_live_write_lane_enabled
        && !preview_adapter_lane.kg_live_write_allowed_by_lane
        && !preview_adapter_lane.kg_live_write_performed_by_report_route
        && !preview_adapter_lane.provider_model_invocation_lane_enabled
        && !preview_adapter_lane.provider_model_invocation_allowed_by_lane
        && !preview_adapter_lane.channel_delivery_lane_enabled
        && preview_adapter_lane.live_mutation_enabled_count == 1
        && preview_adapter_lane.current_live_enabled_lane_count == 3
        && !preview_adapter_lane.side_effects.memory_store_mutated
        && !preview_adapter_lane
            .side_effects
            .memory_store_write_performed
        && !preview_adapter_lane
            .side_effects
            .hepta_intelligence_context_attached
        && !preview_adapter_lane.side_effects.prompt_preview_rendered
        && !preview_adapter_lane
            .side_effects
            .prompt_payload_materialized
        && !preview_adapter_lane
            .side_effects
            .context_injection_performed
        && !preview_adapter_lane.side_effects.provider_invoked
        && !preview_adapter_lane.side_effects.model_invoked
        && !preview_adapter_lane.side_effects.auth_secret_read
        && !preview_adapter_lane.side_effects.credential_read
        && !preview_adapter_lane
            .side_effects
            .external_kg_adapter_read_performed
        && !preview_adapter_lane.side_effects.live_kg_write_performed
        && !preview_adapter_lane.side_effects.channel_send_performed
        && !preview_adapter_lane.side_effects.service_restarted
        && !preview_adapter_lane.side_effects.active_binary_mutated
        && !preview_adapter_lane.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedKgPromptPayloadMaterializationLaneResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command:
            "/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane --json",
        native_route: true,
        compatibility_mode:
            "native_full_enablement_operator_approved_kg_prompt_payload_materialization_lane_status",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_ENDPOINT,
        kg_prompt_preview_read_only_adapter_lane_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_ENDPOINT,
        kg_prompt_preview_read_only_adapter_lane_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_GATE.md",
        kg_prompt_payload_materialization_lane_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_GATE.md",
        source_kg_prompt_preview_read_only_adapter_lane_gate:
            "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-preview-read-only-adapter-lane-gate.sh",
        source_kg_prompt_payload_materialization_lane_gate:
            "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        kg_prompt_preview_read_only_adapter_lane_ready:
            preview_adapter_lane.kg_prompt_preview_lane_enabled
                && preview_adapter_lane.kg_external_adapter_read_lane_enabled,
        kg_prompt_preview_read_only_adapter_lane_status: preview_adapter_lane.status,
        operator_authorization_source: "telegram_direct_operator_authorization_2026_06_12_18_50_49_asia_shanghai",
        operator_authorization_scope:
            "kg_prompt_payload_materialization_lane_no_report_payload_no_kg_live_write_provider_model_channel_or_public_release",
        operator_authorization_received: true,
        operator_approved_activation_lane_present: true,
        operator_approved_activation_lane_effective: true,
        memory_durable_mutation_lane_enabled: true,
        memory_store_write_path_enabled: true,
        memory_store_mutation_enabled: true,
        live_memory_write_allowed_by_lane: true,
        live_memory_write_performed_by_report_route: false,
        hepta_intelligence_context_attachment_lane_enabled: true,
        hepta_intelligence_context_attachment_allowed_by_lane: true,
        hepta_intelligence_context_attached_by_report_route: false,
        bounded_prompt_preview_lane_enabled: true,
        bounded_prompt_preview_allowed_by_lane: true,
        prompt_preview_rendered_by_report_route: false,
        prompt_preview_requires_explicit_command: true,
        prompt_payload_materialized_by_report_route: false,
        kg_prompt_preview_lane_enabled: true,
        kg_prompt_preview_allowed_by_lane: true,
        kg_prompt_preview_rendered_by_report_route: false,
        kg_external_adapter_read_lane_enabled: true,
        kg_external_adapter_read_allowed_by_lane: true,
        kg_external_adapter_read_performed_by_report_route: false,
        kg_external_adapter_requires_explicit_command: true,
        kg_external_adapter_credential_reference_required: true,
        kg_external_adapter_credential_read_allowed_by_lane: false,
        kg_external_adapter_credential_read_performed_by_report_route: false,
        supported_kg_adapter_count:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_SUPPORTED_KG_ADAPTERS.len(),
        supported_kg_adapters:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_SUPPORTED_KG_ADAPTERS,
        kg_prompt_payload_materialization_lane_enabled: true,
        kg_prompt_payload_materialization_allowed_by_lane: true,
        kg_prompt_payload_materialized_by_report_route: false,
        kg_prompt_payload_shape_requires_explicit_command: true,
        kg_prompt_payload_redaction_required: true,
        kg_prompt_payload_raw_text_exposed_by_report_route: false,
        kg_prompt_payload_hash_preview_allowed_by_lane: true,
        kg_prompt_payload_hash_preview_rendered_by_report_route: false,
        context_handoff_acceptance_required: true,
        context_attachment_requires_explicit_command: true,
        context_injection_allowed_by_lane: false,
        context_injection_performed_by_report_route: false,
        kg_live_write_lane_enabled: false,
        kg_live_write_allowed_by_lane: false,
        kg_live_write_performed_by_report_route: false,
        provider_model_invocation_lane_enabled: false,
        provider_model_invocation_allowed_by_lane: false,
        channel_delivery_lane_enabled: false,
        live_mutation_enabled_count: 1,
        current_live_enabled_lane_count: 4,
        enablement_lane_count: 7,
        ready_enablement_lane_count: 7,
        blocked_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedMemoryLiveMutationDurableLaneSideEffects {
                report_route_invoked_runtime_execution: false,
                live_7373_router_mutated_by_report_route: false,
                operator_approval_lane_recorded: false,
                operator_approval_lane_persisted: false,
                memory_store_write_path_enabled_by_report_route: false,
                memory_store_mutated: false,
                memory_store_write_performed: false,
                memory_write_receipt_recorded: false,
                memory_write_receipt_persisted: false,
                rollback_kill_switch_mutated: false,
                post_write_validation_performed: false,
                hepta_intelligence_context_attached: false,
                prompt_preview_rendered: false,
                prompt_payload_materialized: false,
                context_injection_performed: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                external_kg_adapter_read_performed: false,
                live_kg_write_performed: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
                public_ga_claimed: false,
            },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_acceptance_receipt_lane_report()
-> HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedKgPromptPayloadAcceptanceReceiptLaneResponse
{
    let route_matrix = control_ui_route_parity_report();
    let payload_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_materialization_lane_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && payload_lane.status == "ready"
        && payload_lane.operator_approved_activation_lane_present
        && payload_lane.operator_approved_activation_lane_effective
        && payload_lane.memory_durable_mutation_lane_enabled
        && payload_lane.memory_store_write_path_enabled
        && payload_lane.memory_store_mutation_enabled
        && payload_lane.live_memory_write_allowed_by_lane
        && !payload_lane.live_memory_write_performed_by_report_route
        && payload_lane.hepta_intelligence_context_attachment_lane_enabled
        && payload_lane.hepta_intelligence_context_attachment_allowed_by_lane
        && !payload_lane.hepta_intelligence_context_attached_by_report_route
        && payload_lane.bounded_prompt_preview_lane_enabled
        && payload_lane.bounded_prompt_preview_allowed_by_lane
        && !payload_lane.prompt_preview_rendered_by_report_route
        && payload_lane.prompt_preview_requires_explicit_command
        && !payload_lane.prompt_payload_materialized_by_report_route
        && payload_lane.kg_prompt_preview_lane_enabled
        && payload_lane.kg_prompt_preview_allowed_by_lane
        && !payload_lane.kg_prompt_preview_rendered_by_report_route
        && payload_lane.kg_external_adapter_read_lane_enabled
        && payload_lane.kg_external_adapter_read_allowed_by_lane
        && !payload_lane.kg_external_adapter_read_performed_by_report_route
        && payload_lane.kg_external_adapter_requires_explicit_command
        && payload_lane.kg_external_adapter_credential_reference_required
        && !payload_lane.kg_external_adapter_credential_read_allowed_by_lane
        && !payload_lane.kg_external_adapter_credential_read_performed_by_report_route
        && payload_lane.kg_prompt_payload_materialization_lane_enabled
        && payload_lane.kg_prompt_payload_materialization_allowed_by_lane
        && !payload_lane.kg_prompt_payload_materialized_by_report_route
        && payload_lane.kg_prompt_payload_shape_requires_explicit_command
        && payload_lane.kg_prompt_payload_redaction_required
        && !payload_lane.kg_prompt_payload_raw_text_exposed_by_report_route
        && payload_lane.kg_prompt_payload_hash_preview_allowed_by_lane
        && !payload_lane.kg_prompt_payload_hash_preview_rendered_by_report_route
        && payload_lane.context_handoff_acceptance_required
        && payload_lane.context_attachment_requires_explicit_command
        && !payload_lane.context_injection_allowed_by_lane
        && !payload_lane.context_injection_performed_by_report_route
        && !payload_lane.kg_live_write_lane_enabled
        && !payload_lane.kg_live_write_allowed_by_lane
        && !payload_lane.kg_live_write_performed_by_report_route
        && !payload_lane.provider_model_invocation_lane_enabled
        && !payload_lane.provider_model_invocation_allowed_by_lane
        && !payload_lane.channel_delivery_lane_enabled
        && payload_lane.live_mutation_enabled_count == 1
        && payload_lane.current_live_enabled_lane_count == 4
        && !payload_lane.side_effects.memory_store_mutated
        && !payload_lane.side_effects.memory_store_write_performed
        && !payload_lane
            .side_effects
            .hepta_intelligence_context_attached
        && !payload_lane.side_effects.prompt_preview_rendered
        && !payload_lane.side_effects.prompt_payload_materialized
        && !payload_lane.side_effects.context_injection_performed
        && !payload_lane.side_effects.provider_invoked
        && !payload_lane.side_effects.model_invoked
        && !payload_lane.side_effects.auth_secret_read
        && !payload_lane.side_effects.credential_read
        && !payload_lane.side_effects.external_kg_adapter_read_performed
        && !payload_lane.side_effects.live_kg_write_performed
        && !payload_lane.side_effects.channel_send_performed
        && !payload_lane.side_effects.service_restarted
        && !payload_lane.side_effects.active_binary_mutated
        && !payload_lane.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedKgPromptPayloadAcceptanceReceiptLaneResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command:
            "/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-acceptance-receipt-lane --json",
        native_route: true,
        compatibility_mode:
            "native_full_enablement_operator_approved_kg_prompt_payload_acceptance_receipt_lane_status",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_ENDPOINT,
        kg_prompt_payload_materialization_lane_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_ENDPOINT,
        kg_prompt_payload_materialization_lane_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_GATE.md",
        kg_prompt_payload_acceptance_receipt_lane_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_GATE.md",
        source_kg_prompt_payload_materialization_lane_gate:
            "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane-gate.sh",
        source_kg_prompt_payload_acceptance_receipt_lane_gate:
            "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-acceptance-receipt-lane-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        kg_prompt_payload_materialization_lane_ready:
            payload_lane.kg_prompt_payload_materialization_lane_enabled
                && payload_lane.kg_prompt_payload_materialization_allowed_by_lane,
        kg_prompt_payload_materialization_lane_status: payload_lane.status,
        operator_authorization_source: "telegram_direct_operator_authorization_2026_06_12_18_50_49_asia_shanghai",
        operator_authorization_scope:
            "kg_prompt_payload_acceptance_receipt_lane_no_report_receipt_persistence_no_kg_live_write_provider_model_channel_or_public_release",
        operator_authorization_received: true,
        operator_approved_activation_lane_present: true,
        operator_approved_activation_lane_effective: true,
        memory_durable_mutation_lane_enabled: true,
        memory_store_write_path_enabled: true,
        memory_store_mutation_enabled: true,
        live_memory_write_allowed_by_lane: true,
        live_memory_write_performed_by_report_route: false,
        hepta_intelligence_context_attachment_lane_enabled: true,
        hepta_intelligence_context_attachment_allowed_by_lane: true,
        hepta_intelligence_context_attached_by_report_route: false,
        bounded_prompt_preview_lane_enabled: true,
        bounded_prompt_preview_allowed_by_lane: true,
        prompt_preview_rendered_by_report_route: false,
        prompt_preview_requires_explicit_command: true,
        prompt_payload_materialized_by_report_route: false,
        kg_prompt_preview_lane_enabled: true,
        kg_prompt_preview_allowed_by_lane: true,
        kg_prompt_preview_rendered_by_report_route: false,
        kg_external_adapter_read_lane_enabled: true,
        kg_external_adapter_read_allowed_by_lane: true,
        kg_external_adapter_read_performed_by_report_route: false,
        kg_external_adapter_requires_explicit_command: true,
        kg_external_adapter_credential_reference_required: true,
        kg_external_adapter_credential_read_allowed_by_lane: false,
        kg_external_adapter_credential_read_performed_by_report_route: false,
        supported_kg_adapter_count:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_SUPPORTED_KG_ADAPTERS.len(),
        supported_kg_adapters:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_SUPPORTED_KG_ADAPTERS,
        kg_prompt_payload_materialization_lane_enabled: true,
        kg_prompt_payload_materialization_allowed_by_lane: true,
        kg_prompt_payload_materialized_by_report_route: false,
        kg_prompt_payload_shape_requires_explicit_command: true,
        kg_prompt_payload_redaction_required: true,
        kg_prompt_payload_raw_text_exposed_by_report_route: false,
        kg_prompt_payload_hash_preview_allowed_by_lane: true,
        kg_prompt_payload_hash_preview_rendered_by_report_route: false,
        kg_prompt_payload_acceptance_receipt_lane_enabled: true,
        kg_prompt_payload_acceptance_receipt_allowed_by_lane: true,
        kg_prompt_payload_acceptance_receipt_requires_explicit_command: true,
        kg_prompt_payload_acceptance_receipt_redaction_required: true,
        kg_prompt_payload_acceptance_receipt_redaction_proof_required: true,
        kg_prompt_payload_acceptance_receipt_hash_binding_required: true,
        kg_prompt_payload_acceptance_receipt_raw_payload_allowed: false,
        kg_prompt_payload_acceptance_receipt_recorded_by_report_route: false,
        kg_prompt_payload_acceptance_receipt_persisted_by_report_route: false,
        kg_prompt_payload_acceptance_receipt_accepted_by_report_route: false,
        kg_prompt_payload_acceptance_receipt_filesystem_written_by_report_route: false,
        kg_prompt_payload_acceptance_receipt_ledger_recorded_by_report_route: false,
        kg_prompt_payload_acceptance_receipt_promotes_activation_authority: false,
        context_handoff_acceptance_required: true,
        context_attachment_requires_explicit_command: true,
        context_injection_allowed_by_lane: false,
        context_injection_performed_by_report_route: false,
        kg_live_write_lane_enabled: false,
        kg_live_write_allowed_by_lane: false,
        kg_live_write_performed_by_report_route: false,
        provider_model_invocation_lane_enabled: false,
        provider_model_invocation_allowed_by_lane: false,
        channel_delivery_lane_enabled: false,
        live_mutation_enabled_count: 1,
        current_live_enabled_lane_count: 5,
        enablement_lane_count: 8,
        ready_enablement_lane_count: 8,
        blocked_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedMemoryLiveMutationDurableLaneSideEffects {
                report_route_invoked_runtime_execution: false,
                live_7373_router_mutated_by_report_route: false,
                operator_approval_lane_recorded: false,
                operator_approval_lane_persisted: false,
                memory_store_write_path_enabled_by_report_route: false,
                memory_store_mutated: false,
                memory_store_write_performed: false,
                memory_write_receipt_recorded: false,
                memory_write_receipt_persisted: false,
                rollback_kill_switch_mutated: false,
                post_write_validation_performed: false,
                hepta_intelligence_context_attached: false,
                prompt_preview_rendered: false,
                prompt_payload_materialized: false,
                context_injection_performed: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                external_kg_adapter_read_performed: false,
                live_kg_write_performed: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
                public_ga_claimed: false,
            },
    }
}

fn extend_json_object(target: &mut serde_json::Value, extension: serde_json::Value) {
    let Some(target) = target.as_object_mut() else {
        return;
    };
    let Some(extension) = extension.as_object() else {
        return;
    };
    for (key, value) in extension {
        target.insert(key.clone(), value.clone());
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn sha256_json_value(value: &serde_json::Value) -> String {
    let bytes = serde_json::to_vec(value).unwrap_or_default();
    sha256_hex(&bytes)
}

fn sha256_text_value(text: &str) -> String {
    sha256_hex(text.as_bytes())
}

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_readback_audit_receipt_lane_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let acceptance_receipt_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_acceptance_receipt_lane_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_acceptance_receipt_lane_ready = acceptance_receipt_lane.status == "ready"
        && acceptance_receipt_lane.kg_prompt_payload_acceptance_receipt_lane_enabled
        && acceptance_receipt_lane.kg_prompt_payload_acceptance_receipt_allowed_by_lane
        && acceptance_receipt_lane.kg_prompt_payload_acceptance_receipt_requires_explicit_command
        && acceptance_receipt_lane.kg_prompt_payload_acceptance_receipt_redaction_required
        && acceptance_receipt_lane.kg_prompt_payload_acceptance_receipt_redaction_proof_required
        && acceptance_receipt_lane.kg_prompt_payload_acceptance_receipt_hash_binding_required
        && !acceptance_receipt_lane.kg_prompt_payload_acceptance_receipt_raw_payload_allowed
        && !acceptance_receipt_lane.kg_prompt_payload_acceptance_receipt_recorded_by_report_route
        && !acceptance_receipt_lane.kg_prompt_payload_acceptance_receipt_persisted_by_report_route
        && !acceptance_receipt_lane.kg_prompt_payload_acceptance_receipt_accepted_by_report_route
        && !acceptance_receipt_lane
            .kg_prompt_payload_acceptance_receipt_filesystem_written_by_report_route
        && !acceptance_receipt_lane
            .kg_prompt_payload_acceptance_receipt_ledger_recorded_by_report_route
        && !acceptance_receipt_lane
            .kg_prompt_payload_acceptance_receipt_promotes_activation_authority
        && !acceptance_receipt_lane.kg_live_write_lane_enabled
        && !acceptance_receipt_lane.provider_model_invocation_lane_enabled
        && !acceptance_receipt_lane.channel_delivery_lane_enabled;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_acceptance_receipt_lane_ready;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-readback-audit-receipt-lane --json",
        "native_route": true,
        "compatibility_mode": "native_full_enablement_operator_approved_kg_prompt_payload_readback_audit_receipt_lane_status",
        "side_effect_free": true,
        "audit_date": "2026-06-12",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT,
        "kg_prompt_payload_acceptance_receipt_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_ENDPOINT,
        "kg_prompt_payload_acceptance_receipt_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_GATE.md",
        "kg_prompt_payload_readback_audit_receipt_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_GATE.md",
        "source_kg_prompt_payload_acceptance_receipt_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-acceptance-receipt-lane-gate.sh",
        "source_kg_prompt_payload_readback_audit_receipt_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-readback-audit-receipt-lane-gate.sh",
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        "route_count_floor_preserved": route_count_floor_preserved,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "source_acceptance_receipt_lane_ready": source_acceptance_receipt_lane_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "kg_prompt_payload_acceptance_receipt_lane_status": acceptance_receipt_lane.status,
        "operator_authorization_source": "telegram_direct_operator_authorization_2026_06_12_18_50_49_asia_shanghai",
        "operator_authorization_scope": "kg_prompt_payload_readback_audit_receipt_lane_no_report_receipt_render_record_persist_accept_no_kg_live_write_provider_model_channel_or_public_release",
        "operator_authorization_received": true,
        "operator_approved_activation_lane_present": true,
        "operator_approved_activation_lane_effective": true,
        "memory_durable_mutation_lane_enabled": true,
        "memory_store_write_path_enabled": true,
        "memory_store_mutation_enabled": true,
        "live_memory_write_allowed_by_lane": true,
        "live_memory_write_performed_by_report_route": false,
        "hepta_intelligence_context_attachment_lane_enabled": true,
        "hepta_intelligence_context_attachment_allowed_by_lane": true,
        "hepta_intelligence_context_attached_by_report_route": false,
        "bounded_prompt_preview_lane_enabled": true,
        "bounded_prompt_preview_allowed_by_lane": true,
        "prompt_preview_rendered_by_report_route": false,
        "prompt_preview_requires_explicit_command": true,
        "prompt_payload_materialized_by_report_route": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "kg_prompt_preview_lane_enabled": true,
            "kg_prompt_preview_allowed_by_lane": true,
            "kg_prompt_preview_rendered_by_report_route": false,
            "kg_external_adapter_read_lane_enabled": true,
            "kg_external_adapter_read_allowed_by_lane": true,
            "kg_external_adapter_read_performed_by_report_route": false,
            "kg_external_adapter_requires_explicit_command": true,
            "kg_external_adapter_credential_reference_required": true,
            "kg_external_adapter_credential_read_allowed_by_lane": false,
            "kg_external_adapter_credential_read_performed_by_report_route": false,
            "supported_kg_adapter_count": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_SUPPORTED_KG_ADAPTERS.len(),
            "supported_kg_adapters": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_SUPPORTED_KG_ADAPTERS,
            "kg_prompt_payload_materialization_lane_enabled": true,
            "kg_prompt_payload_materialization_allowed_by_lane": true,
            "kg_prompt_payload_materialized_by_report_route": false,
            "kg_prompt_payload_shape_requires_explicit_command": true,
            "kg_prompt_payload_redaction_required": true,
            "kg_prompt_payload_raw_text_exposed_by_report_route": false,
            "kg_prompt_payload_hash_preview_allowed_by_lane": true,
            "kg_prompt_payload_hash_preview_rendered_by_report_route": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "kg_prompt_payload_acceptance_receipt_lane_enabled": true,
            "kg_prompt_payload_acceptance_receipt_allowed_by_lane": true,
            "kg_prompt_payload_acceptance_receipt_requires_explicit_command": true,
            "kg_prompt_payload_acceptance_receipt_redaction_required": true,
            "kg_prompt_payload_acceptance_receipt_redaction_proof_required": true,
            "kg_prompt_payload_acceptance_receipt_hash_binding_required": true,
            "kg_prompt_payload_acceptance_receipt_raw_payload_allowed": false,
            "kg_prompt_payload_acceptance_receipt_recorded_by_report_route": false,
            "kg_prompt_payload_acceptance_receipt_persisted_by_report_route": false,
            "kg_prompt_payload_acceptance_receipt_accepted_by_report_route": false,
            "kg_prompt_payload_acceptance_receipt_filesystem_written_by_report_route": false,
            "kg_prompt_payload_acceptance_receipt_ledger_recorded_by_report_route": false,
            "kg_prompt_payload_acceptance_receipt_promotes_activation_authority": false,
            "kg_prompt_payload_readback_audit_receipt_lane_enabled": true,
            "kg_prompt_payload_readback_audit_receipt_allowed_by_lane": true,
            "kg_prompt_payload_readback_audit_receipt_requires_explicit_command": true,
            "kg_prompt_payload_readback_audit_receipt_requires_acceptance_receipt": true,
            "kg_prompt_payload_readback_audit_receipt_redaction_required": true,
            "kg_prompt_payload_readback_audit_receipt_redaction_proof_required": true,
            "kg_prompt_payload_readback_audit_receipt_hash_binding_required": true,
            "kg_prompt_payload_readback_audit_receipt_raw_payload_allowed": false,
            "kg_prompt_payload_readback_audit_receipt_rendered_by_report_route": false,
            "kg_prompt_payload_readback_audit_receipt_recorded_by_report_route": false,
            "kg_prompt_payload_readback_audit_receipt_persisted_by_report_route": false,
            "kg_prompt_payload_readback_audit_receipt_accepted_by_report_route": false,
            "kg_prompt_payload_readback_audit_receipt_filesystem_written_by_report_route": false,
            "kg_prompt_payload_readback_audit_receipt_ledger_recorded_by_report_route": false,
            "kg_prompt_payload_readback_audit_receipt_promotes_activation_authority": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "context_handoff_acceptance_required": true,
            "context_attachment_requires_explicit_command": true,
            "context_injection_allowed_by_lane": false,
            "context_injection_performed_by_report_route": false,
            "kg_live_write_lane_enabled": false,
            "kg_live_write_allowed_by_lane": false,
            "kg_live_write_performed_by_report_route": false,
            "provider_model_invocation_lane_enabled": false,
            "provider_model_invocation_allowed_by_lane": false,
            "channel_delivery_lane_enabled": false,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 6,
            "enablement_lane_count": 9,
            "ready_enablement_lane_count": 9,
            "blocked_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_BLOCKED_ACTIONS,
            "allowed_next_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_NEXT_ACTIONS,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": {
                "report_route_invoked_runtime_execution": false,
                "live_7373_router_mutated_by_report_route": false,
                "operator_approval_lane_recorded": false,
                "operator_approval_lane_persisted": false,
                "memory_store_write_path_enabled_by_report_route": false,
                "memory_store_mutated": false,
                "memory_store_write_performed": false,
                "memory_write_receipt_recorded": false,
                "memory_write_receipt_persisted": false,
                "rollback_kill_switch_mutated": false,
                "post_write_validation_performed": false,
                "hepta_intelligence_context_attached": false,
                "prompt_preview_rendered": false,
                "prompt_payload_materialized": false,
                "prompt_payload_acceptance_receipt_rendered": false,
                "prompt_payload_acceptance_receipt_recorded": false,
                "prompt_payload_acceptance_receipt_persisted": false,
                "prompt_payload_readback_audit_receipt_rendered": false,
                "prompt_payload_readback_audit_receipt_recorded": false,
                "prompt_payload_readback_audit_receipt_persisted": false,
                "context_injection_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "auth_secret_read": false,
                "credential_read": false,
                "external_network_call_performed": false,
                "external_kg_adapter_read_performed": false,
                "live_kg_write_performed": false,
                "channel_send_performed": false,
                "external_send_performed": false,
                "gateway_route_migration_performed": false,
                "source_command_migration_performed": false,
                "service_restarted": false,
                "active_binary_mutated": false,
                "release_artifact_written": false,
                "public_release_claimed": false,
                "public_ga_claimed": false
            }
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_acceptance_lane_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let readback_audit_receipt_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_readback_audit_receipt_lane_report();
    let readback_bool = |key: &str| {
        readback_audit_receipt_lane
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let readback_status = readback_audit_receipt_lane
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_readback_audit_receipt_lane_ready = readback_status.as_str() == "ready"
        && readback_bool("kg_prompt_payload_readback_audit_receipt_lane_enabled")
        && readback_bool("kg_prompt_payload_readback_audit_receipt_allowed_by_lane")
        && readback_bool("kg_prompt_payload_readback_audit_receipt_requires_explicit_command")
        && readback_bool("kg_prompt_payload_readback_audit_receipt_requires_acceptance_receipt")
        && readback_bool("kg_prompt_payload_readback_audit_receipt_redaction_required")
        && readback_bool("kg_prompt_payload_readback_audit_receipt_redaction_proof_required")
        && readback_bool("kg_prompt_payload_readback_audit_receipt_hash_binding_required")
        && !readback_bool("kg_prompt_payload_readback_audit_receipt_raw_payload_allowed")
        && !readback_bool("kg_prompt_payload_readback_audit_receipt_rendered_by_report_route")
        && !readback_bool("kg_prompt_payload_readback_audit_receipt_recorded_by_report_route")
        && !readback_bool("kg_prompt_payload_readback_audit_receipt_persisted_by_report_route")
        && !readback_bool("kg_prompt_payload_readback_audit_receipt_accepted_by_report_route")
        && !readback_bool(
            "kg_prompt_payload_readback_audit_receipt_filesystem_written_by_report_route",
        )
        && !readback_bool(
            "kg_prompt_payload_readback_audit_receipt_ledger_recorded_by_report_route",
        )
        && !readback_bool("kg_prompt_payload_readback_audit_receipt_promotes_activation_authority")
        && readback_bool("context_handoff_acceptance_required")
        && !readback_bool("context_injection_allowed_by_lane")
        && !readback_bool("context_injection_performed_by_report_route")
        && !readback_bool("kg_live_write_lane_enabled")
        && !readback_bool("provider_model_invocation_lane_enabled")
        && !readback_bool("channel_delivery_lane_enabled");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_readback_audit_receipt_lane_ready;

    let mut report = readback_audit_receipt_lane;
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane --json",
            "compatibility_mode": "native_full_enablement_operator_approved_context_handoff_acceptance_lane_status",
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_ENDPOINT,
            "kg_prompt_payload_readback_audit_receipt_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT,
            "kg_prompt_payload_readback_audit_receipt_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_GATE.md",
            "context_handoff_acceptance_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_GATE.md",
            "source_kg_prompt_payload_readback_audit_receipt_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-readback-audit-receipt-lane-gate.sh",
            "source_context_handoff_acceptance_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane-gate.sh",
            "kg_prompt_payload_readback_audit_receipt_lane_status": readback_status,
            "source_readback_audit_receipt_lane_ready": source_readback_audit_receipt_lane_ready,
            "operator_authorization_scope": "context_handoff_acceptance_lane_no_report_context_attach_inject_record_persist_accept_no_kg_live_write_provider_model_channel_or_public_release",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "context_handoff_acceptance_required": true,
            "context_handoff_acceptance_lane_enabled": true,
            "context_handoff_acceptance_allowed_by_lane": true,
            "context_handoff_acceptance_requires_explicit_command": true,
            "context_handoff_acceptance_requires_readback_audit_receipt": true,
            "context_handoff_acceptance_redaction_required": true,
            "context_handoff_acceptance_scope_binding_required": true,
            "context_handoff_acceptance_operator_identity_binding_required": true,
            "context_handoff_acceptance_recorded_by_report_route": false,
            "context_handoff_acceptance_persisted_by_report_route": false,
            "context_handoff_acceptance_accepted_by_report_route": false,
            "context_handoff_acceptance_filesystem_written_by_report_route": false,
            "context_handoff_acceptance_ledger_recorded_by_report_route": false,
            "context_handoff_acceptance_promotes_activation_authority": false,
            "context_attachment_performed_by_report_route": false,
            "current_live_enabled_lane_count": 7,
            "enablement_lane_count": 10,
            "ready_enablement_lane_count": 10,
            "blocked_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_BLOCKED_ACTIONS,
            "allowed_next_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_NEXT_ACTIONS,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        side_effects.insert("context_attached".to_string(), serde_json::json!(false));
        side_effects.insert(
            "context_handoff_acceptance_recorded".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "context_handoff_acceptance_persisted".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "context_handoff_acceptance_accepted".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert("context_injected".to_string(), serde_json::json!(false));
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_receipt_audit_lane_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let context_handoff_acceptance_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_acceptance_lane_report();
    let context_bool = |key: &str| {
        context_handoff_acceptance_lane
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let context_status = context_handoff_acceptance_lane
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_context_handoff_acceptance_lane_ready = context_status.as_str() == "ready"
        && context_bool("context_handoff_acceptance_lane_enabled")
        && context_bool("context_handoff_acceptance_allowed_by_lane")
        && context_bool("context_handoff_acceptance_requires_explicit_command")
        && context_bool("context_handoff_acceptance_requires_readback_audit_receipt")
        && context_bool("context_handoff_acceptance_redaction_required")
        && context_bool("context_handoff_acceptance_scope_binding_required")
        && context_bool("context_handoff_acceptance_operator_identity_binding_required")
        && !context_bool("context_handoff_acceptance_recorded_by_report_route")
        && !context_bool("context_handoff_acceptance_persisted_by_report_route")
        && !context_bool("context_handoff_acceptance_accepted_by_report_route")
        && !context_bool("context_handoff_acceptance_filesystem_written_by_report_route")
        && !context_bool("context_handoff_acceptance_ledger_recorded_by_report_route")
        && !context_bool("context_handoff_acceptance_promotes_activation_authority")
        && !context_bool("context_attachment_performed_by_report_route")
        && !context_bool("context_injection_allowed_by_lane")
        && !context_bool("context_injection_performed_by_report_route")
        && !context_bool("kg_live_write_lane_enabled")
        && !context_bool("provider_model_invocation_lane_enabled")
        && !context_bool("channel_delivery_lane_enabled");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_context_handoff_acceptance_lane_ready;

    let mut report = context_handoff_acceptance_lane;
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane --json",
            "compatibility_mode": "native_full_enablement_operator_approved_context_handoff_receipt_audit_lane_status",
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_ENDPOINT,
            "context_handoff_acceptance_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_ENDPOINT,
            "context_handoff_acceptance_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_GATE.md",
            "context_handoff_receipt_audit_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_GATE.md",
            "source_context_handoff_acceptance_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane-gate.sh",
            "source_context_handoff_receipt_audit_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane-gate.sh",
            "context_handoff_acceptance_lane_status": context_status,
            "source_context_handoff_acceptance_lane_ready": source_context_handoff_acceptance_lane_ready,
            "operator_authorization_source": "telegram_direct_operator_highest_authorization_2026_06_13_08_01_56_asia_shanghai",
            "operator_authorization_scope": "context_handoff_receipt_audit_lane_no_report_context_attach_inject_render_record_persist_accept_no_kg_live_write_provider_model_channel_or_public_release",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "context_handoff_receipt_audit_lane_enabled": true,
            "context_handoff_receipt_audit_allowed_by_lane": true,
            "context_handoff_receipt_audit_requires_explicit_command": true,
            "context_handoff_receipt_audit_requires_context_handoff_acceptance": true,
            "context_handoff_receipt_audit_redaction_required": true,
            "context_handoff_receipt_audit_redaction_proof_required": true,
            "context_handoff_receipt_audit_scope_binding_required": true,
            "context_handoff_receipt_audit_operator_identity_binding_required": true,
            "context_handoff_receipt_audit_hash_binding_required": true,
            "context_handoff_receipt_audit_raw_context_allowed": false,
            "context_handoff_receipt_audit_rendered_by_report_route": false,
            "context_handoff_receipt_audit_recorded_by_report_route": false,
            "context_handoff_receipt_audit_persisted_by_report_route": false,
            "context_handoff_receipt_audit_accepted_by_report_route": false,
            "context_handoff_receipt_audit_filesystem_written_by_report_route": false,
            "context_handoff_receipt_audit_ledger_recorded_by_report_route": false,
            "context_handoff_receipt_audit_promotes_activation_authority": false,
            "context_attachment_performed_by_report_route": false,
            "context_injection_allowed_by_lane": false,
            "context_injection_performed_by_report_route": false,
            "kg_live_write_lane_enabled": false,
            "kg_live_write_allowed_by_lane": false,
            "kg_live_write_performed_by_report_route": false,
            "provider_model_invocation_lane_enabled": false,
            "provider_model_invocation_allowed_by_lane": false,
            "channel_delivery_lane_enabled": false,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 8,
            "enablement_lane_count": 11,
            "ready_enablement_lane_count": 11,
            "blocked_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_BLOCKED_ACTIONS,
            "allowed_next_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_NEXT_ACTIONS,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        side_effects.insert(
            "context_handoff_receipt_audit_rendered".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "context_handoff_receipt_audit_recorded".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "context_handoff_receipt_audit_persisted".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "context_handoff_receipt_audit_accepted".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "context_handoff_receipt_audit_ledger_recorded".to_string(),
            serde_json::json!(false),
        );
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_precondition_lane_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let context_handoff_receipt_audit_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_receipt_audit_lane_report();
    let receipt_bool = |key: &str| {
        context_handoff_receipt_audit_lane
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let receipt_status = context_handoff_receipt_audit_lane
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_context_handoff_receipt_audit_lane_ready = receipt_status.as_str() == "ready"
        && receipt_bool("context_handoff_receipt_audit_lane_enabled")
        && receipt_bool("context_handoff_receipt_audit_allowed_by_lane")
        && receipt_bool("context_handoff_receipt_audit_requires_explicit_command")
        && receipt_bool("context_handoff_receipt_audit_requires_context_handoff_acceptance")
        && receipt_bool("context_handoff_receipt_audit_redaction_required")
        && receipt_bool("context_handoff_receipt_audit_redaction_proof_required")
        && receipt_bool("context_handoff_receipt_audit_scope_binding_required")
        && receipt_bool("context_handoff_receipt_audit_operator_identity_binding_required")
        && receipt_bool("context_handoff_receipt_audit_hash_binding_required")
        && !receipt_bool("context_handoff_receipt_audit_raw_context_allowed")
        && !receipt_bool("context_handoff_receipt_audit_rendered_by_report_route")
        && !receipt_bool("context_handoff_receipt_audit_recorded_by_report_route")
        && !receipt_bool("context_handoff_receipt_audit_persisted_by_report_route")
        && !receipt_bool("context_handoff_receipt_audit_accepted_by_report_route")
        && !receipt_bool("context_handoff_receipt_audit_filesystem_written_by_report_route")
        && !receipt_bool("context_handoff_receipt_audit_ledger_recorded_by_report_route")
        && !receipt_bool("context_handoff_receipt_audit_promotes_activation_authority")
        && !receipt_bool("context_attachment_performed_by_report_route")
        && !receipt_bool("context_injection_allowed_by_lane")
        && !receipt_bool("context_injection_performed_by_report_route")
        && !receipt_bool("kg_live_write_lane_enabled")
        && !receipt_bool("provider_model_invocation_lane_enabled")
        && !receipt_bool("channel_delivery_lane_enabled");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_context_handoff_receipt_audit_lane_ready;

    let mut report = context_handoff_receipt_audit_lane;
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane --json",
            "compatibility_mode": "native_full_enablement_operator_approved_bounded_provider_router_injection_precondition_lane_status",
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_ENDPOINT,
            "context_handoff_receipt_audit_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_ENDPOINT,
            "context_handoff_receipt_audit_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_GATE.md",
            "bounded_provider_router_injection_precondition_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_GATE.md",
            "source_context_handoff_receipt_audit_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane-gate.sh",
            "source_bounded_provider_router_injection_precondition_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane-gate.sh",
            "context_handoff_receipt_audit_lane_status": receipt_status,
            "source_context_handoff_receipt_audit_lane_ready": source_context_handoff_receipt_audit_lane_ready,
            "operator_authorization_source": "telegram_direct_operator_highest_authorization_2026_06_13_08_01_56_asia_shanghai",
            "operator_authorization_scope": "bounded_provider_router_injection_precondition_lane_no_report_context_inject_prompt_mutation_record_persist_accept_no_kg_live_write_provider_model_channel_or_public_release",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "bounded_provider_router_injection_precondition_lane_enabled": true,
            "bounded_provider_router_injection_precondition_allowed_by_lane": true,
            "bounded_provider_router_injection_precondition_requires_explicit_command": true,
            "bounded_provider_router_injection_precondition_requires_context_handoff_receipt_audit": true,
            "bounded_provider_router_injection_precondition_redaction_required": true,
            "bounded_provider_router_injection_precondition_redaction_proof_required": true,
            "bounded_provider_router_injection_precondition_scope_binding_required": true,
            "bounded_provider_router_injection_precondition_operator_identity_binding_required": true,
            "bounded_provider_router_injection_precondition_hash_binding_required": true,
            "bounded_provider_router_injection_precondition_provider_router_target_binding_required": true,
            "bounded_provider_router_injection_precondition_budget_binding_required": true,
            "bounded_provider_router_injection_precondition_dry_run_only": true,
            "bounded_provider_router_injection_precondition_raw_context_allowed": false,
            "bounded_provider_router_injection_precondition_rendered_by_report_route": false,
            "bounded_provider_router_injection_precondition_recorded_by_report_route": false,
            "bounded_provider_router_injection_precondition_persisted_by_report_route": false,
            "bounded_provider_router_injection_precondition_accepted_by_report_route": false,
            "bounded_provider_router_injection_precondition_filesystem_written_by_report_route": false,
            "bounded_provider_router_injection_precondition_ledger_recorded_by_report_route": false,
            "bounded_provider_router_injection_precondition_promotes_activation_authority": false,
            "provider_router_prompt_mutated_by_report_route": false,
            "provider_router_context_packet_materialized_by_report_route": false,
            "context_attachment_performed_by_report_route": false,
            "context_injection_allowed_by_lane": false,
            "context_injection_performed_by_report_route": false,
            "kg_live_write_lane_enabled": false,
            "kg_live_write_allowed_by_lane": false,
            "kg_live_write_performed_by_report_route": false,
            "provider_model_invocation_lane_enabled": false,
            "provider_model_invocation_allowed_by_lane": false,
            "channel_delivery_lane_enabled": false,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 9,
            "enablement_lane_count": 12,
            "ready_enablement_lane_count": 12,
            "blocked_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_BLOCKED_ACTIONS,
            "allowed_next_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_NEXT_ACTIONS,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        side_effects.insert(
            "bounded_provider_router_injection_precondition_rendered".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_precondition_recorded".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_precondition_persisted".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_precondition_accepted".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_precondition_ledger_recorded".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "provider_router_prompt_mutated".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "provider_router_context_packet_materialized".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert("context_injected".to_string(), serde_json::json!(false));
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_lane_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let bounded_provider_router_injection_precondition_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_precondition_lane_report();
    let precondition_bool = |key: &str| {
        bounded_provider_router_injection_precondition_lane
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let precondition_u64 = |key: &str| {
        bounded_provider_router_injection_precondition_lane
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let precondition_status = bounded_provider_router_injection_precondition_lane
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_bounded_provider_router_injection_precondition_lane_ready = precondition_status
        .as_str()
        == "ready"
        && precondition_bool("bounded_provider_router_injection_precondition_lane_enabled")
        && precondition_bool("bounded_provider_router_injection_precondition_allowed_by_lane")
        && precondition_bool(
            "bounded_provider_router_injection_precondition_requires_explicit_command",
        )
        && precondition_bool(
            "bounded_provider_router_injection_precondition_requires_context_handoff_receipt_audit",
        )
        && precondition_bool("bounded_provider_router_injection_precondition_redaction_required")
        && precondition_bool(
            "bounded_provider_router_injection_precondition_redaction_proof_required",
        )
        && precondition_bool(
            "bounded_provider_router_injection_precondition_scope_binding_required",
        )
        && precondition_bool(
            "bounded_provider_router_injection_precondition_operator_identity_binding_required",
        )
        && precondition_bool(
            "bounded_provider_router_injection_precondition_hash_binding_required",
        )
        && precondition_bool(
            "bounded_provider_router_injection_precondition_provider_router_target_binding_required",
        )
        && precondition_bool(
            "bounded_provider_router_injection_precondition_budget_binding_required",
        )
        && precondition_bool("bounded_provider_router_injection_precondition_dry_run_only")
        && !precondition_bool("bounded_provider_router_injection_precondition_raw_context_allowed")
        && !precondition_bool(
            "bounded_provider_router_injection_precondition_rendered_by_report_route",
        )
        && !precondition_bool(
            "bounded_provider_router_injection_precondition_recorded_by_report_route",
        )
        && !precondition_bool(
            "bounded_provider_router_injection_precondition_persisted_by_report_route",
        )
        && !precondition_bool(
            "bounded_provider_router_injection_precondition_accepted_by_report_route",
        )
        && !precondition_bool(
            "bounded_provider_router_injection_precondition_filesystem_written_by_report_route",
        )
        && !precondition_bool(
            "bounded_provider_router_injection_precondition_ledger_recorded_by_report_route",
        )
        && !precondition_bool(
            "bounded_provider_router_injection_precondition_promotes_activation_authority",
        )
        && !precondition_bool("provider_router_prompt_mutated_by_report_route")
        && !precondition_bool("provider_router_context_packet_materialized_by_report_route")
        && !precondition_bool("context_attachment_performed_by_report_route")
        && !precondition_bool("context_injection_allowed_by_lane")
        && !precondition_bool("context_injection_performed_by_report_route")
        && !precondition_bool("kg_live_write_lane_enabled")
        && !precondition_bool("provider_model_invocation_lane_enabled")
        && !precondition_bool("channel_delivery_lane_enabled")
        && precondition_u64("live_mutation_enabled_count") == 1
        && precondition_u64("current_live_enabled_lane_count") == 9
        && precondition_u64("enablement_lane_count") == 12
        && precondition_u64("ready_enablement_lane_count") == 12;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_bounded_provider_router_injection_precondition_lane_ready;

    let mut report = bounded_provider_router_injection_precondition_lane;
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-lane --json",
            "compatibility_mode": "native_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_lane_status",
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_ENDPOINT,
            "bounded_provider_router_injection_precondition_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_ENDPOINT,
            "bounded_provider_router_injection_precondition_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_GATE.md",
            "bounded_provider_router_injection_dry_run_envelope_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_GATE.md",
            "source_bounded_provider_router_injection_precondition_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane-gate.sh",
            "source_bounded_provider_router_injection_dry_run_envelope_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-lane-gate.sh",
            "bounded_provider_router_injection_precondition_lane_status": precondition_status,
            "source_bounded_provider_router_injection_precondition_lane_ready": source_bounded_provider_router_injection_precondition_lane_ready,
            "operator_authorization_source": "telegram_direct_operator_highest_authorization_2026_06_13_11_12_08_asia_shanghai",
            "operator_authorization_scope": "bounded_provider_router_injection_dry_run_envelope_lane_no_report_envelope_construct_render_record_persist_accept_execute_no_context_inject_prompt_mutation_kg_live_write_provider_model_channel_or_public_release",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "bounded_provider_router_injection_dry_run_envelope_lane_enabled": true,
            "bounded_provider_router_injection_dry_run_envelope_allowed_by_lane": true,
            "bounded_provider_router_injection_dry_run_envelope_requires_explicit_command": true,
            "bounded_provider_router_injection_dry_run_envelope_requires_bounded_provider_router_injection_precondition": true,
            "bounded_provider_router_injection_dry_run_envelope_redaction_required": true,
            "bounded_provider_router_injection_dry_run_envelope_redaction_proof_required": true,
            "bounded_provider_router_injection_dry_run_envelope_scope_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_operator_identity_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_hash_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_provider_router_target_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_budget_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_shape_locked": true,
            "bounded_provider_router_injection_dry_run_envelope_dry_run_only": true,
            "bounded_provider_router_injection_dry_run_envelope_raw_context_allowed": false,
            "bounded_provider_router_injection_dry_run_envelope_constructed_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_rendered_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_recorded_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_persisted_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_accepted_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_executed_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_filesystem_written_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_ledger_recorded_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_promotes_activation_authority": false,
            "provider_router_injection_execution_allowed_by_lane": false,
            "provider_router_prompt_mutated_by_report_route": false,
            "provider_router_context_packet_materialized_by_report_route": false,
            "context_attachment_performed_by_report_route": false,
            "context_injection_allowed_by_lane": false,
            "context_injection_performed_by_report_route": false,
            "kg_live_write_lane_enabled": false,
            "kg_live_write_allowed_by_lane": false,
            "kg_live_write_performed_by_report_route": false,
            "provider_model_invocation_lane_enabled": false,
            "provider_model_invocation_allowed_by_lane": false,
            "channel_delivery_lane_enabled": false,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 10,
            "enablement_lane_count": 13,
            "ready_enablement_lane_count": 13,
            "blocked_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_BLOCKED_ACTIONS,
            "allowed_next_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_NEXT_ACTIONS,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_constructed".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_rendered".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_recorded".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_persisted".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_accepted".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_executed".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_filesystem_written".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_ledger_recorded".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "provider_router_prompt_mutated".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "provider_router_context_packet_materialized".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert("context_injected".to_string(), serde_json::json!(false));
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let dry_run_envelope_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_lane_report();
    let envelope_bool = |key: &str| {
        dry_run_envelope_lane
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let envelope_u64 = |key: &str| {
        dry_run_envelope_lane
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let envelope_status = dry_run_envelope_lane
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_bounded_provider_router_injection_dry_run_envelope_lane_ready = envelope_status
        .as_str()
        == "ready"
        && envelope_bool("bounded_provider_router_injection_dry_run_envelope_lane_enabled")
        && envelope_bool("bounded_provider_router_injection_dry_run_envelope_allowed_by_lane")
        && envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_requires_explicit_command",
        )
        && envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_requires_bounded_provider_router_injection_precondition",
        )
        && envelope_bool("bounded_provider_router_injection_dry_run_envelope_redaction_required")
        && envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_redaction_proof_required",
        )
        && envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_scope_binding_required",
        )
        && envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_operator_identity_binding_required",
        )
        && envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_hash_binding_required",
        )
        && envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_provider_router_target_binding_required",
        )
        && envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_budget_binding_required",
        )
        && envelope_bool("bounded_provider_router_injection_dry_run_envelope_shape_locked")
        && envelope_bool("bounded_provider_router_injection_dry_run_envelope_dry_run_only")
        && !envelope_bool("bounded_provider_router_injection_dry_run_envelope_raw_context_allowed")
        && !envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_constructed_by_report_route",
        )
        && !envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_rendered_by_report_route",
        )
        && !envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_recorded_by_report_route",
        )
        && !envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_persisted_by_report_route",
        )
        && !envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_accepted_by_report_route",
        )
        && !envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_executed_by_report_route",
        )
        && !envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_filesystem_written_by_report_route",
        )
        && !envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_ledger_recorded_by_report_route",
        )
        && !envelope_bool(
            "bounded_provider_router_injection_dry_run_envelope_promotes_activation_authority",
        )
        && !envelope_bool("provider_router_injection_execution_allowed_by_lane")
        && !envelope_bool("provider_router_prompt_mutated_by_report_route")
        && !envelope_bool("provider_router_context_packet_materialized_by_report_route")
        && !envelope_bool("context_attachment_performed_by_report_route")
        && !envelope_bool("context_injection_allowed_by_lane")
        && !envelope_bool("context_injection_performed_by_report_route")
        && !envelope_bool("kg_live_write_lane_enabled")
        && !envelope_bool("provider_model_invocation_lane_enabled")
        && !envelope_bool("channel_delivery_lane_enabled")
        && envelope_u64("live_mutation_enabled_count") == 1
        && envelope_u64("current_live_enabled_lane_count") == 10
        && envelope_u64("enablement_lane_count") == 13
        && envelope_u64("ready_enablement_lane_count") == 13;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_bounded_provider_router_injection_dry_run_envelope_lane_ready;

    let mut report = dry_run_envelope_lane;
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-lane --json",
            "compatibility_mode": "native_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_status",
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT,
            "bounded_provider_router_injection_dry_run_envelope_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_ENDPOINT,
            "bounded_provider_router_injection_dry_run_envelope_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_GATE.md",
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_GATE.md",
            "source_bounded_provider_router_injection_dry_run_envelope_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-lane-gate.sh",
            "source_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-lane-gate.sh",
            "bounded_provider_router_injection_dry_run_envelope_lane_status": envelope_status,
            "source_bounded_provider_router_injection_dry_run_envelope_lane_ready": source_bounded_provider_router_injection_dry_run_envelope_lane_ready,
            "operator_authorization_source": "telegram_direct_operator_highest_authorization_2026_06_13_11_12_08_asia_shanghai",
            "operator_authorization_scope": "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_no_report_receipt_render_record_persist_accept_no_envelope_construct_execute_no_context_inject_prompt_mutation_kg_live_write_provider_model_channel_or_public_release",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_enabled": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_allowed_by_lane": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_requires_explicit_command": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_requires_bounded_provider_router_injection_dry_run_envelope": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_redaction_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_redaction_proof_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_scope_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_operator_identity_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_hash_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_provider_router_target_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_budget_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_envelope_shape_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_raw_context_allowed": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_rendered_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_recorded_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_persisted_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_accepted_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_filesystem_written_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_ledger_recorded_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_promotes_activation_authority": false,
            "provider_router_injection_execution_allowed_by_lane": false,
            "provider_router_prompt_mutated_by_report_route": false,
            "provider_router_context_packet_materialized_by_report_route": false,
            "context_attachment_performed_by_report_route": false,
            "context_injection_allowed_by_lane": false,
            "context_injection_performed_by_report_route": false,
            "kg_live_write_lane_enabled": false,
            "kg_live_write_allowed_by_lane": false,
            "kg_live_write_performed_by_report_route": false,
            "provider_model_invocation_lane_enabled": false,
            "provider_model_invocation_allowed_by_lane": false,
            "channel_delivery_lane_enabled": false,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 11,
            "enablement_lane_count": 14,
            "ready_enablement_lane_count": 14,
            "blocked_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_BLOCKED_ACTIONS,
            "allowed_next_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_NEXT_ACTIONS,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_rendered"
                .to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_recorded"
                .to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_persisted"
                .to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_accepted"
                .to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_filesystem_written".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_ledger_recorded".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "provider_router_prompt_mutated".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "provider_router_context_packet_materialized".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert("context_injected".to_string(), serde_json::json!(false));
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let readback_audit_receipt_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_report();
    let receipt_bool = |key: &str| {
        readback_audit_receipt_lane
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let receipt_u64 = |key: &str| {
        readback_audit_receipt_lane
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let receipt_status = readback_audit_receipt_lane
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_ready =
        receipt_status.as_str() == "ready"
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_enabled",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_allowed_by_lane",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_requires_explicit_command",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_requires_bounded_provider_router_injection_dry_run_envelope",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_redaction_required",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_redaction_proof_required",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_scope_binding_required",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_operator_identity_binding_required",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_hash_binding_required",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_provider_router_target_binding_required",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_budget_binding_required",
            )
            && receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_envelope_shape_binding_required",
            )
            && !receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_raw_context_allowed",
            )
            && !receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_rendered_by_report_route",
            )
            && !receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_recorded_by_report_route",
            )
            && !receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_persisted_by_report_route",
            )
            && !receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_accepted_by_report_route",
            )
            && !receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_filesystem_written_by_report_route",
            )
            && !receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_ledger_recorded_by_report_route",
            )
            && !receipt_bool(
                "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_promotes_activation_authority",
            )
            && !receipt_bool("provider_router_injection_execution_allowed_by_lane")
            && !receipt_bool("provider_router_prompt_mutated_by_report_route")
            && !receipt_bool("provider_router_context_packet_materialized_by_report_route")
            && !receipt_bool("context_attachment_performed_by_report_route")
            && !receipt_bool("context_injection_allowed_by_lane")
            && !receipt_bool("context_injection_performed_by_report_route")
            && !receipt_bool("kg_live_write_lane_enabled")
            && !receipt_bool("provider_model_invocation_lane_enabled")
            && !receipt_bool("channel_delivery_lane_enabled")
            && receipt_u64("live_mutation_enabled_count") == 1
            && receipt_u64("current_live_enabled_lane_count") == 11
            && receipt_u64("enablement_lane_count") == 14
            && receipt_u64("ready_enablement_lane_count") == 14;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_ready;

    let mut report = readback_audit_receipt_lane;
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-acknowledgement-no-op-handoff-lane --json",
            "compatibility_mode": "native_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_status",
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_ENDPOINT,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_GATE.md",
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_GATE.md",
            "source_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-lane-gate.sh",
            "source_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-acknowledgement-no-op-handoff-lane-gate.sh",
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_status": receipt_status,
            "source_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_ready": source_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_ready,
            "operator_authorization_source": "telegram_direct_operator_highest_authorization_2026_06_13_14_01_56_asia_shanghai",
            "operator_authorization_scope": "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_no_report_acknowledge_handoff_record_persist_accept_no_envelope_construct_execute_no_context_inject_prompt_mutation_kg_live_write_provider_model_channel_or_public_release",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_enabled": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_allowed_by_lane": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_explicit_command": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_readback_audit_receipt_lane": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_acknowledgement_shape_binding": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_no_op_handoff_boundary": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_redaction_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_redaction_proof_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_scope_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_operator_identity_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_hash_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_provider_router_target_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_budget_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_envelope_shape_binding_required": true,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_raw_context_allowed": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_acknowledged_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_handoff_performed_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_recorded_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_persisted_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_accepted_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_filesystem_written_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_ledger_recorded_by_report_route": false,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_promotes_activation_authority": false,
            "provider_router_injection_execution_allowed_by_lane": false,
            "provider_router_prompt_mutated_by_report_route": false,
            "provider_router_context_packet_materialized_by_report_route": false,
            "context_attachment_performed_by_report_route": false,
            "context_injection_allowed_by_lane": false,
            "context_injection_performed_by_report_route": false,
            "kg_live_write_lane_enabled": false,
            "kg_live_write_allowed_by_lane": false,
            "kg_live_write_performed_by_report_route": false,
            "provider_model_invocation_lane_enabled": false,
            "provider_model_invocation_allowed_by_lane": false,
            "channel_delivery_lane_enabled": false,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 12,
            "enablement_lane_count": 15,
            "ready_enablement_lane_count": 15,
            "blocked_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_BLOCKED_ACTIONS,
            "allowed_next_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_NEXT_ACTIONS,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledged"
                .to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_no_op_handoff_performed".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_recorded".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_persisted".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_accepted".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_filesystem_written".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_ledger_recorded".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "provider_router_prompt_mutated".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert(
            "provider_router_context_packet_materialized".to_string(),
            serde_json::json!(false),
        );
        side_effects.insert("context_injected".to_string(), serde_json::json!(false));
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_acknowledgement_no_op_handoff_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_report();
    let source_bool = |key: &str| {
        source_acknowledgement_no_op_handoff_lane
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_acknowledgement_no_op_handoff_lane
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_acknowledgement_no_op_handoff_lane
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_acknowledgement_no_op_handoff_lane_ready = source_status == "ready"
        && source_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_enabled",
        )
        && source_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_allowed_by_lane",
        )
        && source_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_explicit_command",
        )
        && source_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_readback_audit_receipt_lane",
        )
        && !source_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_acknowledged_by_report_route",
        )
        && !source_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_handoff_performed_by_report_route",
        )
        && !source_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_recorded_by_report_route",
        )
        && !source_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_persisted_by_report_route",
        )
        && !source_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_accepted_by_report_route",
        )
        && !source_bool("provider_router_injection_execution_allowed_by_lane")
        && !source_bool("provider_router_prompt_mutated_by_report_route")
        && !source_bool("provider_router_context_packet_materialized_by_report_route")
        && !source_bool("context_attachment_performed_by_report_route")
        && !source_bool("context_injection_allowed_by_lane")
        && !source_bool("context_injection_performed_by_report_route")
        && !source_bool("kg_live_write_lane_enabled")
        && !source_bool("provider_model_invocation_lane_enabled")
        && !source_bool("channel_delivery_lane_enabled")
        && source_u64("live_mutation_enabled_count") == 1
        && source_u64("current_live_enabled_lane_count") == 12
        && source_u64("enablement_lane_count") == 15
        && source_u64("ready_enablement_lane_count") == 15;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_acknowledgement_no_op_handoff_lane_ready;

    let mut report = source_acknowledgement_no_op_handoff_lane;
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_ENDPOINT,
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_ENDPOINT,
            "operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_GATE.md",
            "operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_ROUTE_GATE.md",
            "source_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-acknowledgement-no-op-handoff-lane-gate.sh",
            "source_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt-gate.sh",
            "source_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_acknowledgement_no_op_handoff_lane_status": source_status,
            "source_acknowledgement_no_op_handoff_lane_ready": source_acknowledgement_no_op_handoff_lane_ready,
            "operator_authorization_source": "telegram_direct_operator_highest_authorization_2026_06_13_16_27_10_asia_shanghai",
            "operator_authorization_scope": "operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_no_budget_accept_consume_no_dispatch_execute_no_receipt_record_persist_accept_no_payload_materialization_context_inject_memory_kg_write_provider_model_credential_channel_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_enabled": true,
            "operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_ready": true,
            "operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status": "blocked",
            "dispatch_dry_run_noop_receipt_mode": "native_route_single_budget_dispatch_dry_run_noop_receipt_no_accept_no_consume_no_dispatch_no_execute_no_persist_no_live",
            "dispatch_dry_run_noop_receipt_decision": "single_budget_dispatch_dry_run_and_noop_receipt_shapes_are_reported_without_accepting_consuming_dispatching_executing_recording_persisting_or_materializing",
            "source_receipt_hash_preview_count": 2,
            "source_receipt_hash_accepted_count": 0,
            "source_receipt_recorded_count": 0,
            "source_receipt_persisted_count": 0,
            "source_receipt_delivered_count": 0,
            "source_receipt_accepted_count": 0,
            "source_receipt_materialized_count": 0,
            "source_acceptance_skeleton_declared_count": 2,
            "source_acceptance_skeleton_operator_input_required_count": 2,
            "source_acceptance_skeleton_operator_input_supplied_count": 0,
            "source_acceptance_skeleton_accepted_count": 0,
            "source_controlled_request_dispatch_budget_declared": 1,
            "source_controlled_request_dispatch_budget_accepted": false,
            "source_controlled_request_dispatch_budget_consumed": 0,
            "source_controlled_request_dispatch_budget_remaining": 0,
            "dispatch_dry_run_noop_receipt_count": 1,
            "dispatch_dry_run_shape_declared_count": 1,
            "dispatch_intent_shape_declared_count": 1,
            "single_budget_shape_declared_count": 1,
            "single_budget_declared": 1,
            "single_budget_accepted": false,
            "single_budget_consumed": 0,
            "single_budget_remaining": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_authority_accepted_count": 0,
            "dispatch_preconditions_satisfied_count": 0,
            "controlled_request_dispatch_ready_count": 0,
            "controlled_request_dispatch_allowed_count": 0,
            "controlled_request_dispatched_count": 0,
            "controlled_request_execution_allowed_count": 0,
            "controlled_request_executed_count": 0,
            "noop_receipt_shape_declared_count": 1,
            "noop_receipt_hash_shape_declared_count": 1,
            "noop_receipt_hash_bound_to_payload_preview_count": 1,
            "noop_receipt_hash_bound_to_receipt_hash_preview_count": 1,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "noop_receipt_recorded_count": 0,
            "noop_receipt_persisted_count": 0,
            "noop_receipt_delivered_count": 0,
            "noop_receipt_accepted_count": 0,
            "noop_receipt_materialized_count": 0,
            "readback_receipt_hash_preview_accepted_count": 0,
            "audit_receipt_hash_preview_accepted_count": 0,
            "acceptance_skeleton_accepted_count": 0,
            "request_payload_materialized_count": 0,
            "request_payload_file_written_count": 0,
            "raw_payload_inspected_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
            "dispatch_dry_run_noop_receipt_negative_fixture_count": 7,
            "dispatch_dry_run_noop_receipt_blocked_negative_fixture_count": 7,
            "dispatch_dry_run_noop_receipt_allowed_negative_fixture_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_dispatch_dry_run_noop_receipt_count": 16,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 13,
            "enablement_lane_count": 16,
            "ready_enablement_lane_count": 16,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "blocked_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_BLOCKED_ACTIONS,
            "allowed_next_actions": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_NEXT_ACTIONS,
            "denied_by_dispatch_dry_run_noop_receipt": [
                "single_budget_dispatch_dry_run_not_operator_approval",
                "dispatch_budget_acceptance_denied",
                "dispatch_budget_consumption_denied",
                "dispatch_execution_denied",
                "noop_receipt_recording_denied",
                "noop_receipt_persistence_denied",
                "noop_receipt_delivery_denied",
                "noop_receipt_acceptance_denied",
                "request_payload_materialization_denied",
                "context_attachment_denied",
                "provider_model_invocation_denied",
                "memory_write_denied",
                "external_kg_read_denied",
                "live_kg_write_denied",
                "credential_secret_read_denied",
                "channel_delivery_denied"
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "single_budget_accepted",
            "single_budget_consumed",
            "dispatch_performed",
            "execution_performed",
            "noop_receipt_recorded",
            "noop_receipt_persisted",
            "noop_receipt_delivered",
            "noop_receipt_accepted",
            "noop_receipt_materialized",
            "request_payload_materialized",
            "request_payload_file_written",
            "raw_payload_inspected",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "service_restarted",
            "active_binary_mutated",
            "install_performed",
            "upstream_fetch_performed",
            "upstream_merge_performed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_single_budget =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_report();
    let source_bool = |key: &str| {
        source_single_budget
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_single_budget
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_single_budget
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_single_budget_ready = source_status == "ready"
        && source_bool(
            "operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_enabled",
        )
        && source_bool(
            "operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_ready",
        )
        && source_u64("single_budget_declared") == 1
        && !source_bool("single_budget_accepted")
        && source_u64("single_budget_consumed") == 0
        && source_u64("single_budget_remaining") == 0
        && source_u64("controlled_request_dispatched_count") == 0
        && source_u64("controlled_request_executed_count") == 0
        && source_u64("noop_receipt_persisted_count") == 0
        && source_u64("noop_receipt_accepted_count") == 0
        && source_u64("request_payload_materialized_count") == 0
        && source_u64("request_payload_file_written_count") == 0
        && source_u64("context_injection_performed_count") == 0
        && source_u64("provider_invoked_count") == 0
        && source_u64("model_invoked_count") == 0
        && source_u64("memory_store_write_performed_count") == 0
        && source_u64("live_kg_write_performed_count") == 0
        && source_u64("credential_read_count") == 0
        && source_u64("secret_file_read_count") == 0
        && source_u64("channel_send_performed_count") == 0
        && !source_bool("canary_harness_armed")
        && !source_bool("canary_live_enabled")
        && source_u64("current_live_enabled_lane_count") == 13
        && source_u64("enablement_lane_count") == 16
        && source_u64("ready_enablement_lane_count") == 16;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_single_budget_ready;

    let mut report = source_single_budget;
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_READBACK_INDEX_NO_PERSISTENCE_ENDPOINT,
            "source_single_budget_dispatch_dry_run_noop_receipt_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_READBACK_INDEX_NO_PERSISTENCE_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_READBACK_INDEX_NO_PERSISTENCE_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_single_budget_dispatch_dry_run_noop_receipt_route_status": source_status,
            "source_single_budget_dispatch_dry_run_noop_receipt_route_ready": source_single_budget_ready,
            "operator_authorization_source": "telegram_direct_operator_highest_authorization_2026_06_13_16_27_10_asia_shanghai",
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_readback_index_no_review_accept_no_index_record_persist_materialize_deliver_no_dispatch_execute_no_context_inject_memory_kg_write_provider_model_credential_channel_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_ready": true,
            "operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status": "blocked",
            "operator_review_readback_index_mode": "native_route_operator_review_readback_index_no_review_supplied_no_persistence_no_delivery_no_dispatch_no_live",
            "operator_review_readback_index_decision": "operator_review_and_readback_index_shapes_are_reported_without_accepting_recording_persisting_materializing_delivering_dispatching_executing_or_authorizing_live_mutation",
            "source_single_budget_dispatch_dry_run_noop_receipt_status": "blocked",
            "source_dispatch_dry_run_noop_receipt_count": 1,
            "source_single_budget_declared": 1,
            "source_single_budget_accepted": false,
            "source_single_budget_consumed": 0,
            "source_single_budget_remaining": 0,
            "source_controlled_request_dispatched_count": 0,
            "source_controlled_request_executed_count": 0,
            "source_noop_receipt_persisted_count": 0,
            "source_noop_receipt_accepted_count": 0,
            "operator_review_readback_index_section_count": 8,
            "operator_review_section_declared_count": 8,
            "operator_review_required_count": 8,
            "operator_review_supplied_count": 0,
            "operator_review_recorded_count": 0,
            "operator_review_persisted_count": 0,
            "operator_review_delivered_count": 0,
            "operator_review_accepted_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "readback_index_declared_count": 1,
            "readback_index_bound_to_payload_hash_count": 1,
            "readback_index_bound_to_readback_receipt_hash_count": 1,
            "readback_index_bound_to_audit_receipt_hash_count": 1,
            "readback_index_bound_to_noop_receipt_hash_count": 1,
            "readback_index_recorded_count": 0,
            "readback_index_persisted_count": 0,
            "readback_index_materialized_count": 0,
            "readback_index_filesystem_written_count": 0,
            "operator_review_index_recorded": false,
            "operator_review_index_persisted": false,
            "operator_review_index_materialized": false,
            "operator_review_index_filesystem_written": false,
            "operator_review_index_channel_delivered": false,
            "operator_review_index_external_sent": false,
            "operator_review_index_telegram_sent": false,
            "review_authorizes_dispatch_count": 0,
            "review_authorizes_execution_count": 0,
            "review_authorizes_live_count": 0,
            "dispatch_allowed_count": 0,
            "dispatch_performed_count": 0,
            "execution_allowed_count": 0,
            "execution_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
            "operator_review_readback_index_negative_fixture_count": 8,
            "operator_review_readback_index_blocked_negative_fixture_count": 8,
            "operator_review_readback_index_allowed_negative_fixture_count": 0,
            "denied_by_operator_review_readback_index_count": 17,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 14,
            "enablement_lane_count": 17,
            "ready_enablement_lane_count": 17,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "blocked_actions": [
                "accept_operator_canary_controlled_request_harness_operator_review_from_report_route",
                "record_operator_canary_controlled_request_harness_operator_review_from_report_route",
                "persist_operator_canary_controlled_request_harness_operator_review_from_report_route",
                "deliver_operator_canary_controlled_request_harness_operator_review_from_report_route",
                "accept_operator_canary_controlled_request_harness_operator_review_readback_index",
                "record_operator_canary_controlled_request_harness_operator_review_readback_index",
                "persist_operator_canary_controlled_request_harness_operator_review_readback_index",
                "materialize_operator_canary_controlled_request_harness_operator_review_readback_index",
                "deliver_operator_canary_controlled_request_harness_operator_review_readback_index",
                "dispatch_operator_canary_controlled_request_from_operator_review",
                "execute_operator_canary_controlled_request_from_operator_review",
                "attach_or_inject_context_from_operator_review",
                "write_memory_or_live_kg_from_operator_review",
                "invoke_provider_or_model_from_operator_review",
                "read_credential_or_secret_from_operator_review",
                "telegram_or_channel_delivery_from_operator_review",
                "release_or_public_claim_from_operator_review"
            ],
            "allowed_next_actions": [
                "run operator canary controlled-request harness operator-review/readback index no-persistence route gate against the single-budget route",
                "install canary operator-review/readback index no-persistence route through controlled live catch-up after full preflight",
                "slice operator-review acknowledgement non-acceptance while keeping review acceptance, dispatch, execution, persistence, context injection, Memory/KG writes, provider/model invocation, credential reads, channel delivery, and public release disabled"
            ],
            "denied_by_operator_review_readback_index": [
                "operator_review_acceptance_denied",
                "operator_review_recording_denied",
                "operator_review_persistence_denied",
                "operator_review_delivery_denied",
                "readback_index_recording_denied",
                "readback_index_persistence_denied",
                "readback_index_materialization_denied",
                "readback_index_delivery_denied",
                "review_dispatch_authority_denied",
                "review_execution_authority_denied",
                "review_live_authority_denied",
                "context_injection_denied",
                "provider_model_invocation_denied",
                "memory_live_kg_write_denied",
                "credential_secret_read_denied",
                "channel_delivery_denied",
                "release_public_claim_denied"
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "operator_review_recorded",
            "operator_review_persisted",
            "operator_review_delivered",
            "operator_review_accepted",
            "operator_review_index_recorded",
            "operator_review_index_persisted",
            "operator_review_index_materialized",
            "operator_review_index_filesystem_written",
            "readback_index_persisted",
            "readback_index_materialized",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "service_restarted",
            "active_binary_mutated",
            "install_performed",
            "upstream_fetch_performed",
            "upstream_merge_performed",
            "public_release_claimed",
            "public_ga_claimed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_operator_review =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_report();
    let source_bool = |key: &str| {
        source_operator_review
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_operator_review
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_operator_review
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_operator_review_ready = source_status == "ready"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_enabled",
        )
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_ready",
        )
        && source_u64("operator_review_readback_index_section_count") == 8
        && source_u64("operator_review_required_count") == 8
        && source_u64("operator_review_supplied_count") == 0
        && source_u64("operator_review_recorded_count") == 0
        && source_u64("operator_review_persisted_count") == 0
        && source_u64("operator_review_delivered_count") == 0
        && source_u64("operator_review_accepted_count") == 0
        && source_u64("readback_index_declared_count") == 1
        && source_u64("readback_index_recorded_count") == 0
        && source_u64("readback_index_persisted_count") == 0
        && source_u64("readback_index_materialized_count") == 0
        && source_u64("readback_index_filesystem_written_count") == 0
        && source_u64("review_authorizes_dispatch_count") == 0
        && source_u64("review_authorizes_execution_count") == 0
        && source_u64("review_authorizes_live_count") == 0
        && source_u64("dispatch_performed_count") == 0
        && source_u64("execution_performed_count") == 0
        && source_u64("context_injection_performed_count") == 0
        && source_u64("provider_invoked_count") == 0
        && source_u64("model_invoked_count") == 0
        && source_u64("memory_store_write_performed_count") == 0
        && source_u64("external_kg_adapter_read_performed_count") == 0
        && source_u64("live_kg_write_performed_count") == 0
        && source_u64("credential_read_count") == 0
        && source_u64("secret_file_read_count") == 0
        && source_u64("channel_send_performed_count") == 0
        && !source_bool("canary_harness_armed")
        && !source_bool("canary_live_enabled")
        && source_u64("current_live_enabled_lane_count") == 14
        && source_u64("enablement_lane_count") == 17
        && source_u64("ready_enablement_lane_count") == 17;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_operator_review_ready;
    let source_operator_review_required_count = source_u64("operator_review_required_count");
    let source_operator_review_accepted_count = source_u64("operator_review_accepted_count");
    let source_readback_index_declared_count = source_u64("readback_index_declared_count");
    let source_readback_index_persisted_count = source_u64("readback_index_persisted_count");
    let source_review_authorizes_dispatch_count = source_u64("review_authorizes_dispatch_count");
    let source_review_authorizes_execution_count = source_u64("review_authorizes_execution_count");
    let source_review_authorizes_live_count = source_u64("review_authorizes_live_count");

    let acknowledgement_fixtures = serde_json::json!([
        {
            "fixture_id": "seen-review-index-without-approval",
            "fixture_kind": "seen_review_index_without_approval",
            "acknowledgement_requested": true,
            "acknowledgement_status": "blocked_noop",
            "acknowledgement_performed": false,
            "acknowledgement_accepted": false,
            "acknowledgement_recorded": false,
            "acknowledgement_persisted": false,
            "acknowledgement_materialized": false,
            "acknowledgement_filesystem_written": false,
            "acknowledgement_delivered": false,
            "identity_accepted": false,
            "signature_accepted": false,
            "operator_approval_recorded": false,
            "review_index_persisted": false,
            "readback_index_persisted": false,
            "dispatch_allowed": false,
            "dispatch_performed": false,
            "execution_allowed": false,
            "execution_performed": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "memory_store_write_performed": false,
            "external_kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "channel_send_performed": false,
        },
        {
            "fixture_id": "reviewed-readback-index-attempt",
            "fixture_kind": "reviewed_readback_index_attempt",
            "acknowledgement_requested": true,
            "acknowledgement_status": "blocked_noop",
            "acknowledgement_performed": false,
            "acknowledgement_accepted": false,
            "acknowledgement_recorded": false,
            "acknowledgement_persisted": false,
            "acknowledgement_materialized": false,
            "acknowledgement_filesystem_written": false,
            "acknowledgement_delivered": false,
            "identity_accepted": false,
            "signature_accepted": false,
            "operator_approval_recorded": false,
            "review_index_persisted": false,
            "readback_index_persisted": false,
            "dispatch_allowed": false,
            "dispatch_performed": false,
            "execution_allowed": false,
            "execution_performed": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "memory_store_write_performed": false,
            "external_kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "channel_send_performed": false,
        },
        {
            "fixture_id": "acknowledged-noop-receipt-attempt",
            "fixture_kind": "acknowledged_noop_receipt_attempt",
            "acknowledgement_requested": true,
            "acknowledgement_status": "blocked_noop",
            "acknowledgement_performed": false,
            "acknowledgement_accepted": false,
            "acknowledgement_recorded": false,
            "acknowledgement_persisted": false,
            "acknowledgement_materialized": false,
            "acknowledgement_filesystem_written": false,
            "acknowledgement_delivered": false,
            "identity_accepted": false,
            "signature_accepted": false,
            "operator_approval_recorded": false,
            "review_index_persisted": false,
            "readback_index_persisted": false,
            "dispatch_allowed": false,
            "dispatch_performed": false,
            "execution_allowed": false,
            "execution_performed": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "memory_store_write_performed": false,
            "external_kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "channel_send_performed": false,
        },
        {
            "fixture_id": "acknowledgement-recording-attempt",
            "fixture_kind": "acknowledgement_recording_attempt",
            "acknowledgement_requested": true,
            "acknowledgement_status": "blocked_noop",
            "acknowledgement_performed": false,
            "acknowledgement_accepted": false,
            "acknowledgement_recorded": false,
            "acknowledgement_persisted": false,
            "acknowledgement_materialized": false,
            "acknowledgement_filesystem_written": false,
            "acknowledgement_delivered": false,
            "identity_accepted": false,
            "signature_accepted": false,
            "operator_approval_recorded": false,
            "review_index_persisted": false,
            "readback_index_persisted": false,
            "dispatch_allowed": false,
            "dispatch_performed": false,
            "execution_allowed": false,
            "execution_performed": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "memory_store_write_performed": false,
            "external_kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "channel_send_performed": false,
        },
        {
            "fixture_id": "acknowledgement-delivery-attempt",
            "fixture_kind": "acknowledgement_delivery_attempt",
            "acknowledgement_requested": true,
            "acknowledgement_status": "blocked_noop",
            "acknowledgement_performed": false,
            "acknowledgement_accepted": false,
            "acknowledgement_recorded": false,
            "acknowledgement_persisted": false,
            "acknowledgement_materialized": false,
            "acknowledgement_filesystem_written": false,
            "acknowledgement_delivered": false,
            "identity_accepted": false,
            "signature_accepted": false,
            "operator_approval_recorded": false,
            "review_index_persisted": false,
            "readback_index_persisted": false,
            "dispatch_allowed": false,
            "dispatch_performed": false,
            "execution_allowed": false,
            "execution_performed": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "memory_store_write_performed": false,
            "external_kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "channel_send_performed": false,
        },
        {
            "fixture_id": "dispatch-from-acknowledgement-attempt",
            "fixture_kind": "dispatch_from_acknowledgement_attempt",
            "acknowledgement_requested": true,
            "acknowledgement_status": "blocked_noop",
            "acknowledgement_performed": false,
            "acknowledgement_accepted": false,
            "acknowledgement_recorded": false,
            "acknowledgement_persisted": false,
            "acknowledgement_materialized": false,
            "acknowledgement_filesystem_written": false,
            "acknowledgement_delivered": false,
            "identity_accepted": false,
            "signature_accepted": false,
            "operator_approval_recorded": false,
            "review_index_persisted": false,
            "readback_index_persisted": false,
            "dispatch_allowed": false,
            "dispatch_performed": false,
            "execution_allowed": false,
            "execution_performed": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "memory_store_write_performed": false,
            "external_kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "channel_send_performed": false,
        },
        {
            "fixture_id": "provider-model-from-acknowledgement-attempt",
            "fixture_kind": "provider_model_from_acknowledgement_attempt",
            "acknowledgement_requested": true,
            "acknowledgement_status": "blocked_noop",
            "acknowledgement_performed": false,
            "acknowledgement_accepted": false,
            "acknowledgement_recorded": false,
            "acknowledgement_persisted": false,
            "acknowledgement_materialized": false,
            "acknowledgement_filesystem_written": false,
            "acknowledgement_delivered": false,
            "identity_accepted": false,
            "signature_accepted": false,
            "operator_approval_recorded": false,
            "review_index_persisted": false,
            "readback_index_persisted": false,
            "dispatch_allowed": false,
            "dispatch_performed": false,
            "execution_allowed": false,
            "execution_performed": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "memory_store_write_performed": false,
            "external_kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "channel_send_performed": false,
        },
        {
            "fixture_id": "memory-kg-write-from-acknowledgement-attempt",
            "fixture_kind": "memory_kg_write_from_acknowledgement_attempt",
            "acknowledgement_requested": true,
            "acknowledgement_status": "blocked_noop",
            "acknowledgement_performed": false,
            "acknowledgement_accepted": false,
            "acknowledgement_recorded": false,
            "acknowledgement_persisted": false,
            "acknowledgement_materialized": false,
            "acknowledgement_filesystem_written": false,
            "acknowledgement_delivered": false,
            "identity_accepted": false,
            "signature_accepted": false,
            "operator_approval_recorded": false,
            "review_index_persisted": false,
            "readback_index_persisted": false,
            "dispatch_allowed": false,
            "dispatch_performed": false,
            "execution_allowed": false,
            "execution_performed": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "memory_store_write_performed": false,
            "external_kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "channel_send_performed": false,
        },
    ]);

    let mut report = source_operator_review;
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT,
            "source_operator_review_readback_index_no_persistence_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_READBACK_INDEX_NO_PERSISTENCE_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_NON_ACCEPTANCE_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_readback_index_no_persistence_route_status": source_status,
            "source_operator_review_readback_index_no_persistence_route_ready": source_operator_review_ready,
            "operator_authorization_source": "telegram_direct_operator_highest_authorization_2026_06_13_16_27_10_asia_shanghai",
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_no_ack_accept_record_persist_materialize_deliver_no_operator_approval_identity_signature_no_dispatch_execute_context_inject_memory_kg_write_provider_model_credential_channel_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status": "blocked",
            "operator_review_acknowledgement_mode": "native_route_stdout_only_acknowledgement_shapes_no_acceptance_no_recording_no_persistence_no_dispatch_no_live",
            "operator_review_acknowledgement_decision": "review_acknowledgement_attempts_remain_blocked_noop_and_do_not_promote_operator_review_or_readback_index_to_authority",
            "source_operator_review_readback_index_status": "blocked",
            "source_operator_review_required_count": source_operator_review_required_count,
            "source_operator_review_accepted_count": source_operator_review_accepted_count,
            "source_readback_index_declared_count": source_readback_index_declared_count,
            "source_readback_index_persisted_count": source_readback_index_persisted_count,
            "source_review_authorizes_dispatch_count": source_review_authorizes_dispatch_count,
            "source_review_authorizes_execution_count": source_review_authorizes_execution_count,
            "source_review_authorizes_live_count": source_review_authorizes_live_count,
            "operator_review_acknowledgement_fixtures": acknowledgement_fixtures,
            "operator_review_acknowledgement_fixture_count": 8,
            "operator_review_acknowledgement_requested_fixture_count": 8,
            "blocked_operator_review_acknowledgement_fixture_count": 8,
            "noop_operator_review_acknowledgement_fixture_count": 8,
            "allowed_operator_review_acknowledgement_fixture_count": 0,
            "accepted_operator_review_acknowledgement_fixture_count": 0,
            "operator_review_acknowledgement_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_review_acknowledgement_allowed": false,
            "operator_review_acknowledgement_accepted": false,
            "operator_review_acknowledgement_recorded": false,
            "operator_review_acknowledgement_persisted": false,
            "operator_review_acknowledgement_materialized": false,
            "operator_review_acknowledgement_filesystem_written": false,
            "operator_review_acknowledgement_delivered": false,
            "operator_review_acknowledgement_identity_accepted": false,
            "operator_review_acknowledgement_signature_accepted": false,
            "operator_review_acknowledgement_final_state_promoted": false,
            "operator_review_acknowledgement_completion_promoted": false,
            "operator_review_acknowledgement_authorizes_dispatch_count": 0,
            "operator_review_acknowledgement_authorizes_execution_count": 0,
            "operator_review_acknowledgement_authorizes_live_count": 0,
            "operator_approval_recorded": false,
            "operator_identity_accepted": false,
            "readback_index_recorded_count": 0,
            "readback_index_persisted_count": 0,
            "readback_index_materialized_count": 0,
            "readback_index_filesystem_written_count": 0,
            "dispatch_allowed_count": 0,
            "dispatch_performed_count": 0,
            "execution_allowed_count": 0,
            "execution_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
            "denied_by_operator_review_acknowledgement_non_acceptance_count": 19,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 15,
            "enablement_lane_count": 18,
            "ready_enablement_lane_count": 18,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "blocked_actions": [
                "accept_operator_canary_controlled_request_harness_operator_review_acknowledgement_from_report_route",
                "record_operator_canary_controlled_request_harness_operator_review_acknowledgement_from_report_route",
                "persist_operator_canary_controlled_request_harness_operator_review_acknowledgement_from_report_route",
                "materialize_operator_canary_controlled_request_harness_operator_review_acknowledgement_from_report_route",
                "write_operator_canary_controlled_request_harness_operator_review_acknowledgement_file_from_report_route",
                "deliver_operator_canary_controlled_request_harness_operator_review_acknowledgement_from_report_route",
                "accept_operator_identity_from_operator_review_acknowledgement",
                "accept_operator_signature_from_operator_review_acknowledgement",
                "record_operator_approval_from_operator_review_acknowledgement",
                "promote_operator_review_acknowledgement_to_final_state",
                "promote_operator_review_acknowledgement_to_completion",
                "dispatch_operator_canary_controlled_request_from_acknowledgement",
                "execute_operator_canary_controlled_request_from_acknowledgement",
                "attach_or_inject_context_from_operator_review_acknowledgement",
                "write_memory_or_live_kg_from_operator_review_acknowledgement",
                "invoke_provider_or_model_from_operator_review_acknowledgement",
                "read_credential_or_secret_from_operator_review_acknowledgement",
                "telegram_or_channel_delivery_from_operator_review_acknowledgement",
                "release_or_public_claim_from_operator_review_acknowledgement"
            ],
            "allowed_next_actions": [
                "run operator canary controlled-request harness operator-review acknowledgement non-acceptance route gate against the readback index route",
                "install canary operator-review acknowledgement non-acceptance route through controlled live catch-up after full preflight",
                "slice operator-review acknowledgement activation request denial matrix while keeping acknowledgement acceptance, dispatch, execution, persistence, context injection, Memory/KG writes, provider/model invocation, credential reads, channel delivery, and public release disabled"
            ],
            "denied_by_operator_review_acknowledgement_non_acceptance": [
                "operator_review_acknowledgement_acceptance_denied",
                "operator_review_acknowledgement_recording_denied",
                "operator_review_acknowledgement_persistence_denied",
                "operator_review_acknowledgement_materialization_denied",
                "operator_review_acknowledgement_filesystem_write_denied",
                "operator_review_acknowledgement_delivery_denied",
                "operator_review_acknowledgement_identity_acceptance_denied",
                "operator_review_acknowledgement_signature_acceptance_denied",
                "operator_review_acknowledgement_cannot_promote_review_index",
                "operator_review_acknowledgement_cannot_promote_readback_index",
                "operator_review_acknowledgement_cannot_promote_dispatch_authority",
                "operator_review_acknowledgement_cannot_promote_execution_authority",
                "operator_review_acknowledgement_cannot_promote_live_authority",
                "provider_model_invocation_denied",
                "memory_write_denied",
                "external_kg_read_denied",
                "live_kg_write_denied",
                "credential_secret_read_denied",
                "channel_delivery_denied"
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "operator_review_acknowledgement_performed",
            "operator_review_acknowledgement_recorded",
            "operator_review_acknowledgement_persisted",
            "operator_review_acknowledgement_materialized",
            "operator_review_acknowledgement_filesystem_written",
            "operator_review_acknowledgement_delivered",
            "operator_review_acknowledgement_accepted",
            "operator_approval_recorded",
            "operator_identity_accepted",
            "readback_index_recorded",
            "readback_index_persisted",
            "readback_index_materialized",
            "readback_index_filesystem_written",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "service_restarted",
            "active_binary_mutated",
            "install_performed",
            "upstream_fetch_performed",
            "upstream_merge_performed",
            "public_release_claimed",
            "public_ga_claimed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_acknowledgement =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_report();
    let source_bool = |key: &str| {
        source_acknowledgement
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_acknowledgement
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_acknowledgement
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_acknowledgement_route_ready = source_status == "ready"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_enabled",
        )
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_ready",
        )
        && source_u64("operator_review_acknowledgement_fixture_count") == 8
        && source_u64("operator_review_acknowledgement_requested_fixture_count") == 8
        && source_u64("blocked_operator_review_acknowledgement_fixture_count") == 8
        && source_u64("noop_operator_review_acknowledgement_fixture_count") == 8
        && source_u64("allowed_operator_review_acknowledgement_fixture_count") == 0
        && source_u64("accepted_operator_review_acknowledgement_fixture_count") == 0
        && source_u64("operator_review_acknowledgement_performed_count") == 0
        && source_u64("operator_review_acknowledgement_authorizes_dispatch_count") == 0
        && source_u64("operator_review_acknowledgement_authorizes_execution_count") == 0
        && source_u64("operator_review_acknowledgement_authorizes_live_count") == 0
        && source_u64("dispatch_performed_count") == 0
        && source_u64("execution_performed_count") == 0
        && source_u64("context_injection_performed_count") == 0
        && source_u64("provider_invoked_count") == 0
        && source_u64("model_invoked_count") == 0
        && source_u64("memory_store_write_performed_count") == 0
        && source_u64("external_kg_adapter_read_performed_count") == 0
        && source_u64("live_kg_write_performed_count") == 0
        && source_u64("credential_read_count") == 0
        && source_u64("secret_file_read_count") == 0
        && source_u64("channel_send_performed_count") == 0
        && !source_bool("operator_review_acknowledgement_accepted")
        && !source_bool("operator_review_acknowledgement_recorded")
        && !source_bool("operator_review_acknowledgement_persisted")
        && !source_bool("operator_approval_recorded")
        && !source_bool("operator_identity_accepted")
        && !source_bool("canary_harness_armed")
        && !source_bool("canary_harness_executable")
        && !source_bool("canary_live_enabled")
        && source_u64("current_live_enabled_lane_count") == 15
        && source_u64("enablement_lane_count") == 18
        && source_u64("ready_enablement_lane_count") == 18;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_acknowledgement_route_ready;
    let source_operator_review_acknowledgement_fixture_count =
        source_u64("operator_review_acknowledgement_fixture_count");
    let source_operator_review_acknowledgement_accepted_count =
        source_u64("accepted_operator_review_acknowledgement_fixture_count");
    let source_operator_review_acknowledgement_performed_count =
        source_u64("operator_review_acknowledgement_performed_count");
    let source_operator_review_acknowledgement_authorizes_dispatch_count =
        source_u64("operator_review_acknowledgement_authorizes_dispatch_count");
    let source_operator_review_acknowledgement_authorizes_execution_count =
        source_u64("operator_review_acknowledgement_authorizes_execution_count");
    let source_operator_review_acknowledgement_authorizes_live_count =
        source_u64("operator_review_acknowledgement_authorizes_live_count");

    let activation_fixture =
        |fixture_id: &str, fixture_kind: &str, denial_reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::json!({
                "fixture_id": fixture_id,
                "fixture_kind": fixture_kind,
                "activation_request_requested": true,
                "activation_request_status": "blocked_noop",
                "source_acknowledgement_present": true,
                "source_acknowledgement_ready": true,
                "acknowledgement_accepted": false,
                "activation_request_allowed": false,
                "activation_request_accepted": false,
                "activation_request_recorded": false,
                "activation_request_persisted": false,
                "activation_request_materialized": false,
                "activation_request_filesystem_written": false,
                "activation_request_delivered": false,
                "activation_request_executed": false,
                "activation_nonce_generated": false,
                "activation_identity_accepted": false,
                "activation_scope_accepted": false,
                "activation_final_state_promoted": false,
                "dispatch_allowed": false,
                "dispatch_performed": false,
                "execution_allowed": false,
                "execution_performed": false,
                "context_injection_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "memory_store_write_performed": false,
                "external_kg_adapter_read_performed": false,
                "live_kg_write_performed": false,
                "credential_read": false,
                "secret_file_read": false,
                "channel_send_performed": false,
                "install_performed": false,
                "service_restarted": false,
                "active_binary_mutated": false,
                "upstream_fetch_performed": false,
                "upstream_merge_performed": false,
                "denial_reason": denial_reason,
            });
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let activation_request_denial_fixtures = serde_json::Value::Array(vec![
        activation_fixture(
            "missing-source-acknowledgement-report",
            "missing_source_acknowledgement_report",
            "source_acknowledgement_non_acceptance_report_required",
            serde_json::json!({
                "source_acknowledgement_present": false,
                "source_acknowledgement_ready": false,
            }),
        ),
        activation_fixture(
            "acknowledgement-to-activation-request-shape",
            "activation_request_shape_from_acknowledgement",
            "acknowledgement_cannot_create_activation_request",
            serde_json::json!({}),
        ),
        activation_fixture(
            "acknowledgement-identity-scope-request",
            "identity_scope_from_acknowledgement",
            "acknowledgement_cannot_accept_identity_or_scope",
            serde_json::json!({
                "identity_scope_requested": true,
            }),
        ),
        activation_fixture(
            "acknowledgement-nonce-generation-request",
            "nonce_generation_from_acknowledgement",
            "acknowledgement_cannot_generate_activation_nonce",
            serde_json::json!({
                "nonce_generation_requested": true,
            }),
        ),
        activation_fixture(
            "acknowledgement-dispatch-request",
            "dispatch_request_from_acknowledgement",
            "acknowledgement_cannot_authorize_dispatch",
            serde_json::json!({
                "dispatch_requested": true,
            }),
        ),
        activation_fixture(
            "acknowledgement-execution-request",
            "execution_request_from_acknowledgement",
            "acknowledgement_cannot_authorize_execution",
            serde_json::json!({
                "execution_requested": true,
            }),
        ),
        activation_fixture(
            "acknowledgement-context-provider-model-request",
            "context_provider_model_from_acknowledgement",
            "acknowledgement_cannot_authorize_context_or_provider",
            serde_json::json!({
                "context_attachment_requested": true,
                "provider_invocation_requested": true,
                "model_invocation_requested": true,
            }),
        ),
        activation_fixture(
            "acknowledgement-memory-kg-write-request",
            "memory_kg_write_from_acknowledgement",
            "acknowledgement_cannot_authorize_memory_or_kg_write",
            serde_json::json!({
                "memory_write_requested": true,
                "kg_write_requested": true,
            }),
        ),
        activation_fixture(
            "acknowledgement-external-public-install-secret-request",
            "external_public_install_secret_from_acknowledgement",
            "acknowledgement_cannot_authorize_external_public_install_or_secret_access",
            serde_json::json!({
                "external_send_requested": true,
                "public_claim_requested": true,
                "install_requested": true,
                "restart_requested": true,
                "secret_access_requested": true,
            }),
        ),
    ]);

    let mut report = source_acknowledgement;
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_REQUEST_DENIAL_MATRIX_ENDPOINT,
            "source_operator_review_acknowledgement_non_acceptance_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_REQUEST_DENIAL_MATRIX_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_REQUEST_DENIAL_MATRIX_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_non_acceptance_route_status": source_status,
            "source_operator_review_acknowledgement_non_acceptance_route_ready": source_acknowledgement_route_ready,
            "operator_authorization_source": "telegram_direct_operator_highest_authorization_2026_06_13_19_36_01_asia_shanghai",
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_no_activation_request_accept_record_persist_materialize_execute_no_dispatch_context_inject_memory_kg_write_provider_model_credential_channel_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_request_denial_matrix_v1",
            "activation_request_denial_matrix_mode": "native_route_stdout_only_activation_request_shapes_no_acceptance_no_recording_no_persistence_no_dispatch_no_execution_no_live",
            "activation_request_denial_matrix_decision": "operator_review_acknowledgement_attempts_do_not_create_or_authorize_activation_requests",
            "source_operator_review_acknowledgement_fixture_count": source_operator_review_acknowledgement_fixture_count,
            "source_operator_review_acknowledgement_accepted_count": source_operator_review_acknowledgement_accepted_count,
            "source_operator_review_acknowledgement_performed_count": source_operator_review_acknowledgement_performed_count,
            "source_operator_review_acknowledgement_authorizes_dispatch_count": source_operator_review_acknowledgement_authorizes_dispatch_count,
            "source_operator_review_acknowledgement_authorizes_execution_count": source_operator_review_acknowledgement_authorizes_execution_count,
            "source_operator_review_acknowledgement_authorizes_live_count": source_operator_review_acknowledgement_authorizes_live_count,
            "activation_request_denial_fixtures": activation_request_denial_fixtures,
            "activation_request_denial_fixture_count": 9,
            "activation_request_requested_fixture_count": 9,
            "blocked_activation_request_fixture_count": 9,
            "noop_activation_request_fixture_count": 9,
            "allowed_activation_request_fixture_count": 0,
            "accepted_activation_request_fixture_count": 0,
            "activation_request_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_request_allowed": false,
            "activation_request_accepted": false,
            "activation_request_recorded": false,
            "activation_request_persisted": false,
            "activation_request_materialized": false,
            "activation_request_filesystem_written": false,
            "activation_request_delivered": false,
            "activation_request_executed": false,
            "activation_nonce_generated": false,
            "activation_identity_accepted": false,
            "activation_scope_accepted": false,
            "activation_final_state_promoted": false,
            "operator_review_acknowledgement_accepted": false,
            "operator_review_acknowledgement_recorded": false,
            "operator_review_acknowledgement_persisted": false,
            "operator_approval_recorded": false,
            "operator_identity_accepted": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_allowed_count": 0,
            "dispatch_performed_count": 0,
            "execution_allowed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
            "denied_by_operator_review_acknowledgement_activation_request_denial_matrix_count": 26,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 16,
            "enablement_lane_count": 19,
            "ready_enablement_lane_count": 19,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "blocked_actions": [
                "accept_activation_request_from_operator_review_acknowledgement",
                "record_activation_request_from_operator_review_acknowledgement",
                "persist_activation_request_from_operator_review_acknowledgement",
                "materialize_activation_request_from_operator_review_acknowledgement",
                "write_activation_request_file_from_operator_review_acknowledgement",
                "deliver_activation_request_from_operator_review_acknowledgement",
                "execute_activation_request_from_operator_review_acknowledgement",
                "generate_activation_nonce_from_operator_review_acknowledgement",
                "accept_activation_identity_or_scope_from_operator_review_acknowledgement",
                "promote_activation_request_to_final_state",
                "record_operator_approval_from_operator_review_acknowledgement",
                "dispatch_operator_canary_controlled_request_from_activation_request",
                "execute_operator_canary_controlled_request_from_activation_request",
                "attach_or_inject_context_from_activation_request",
                "write_memory_or_live_kg_from_activation_request",
                "invoke_provider_or_model_from_activation_request",
                "read_credential_or_secret_from_activation_request",
                "telegram_or_channel_delivery_from_activation_request",
                "install_restart_or_active_binary_mutation_from_activation_request",
                "upstream_fetch_merge_or_public_claim_from_activation_request"
            ],
            "allowed_next_actions": [
                "run operator canary controlled-request harness operator-review acknowledgement activation request denial matrix route gate against the acknowledgement non-acceptance route",
                "install canary operator-review acknowledgement activation request denial matrix route through controlled live catch-up after full preflight",
                "slice operator-review acknowledgement activation command no-op handoff while keeping activation requests, dispatch, execution, persistence, context injection, Memory/KG writes, provider/model invocation, credential reads, channel delivery, and public release disabled"
            ],
            "denied_by_operator_review_acknowledgement_activation_request_denial_matrix": [
                "source_acknowledgement_non_acceptance_report_required",
                "activation_request_acceptance_denied",
                "activation_request_recording_denied",
                "activation_request_persistence_denied",
                "activation_request_materialization_denied",
                "activation_request_filesystem_write_denied",
                "activation_request_delivery_denied",
                "activation_request_execution_denied",
                "activation_nonce_generation_denied",
                "activation_identity_acceptance_denied",
                "activation_scope_acceptance_denied",
                "activation_final_state_promotion_denied",
                "operator_review_acknowledgement_not_authority",
                "operator_approval_not_recorded",
                "dispatch_from_acknowledgement_denied",
                "execution_from_acknowledgement_denied",
                "context_injection_from_acknowledgement_denied",
                "provider_model_invocation_denied",
                "memory_write_denied",
                "external_kg_read_denied",
                "live_kg_write_denied",
                "credential_secret_read_denied",
                "channel_delivery_denied",
                "install_restart_denied",
                "active_binary_mutation_denied",
                "upstream_fetch_merge_denied"
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "activation_request_performed",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_materialized",
            "activation_request_filesystem_written",
            "activation_request_delivered",
            "activation_request_executed",
            "activation_nonce_generated",
            "activation_identity_accepted",
            "activation_scope_accepted",
            "activation_final_state_promoted",
            "operator_review_acknowledgement_accepted",
            "operator_approval_recorded",
            "operator_identity_accepted",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
            "public_release_claimed",
            "public_ga_claimed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_activation_request =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_report();
    let source_bool = |key: &str| {
        source_activation_request
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_activation_request
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_activation_request
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked")
        .to_string();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_activation_request_route_ready = source_status == "ready"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_enabled",
        )
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_ready",
        )
        && !source_bool("activation_request_allowed")
        && !source_bool("activation_request_accepted")
        && source_u64("activation_request_denial_fixture_count") == 9
        && source_u64("blocked_activation_request_fixture_count") == 9
        && source_u64("noop_activation_request_fixture_count") == 9
        && source_u64("accepted_activation_request_fixture_count") == 0
        && source_u64("activation_request_performed_count") == 0
        && source_u64("dispatch_performed_count") == 0
        && source_u64("execution_performed_count") == 0
        && source_u64("context_injection_performed_count") == 0
        && source_u64("provider_invoked_count") == 0
        && source_u64("model_invoked_count") == 0
        && source_u64("memory_store_write_performed_count") == 0
        && source_u64("external_kg_adapter_read_performed_count") == 0
        && source_u64("live_kg_write_performed_count") == 0
        && source_u64("credential_read_count") == 0
        && source_u64("secret_file_read_count") == 0
        && source_u64("channel_send_performed_count") == 0
        && source_u64("current_live_enabled_lane_count") == 16
        && source_u64("enablement_lane_count") == 19
        && source_u64("ready_enablement_lane_count") == 19
        && !source_bool("activation_request_recorded")
        && !source_bool("activation_request_persisted")
        && !source_bool("activation_request_executed")
        && !source_bool("operator_approval_recorded")
        && !source_bool("operator_identity_accepted")
        && !source_bool("canary_harness_armed")
        && !source_bool("canary_harness_executable")
        && !source_bool("canary_live_enabled");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_activation_request_route_ready;

    let activation_command_fixture =
        |fixture_id: &str, status: &str, denial_reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "activation_command_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(denial_reason.to_string()),
            );
            for key in [
                "source_activation_request_denial_matrix_present",
                "source_activation_request_denial_matrix_ready",
                "activation_command_requested",
                "activation_command_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "activation_command_shape_registered",
                "activation_command_allowed",
                "activation_command_accepted",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_command_dispatch_performed",
                "activation_command_noop_decision_recorded",
                "activation_command_noop_decision_persisted",
                "activation_command_noop_decision_accepted",
                "activation_command_handoff_recorded",
                "activation_command_handoff_persisted",
                "activation_command_handoff_accepted",
                "activation_command_handoff_materialized",
                "activation_command_handoff_filesystem_written",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_exported",
                "activation_command_result_receipt_query_registered",
                "activation_command_result_receipt_observability_recorded",
                "activation_request_allowed",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_materialized",
                "activation_request_filesystem_written",
                "activation_request_delivered",
                "activation_request_executed",
                "activation_nonce_generated",
                "activation_identity_accepted",
                "activation_scope_accepted",
                "activation_final_state_promoted",
                "operator_review_acknowledgement_accepted",
                "operator_review_acknowledgement_recorded",
                "operator_review_acknowledgement_persisted",
                "operator_approval_recorded",
                "operator_identity_accepted",
                "dispatch_allowed",
                "dispatch_performed",
                "execution_allowed",
                "execution_performed",
                "context_injection_performed",
                "provider_invoked",
                "model_invoked",
                "memory_store_write_performed",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "credential_read",
                "secret_file_read",
                "channel_send_performed",
                "install_performed",
                "service_restarted",
                "active_binary_mutated",
                "upstream_fetch_performed",
                "upstream_merge_performed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let activation_command_fixtures = serde_json::Value::Array(vec![
        activation_command_fixture(
            "missing-source-activation-request-denial-matrix-report",
            "blocked_noop",
            "source_activation_request_denial_matrix_report_required",
            serde_json::json!({
                "source_activation_request_denial_matrix_present": false,
                "source_activation_request_denial_matrix_ready": false,
            }),
        ),
        activation_command_fixture(
            "acknowledgement-activation-command-handoff-request",
            "blocked_command_noop",
            "activation_command_handoff_shape_denied",
            serde_json::json!({}),
        ),
        activation_command_fixture(
            "acknowledgement-activation-command-registration-enable-request",
            "blocked_register_enable_noop",
            "activation_command_registration_enablement_denied",
            serde_json::json!({
                "activation_command_registration_requested": true,
                "activation_command_enable_requested": true,
            }),
        ),
        activation_command_fixture(
            "acknowledgement-activation-command-direct-invocation-request",
            "blocked_invocation_noop",
            "activation_command_invocation_denied",
            serde_json::json!({
                "activation_command_invocation_requested": true,
            }),
        ),
        activation_command_fixture(
            "acknowledgement-activation-command-dispatch-request",
            "blocked_dispatch_noop",
            "activation_command_dispatch_denied",
            serde_json::json!({
                "activation_command_dispatch_requested": true,
            }),
        ),
        activation_command_fixture(
            "acknowledgement-activation-command-execution-request",
            "blocked_execution_noop",
            "activation_command_execution_denied",
            serde_json::json!({
                "activation_command_execution_requested": true,
            }),
        ),
        activation_command_fixture(
            "acknowledgement-activation-command-context-provider-model-request",
            "blocked_context_provider_model_noop",
            "context_provider_model_command_denied",
            serde_json::json!({
                "context_attachment_requested": true,
                "provider_invocation_requested": true,
                "model_invocation_requested": true,
            }),
        ),
        activation_command_fixture(
            "acknowledgement-activation-command-memory-kg-request",
            "blocked_memory_kg_noop",
            "memory_kg_command_denied",
            serde_json::json!({
                "memory_write_requested": true,
                "kg_write_requested": true,
            }),
        ),
        activation_command_fixture(
            "acknowledgement-activation-command-result-receipt-readback-request",
            "blocked_receipt_readback_noop",
            "command_result_receipt_readback_denied",
            serde_json::json!({
                "command_result_receipt_record_requested": true,
                "command_result_receipt_persist_requested": true,
                "readback_requested": true,
            }),
        ),
        activation_command_fixture(
            "acknowledgement-activation-command-external-public-install-secret-request",
            "blocked_external_secret_noop",
            "external_public_install_secret_command_denied",
            serde_json::json!({
                "external_send_requested": true,
                "public_claim_requested": true,
                "install_requested": true,
                "restart_requested": true,
                "secret_access_requested": true,
            }),
        ),
    ]);

    let mut report = source_activation_request.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-noop-handoff --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_NOOP_HANDOFF_ENDPOINT,
            "source_operator_review_acknowledgement_activation_request_denial_matrix_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_REQUEST_DENIAL_MATRIX_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_NOOP_HANDOFF_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_NOOP_HANDOFF_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-noop-handoff-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-noop-handoff-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_request_denial_matrix_route_status": source_status,
            "source_operator_review_acknowledgement_activation_request_denial_matrix_route_ready": source_activation_request_route_ready,
            "operator_authorization_source": "telegram_direct_operator_highest_authorization_2026_06_13_19_36_01_asia_shanghai",
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_no_command_register_enable_accept_invoke_dispatch_execute_handoff_persist_result_receipt_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_noop_handoff_v1",
            "activation_command_noop_handoff_mode": "native_route_stdout_only_activation_command_shapes_no_register_no_enable_no_invoke_no_dispatch_no_result_receipt_no_live",
            "activation_command_noop_handoff_decision": "operator_review_acknowledgement_activation_request_denial_cannot_create_or_authorize_activation_commands",
            "source_activation_request_denial_fixture_count": source_u64("activation_request_denial_fixture_count"),
            "source_blocked_activation_request_fixture_count": source_u64("blocked_activation_request_fixture_count"),
            "source_noop_activation_request_fixture_count": source_u64("noop_activation_request_fixture_count"),
            "source_accepted_activation_request_fixture_count": source_u64("accepted_activation_request_fixture_count"),
            "source_activation_request_performed_count": source_u64("activation_request_performed_count"),
            "activation_command_surface_count": 13,
            "activation_command_surface_ready_count": 13,
            "activation_command_side_effect_free_surface_count": 13,
            "activation_command_fixtures": activation_command_fixtures,
            "activation_command_fixture_count": 10,
            "activation_command_requested_fixture_count": 10,
            "blocked_activation_command_fixture_count": 10,
            "noop_activation_command_fixture_count": 10,
            "allowed_activation_command_fixture_count": 0,
            "accepted_activation_command_fixture_count": 0,
            "activation_command_performed_count": 0,
            "activation_command_dispatch_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_shape_registered": false,
            "activation_command_allowed": false,
            "activation_command_accepted": false,
            "activation_command_enabled": false,
            "activation_command_invoked": false,
            "activation_command_dispatched": false,
            "activation_command_noop_decision_recorded": false,
            "activation_command_noop_decision_persisted": false,
            "activation_command_handoff_recorded": false,
            "activation_command_handoff_persisted": false,
            "activation_command_result_receipt_recorded": false,
            "activation_command_result_receipt_persisted": false,
            "activation_command_result_receipt_accepted": false,
            "activation_request_accepted": false,
            "activation_request_recorded": false,
            "activation_request_persisted": false,
            "activation_request_executed": false,
            "operator_approval_recorded": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
            "denied_by_operator_review_acknowledgement_activation_command_noop_handoff_count": 57,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 17,
            "enablement_lane_count": 20,
            "ready_enablement_lane_count": 20,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "blocked_actions": [
                "register_activation_command_from_denied_activation_request",
                "enable_activation_command_from_denied_activation_request",
                "accept_activation_command_from_operator_acknowledgement",
                "invoke_activation_command_from_operator_acknowledgement",
                "dispatch_activation_command_from_operator_acknowledgement",
                "execute_activation_command_from_operator_acknowledgement",
                "record_activation_command_noop_decision",
                "persist_activation_command_handoff",
                "record_or_persist_activation_command_result_receipt",
                "export_query_or_observe_activation_command_result_receipt",
                "inject_context_from_activation_command",
                "invoke_provider_or_model_from_activation_command",
                "write_memory_or_live_kg_from_activation_command",
                "read_credential_or_secret_from_activation_command",
                "send_channel_or_telegram_message_from_activation_command",
                "install_restart_mutate_binary_or_public_claim_from_activation_command"
            ],
            "allowed_next_actions": [
                "run operator canary controlled-request harness operator-review acknowledgement activation command no-op handoff route gate against the activation request denial matrix route",
                "install canary operator-review acknowledgement activation command no-op handoff route through controlled live catch-up after full preflight",
                "slice operator-review acknowledgement activation command result receipt no-persistence while keeping result recording, persistence, acceptance, export, query, observability, Memory/KG writes, provider/model invocation, and live execution blocked"
            ],
            "denied_by_operator_review_acknowledgement_activation_command_noop_handoff": [
                "source_acknowledgement_non_acceptance_report_required",
                "activation_request_acceptance_denied",
                "activation_request_recording_denied",
                "activation_request_persistence_denied",
                "activation_request_materialization_denied",
                "activation_request_filesystem_write_denied",
                "activation_request_delivery_denied",
                "activation_request_execution_denied",
                "activation_nonce_generation_denied",
                "activation_identity_acceptance_denied",
                "activation_scope_acceptance_denied",
                "activation_final_state_promotion_denied",
                "operator_review_acknowledgement_not_authority",
                "operator_approval_not_recorded",
                "dispatch_from_acknowledgement_denied",
                "execution_from_acknowledgement_denied",
                "context_injection_from_acknowledgement_denied",
                "provider_model_invocation_denied",
                "memory_write_denied",
                "external_kg_read_denied",
                "live_kg_write_denied",
                "credential_secret_read_denied",
                "channel_delivery_denied",
                "install_restart_denied",
                "active_binary_mutation_denied",
                "upstream_fetch_merge_denied",
                "source_activation_request_denial_matrix_report_required",
                "activation_command_shape_registration_denied",
                "activation_command_acceptance_denied",
                "activation_command_enablement_denied",
                "activation_command_invocation_denied",
                "activation_command_dispatch_denied",
                "activation_command_dispatch_execution_denied",
                "activation_command_noop_decision_recording_denied",
                "activation_command_noop_decision_persistence_denied",
                "activation_command_handoff_recording_denied",
                "activation_command_handoff_persistence_denied",
                "activation_command_result_receipt_recording_denied",
                "activation_command_result_receipt_persistence_denied",
                "activation_command_result_receipt_acceptance_denied",
                "activation_command_result_receipt_export_query_observability_denied",
                "activation_request_acceptance_denied",
                "activation_request_execution_denied",
                "operator_review_acknowledgement_not_authority",
                "operator_approval_not_recorded",
                "dispatch_from_command_denied",
                "execution_from_command_denied",
                "context_injection_from_command_denied",
                "provider_model_invocation_denied",
                "memory_write_denied",
                "external_kg_read_denied",
                "live_kg_write_denied",
                "credential_secret_read_denied",
                "channel_delivery_denied",
                "install_restart_denied",
                "active_binary_mutation_denied",
                "upstream_fetch_merge_denied"
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "activation_command_registered",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_handoff_recorded",
            "activation_command_handoff_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
            "public_release_claimed",
            "public_ga_claimed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_activation_command =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_report();
    let source_bool = |key: &str| {
        source_activation_command
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_activation_command
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_activation_command
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_activation_command_noop_handoff_ready = source_status == "ready"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_ready",
        )
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_route_enabled",
        )
        && source_u64("activation_command_fixture_count") == 10
        && source_u64("blocked_activation_command_fixture_count") == 10
        && source_u64("noop_activation_command_fixture_count") == 10
        && source_u64("accepted_activation_command_fixture_count") == 0
        && source_u64("activation_command_performed_count") == 0
        && source_u64("activation_command_dispatch_performed_count") == 0
        && !source_bool("activation_command_allowed")
        && !source_bool("activation_command_accepted")
        && !source_bool("activation_command_enabled")
        && !source_bool("activation_command_invoked")
        && !source_bool("activation_command_dispatched")
        && !source_bool("activation_command_handoff_recorded")
        && !source_bool("activation_command_handoff_persisted")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_request_accepted")
        && !source_bool("activation_request_recorded")
        && !source_bool("activation_request_executed")
        && source_u64("dispatch_performed_count") == 0
        && source_u64("execution_performed_count") == 0
        && source_u64("context_injection_performed_count") == 0
        && source_u64("provider_invoked_count") == 0
        && source_u64("model_invoked_count") == 0
        && source_u64("memory_store_write_performed_count") == 0
        && source_u64("external_kg_adapter_read_performed_count") == 0
        && source_u64("live_kg_write_performed_count") == 0
        && source_u64("credential_read_count") == 0
        && source_u64("secret_file_read_count") == 0
        && source_u64("channel_send_performed_count") == 0
        && source_u64("current_live_enabled_lane_count") == 17
        && source_u64("enablement_lane_count") == 20
        && source_u64("ready_enablement_lane_count") == 20
        && !source_bool("canary_harness_armed")
        && !source_bool("canary_harness_executable")
        && !source_bool("canary_live_enabled");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_activation_command_noop_handoff_ready;

    let result_receipt_fixture =
        |fixture_id: &str, status: &str, denial_reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "activation_command_result_receipt_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(denial_reason.to_string()),
            );
            for key in [
                "source_activation_command_noop_handoff_present",
                "source_activation_command_noop_handoff_ready",
                "activation_command_result_receipt_requested",
                "activation_command_result_receipt_non_authority_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "activation_command_result_receipt_shape_registered",
                "activation_command_result_receipt_allowed",
                "activation_command_result_receipt_schema_accepted",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_result_receipt_ledger_written",
                "activation_command_result_receipt_indexed",
                "activation_command_result_receipt_enqueued",
                "activation_command_result_receipt_delivered",
                "activation_command_result_receipt_exported",
                "activation_command_result_receipt_query_registered",
                "activation_command_result_receipt_observability_recorded",
                "activation_command_result_receipt_hash_bound",
                "activation_command_result_receipt_signature_hash_recorded",
                "activation_command_result_receipt_timestamp_recorded",
                "activation_command_result_receipt_operator_identity_accepted",
                "activation_command_result_receipt_status_accepted",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "operator_approval_from_receipt_accepted",
                "activation_from_receipt_allowed",
                "activation_command_allowed",
                "activation_command_accepted",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_command_dispatch_performed",
                "activation_command_handoff_recorded",
                "activation_command_handoff_persisted",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "dispatch_performed",
                "execution_performed",
                "context_injection_performed",
                "provider_invoked",
                "model_invoked",
                "memory_store_write_performed",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "credential_read",
                "secret_file_read",
                "channel_send_performed",
                "external_send_performed",
                "public_claim_performed",
                "install_performed",
                "service_restarted",
                "active_binary_mutated",
                "upstream_fetch_performed",
                "upstream_merge_performed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let result_receipt_fixtures = serde_json::Value::Array(vec![
        result_receipt_fixture(
            "missing-source-activation-command-noop-handoff-report",
            "blocked_noop",
            "source_activation_command_noop_handoff_report_required",
            serde_json::json!({
                "source_activation_command_noop_handoff_present": false,
                "source_activation_command_noop_handoff_ready": false,
            }),
        ),
        result_receipt_fixture(
            "acknowledgement-activation-command-result-receipt-schema-registration-attempt",
            "blocked_schema_noop",
            "result_receipt_schema_registration_denied",
            serde_json::json!({"result_receipt_schema_registration_requested": true}),
        ),
        result_receipt_fixture(
            "acknowledgement-activation-command-result-receipt-record-attempt",
            "blocked_record_noop",
            "result_receipt_recording_denied",
            serde_json::json!({"result_receipt_record_requested": true}),
        ),
        result_receipt_fixture(
            "acknowledgement-activation-command-result-receipt-persist-attempt",
            "blocked_persist_noop",
            "result_receipt_persistence_denied",
            serde_json::json!({"result_receipt_persist_requested": true}),
        ),
        result_receipt_fixture(
            "acknowledgement-activation-command-result-receipt-materialize-filesystem-attempt",
            "blocked_materialize_noop",
            "result_receipt_materialization_filesystem_write_denied",
            serde_json::json!({
                "result_receipt_materialize_requested": true,
                "result_receipt_filesystem_write_requested": true,
            }),
        ),
        result_receipt_fixture(
            "acknowledgement-activation-command-result-receipt-ledger-index-delivery-attempt",
            "blocked_ledger_index_delivery_noop",
            "result_receipt_ledger_index_delivery_denied",
            serde_json::json!({
                "result_receipt_ledger_write_requested": true,
                "result_receipt_index_requested": true,
                "result_receipt_enqueue_requested": true,
                "result_receipt_delivery_requested": true,
            }),
        ),
        result_receipt_fixture(
            "acknowledgement-activation-command-result-receipt-export-query-observability-attempt",
            "blocked_export_query_observability_noop",
            "result_receipt_export_query_observability_denied",
            serde_json::json!({
                "result_receipt_export_requested": true,
                "result_receipt_query_requested": true,
                "result_receipt_observability_requested": true,
            }),
        ),
        result_receipt_fixture(
            "acknowledgement-activation-command-result-receipt-acceptance-completion-ack-attempt",
            "blocked_acceptance_ack_noop",
            "result_receipt_acceptance_completion_ack_denied",
            serde_json::json!({
                "result_receipt_acceptance_requested": true,
                "completion_ack_requested": true,
                "operator_approval_from_receipt_requested": true,
            }),
        ),
        result_receipt_fixture(
            "acknowledgement-activation-command-result-receipt-activation-authority-attempt",
            "blocked_activation_authority_noop",
            "result_receipt_cannot_authorize_activation",
            serde_json::json!({
                "activation_from_receipt_requested": true,
                "activation_request_record_requested": true,
                "activation_execution_requested": true,
                "dispatch_requested": true,
                "execution_requested": true,
            }),
        ),
        result_receipt_fixture(
            "acknowledgement-activation-command-result-receipt-provider-memory-kg-external-attempt",
            "blocked_provider_memory_kg_external_noop",
            "result_receipt_cannot_invoke_provider_write_memory_kg_or_externalize",
            serde_json::json!({
                "context_attachment_requested": true,
                "provider_invocation_requested": true,
                "model_invocation_requested": true,
                "memory_write_requested": true,
                "kg_write_requested": true,
                "external_send_requested": true,
                "public_claim_requested": true,
                "install_requested": true,
                "restart_requested": true,
                "secret_access_requested": true,
            }),
        ),
    ]);
    let result_receipt_fixture_count = result_receipt_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let mut denials = source_activation_command
        .get("denied_by_operator_review_acknowledgement_activation_command_noop_handoff")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_activation_command_noop_handoff_report_required",
        "activation_command_disabled_required",
        "activation_command_result_receipt_schema_registration_denied",
        "activation_command_result_receipt_schema_acceptance_denied",
        "activation_command_result_receipt_recording_denied",
        "activation_command_result_receipt_persistence_denied",
        "activation_command_result_receipt_acceptance_denied",
        "activation_command_result_receipt_materialization_denied",
        "activation_command_result_receipt_filesystem_write_denied",
        "activation_command_result_receipt_ledger_write_denied",
        "activation_command_result_receipt_indexing_denied",
        "activation_command_result_receipt_enqueue_denied",
        "activation_command_result_receipt_delivery_denied",
        "activation_command_result_receipt_export_denied",
        "activation_command_result_receipt_query_registration_denied",
        "activation_command_result_receipt_observability_recording_denied",
        "activation_command_result_receipt_hash_binding_denied",
        "activation_command_result_receipt_status_acceptance_denied",
        "completion_ack_recording_denied",
        "completion_ack_persistence_denied",
        "completion_ack_acceptance_denied",
        "operator_approval_from_receipt_denied",
        "activation_from_receipt_denied",
        "activation_request_from_receipt_denied",
        "dispatch_from_receipt_denied",
        "execution_from_receipt_denied",
        "context_injection_from_receipt_denied",
        "provider_model_invocation_denied",
        "memory_store_write_denied",
        "external_kg_read_denied",
        "live_kg_write_denied",
        "credential_secret_read_denied",
        "channel_delivery_denied",
        "external_public_claim_denied",
        "install_restart_denied",
        "active_binary_mutation_denied",
        "upstream_fetch_merge_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_activation_command.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_noop_handoff_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_NOOP_HANDOFF_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-noop-handoff-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_review_acknowledgement_activation_command_noop_handoff_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_noop_handoff_route_ready": source_activation_command_noop_handoff_ready,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_no_record_no_persist_no_accept_no_ack_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_v1",
            "activation_command_result_receipt_no_persistence_mode": "native_route_stdout_only_command_result_receipt_shapes_no_record_no_persist_no_accept_no_authority_no_live",
            "activation_command_result_receipt_no_persistence_decision": "operator_review_acknowledgement_activation_command_noop_handoff_cannot_create_or_authorize_result_receipts",
            "source_activation_command_fixture_count": source_u64("activation_command_fixture_count"),
            "source_blocked_activation_command_fixture_count": source_u64("blocked_activation_command_fixture_count"),
            "source_noop_activation_command_fixture_count": source_u64("noop_activation_command_fixture_count"),
            "source_accepted_activation_command_fixture_count": source_u64("accepted_activation_command_fixture_count"),
            "source_activation_command_performed_count": source_u64("activation_command_performed_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_surface_count": 14,
            "activation_command_result_receipt_surface_ready_count": 14,
            "activation_command_result_receipt_side_effect_free_surface_count": 14,
            "activation_command_result_receipt_fixtures": result_receipt_fixtures,
            "activation_command_result_receipt_fixture_count": result_receipt_fixture_count,
            "activation_command_result_receipt_requested_fixture_count": result_receipt_fixture_count,
            "blocked_activation_command_result_receipt_fixture_count": result_receipt_fixture_count,
            "noop_activation_command_result_receipt_fixture_count": result_receipt_fixture_count,
            "allowed_activation_command_result_receipt_fixture_count": 0,
            "accepted_activation_command_result_receipt_fixture_count": 0,
            "activation_command_result_receipt_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_shape_registered": false,
            "activation_command_result_receipt_allowed": false,
            "activation_command_result_receipt_schema_accepted": false,
            "activation_command_result_receipt_recorded": false,
            "activation_command_result_receipt_persisted": false,
            "activation_command_result_receipt_accepted": false,
            "activation_command_result_receipt_materialized": false,
            "activation_command_result_receipt_filesystem_written": false,
            "activation_command_result_receipt_ledger_written": false,
            "activation_command_result_receipt_indexed": false,
            "activation_command_result_receipt_enqueued": false,
            "activation_command_result_receipt_delivered": false,
            "activation_command_result_receipt_exported": false,
            "activation_command_result_receipt_query_registered": false,
            "activation_command_result_receipt_observability_recorded": false,
            "activation_command_completion_ack_recorded": false,
            "activation_command_completion_ack_persisted": false,
            "activation_command_completion_ack_accepted": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_approval_from_receipt_accepted": false,
            "activation_from_receipt_allowed": false,
            "activation_command_allowed": false,
            "activation_command_accepted": false,
            "activation_command_enabled": false,
            "activation_command_invoked": false,
            "activation_command_dispatched": false,
            "activation_command_handoff_recorded": false,
            "activation_command_handoff_persisted": false,
            "activation_request_accepted": false,
            "activation_request_recorded": false,
            "activation_request_persisted": false,
            "activation_request_executed": false,
            "operator_approval_recorded": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 18,
            "enablement_lane_count": 21,
            "ready_enablement_lane_count": 21,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_no_persistence": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_duplicate_receipt": false,
                    "records_idempotency": false,
                    "persists_replay_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "activation_command_result_receipt_shape_registered",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_result_receipt_exported",
            "activation_command_result_receipt_query_registered",
            "activation_command_result_receipt_observability_recorded",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_recorded",
            "activation_from_receipt_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_handoff_recorded",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
            "public_release_claimed",
            "public_ga_claimed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_no_persistence =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_report();
    let source_bool = |key: &str| {
        source_no_persistence
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_no_persistence
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_no_persistence
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_no_persistence_ready = source_status == "ready"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready",
        )
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_enabled",
        )
        && source_u64("activation_command_result_receipt_fixture_count") == 10
        && source_u64("blocked_activation_command_result_receipt_fixture_count") == 10
        && source_u64("noop_activation_command_result_receipt_fixture_count") == 10
        && source_u64("accepted_activation_command_result_receipt_fixture_count") == 0
        && source_u64("activation_command_result_receipt_performed_count") == 0
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_command_completion_ack_recorded")
        && !source_bool("activation_command_completion_ack_accepted")
        && !source_bool("operator_approval_from_receipt_accepted")
        && !source_bool("activation_from_receipt_allowed")
        && !source_bool("activation_command_enabled")
        && !source_bool("activation_command_invoked")
        && !source_bool("activation_command_dispatched")
        && !source_bool("activation_request_accepted")
        && !source_bool("activation_request_recorded")
        && !source_bool("activation_request_executed")
        && source_u64("dispatch_performed_count") == 0
        && source_u64("execution_performed_count") == 0
        && source_u64("context_injection_performed_count") == 0
        && source_u64("provider_invoked_count") == 0
        && source_u64("model_invoked_count") == 0
        && source_u64("memory_store_write_performed_count") == 0
        && source_u64("external_kg_adapter_read_performed_count") == 0
        && source_u64("live_kg_write_performed_count") == 0
        && source_u64("credential_read_count") == 0
        && source_u64("secret_file_read_count") == 0
        && source_u64("channel_send_performed_count") == 0
        && source_u64("current_live_enabled_lane_count") == 18
        && source_u64("enablement_lane_count") == 21
        && source_u64("ready_enablement_lane_count") == 21
        && !source_bool("canary_harness_armed")
        && !source_bool("canary_harness_executable")
        && !source_bool("canary_live_enabled");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_no_persistence_ready;

    let replay_fixture =
        |fixture_id: &str, status: &str, denial_reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "replay_idempotency_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(denial_reason.to_string()),
            );
            for key in [
                "source_result_receipt_no_persistence_present",
                "source_result_receipt_no_persistence_ready",
                "replay_requested",
                "canonical_blocked_noop_result_receipt_identity_required",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "activation_command_result_receipt_replay_allowed",
                "activation_command_result_receipt_replay_recorded",
                "activation_command_result_receipt_replay_persisted",
                "activation_command_result_receipt_replay_materialized",
                "activation_command_result_receipt_replay_filesystem_written",
                "activation_command_result_receipt_replay_performed",
                "activation_command_result_receipt_duplicate_accepted",
                "activation_command_result_receipt_duplicate_recorded",
                "activation_command_result_receipt_duplicate_persisted",
                "activation_command_result_receipt_idempotency_key_accepted",
                "activation_command_result_receipt_idempotency_key_recorded",
                "activation_command_result_receipt_idempotency_state_recorded",
                "activation_command_result_receipt_idempotency_state_persisted",
                "activation_command_result_receipt_idempotency_state_materialized",
                "activation_command_result_receipt_idempotency_filesystem_written",
                "activation_command_result_receipt_replay_nonce_accepted",
                "activation_command_result_receipt_replay_nonce_recorded",
                "activation_command_result_receipt_cross_scope_reuse_accepted",
                "activation_command_result_receipt_status_upgrade_accepted",
                "activation_command_result_receipt_completed_status_accepted",
                "activation_command_result_receipt_ack_replay_accepted",
                "activation_command_result_receipt_ledger_replay_accepted",
                "activation_command_result_receipt_index_replay_accepted",
                "activation_command_result_receipt_delivery_replay_accepted",
                "activation_command_result_receipt_export_replay_accepted",
                "activation_command_result_receipt_query_replay_accepted",
                "activation_command_result_receipt_observability_replay_accepted",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_result_receipt_ledger_written",
                "activation_command_result_receipt_indexed",
                "activation_command_result_receipt_enqueued",
                "activation_command_result_receipt_delivered",
                "activation_command_result_receipt_exported",
                "activation_command_result_receipt_query_registered",
                "activation_command_result_receipt_observability_recorded",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "activation_command_completion_ack_delivered",
                "operator_approval_from_replay_accepted",
                "operator_approval_from_receipt_accepted",
                "activation_from_replay_allowed",
                "activation_from_receipt_allowed",
                "activation_command_allowed",
                "activation_command_accepted",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_command_handoff_recorded",
                "activation_command_handoff_persisted",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "operator_approval_recorded",
                "dispatch_performed",
                "execution_performed",
                "context_injection_performed",
                "provider_invoked",
                "model_invoked",
                "memory_store_write_performed",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "credential_read",
                "secret_file_read",
                "channel_send_performed",
                "external_send_performed",
                "public_claim_performed",
                "install_performed",
                "service_restarted",
                "active_binary_mutated",
                "upstream_fetch_performed",
                "upstream_merge_performed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let replay_idempotency_fixtures = serde_json::Value::Array(vec![
        replay_fixture(
            "missing-source-result-receipt-no-persistence-report",
            "blocked_noop",
            "source_result_receipt_no_persistence_report_required",
            serde_json::json!({
                "source_result_receipt_no_persistence_present": false,
                "source_result_receipt_no_persistence_ready": false,
            }),
        ),
        replay_fixture(
            "acknowledgement-activation-command-result-receipt-duplicate-identity-replay-attempt",
            "blocked_duplicate_noop",
            "duplicate_result_receipt_identity_replay_denied",
            serde_json::json!({"duplicate_result_receipt_identity_requested": true}),
        ),
        replay_fixture(
            "acknowledgement-activation-command-result-receipt-replay-acceptance-attempt",
            "blocked_replay_noop",
            "result_receipt_replay_acceptance_denied",
            serde_json::json!({"result_receipt_replay_acceptance_requested": true}),
        ),
        replay_fixture(
            "acknowledgement-activation-command-result-receipt-idempotency-key-recording-attempt",
            "blocked_idempotency_key_noop",
            "idempotency_key_recording_denied",
            serde_json::json!({
                "idempotency_key_acceptance_requested": true,
                "idempotency_key_recording_requested": true,
            }),
        ),
        replay_fixture(
            "acknowledgement-activation-command-result-receipt-idempotency-state-persistence-attempt",
            "blocked_idempotency_state_noop",
            "idempotency_state_persistence_materialization_denied",
            serde_json::json!({
                "idempotency_state_recording_requested": true,
                "idempotency_state_persistence_requested": true,
                "idempotency_state_materialization_requested": true,
                "idempotency_filesystem_write_requested": true,
            }),
        ),
        replay_fixture(
            "acknowledgement-activation-command-result-receipt-cross-scope-reuse-attempt",
            "blocked_cross_scope_noop",
            "cross_scope_result_receipt_reuse_denied",
            serde_json::json!({"cross_scope_reuse_requested": true}),
        ),
        replay_fixture(
            "acknowledgement-activation-command-result-receipt-stale-nonce-out-of-order-replay-attempt",
            "blocked_nonce_order_noop",
            "stale_nonce_out_of_order_receipt_replay_denied",
            serde_json::json!({
                "stale_nonce_replay_requested": true,
                "out_of_order_replay_requested": true,
                "replay_nonce_acceptance_requested": true,
            }),
        ),
        replay_fixture(
            "acknowledgement-activation-command-result-receipt-completion-ledger-delivery-replay-attempt",
            "blocked_completion_ledger_delivery_noop",
            "completion_ack_ledger_delivery_replay_denied",
            serde_json::json!({
                "completion_ack_replay_requested": true,
                "ledger_replay_requested": true,
                "index_replay_requested": true,
                "delivery_replay_requested": true,
            }),
        ),
        replay_fixture(
            "acknowledgement-activation-command-result-receipt-activation-provider-memory-kg-replay-attempt",
            "blocked_activation_provider_memory_kg_noop",
            "activation_provider_memory_kg_replay_denied",
            serde_json::json!({
                "result_receipt_status_upgrade_requested": true,
                "completed_status_acceptance_requested": true,
                "operator_approval_from_replay_requested": true,
                "activation_from_replay_requested": true,
                "context_injection_replay_requested": true,
                "provider_replay_requested": true,
                "model_replay_requested": true,
                "memory_store_replay_requested": true,
                "external_kg_replay_requested": true,
                "live_kg_replay_requested": true,
            }),
        ),
        replay_fixture(
            "acknowledgement-activation-command-result-receipt-external-public-install-upstream-secret-replay-attempt",
            "blocked_external_noop",
            "external_public_install_restart_upstream_secret_replay_denied",
            serde_json::json!({
                "external_send_replay_requested": true,
                "public_claim_replay_requested": true,
                "release_artifact_replay_requested": true,
                "install_replay_requested": true,
                "launchd_restart_replay_requested": true,
                "service_restart_replay_requested": true,
                "active_binary_mutation_replay_requested": true,
                "upstream_replay_requested": true,
                "credential_replay_requested": true,
                "secret_value_replay_requested": true,
            }),
        ),
    ]);
    let replay_idempotency_fixture_count = replay_idempotency_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let mut denials = source_no_persistence
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_no_persistence")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_result_receipt_no_persistence_report_required",
        "canonical_blocked_noop_result_receipt_identity_required",
        "duplicate_result_receipt_identity_replay_denied",
        "result_receipt_replay_acceptance_denied",
        "idempotency_key_acceptance_denied",
        "idempotency_key_recording_denied",
        "idempotency_state_recording_denied",
        "idempotency_state_persistence_denied",
        "idempotency_state_materialization_denied",
        "idempotency_filesystem_write_denied",
        "cross_scope_result_receipt_reuse_denied",
        "stale_nonce_replay_denied",
        "out_of_order_receipt_replay_denied",
        "completion_ack_replay_denied",
        "ledger_index_delivery_replay_denied",
        "export_query_observability_replay_denied",
        "status_upgrade_replay_denied",
        "completed_status_replay_denied",
        "operator_approval_from_replay_denied",
        "activation_from_replay_denied",
        "context_injection_replay_denied",
        "provider_model_replay_denied",
        "memory_store_replay_denied",
        "external_kg_replay_denied",
        "live_kg_replay_denied",
        "credential_secret_replay_denied",
        "external_public_install_restart_replay_denied",
        "active_binary_mutation_replay_denied",
        "upstream_replay_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_no_persistence.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_ready": source_no_persistence_ready,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_no_replay_no_duplicate_no_idempotency_record_no_persist_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_v1",
            "activation_command_result_receipt_replay_idempotency_mode": "native_route_stdout_only_duplicate_replay_and_idempotency_denial_no_record_no_persist_no_authority_no_live",
            "activation_command_result_receipt_replay_idempotency_decision": "operator_review_acknowledgement_activation_command_result_receipt_cannot_be_replayed_duplicated_or_converted_into_idempotency_authority",
            "source_activation_command_result_receipt_fixture_count": source_u64("activation_command_result_receipt_fixture_count"),
            "source_blocked_activation_command_result_receipt_fixture_count": source_u64("blocked_activation_command_result_receipt_fixture_count"),
            "source_noop_activation_command_result_receipt_fixture_count": source_u64("noop_activation_command_result_receipt_fixture_count"),
            "source_accepted_activation_command_result_receipt_fixture_count": source_u64("accepted_activation_command_result_receipt_fixture_count"),
            "source_activation_command_result_receipt_performed_count": source_u64("activation_command_result_receipt_performed_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "replay_idempotency_surface_count": 14,
            "replay_idempotency_surface_ready_count": 14,
            "replay_idempotency_side_effect_free_surface_count": 14,
            "replay_idempotency_fixtures": replay_idempotency_fixtures,
            "replay_idempotency_fixture_count": replay_idempotency_fixture_count,
            "blocked_replay_idempotency_fixture_count": replay_idempotency_fixture_count,
            "noop_replay_idempotency_fixture_count": replay_idempotency_fixture_count,
            "allowed_replay_idempotency_fixture_count": 0,
            "accepted_replay_idempotency_fixture_count": 0,
            "duplicate_result_receipt_replay_fixture_count": 1,
            "result_receipt_replay_acceptance_fixture_count": 1,
            "idempotency_key_recording_fixture_count": 1,
            "idempotency_state_persistence_fixture_count": 1,
            "cross_scope_result_receipt_reuse_fixture_count": 1,
            "stale_nonce_out_of_order_replay_fixture_count": 1,
            "completion_ledger_delivery_replay_fixture_count": 1,
            "activation_provider_memory_kg_replay_fixture_count": 1,
            "external_public_install_upstream_secret_replay_fixture_count": 1,
            "replay_idempotency_denied_count": replay_idempotency_fixture_count,
            "replay_idempotency_performed_count": 0,
            "duplicate_result_receipt_accepted_count": 0,
            "idempotency_state_recorded_count": 0,
            "idempotency_state_persisted_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_replay_allowed": false,
            "activation_command_result_receipt_replay_recorded": false,
            "activation_command_result_receipt_replay_persisted": false,
            "activation_command_result_receipt_replay_materialized": false,
            "activation_command_result_receipt_replay_filesystem_written": false,
            "activation_command_result_receipt_replay_performed": false,
            "activation_command_result_receipt_duplicate_accepted": false,
            "activation_command_result_receipt_duplicate_recorded": false,
            "activation_command_result_receipt_duplicate_persisted": false,
            "activation_command_result_receipt_idempotency_key_accepted": false,
            "activation_command_result_receipt_idempotency_key_recorded": false,
            "activation_command_result_receipt_idempotency_state_recorded": false,
            "activation_command_result_receipt_idempotency_state_persisted": false,
            "activation_command_result_receipt_idempotency_state_materialized": false,
            "activation_command_result_receipt_idempotency_filesystem_written": false,
            "activation_command_result_receipt_replay_nonce_accepted": false,
            "activation_command_result_receipt_replay_nonce_recorded": false,
            "activation_command_result_receipt_cross_scope_reuse_accepted": false,
            "activation_command_result_receipt_status_upgrade_accepted": false,
            "activation_command_result_receipt_completed_status_accepted": false,
            "activation_command_result_receipt_ack_replay_accepted": false,
            "activation_command_result_receipt_ledger_replay_accepted": false,
            "activation_command_result_receipt_index_replay_accepted": false,
            "activation_command_result_receipt_delivery_replay_accepted": false,
            "activation_command_result_receipt_export_replay_accepted": false,
            "activation_command_result_receipt_query_replay_accepted": false,
            "activation_command_result_receipt_observability_replay_accepted": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_recorded": false,
            "activation_command_result_receipt_persisted": false,
            "activation_command_result_receipt_accepted": false,
            "activation_command_completion_ack_recorded": false,
            "activation_command_completion_ack_persisted": false,
            "activation_command_completion_ack_accepted": false,
            "operator_approval_from_replay_accepted": false,
            "operator_approval_from_receipt_accepted": false,
            "activation_from_replay_allowed": false,
            "activation_from_receipt_allowed": false,
            "activation_command_allowed": false,
            "activation_command_accepted": false,
            "activation_command_enabled": false,
            "activation_command_invoked": false,
            "activation_command_dispatched": false,
            "activation_command_handoff_recorded": false,
            "activation_command_handoff_persisted": false,
            "activation_request_accepted": false,
            "activation_request_recorded": false,
            "activation_request_persisted": false,
            "activation_request_executed": false,
            "operator_approval_recorded": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 19,
            "enablement_lane_count": 22,
            "ready_enablement_lane_count": 22,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial",
                    "status": "allowed_report_only",
                    "accepts_duplicate_receipt": false,
                    "records_idempotency": false,
                    "persists_replay_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_out_of_order_receipt": false,
                    "records_sequence_cursor": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "activation_command_result_receipt_replay_recorded",
            "activation_command_result_receipt_replay_persisted",
            "activation_command_result_receipt_replay_performed",
            "activation_command_result_receipt_duplicate_accepted",
            "activation_command_result_receipt_duplicate_recorded",
            "activation_command_result_receipt_duplicate_persisted",
            "activation_command_result_receipt_idempotency_key_recorded",
            "activation_command_result_receipt_idempotency_state_recorded",
            "activation_command_result_receipt_idempotency_state_persisted",
            "activation_command_result_receipt_idempotency_state_materialized",
            "activation_command_result_receipt_idempotency_filesystem_written",
            "activation_command_result_receipt_replay_nonce_recorded",
            "activation_command_result_receipt_cross_scope_reuse_accepted",
            "activation_command_result_receipt_status_upgrade_accepted",
            "activation_command_result_receipt_completed_status_accepted",
            "activation_command_result_receipt_ack_replay_accepted",
            "activation_command_result_receipt_ledger_replay_accepted",
            "activation_command_result_receipt_index_replay_accepted",
            "activation_command_result_receipt_delivery_replay_accepted",
            "activation_command_result_receipt_export_replay_accepted",
            "activation_command_result_receipt_query_replay_accepted",
            "activation_command_result_receipt_observability_replay_accepted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_replay_accepted",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_handoff_recorded",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
            "public_release_claimed",
            "public_ga_claimed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let source_replay =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_report();
    let source_status = source_replay
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let source_replay_ready = source_status == "blocked"
        && source_replay
            .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        && source_replay
            .get("accepted_replay_idempotency_fixture_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0
        && source_replay
            .get("replay_idempotency_performed_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0;
    let report_ready = source_replay_ready && route_count_source_command_accepted;
    let source_u64 = |key: &str| {
        source_replay
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };

    let ordering_fixture = |fixture_id: &str,
                            status: &str,
                            reason: &str,
                            extra: serde_json::Value| {
        let mut fixture = serde_json::Map::new();
        for (key, value) in [
            ("fixture_id", fixture_id),
            ("ordering_monotonicity_status", status),
            ("denial_reason", reason),
        ] {
            fixture.insert(
                key.to_string(),
                serde_json::Value::String(value.to_string()),
            );
        }
        fixture.insert(
            "source_replay_idempotency_present".to_string(),
            serde_json::Value::Bool(true),
        );
        fixture.insert(
            "source_replay_idempotency_ready".to_string(),
            serde_json::Value::Bool(true),
        );
        fixture.insert(
            "ordering_requested".to_string(),
            serde_json::Value::Bool(true),
        );
        fixture.insert(
            "canonical_blocked_noop_result_receipt_order_identity_required".to_string(),
            serde_json::Value::Bool(true),
        );
        fixture.insert(
            "receipt_noop_confirmed".to_string(),
            serde_json::Value::Bool(true),
        );
        for key in [
            "activation_command_result_receipt_ordering_allowed",
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_ordering_materialized",
            "activation_command_result_receipt_ordering_filesystem_written",
            "activation_command_result_receipt_ordering_performed",
            "activation_command_result_receipt_sequence_cursor_accepted",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_command_result_receipt_monotonicity_state_materialized",
            "activation_command_result_receipt_monotonicity_filesystem_written",
            "activation_command_result_receipt_out_of_order_accepted",
            "activation_command_result_receipt_stale_sequence_accepted",
            "activation_command_result_receipt_future_sequence_accepted",
            "activation_command_result_receipt_sequence_gap_accepted",
            "activation_command_result_receipt_timestamp_rollback_accepted",
            "activation_command_result_receipt_epoch_rollback_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_ack_before_noop_accepted",
            "activation_command_result_receipt_ledger_ordering_bypass_accepted",
            "activation_command_result_receipt_index_ordering_bypass_accepted",
            "activation_command_result_receipt_delivery_ordering_bypass_accepted",
            "activation_command_result_receipt_export_ordering_bypass_accepted",
            "activation_command_result_receipt_query_ordering_bypass_accepted",
            "activation_command_result_receipt_observability_ordering_bypass_accepted",
            "activation_command_result_receipt_provider_ordering_bypass_accepted",
            "activation_command_result_receipt_memory_kg_ordering_bypass_accepted",
            "activation_command_result_receipt_external_public_install_ordering_bypass_accepted",
            "activation_command_result_receipt_replay_allowed",
            "activation_command_result_receipt_replay_recorded",
            "activation_command_result_receipt_replay_persisted",
            "activation_command_result_receipt_duplicate_accepted",
            "activation_command_result_receipt_idempotency_key_accepted",
            "activation_command_result_receipt_idempotency_state_recorded",
            "activation_command_result_receipt_idempotency_state_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_ordering_accepted",
            "activation_from_ordering_allowed",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_allowed",
            "activation_command_accepted",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
        ] {
            fixture.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        let mut fixture = serde_json::Value::Object(fixture);
        extend_json_object(&mut fixture, extra);
        fixture
    };
    let ordering_monotonicity_fixtures = serde_json::Value::Array(vec![
        ordering_fixture(
            "missing-source-replay-idempotency-report",
            "blocked_noop",
            "source_result_receipt_replay_idempotency_report_required",
            serde_json::json!({
                "source_replay_idempotency_present": false,
                "source_replay_idempotency_ready": false,
            }),
        ),
        ordering_fixture(
            "sequence-cursor-recording-attempt",
            "blocked_sequence_cursor_noop",
            "sequence_cursor_recording_denied",
            serde_json::json!({
                "sequence_cursor_recording_requested": true,
                "requested_sequence_cursor": "operator_canary_ack_result_receipt_sequence_1",
            }),
        ),
        ordering_fixture(
            "out-of-order-sequence-attempt",
            "blocked_out_of_order_noop",
            "out_of_order_result_receipt_sequence_denied",
            serde_json::json!({
                "out_of_order_sequence_requested": true,
                "requested_sequence": 2,
                "observed_previous_sequence": 3,
            }),
        ),
        ordering_fixture(
            "stale-sequence-replay-attempt",
            "blocked_stale_sequence_noop",
            "stale_sequence_result_receipt_replay_denied",
            serde_json::json!({
                "stale_sequence_requested": true,
                "requested_sequence": 1,
                "observed_previous_sequence": 3,
            }),
        ),
        ordering_fixture(
            "future-sequence-gap-attempt",
            "blocked_future_sequence_noop",
            "future_sequence_gap_result_receipt_denied",
            serde_json::json!({
                "future_sequence_requested": true,
                "requested_sequence": 5,
                "expected_next_sequence": 1,
            }),
        ),
        ordering_fixture(
            "timestamp-epoch-rollback-attempt",
            "blocked_rollback_noop",
            "timestamp_epoch_rollback_result_receipt_denied",
            serde_json::json!({
                "timestamp_rollback_requested": true,
                "epoch_rollback_requested": true,
            }),
        ),
        ordering_fixture(
            "same-sequence-different-hash-attempt",
            "blocked_same_sequence_hash_noop",
            "same_sequence_different_hash_result_receipt_denied",
            serde_json::json!({
                "same_sequence_different_hash_requested": true,
                "requested_sequence": 1,
                "requested_hash_relation": "different_hash_for_same_sequence",
            }),
        ),
        ordering_fixture(
            "latest-wins-overwrite-attempt",
            "blocked_latest_wins_noop",
            "latest_wins_result_receipt_overwrite_denied",
            serde_json::json!({
                "latest_wins_overwrite_requested": true,
                "overwrite_existing_noop_requested": true,
            }),
        ),
        ordering_fixture(
            "ack-ledger-index-delivery-ordering-bypass-attempt",
            "blocked_ledger_delivery_noop",
            "ack_ledger_index_delivery_ordering_bypass_denied",
            serde_json::json!({
                "completion_ack_before_noop_requested": true,
                "ledger_ordering_bypass_requested": true,
                "index_ordering_bypass_requested": true,
                "delivery_ordering_bypass_requested": true,
                "export_ordering_bypass_requested": true,
                "query_ordering_bypass_requested": true,
                "observability_ordering_bypass_requested": true,
            }),
        ),
        ordering_fixture(
            "activation-provider-memory-kg-external-ordering-bypass-attempt",
            "blocked_activation_provider_memory_kg_external_noop",
            "activation_provider_memory_kg_external_ordering_bypass_denied",
            serde_json::json!({
                "operator_approval_from_ordering_requested": true,
                "activation_from_ordering_requested": true,
                "context_injection_ordering_bypass_requested": true,
                "provider_ordering_bypass_requested": true,
                "model_ordering_bypass_requested": true,
                "memory_store_ordering_bypass_requested": true,
                "external_kg_ordering_bypass_requested": true,
                "live_kg_ordering_bypass_requested": true,
                "external_send_ordering_bypass_requested": true,
                "public_claim_ordering_bypass_requested": true,
                "install_ordering_bypass_requested": true,
                "service_restart_ordering_bypass_requested": true,
                "active_binary_mutation_ordering_bypass_requested": true,
                "upstream_ordering_bypass_requested": true,
                "credential_ordering_bypass_requested": true,
                "secret_value_ordering_bypass_requested": true,
            }),
        ),
    ]);
    let ordering_monotonicity_fixture_count = ordering_monotonicity_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let ordering_monotonicity_fixtures_sha256 = sha256_json_value(&ordering_monotonicity_fixtures);
    let source_replay_report_sha256 = sha256_json_value(&source_replay);
    let source_replay_contract_hash_sha256 = source_replay
        .get("replay_idempotency_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_result_receipt_no_persistence_hash_sha256 = source_replay
        .get("source_result_receipt_no_persistence_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let ordering_monotonicity_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial:v1:source={source_replay_report_sha256}:replay={source_replay_contract_hash_sha256}:receipt={source_result_receipt_no_persistence_hash_sha256}:fixtures={ordering_monotonicity_fixtures_sha256}:ordering=0:cursor=0:monotonicity=0:persist=0:authority=0:live=0"
    ));
    let ordering_monotonicity_policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial:v1:no-ordering:no-sequence-cursor:no-monotonicity-state:no-latest-wins:no-ack-ledger-bypass:no-authority:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_side_effects=false;fixtures=10;ordering=0;cursor=0;monotonicity=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );

    let mut denials = source_replay
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_result_receipt_replay_idempotency_report_required",
        "canonical_blocked_noop_result_receipt_order_identity_required",
        "sequence_cursor_acceptance_denied",
        "sequence_cursor_recording_denied",
        "sequence_cursor_persistence_denied",
        "monotonicity_state_recording_denied",
        "monotonicity_state_persistence_denied",
        "monotonicity_state_materialization_denied",
        "monotonicity_filesystem_write_denied",
        "out_of_order_sequence_denied",
        "stale_sequence_denied",
        "future_sequence_denied",
        "sequence_gap_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "completion_ack_before_noop_denied",
        "ledger_index_delivery_ordering_bypass_denied",
        "export_query_observability_ordering_bypass_denied",
        "operator_approval_from_ordering_denied",
        "activation_from_ordering_denied",
        "context_injection_ordering_bypass_denied",
        "provider_model_ordering_bypass_denied",
        "memory_kg_ordering_bypass_denied",
        "credential_secret_ordering_bypass_denied",
        "external_public_install_restart_ordering_bypass_denied",
        "active_binary_mutation_ordering_bypass_denied",
        "upstream_ordering_bypass_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_replay.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_route_ready": source_replay_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_report_sha256": source_replay_report_sha256,
            "source_replay_idempotency_contract_hash_sha256": source_replay_contract_hash_sha256,
            "source_result_receipt_no_persistence_hash_sha256": source_result_receipt_no_persistence_hash_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_no_sequence_cursor_no_monotonicity_state_no_ordering_record_no_persist_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_v1",
            "ordering_monotonicity_mode": "native_route_stdout_only_sequence_cursor_and_monotonicity_denial_no_record_no_persist_no_authority_no_live",
            "ordering_monotonicity_decision": "operator_review_acknowledgement_activation_command_result_receipt_cannot_create_ordering_sequence_cursor_or_monotonicity_authority",
            "minimum_required_samples": 24,
            "ordering_monotonicity_fixtures_sha256": ordering_monotonicity_fixtures_sha256,
            "ordering_monotonicity_contract_hash_sha256": ordering_monotonicity_contract_hash_sha256,
            "ordering_monotonicity_policy_hash_sha256": ordering_monotonicity_policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "source_activation_command_result_receipt_surface_count": source_u64("source_activation_command_result_receipt_surface_count"),
            "source_activation_command_result_receipt_fixture_count": source_u64("source_activation_command_result_receipt_fixture_count"),
            "source_accepted_activation_command_result_receipt_fixture_count": source_u64("source_accepted_activation_command_result_receipt_fixture_count"),
            "source_replay_idempotency_fixture_count": source_u64("replay_idempotency_fixture_count"),
            "source_blocked_replay_idempotency_fixture_count": source_u64("blocked_replay_idempotency_fixture_count"),
            "source_noop_replay_idempotency_fixture_count": source_u64("noop_replay_idempotency_fixture_count"),
            "source_accepted_replay_idempotency_fixture_count": source_u64("accepted_replay_idempotency_fixture_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "ordering_monotonicity_surface_count": 14,
            "ordering_monotonicity_surface_ready_count": 14,
            "ordering_monotonicity_side_effect_free_surface_count": 14,
            "ordering_monotonicity_fixtures": ordering_monotonicity_fixtures,
            "ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "blocked_ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "noop_ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "allowed_ordering_monotonicity_fixture_count": 0,
            "accepted_ordering_monotonicity_fixture_count": 0,
            "sequence_cursor_recording_fixture_count": 1,
            "out_of_order_sequence_fixture_count": 1,
            "stale_sequence_fixture_count": 1,
            "future_sequence_gap_fixture_count": 1,
            "timestamp_epoch_rollback_fixture_count": 1,
            "same_sequence_hash_fixture_count": 1,
            "latest_wins_overwrite_fixture_count": 1,
            "ack_ledger_index_delivery_bypass_fixture_count": 1,
            "activation_provider_memory_kg_external_bypass_fixture_count": 1,
            "ordering_monotonicity_denied_count": ordering_monotonicity_fixture_count,
            "ordering_monotonicity_performed_count": 0,
            "sequence_cursor_accepted_count": 0,
            "sequence_cursor_recorded_count": 0,
            "monotonicity_state_recorded_count": 0,
            "monotonicity_state_persisted_count": 0,
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_ordering_allowed",
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_ordering_materialized",
            "activation_command_result_receipt_ordering_filesystem_written",
            "activation_command_result_receipt_ordering_performed",
            "activation_command_result_receipt_sequence_cursor_accepted",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_command_result_receipt_monotonicity_state_materialized",
            "activation_command_result_receipt_monotonicity_filesystem_written",
            "activation_command_result_receipt_out_of_order_accepted",
            "activation_command_result_receipt_stale_sequence_accepted",
            "activation_command_result_receipt_future_sequence_accepted",
            "activation_command_result_receipt_sequence_gap_accepted",
            "activation_command_result_receipt_timestamp_rollback_accepted",
            "activation_command_result_receipt_epoch_rollback_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_ack_before_noop_accepted",
            "activation_command_result_receipt_ledger_ordering_bypass_accepted",
            "activation_command_result_receipt_index_ordering_bypass_accepted",
            "activation_command_result_receipt_delivery_ordering_bypass_accepted",
            "activation_command_result_receipt_export_ordering_bypass_accepted",
            "activation_command_result_receipt_query_ordering_bypass_accepted",
            "activation_command_result_receipt_observability_ordering_bypass_accepted",
            "activation_command_result_receipt_provider_ordering_bypass_accepted",
            "activation_command_result_receipt_memory_kg_ordering_bypass_accepted",
            "activation_command_result_receipt_external_public_install_ordering_bypass_accepted",
            "activation_command_result_receipt_replay_allowed",
            "activation_command_result_receipt_replay_recorded",
            "activation_command_result_receipt_replay_persisted",
            "activation_command_result_receipt_duplicate_accepted",
            "activation_command_result_receipt_idempotency_key_accepted",
            "activation_command_result_receipt_idempotency_state_recorded",
            "activation_command_result_receipt_idempotency_state_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_ordering_accepted",
            "operator_approval_from_replay_accepted",
            "operator_approval_from_receipt_accepted",
            "activation_from_ordering_allowed",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_allowed",
            "activation_command_accepted",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
        ] {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 20,
            "enablement_lane_count": 23,
            "ready_enablement_lane_count": 23,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial",
                    "status": "allowed_report_only",
                    "records_sequence_cursor": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_ordering_performed",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_command_result_receipt_monotonicity_state_materialized",
            "activation_command_result_receipt_monotonicity_filesystem_written",
            "activation_command_result_receipt_out_of_order_accepted",
            "activation_command_result_receipt_stale_sequence_accepted",
            "activation_command_result_receipt_future_sequence_accepted",
            "activation_command_result_receipt_sequence_gap_accepted",
            "activation_command_result_receipt_timestamp_rollback_accepted",
            "activation_command_result_receipt_epoch_rollback_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_ack_before_noop_accepted",
            "activation_command_result_receipt_ledger_ordering_bypass_accepted",
            "activation_command_result_receipt_index_ordering_bypass_accepted",
            "activation_command_result_receipt_delivery_ordering_bypass_accepted",
            "activation_command_result_receipt_export_ordering_bypass_accepted",
            "activation_command_result_receipt_query_ordering_bypass_accepted",
            "activation_command_result_receipt_observability_ordering_bypass_accepted",
            "activation_command_result_receipt_provider_ordering_bypass_accepted",
            "activation_command_result_receipt_memory_kg_ordering_bypass_accepted",
            "activation_command_result_receipt_external_public_install_ordering_bypass_accepted",
            "activation_command_result_receipt_replay_recorded",
            "activation_command_result_receipt_replay_persisted",
            "activation_command_result_receipt_duplicate_accepted",
            "activation_command_result_receipt_idempotency_state_recorded",
            "activation_command_result_receipt_idempotency_state_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_completion_ack_recorded",
            "operator_approval_from_ordering_accepted",
            "activation_from_ordering_allowed",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
            "public_release_claimed",
            "public_ga_claimed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let source_ordering =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_report();
    let source_status = source_ordering
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let source_ordering_ready = source_status == "blocked"
        && source_ordering
            .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        && source_ordering
            .get("accepted_ordering_monotonicity_fixture_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0
        && source_ordering
            .get("ordering_monotonicity_performed_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0
        && source_ordering
            .get("sequence_cursor_recorded_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0
        && source_ordering
            .get("monotonicity_state_recorded_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0;
    let report_ready = source_ordering_ready && route_count_source_command_accepted;
    let source_u64 = |key: &str| {
        source_ordering
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };

    let cancellation_supersession_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            for (key, value) in [
                ("fixture_id", fixture_id),
                ("cancellation_supersession_status", status),
                ("denial_reason", reason),
            ] {
                fixture.insert(
                    key.to_string(),
                    serde_json::Value::String(value.to_string()),
                );
            }
            for (key, value) in [
                ("source_ordering_monotonicity_present", true),
                ("source_ordering_monotonicity_ready", true),
                ("cancellation_requested", true),
                ("supersession_requested", false),
                (
                    "canonical_blocked_noop_result_receipt_identity_required",
                    true,
                ),
                ("receipt_noop_confirmed", true),
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(value));
            }
            for key in [
                "activation_command_result_receipt_cancellation_allowed",
                "activation_command_result_receipt_cancellation_recorded",
                "activation_command_result_receipt_cancellation_persisted",
                "activation_command_result_receipt_cancellation_materialized",
                "activation_command_result_receipt_cancellation_filesystem_written",
                "activation_command_result_receipt_cancellation_request_accepted",
                "activation_command_result_receipt_supersession_allowed",
                "activation_command_result_receipt_supersession_recorded",
                "activation_command_result_receipt_supersession_persisted",
                "activation_command_result_receipt_supersession_materialized",
                "activation_command_result_receipt_supersession_filesystem_written",
                "activation_command_result_receipt_supersession_request_accepted",
                "activation_command_result_receipt_replacement_receipt_accepted",
                "activation_command_result_receipt_replacement_receipt_recorded",
                "activation_command_result_receipt_replacement_receipt_persisted",
                "activation_command_result_receipt_replacement_hash_accepted",
                "activation_command_result_receipt_tombstone_recorded",
                "activation_command_result_receipt_tombstone_persisted",
                "activation_command_result_receipt_delete_marker_recorded",
                "activation_command_result_receipt_ack_cancellation_accepted",
                "activation_command_result_receipt_ledger_cancellation_accepted",
                "activation_command_result_receipt_index_cancellation_accepted",
                "activation_command_result_receipt_delivery_cancellation_accepted",
                "activation_command_result_receipt_export_cancellation_accepted",
                "activation_command_result_receipt_query_cancellation_accepted",
                "activation_command_result_receipt_observability_cancellation_accepted",
                "activation_command_result_receipt_ordering_allowed",
                "activation_command_result_receipt_ordering_recorded",
                "activation_command_result_receipt_ordering_persisted",
                "activation_command_result_receipt_sequence_cursor_accepted",
                "activation_command_result_receipt_sequence_cursor_recorded",
                "activation_command_result_receipt_sequence_cursor_persisted",
                "activation_command_result_receipt_monotonicity_state_recorded",
                "activation_command_result_receipt_monotonicity_state_persisted",
                "activation_command_result_receipt_latest_wins_overwrite_accepted",
                "activation_command_result_receipt_same_sequence_hash_override_accepted",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_result_receipt_ledger_written",
                "activation_command_result_receipt_indexed",
                "activation_command_result_receipt_enqueued",
                "activation_command_result_receipt_delivered",
                "activation_command_result_receipt_exported",
                "activation_command_result_receipt_query_registered",
                "activation_command_result_receipt_observability_recorded",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "activation_command_completion_ack_delivered",
                "operator_approval_from_cancellation_accepted",
                "operator_approval_from_supersession_accepted",
                "operator_approval_from_ordering_accepted",
                "operator_approval_from_replay_accepted",
                "operator_approval_from_receipt_accepted",
                "activation_from_cancellation_allowed",
                "activation_from_supersession_allowed",
                "activation_from_ordering_allowed",
                "activation_from_replay_allowed",
                "activation_from_receipt_allowed",
                "activation_command_allowed",
                "activation_command_accepted",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "operator_approval_recorded",
                "dispatch_performed",
                "execution_performed",
                "context_injection_performed",
                "provider_invoked",
                "model_invoked",
                "memory_store_write_performed",
                "memory_store_mutated",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "credential_read",
                "secret_file_read",
                "auth_secret_read",
                "secret_value_read",
                "raw_payload_plaintext_recorded",
                "raw_payload_plaintext_persisted",
                "channel_send_performed",
                "telegram_send_performed",
                "external_send_performed",
                "public_claim_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_performed",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "service_restart_performed",
                "active_binary_mutated",
                "upstream_fetch_performed",
                "upstream_merge_performed",
                "rollback_executed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let cancellation_supersession_fixtures = serde_json::Value::Array(vec![
        cancellation_supersession_fixture(
            "missing-source-ordering-monotonicity-report",
            "blocked_noop",
            "source_result_receipt_ordering_monotonicity_report_required",
            serde_json::json!({
                "source_ordering_monotonicity_present": false,
                "source_ordering_monotonicity_ready": false,
            }),
        ),
        cancellation_supersession_fixture(
            "cancel-blocked-noop-result-receipt",
            "blocked_cancellation_noop",
            "cancellation_of_blocked_noop_result_receipt_denied",
            serde_json::json!({
                "cancellation_request_shape": "cancel_blocked_noop_result_receipt",
            }),
        ),
        cancellation_supersession_fixture(
            "supersede-blocked-noop-with-completed-result-receipt",
            "blocked_supersession_noop",
            "supersession_of_blocked_noop_with_completed_result_receipt_denied",
            serde_json::json!({
                "cancellation_requested": false,
                "supersession_requested": true,
                "requested_replacement_status": "completed",
            }),
        ),
        cancellation_supersession_fixture(
            "replacement-receipt-recording-persistence-attempt",
            "blocked_supersession_noop",
            "replacement_receipt_recording_persistence_denied",
            serde_json::json!({
                "cancellation_requested": false,
                "supersession_requested": true,
                "replacement_receipt_requested": true,
                "replacement_hash_requested": true,
                "requested_hash_relation": "different_hash_for_same_receipt_identity",
            }),
        ),
        cancellation_supersession_fixture(
            "tombstone-delete-marker-attempt",
            "blocked_cancellation_noop",
            "tombstone_delete_marker_denied",
            serde_json::json!({
                "tombstone_requested": true,
                "delete_marker_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "completion-acknowledgement-cancellation-replacement-attempt",
            "blocked_cancellation_supersession_noop",
            "completion_acknowledgement_cancellation_replacement_denied",
            serde_json::json!({
                "completion_ack_cancellation_requested": true,
                "ack_cancellation_requested": true,
                "supersession_requested": true,
                "requested_ack_replacement_status": "accepted",
            }),
        ),
        cancellation_supersession_fixture(
            "ledger-index-delivery-export-query-observability-bypass-attempt",
            "blocked_ledger_index_delivery_noop",
            "ledger_index_delivery_export_query_observability_cancellation_supersession_bypass_denied",
            serde_json::json!({
                "ledger_cancellation_requested": true,
                "index_cancellation_requested": true,
                "delivery_cancellation_requested": true,
                "export_cancellation_requested": true,
                "query_cancellation_requested": true,
                "observability_cancellation_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "context-provider-model-memory-kg-supersession-attempt",
            "blocked_context_provider_memory_kg_noop",
            "context_provider_model_memory_kg_supersession_denied",
            serde_json::json!({
                "cancellation_requested": false,
                "supersession_requested": true,
                "context_injection_supersession_requested": true,
                "provider_supersession_requested": true,
                "model_supersession_requested": true,
                "memory_store_supersession_requested": true,
                "external_kg_supersession_requested": true,
                "live_kg_supersession_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "rollback-secret-external-public-install-supersession-attempt",
            "blocked_secret_external_install_noop",
            "rollback_secret_external_public_install_supersession_denied",
            serde_json::json!({
                "cancellation_requested": false,
                "supersession_requested": true,
                "rollback_supersession_requested": true,
                "credential_secret_supersession_requested": true,
                "external_send_supersession_requested": true,
                "public_claim_supersession_requested": true,
                "release_artifact_supersession_requested": true,
                "install_supersession_requested": true,
                "service_restart_supersession_requested": true,
                "active_binary_mutation_supersession_requested": true,
                "upstream_supersession_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "latest-wins-sequence-cursor-cancellation-supersession-bypass-attempt",
            "blocked_latest_wins_cursor_noop",
            "latest_wins_sequence_cursor_cancellation_supersession_bypass_denied",
            serde_json::json!({
                "latest_wins_cancellation_bypass_requested": true,
                "latest_wins_supersession_bypass_requested": true,
                "sequence_cursor_cancellation_bypass_requested": true,
                "monotonicity_state_supersession_bypass_requested": true,
            }),
        ),
    ]);
    let cancellation_supersession_fixture_count = cancellation_supersession_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let cancellation_fixture_count = cancellation_supersession_fixtures
        .as_array()
        .map(|fixtures| {
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture
                        .get("cancellation_requested")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0);
    let supersession_fixture_count = cancellation_supersession_fixtures
        .as_array()
        .map(|fixtures| {
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture
                        .get("supersession_requested")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0);
    let cancellation_supersession_fixtures_sha256 =
        sha256_json_value(&cancellation_supersession_fixtures);
    let source_ordering_report_sha256 = sha256_json_value(&source_ordering);
    let source_ordering_contract_hash_sha256 = source_ordering
        .get("ordering_monotonicity_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_ordering_policy_hash_sha256 = source_ordering
        .get("ordering_monotonicity_policy_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_replay_idempotency_report_sha256 = source_ordering
        .get("source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_report_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let cancellation_supersession_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial:v1:source={source_ordering_report_sha256}:ordering={source_ordering_contract_hash_sha256}:replay={source_replay_idempotency_report_sha256}:fixtures={cancellation_supersession_fixtures_sha256}:cancel=0:supersede=0:replace=0:persist=0:authority=0:live=0"
    ));
    let cancellation_supersession_policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial:v1:no-cancel:no-supersede:no-replacement:no-tombstone:no-delete:no-authority:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_side_effects=false;fixtures=10;cancel=0;supersede=0;replacement=0;tombstone=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );

    let mut denials = source_ordering
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_result_receipt_ordering_monotonicity_report_required",
        "canonical_blocked_noop_result_receipt_identity_required",
        "cancellation_request_acceptance_denied",
        "cancellation_recording_denied",
        "cancellation_persistence_denied",
        "cancellation_materialization_denied",
        "cancellation_filesystem_write_denied",
        "supersession_request_acceptance_denied",
        "supersession_recording_denied",
        "supersession_persistence_denied",
        "supersession_materialization_denied",
        "supersession_filesystem_write_denied",
        "replacement_receipt_acceptance_denied",
        "replacement_receipt_recording_denied",
        "replacement_receipt_persistence_denied",
        "replacement_hash_acceptance_denied",
        "tombstone_recording_denied",
        "delete_marker_recording_denied",
        "completion_acknowledgement_cancellation_denied",
        "ledger_index_delivery_cancellation_denied",
        "export_query_observability_cancellation_denied",
        "context_provider_model_supersession_denied",
        "memory_kg_supersession_denied",
        "rollback_secret_supersession_denied",
        "external_public_release_supersession_denied",
        "install_restart_active_binary_supersession_denied",
        "upstream_supersession_denied",
        "latest_wins_cancellation_supersession_bypass_denied",
        "sequence_cursor_cancellation_supersession_bypass_denied",
        "operator_approval_from_cancellation_supersession_denied",
        "activation_from_cancellation_supersession_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_ordering.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate",
            "source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_route_ready": source_ordering_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_report_sha256": source_ordering_report_sha256,
            "source_ordering_monotonicity_contract_hash_sha256": source_ordering_contract_hash_sha256,
            "source_ordering_monotonicity_policy_hash_sha256": source_ordering_policy_hash_sha256,
            "source_replay_idempotency_report_sha256": source_replay_idempotency_report_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_no_cancel_no_supersede_no_replacement_no_tombstone_no_persist_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_v1",
            "cancellation_supersession_mode": "native_route_stdout_only_cancellation_supersession_denial_no_record_no_persist_no_replacement_no_authority_no_live",
            "cancellation_supersession_decision": "blocked_noop_activation_command_result_receipt_cannot_be_cancelled_superseded_replaced_or_promoted_to_authority",
            "minimum_required_samples": 24,
            "cancellation_supersession_fixtures_sha256": cancellation_supersession_fixtures_sha256,
            "cancellation_supersession_contract_hash_sha256": cancellation_supersession_contract_hash_sha256,
            "cancellation_supersession_policy_hash_sha256": cancellation_supersession_policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "source_ordering_monotonicity_fixture_count": source_u64("ordering_monotonicity_fixture_count"),
            "source_blocked_ordering_monotonicity_fixture_count": source_u64("blocked_ordering_monotonicity_fixture_count"),
            "source_noop_ordering_monotonicity_fixture_count": source_u64("noop_ordering_monotonicity_fixture_count"),
            "source_accepted_ordering_monotonicity_fixture_count": source_u64("accepted_ordering_monotonicity_fixture_count"),
            "source_ordering_monotonicity_performed_count": source_u64("ordering_monotonicity_performed_count"),
            "source_sequence_cursor_accepted_count": source_u64("sequence_cursor_accepted_count"),
            "source_sequence_cursor_recorded_count": source_u64("sequence_cursor_recorded_count"),
            "source_monotonicity_state_recorded_count": source_u64("monotonicity_state_recorded_count"),
            "source_monotonicity_state_persisted_count": source_u64("monotonicity_state_persisted_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "cancellation_supersession_surface_count": 14,
            "cancellation_supersession_surface_ready_count": 14,
            "cancellation_supersession_side_effect_free_surface_count": 14,
            "cancellation_supersession_fixtures": cancellation_supersession_fixtures,
            "cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "blocked_cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "noop_cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "allowed_cancellation_supersession_fixture_count": 0,
            "accepted_cancellation_supersession_fixture_count": 0,
            "cancellation_fixture_count": cancellation_fixture_count,
            "supersession_fixture_count": supersession_fixture_count,
            "cancellation_denied_count": cancellation_fixture_count,
            "supersession_denied_count": supersession_fixture_count,
            "cancellation_performed_count": 0,
            "supersession_performed_count": 0,
            "replacement_receipt_accepted_count": 0,
            "replacement_receipt_recorded_count": 0,
            "replacement_receipt_persisted_count": 0,
            "tombstone_recorded_count": 0,
            "delete_marker_recorded_count": 0,
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_cancellation_allowed",
            "activation_command_result_receipt_cancellation_recorded",
            "activation_command_result_receipt_cancellation_persisted",
            "activation_command_result_receipt_cancellation_materialized",
            "activation_command_result_receipt_cancellation_filesystem_written",
            "activation_command_result_receipt_cancellation_request_accepted",
            "activation_command_result_receipt_supersession_allowed",
            "activation_command_result_receipt_supersession_recorded",
            "activation_command_result_receipt_supersession_persisted",
            "activation_command_result_receipt_supersession_materialized",
            "activation_command_result_receipt_supersession_filesystem_written",
            "activation_command_result_receipt_supersession_request_accepted",
            "activation_command_result_receipt_replacement_receipt_accepted",
            "activation_command_result_receipt_replacement_receipt_recorded",
            "activation_command_result_receipt_replacement_receipt_persisted",
            "activation_command_result_receipt_replacement_hash_accepted",
            "activation_command_result_receipt_tombstone_recorded",
            "activation_command_result_receipt_tombstone_persisted",
            "activation_command_result_receipt_delete_marker_recorded",
            "activation_command_result_receipt_ack_cancellation_accepted",
            "activation_command_result_receipt_ledger_cancellation_accepted",
            "activation_command_result_receipt_index_cancellation_accepted",
            "activation_command_result_receipt_delivery_cancellation_accepted",
            "activation_command_result_receipt_export_cancellation_accepted",
            "activation_command_result_receipt_query_cancellation_accepted",
            "activation_command_result_receipt_observability_cancellation_accepted",
            "activation_command_result_receipt_ordering_allowed",
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_sequence_cursor_accepted",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_result_receipt_ledger_written",
            "activation_command_result_receipt_indexed",
            "activation_command_result_receipt_enqueued",
            "activation_command_result_receipt_delivered",
            "activation_command_result_receipt_exported",
            "activation_command_result_receipt_query_registered",
            "activation_command_result_receipt_observability_recorded",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "operator_approval_from_cancellation_accepted",
            "operator_approval_from_supersession_accepted",
            "operator_approval_from_ordering_accepted",
            "operator_approval_from_replay_accepted",
            "operator_approval_from_receipt_accepted",
            "activation_from_cancellation_allowed",
            "activation_from_supersession_allowed",
            "activation_from_ordering_allowed",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_allowed",
            "activation_command_accepted",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
        ] {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 21,
            "enablement_lane_count": 24,
            "ready_enablement_lane_count": 24,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial",
                    "status": "allowed_report_only",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "persists_replacement_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "activation_command_result_receipt_cancellation_recorded",
            "activation_command_result_receipt_cancellation_persisted",
            "activation_command_result_receipt_cancellation_materialized",
            "activation_command_result_receipt_cancellation_filesystem_written",
            "activation_command_result_receipt_supersession_recorded",
            "activation_command_result_receipt_supersession_persisted",
            "activation_command_result_receipt_supersession_materialized",
            "activation_command_result_receipt_supersession_filesystem_written",
            "activation_command_result_receipt_replacement_receipt_recorded",
            "activation_command_result_receipt_replacement_receipt_persisted",
            "activation_command_result_receipt_replacement_hash_accepted",
            "activation_command_result_receipt_tombstone_recorded",
            "activation_command_result_receipt_tombstone_persisted",
            "activation_command_result_receipt_delete_marker_recorded",
            "activation_command_result_receipt_ack_cancellation_accepted",
            "activation_command_result_receipt_ledger_cancellation_accepted",
            "activation_command_result_receipt_index_cancellation_accepted",
            "activation_command_result_receipt_delivery_cancellation_accepted",
            "activation_command_result_receipt_export_cancellation_accepted",
            "activation_command_result_receipt_query_cancellation_accepted",
            "activation_command_result_receipt_observability_cancellation_accepted",
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_cancellation_accepted",
            "operator_approval_from_supersession_accepted",
            "activation_from_cancellation_allowed",
            "activation_from_supersession_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
            "public_release_claimed",
            "public_ga_claimed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source = hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_report();
    let source_status = source
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let source_ready = source_status == "blocked"
        && source
            .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_report_sha256 = sha256_json_value(&source);
    let source_contract_hash_sha256 = source
        .get("cancellation_supersession_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_policy_hash_sha256 = source
        .get("cancellation_supersession_policy_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_ordering_report_sha256 = source
        .get("source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_report_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();

    let audit_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert(
                "audit_evidence_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_cancellation_supersession_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_cancellation_supersession_ready".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("audit_trail_requested".to_string(), serde_json::json!(true));
            fixture.insert(
                "immutable_evidence_requested".to_string(),
                serde_json::json!(false),
            );
            for key in [
                "audit_trail_allowed",
                "audit_trail_recorded",
                "audit_trail_persisted",
                "audit_trail_materialized",
                "audit_trail_filesystem_written",
                "immutable_evidence_allowed",
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
                "activation_command_result_receipt_cancellation_allowed",
                "activation_command_result_receipt_cancellation_recorded",
                "activation_command_result_receipt_cancellation_persisted",
                "activation_command_result_receipt_supersession_allowed",
                "activation_command_result_receipt_supersession_recorded",
                "activation_command_result_receipt_supersession_persisted",
                "activation_command_result_receipt_replacement_receipt_accepted",
                "activation_command_result_receipt_replacement_receipt_recorded",
                "activation_command_result_receipt_replacement_receipt_persisted",
                "activation_command_result_receipt_tombstone_recorded",
                "activation_command_result_receipt_delete_marker_recorded",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "operator_approval_from_audit_trail_accepted",
                "operator_approval_from_immutable_evidence_accepted",
                "activation_from_audit_trail_allowed",
                "activation_from_immutable_evidence_allowed",
                "activation_from_cancellation_allowed",
                "activation_from_supersession_allowed",
                "activation_from_receipt_allowed",
                "activation_command_allowed",
                "activation_command_accepted",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "operator_approval_recorded",
                "dispatch_performed",
                "execution_performed",
                "context_injection_performed",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "credential_read",
                "secret_file_read",
                "auth_secret_read",
                "secret_value_read",
                "raw_payload_plaintext_recorded",
                "raw_payload_plaintext_persisted",
                "channel_send_performed",
                "telegram_send_performed",
                "external_send_performed",
                "public_claim_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_performed",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "service_restart_performed",
                "active_binary_mutated",
                "upstream_fetch_performed",
                "upstream_merge_performed",
                "rollback_executed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            fixture.insert(
                "receipt_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("denial_reason".to_string(), serde_json::json!(reason));
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };

    let audit_trail_immutable_evidence_fixtures = serde_json::Value::Array(vec![
        audit_fixture(
            "missing-source-cancellation-supersession-report",
            "blocked_noop",
            "source_result_receipt_cancellation_supersession_report_required",
            serde_json::json!({
                "source_cancellation_supersession_present": false,
                "source_cancellation_supersession_ready": false,
            }),
        ),
        audit_fixture(
            "append-audit-trail-to-blocked-noop-result-receipt",
            "blocked_audit_noop",
            "audit_trail_append_request_denied",
            serde_json::json!({
                "audit_trail_request_shape": "append_blocked_noop_result_receipt",
            }),
        ),
        audit_fixture(
            "seal-blocked-noop-as-immutable-evidence",
            "blocked_evidence_noop",
            "immutable_evidence_packet_request_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "immutable_evidence_request_shape": "seal_blocked_noop_result_receipt",
            }),
        ),
        audit_fixture(
            "hash-chain-merkle-root-evidence-attempt",
            "blocked_evidence_noop",
            "hash_chain_merkle_root_recording_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "hash_chain_requested": true,
                "merkle_root_requested": true,
            }),
        ),
        audit_fixture(
            "attestation-witness-notary-evidence-attempt",
            "blocked_evidence_noop",
            "attestation_witness_notary_recording_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "attestation_requested": true,
                "witness_requested": true,
                "notary_requested": true,
            }),
        ),
        audit_fixture(
            "audit-trail-materialization-filesystem-attempt",
            "blocked_audit_noop",
            "audit_trail_materialization_filesystem_denied",
            serde_json::json!({
                "audit_trail_materialization_requested": true,
                "audit_trail_filesystem_write_requested": true,
            }),
        ),
        audit_fixture(
            "ledger-index-delivery-export-query-observability-evidence-attempt",
            "blocked_evidence_noop",
            "ledger_index_delivery_export_query_observability_evidence_denied",
            serde_json::json!({
                "ledger_evidence_requested": true,
                "index_evidence_requested": true,
                "delivery_evidence_requested": true,
                "export_evidence_requested": true,
                "query_evidence_requested": true,
                "observability_evidence_requested": true,
            }),
        ),
        audit_fixture(
            "activation-from-audit-evidence-attempt",
            "blocked_evidence_noop",
            "activation_from_audit_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "activation_from_audit_evidence_requested": true,
            }),
        ),
        audit_fixture(
            "context-provider-model-memory-kg-readback-evidence-attempt",
            "blocked_evidence_noop",
            "context_provider_model_memory_kg_readback_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "context_evidence_requested": true,
                "provider_prompt_evidence_requested": true,
                "model_output_evidence_requested": true,
                "memory_store_evidence_requested": true,
                "external_kg_evidence_requested": true,
                "live_kg_evidence_requested": true,
                "readback_evidence_requested": true,
            }),
        ),
        audit_fixture(
            "rollback-secret-external-public-install-evidence-attempt",
            "blocked_evidence_noop",
            "rollback_secret_external_public_install_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "rollback_evidence_requested": true,
                "credential_secret_evidence_requested": true,
                "external_send_evidence_requested": true,
                "public_claim_evidence_requested": true,
                "release_artifact_evidence_requested": true,
                "install_evidence_requested": true,
                "service_restart_evidence_requested": true,
                "active_binary_mutation_evidence_requested": true,
                "upstream_evidence_requested": true,
            }),
        ),
    ]);
    let audit_fixture_count = audit_trail_immutable_evidence_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let audit_trail_denied_count = audit_trail_immutable_evidence_fixtures
        .as_array()
        .map(|fixtures| {
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture
                        .get("audit_trail_requested")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0);
    let immutable_evidence_denied_count = audit_trail_immutable_evidence_fixtures
        .as_array()
        .map(|fixtures| {
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture
                        .get("immutable_evidence_requested")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&audit_trail_immutable_evidence_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial:v1:source={source_report_sha256}:cancellation={source_contract_hash_sha256}:ordering={source_ordering_report_sha256}:fixtures={fixtures_sha256}:audit=0:evidence=0:hashchain=0:authority=0:live=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial:v1:no-audit-write:no-evidence-persist:no-hash-chain:no-attestation:no-authority:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_side_effects=false;fixtures=10;audit=0;evidence=0;hashchain=0;attestation=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );

    let mut denials = source
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_result_receipt_cancellation_supersession_report_required",
        "audit_trail_request_acceptance_denied",
        "audit_trail_recording_denied",
        "audit_trail_persistence_denied",
        "audit_trail_materialization_denied",
        "audit_trail_filesystem_write_denied",
        "immutable_evidence_request_acceptance_denied",
        "immutable_evidence_recording_denied",
        "immutable_evidence_persistence_denied",
        "immutable_evidence_materialization_denied",
        "immutable_evidence_filesystem_write_denied",
        "hash_chain_recording_denied",
        "merkle_root_recording_denied",
        "attestation_recording_denied",
        "witness_recording_denied",
        "notary_recording_denied",
        "ledger_index_delivery_evidence_denied",
        "export_query_observability_evidence_denied",
        "activation_from_audit_trail_denied",
        "activation_from_immutable_evidence_denied",
        "operator_approval_from_audit_trail_denied",
        "operator_approval_from_immutable_evidence_denied",
        "context_provider_model_evidence_denied",
        "memory_kg_readback_evidence_denied",
        "rollback_secret_evidence_denied",
        "external_public_install_restart_active_binary_evidence_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if source_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_source_command_accepted": route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "source_route_wired": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate",
            "source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_route_ready": source_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_report_sha256": source_report_sha256,
            "source_cancellation_supersession_contract_hash_sha256": source_contract_hash_sha256,
            "source_cancellation_supersession_policy_hash_sha256": source_policy_hash_sha256,
            "source_ordering_monotonicity_report_sha256": source_ordering_report_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_no_audit_no_evidence_no_hash_chain_no_attestation_no_persist_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_v1",
            "audit_trail_immutable_evidence_mode": "native_route_stdout_only_audit_trail_immutable_evidence_denial_no_record_no_persist_no_authority_no_live",
            "audit_trail_immutable_evidence_decision": "blocked_noop_activation_command_result_receipt_cannot_be_wrapped_as_audit_trail_or_immutable_evidence_authority",
            "minimum_required_samples": 24,
            "audit_trail_immutable_evidence_fixtures_sha256": fixtures_sha256,
            "audit_trail_immutable_evidence_contract_hash_sha256": contract_hash_sha256,
            "audit_trail_immutable_evidence_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "source_cancellation_supersession_fixture_count": source_u64("cancellation_supersession_fixture_count"),
            "source_blocked_cancellation_supersession_fixture_count": source_u64("blocked_cancellation_supersession_fixture_count"),
            "source_noop_cancellation_supersession_fixture_count": source_u64("noop_cancellation_supersession_fixture_count"),
            "source_accepted_cancellation_supersession_fixture_count": source_u64("accepted_cancellation_supersession_fixture_count"),
            "source_cancellation_performed_count": source_u64("cancellation_performed_count"),
            "source_supersession_performed_count": source_u64("supersession_performed_count"),
            "source_replacement_receipt_accepted_count": source_u64("replacement_receipt_accepted_count"),
            "source_replacement_receipt_recorded_count": source_u64("replacement_receipt_recorded_count"),
            "source_replacement_receipt_persisted_count": source_u64("replacement_receipt_persisted_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "audit_trail_immutable_evidence_surface_count": 12,
            "audit_trail_immutable_evidence_surface_ready_count": 12,
            "audit_trail_immutable_evidence_side_effect_free_surface_count": 12,
            "audit_trail_immutable_evidence_fixtures": audit_trail_immutable_evidence_fixtures,
            "audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "blocked_audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "noop_audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "allowed_audit_trail_immutable_evidence_fixture_count": 0,
            "accepted_audit_trail_immutable_evidence_fixture_count": 0,
            "audit_trail_denied_count": audit_trail_denied_count,
            "immutable_evidence_denied_count": immutable_evidence_denied_count,
            "audit_trail_performed_count": 0,
            "immutable_evidence_performed_count": 0,
            "hash_chain_recorded_count": 0,
            "merkle_root_recorded_count": 0,
            "attestation_recorded_count": 0,
            "witness_recorded_count": 0,
            "notary_recorded_count": 0,
            "ledger_evidence_recorded_count": 0,
            "index_evidence_recorded_count": 0,
            "delivery_evidence_recorded_count": 0,
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_audit_trail_allowed",
            "activation_command_result_receipt_audit_trail_recorded",
            "activation_command_result_receipt_audit_trail_persisted",
            "activation_command_result_receipt_audit_trail_materialized",
            "activation_command_result_receipt_audit_trail_filesystem_written",
            "activation_command_result_receipt_immutable_evidence_allowed",
            "activation_command_result_receipt_immutable_evidence_recorded",
            "activation_command_result_receipt_immutable_evidence_persisted",
            "activation_command_result_receipt_immutable_evidence_materialized",
            "activation_command_result_receipt_immutable_evidence_filesystem_written",
            "activation_command_result_receipt_hash_chain_recorded",
            "activation_command_result_receipt_hash_chain_persisted",
            "activation_command_result_receipt_merkle_root_recorded",
            "activation_command_result_receipt_merkle_root_persisted",
            "activation_command_result_receipt_attestation_recorded",
            "activation_command_result_receipt_attestation_persisted",
            "activation_command_result_receipt_witness_recorded",
            "activation_command_result_receipt_witness_persisted",
            "activation_command_result_receipt_notary_recorded",
            "activation_command_result_receipt_notary_persisted",
            "activation_command_result_receipt_ledger_evidence_recorded",
            "activation_command_result_receipt_ledger_evidence_persisted",
            "activation_command_result_receipt_index_evidence_recorded",
            "activation_command_result_receipt_index_evidence_persisted",
            "activation_command_result_receipt_delivery_evidence_recorded",
            "activation_command_result_receipt_delivery_evidence_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_audit_trail_accepted",
            "operator_approval_from_immutable_evidence_accepted",
            "activation_from_audit_trail_allowed",
            "activation_from_immutable_evidence_allowed",
            "activation_from_cancellation_allowed",
            "activation_from_supersession_allowed",
            "activation_from_receipt_allowed",
            "activation_command_allowed",
            "activation_command_accepted",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
        ] {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "readback_evidence_recorded_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 22,
            "enablement_lane_count": 25,
            "ready_enablement_lane_count": 25,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
                    "status": "allowed_report_only",
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
                    "status": "allowed_report_only_next_slice",
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "performs_retention": false,
                    "performs_gc": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "activation_command_result_receipt_audit_trail_recorded",
            "activation_command_result_receipt_audit_trail_persisted",
            "activation_command_result_receipt_audit_trail_materialized",
            "activation_command_result_receipt_audit_trail_filesystem_written",
            "activation_command_result_receipt_immutable_evidence_recorded",
            "activation_command_result_receipt_immutable_evidence_persisted",
            "activation_command_result_receipt_immutable_evidence_materialized",
            "activation_command_result_receipt_immutable_evidence_filesystem_written",
            "activation_command_result_receipt_hash_chain_recorded",
            "activation_command_result_receipt_hash_chain_persisted",
            "activation_command_result_receipt_merkle_root_recorded",
            "activation_command_result_receipt_merkle_root_persisted",
            "activation_command_result_receipt_attestation_recorded",
            "activation_command_result_receipt_attestation_persisted",
            "activation_command_result_receipt_witness_recorded",
            "activation_command_result_receipt_witness_persisted",
            "activation_command_result_receipt_notary_recorded",
            "activation_command_result_receipt_notary_persisted",
            "activation_command_result_receipt_ledger_evidence_recorded",
            "activation_command_result_receipt_index_evidence_recorded",
            "activation_command_result_receipt_delivery_evidence_recorded",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_audit_trail_accepted",
            "operator_approval_from_immutable_evidence_accepted",
            "activation_from_audit_trail_allowed",
            "activation_from_immutable_evidence_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "readback_evidence_recorded",
            "credential_read",
            "secret_file_read",
            "auth_secret_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source = hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report();
    let source_status = source
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let source_ready = source_status == "blocked"
        && source
            .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_report_sha256 = sha256_json_value(&source);
    let source_contract_hash_sha256 = source
        .get("audit_trail_immutable_evidence_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_policy_hash_sha256 = source
        .get("audit_trail_immutable_evidence_policy_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();

    let retention_gc_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert("retention_gc_status".to_string(), serde_json::json!(status));
            fixture.insert(
                "source_audit_trail_immutable_evidence_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_audit_trail_immutable_evidence_ready".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("retention_requested".to_string(), serde_json::json!(true));
            fixture.insert("expiry_requested".to_string(), serde_json::json!(false));
            fixture.insert(
                "garbage_collection_requested".to_string(),
                serde_json::json!(false),
            );
            for key in [
                "retention_policy_allowed",
                "retention_policy_recorded",
                "retention_policy_persisted",
                "retention_policy_materialized",
                "retention_policy_filesystem_written",
                "retention_index_allowed",
                "retention_index_recorded",
                "retention_index_persisted",
                "expiry_allowed",
                "expiry_recorded",
                "expiry_persisted",
                "expiry_scheduler_registered",
                "expiry_timer_started",
                "expiry_materialized",
                "ttl_update_allowed",
                "ttl_update_recorded",
                "ttl_extension_allowed",
                "ttl_extension_recorded",
                "garbage_collection_allowed",
                "garbage_collection_scan_performed",
                "garbage_collection_candidate_recorded",
                "garbage_collection_decision_recorded",
                "garbage_collection_persisted",
                "delete_allowed",
                "delete_performed",
                "delete_marker_recorded",
                "tombstone_recorded",
                "sweep_allowed",
                "sweep_performed",
                "archive_allowed",
                "archive_written",
                "compaction_allowed",
                "compaction_performed",
                "compaction_artifact_written",
                "ledger_retention_recorded",
                "ledger_retention_persisted",
                "index_retention_recorded",
                "index_retention_persisted",
                "delivery_retention_recorded",
                "delivery_retention_persisted",
                "audit_trail_recorded",
                "audit_trail_persisted",
                "immutable_evidence_recorded",
                "immutable_evidence_persisted",
                "hash_chain_recorded",
                "merkle_root_recorded",
                "attestation_recorded",
                "witness_recorded",
                "notary_recorded",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "operator_approval_from_retention_accepted",
                "operator_approval_from_expiry_accepted",
                "operator_approval_from_garbage_collection_accepted",
                "activation_from_retention_allowed",
                "activation_from_expiry_allowed",
                "activation_from_garbage_collection_allowed",
                "activation_from_receipt_allowed",
                "activation_command_allowed",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "operator_approval_recorded",
                "dispatch_performed",
                "execution_performed",
                "context_injection_performed",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "rollback_executed",
                "credential_read",
                "secret_file_read",
                "auth_secret_read",
                "channel_send_performed",
                "telegram_send_performed",
                "external_send_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "active_binary_mutated",
                "upstream_fetch_performed",
                "upstream_merge_performed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            fixture.insert(
                "receipt_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("denial_reason".to_string(), serde_json::json!(reason));
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };

    let retention_expiry_garbage_collection_fixtures = serde_json::Value::Array(vec![
        retention_gc_fixture(
            "missing-source-audit-trail-immutable-evidence-report",
            "blocked_noop",
            "source_audit_trail_immutable_evidence_report_required",
            serde_json::json!({
                "source_audit_trail_immutable_evidence_present": false,
                "source_audit_trail_immutable_evidence_ready": false,
            }),
        ),
        retention_gc_fixture(
            "retention-policy-write-request",
            "blocked_noop",
            "retention_policy_write_request_denied",
            serde_json::json!({
                "retention_policy_request_shape": "record_blocked_noop_receipt_retention_policy",
            }),
        ),
        retention_gc_fixture(
            "retention-index-record-request",
            "blocked_noop",
            "retention_index_recording_denied",
            serde_json::json!({
                "retention_index_requested": true,
            }),
        ),
        retention_gc_fixture(
            "expiry-scheduler-timer-request",
            "blocked_expiry_noop",
            "expiry_scheduler_timer_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "expiry_schedule_requested": true,
                "expiry_timer_requested": true,
            }),
        ),
        retention_gc_fixture(
            "ttl-update-extension-request",
            "blocked_expiry_noop",
            "ttl_update_extension_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "ttl_update_requested": true,
                "ttl_extension_requested": true,
            }),
        ),
        retention_gc_fixture(
            "garbage-collection-scan-request",
            "blocked_gc_noop",
            "garbage_collection_scan_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "garbage_collection_scan_requested": true,
            }),
        ),
        retention_gc_fixture(
            "delete-tombstone-sweep-request",
            "blocked_gc_noop",
            "delete_tombstone_sweep_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "delete_requested": true,
                "tombstone_requested": true,
                "sweep_requested": true,
            }),
        ),
        retention_gc_fixture(
            "archive-compaction-request",
            "blocked_gc_noop",
            "archive_compaction_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "archive_requested": true,
                "compaction_requested": true,
            }),
        ),
        retention_gc_fixture(
            "activation-provider-memory-kg-retention-gc-attempt",
            "blocked_gc_noop",
            "activation_provider_memory_kg_retention_gc_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "garbage_collection_requested": true,
                "activation_from_retention_gc_requested": true,
                "provider_prompt_gc_evidence_requested": true,
                "memory_store_gc_evidence_requested": true,
                "external_kg_gc_evidence_requested": true,
                "live_kg_gc_evidence_requested": true,
                "readback_gc_evidence_requested": true,
            }),
        ),
        retention_gc_fixture(
            "rollback-secret-external-public-install-retention-gc-attempt",
            "blocked_gc_noop",
            "rollback_secret_external_public_install_retention_gc_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "garbage_collection_requested": true,
                "ledger_retention_requested": true,
                "index_retention_requested": true,
                "delivery_retention_requested": true,
                "rollback_gc_evidence_requested": true,
                "credential_secret_gc_evidence_requested": true,
                "external_send_gc_evidence_requested": true,
                "public_claim_gc_evidence_requested": true,
                "release_artifact_gc_evidence_requested": true,
                "install_gc_evidence_requested": true,
                "service_restart_gc_evidence_requested": true,
                "active_binary_gc_evidence_requested": true,
                "upstream_gc_evidence_requested": true,
            }),
        ),
    ]);
    let retention_gc_fixture_count = retention_expiry_garbage_collection_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&retention_expiry_garbage_collection_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial:v1:source={source_report_sha256}:audit={source_contract_hash_sha256}:fixtures={fixtures_sha256}:retention=0:expiry=0:gc=0:delete=0:authority=0:live=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial:v1:no-retention:no-expiry:no-gc:no-delete:no-archive:no-authority:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_side_effects=false;fixtures=10;retention=0;expiry=0;gc=0;delete=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );

    let mut denials = source
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_audit_trail_immutable_evidence_report_required",
        "retention_policy_request_acceptance_denied",
        "retention_policy_recording_denied",
        "retention_policy_persistence_denied",
        "retention_policy_materialization_denied",
        "retention_index_recording_denied",
        "expiry_request_acceptance_denied",
        "expiry_recording_denied",
        "expiry_scheduler_registration_denied",
        "expiry_timer_start_denied",
        "ttl_update_denied",
        "ttl_extension_denied",
        "garbage_collection_request_acceptance_denied",
        "garbage_collection_scan_denied",
        "garbage_collection_candidate_recording_denied",
        "garbage_collection_decision_recording_denied",
        "delete_execution_denied",
        "delete_marker_recording_denied",
        "tombstone_recording_denied",
        "sweep_execution_denied",
        "archive_write_denied",
        "compaction_execution_denied",
        "ledger_retention_recording_denied",
        "index_retention_recording_denied",
        "delivery_retention_recording_denied",
        "operator_approval_from_retention_expiry_gc_denied",
        "activation_from_retention_expiry_gc_denied",
        "provider_model_memory_kg_gc_evidence_denied",
        "rollback_secret_external_public_install_restart_active_binary_gc_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if source_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_source_command_accepted": route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "source_route_wired": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate",
            "source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_route_ready": source_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256": source_report_sha256,
            "source_audit_trail_immutable_evidence_contract_hash_sha256": source_contract_hash_sha256,
            "source_audit_trail_immutable_evidence_policy_hash_sha256": source_policy_hash_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_no_retention_no_expiry_no_gc_no_delete_no_archive_no_compaction_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_v1",
            "retention_expiry_garbage_collection_mode": "native_route_stdout_only_retention_expiry_garbage_collection_denial_no_schedule_no_scan_no_delete_no_authority_no_live",
            "retention_expiry_garbage_collection_decision": "blocked_noop_activation_command_result_receipt_cannot_be_retained_expired_garbage_collected_or_deleted_into_authority",
            "minimum_required_samples": 24,
            "retention_expiry_garbage_collection_fixtures_sha256": fixtures_sha256,
            "retention_expiry_garbage_collection_contract_hash_sha256": contract_hash_sha256,
            "retention_expiry_garbage_collection_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "source_audit_trail_immutable_evidence_fixture_count": source_u64("audit_trail_immutable_evidence_fixture_count"),
            "source_blocked_audit_trail_immutable_evidence_fixture_count": source_u64("blocked_audit_trail_immutable_evidence_fixture_count"),
            "source_accepted_audit_trail_immutable_evidence_fixture_count": source_u64("accepted_audit_trail_immutable_evidence_fixture_count"),
            "source_audit_trail_performed_count": source_u64("audit_trail_performed_count"),
            "source_immutable_evidence_performed_count": source_u64("immutable_evidence_performed_count"),
            "source_hash_chain_recorded_count": source_u64("hash_chain_recorded_count"),
            "source_attestation_recorded_count": source_u64("attestation_recorded_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "retention_expiry_garbage_collection_surface_count": 12,
            "retention_expiry_garbage_collection_surface_ready_count": 12,
            "retention_expiry_garbage_collection_side_effect_free_surface_count": 12,
            "retention_expiry_garbage_collection_fixtures": retention_expiry_garbage_collection_fixtures,
            "retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "blocked_retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "noop_retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "allowed_retention_expiry_garbage_collection_fixture_count": 0,
            "accepted_retention_expiry_garbage_collection_fixture_count": 0,
            "retention_denied_count": retention_gc_fixture_count,
            "expiry_denied_count": retention_gc_fixture_count,
            "garbage_collection_denied_count": retention_gc_fixture_count,
            "retention_performed_count": 0,
            "expiry_performed_count": 0,
            "garbage_collection_performed_count": 0,
            "delete_performed_count": 0,
            "archive_written_count": 0,
            "compaction_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "readback_evidence_recorded_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 23,
            "enablement_lane_count": 26,
            "ready_enablement_lane_count": 26,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
                    "status": "allowed_report_only",
                    "performs_retention": false,
                    "performs_expiry": false,
                    "performs_gc": false,
                    "deletes_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial",
                    "status": "allowed_report_only_next_slice",
                    "performs_retention": false,
                    "performs_expiry": false,
                    "performs_gc": false,
                    "deletes_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_retention_policy_allowed",
            "activation_command_result_receipt_retention_policy_recorded",
            "activation_command_result_receipt_retention_policy_persisted",
            "activation_command_result_receipt_retention_policy_materialized",
            "activation_command_result_receipt_retention_policy_filesystem_written",
            "activation_command_result_receipt_retention_index_allowed",
            "activation_command_result_receipt_retention_index_recorded",
            "activation_command_result_receipt_retention_index_persisted",
            "activation_command_result_receipt_expiry_allowed",
            "activation_command_result_receipt_expiry_recorded",
            "activation_command_result_receipt_expiry_persisted",
            "activation_command_result_receipt_expiry_scheduler_registered",
            "activation_command_result_receipt_expiry_timer_started",
            "activation_command_result_receipt_expiry_materialized",
            "activation_command_result_receipt_ttl_update_allowed",
            "activation_command_result_receipt_ttl_update_recorded",
            "activation_command_result_receipt_ttl_extension_allowed",
            "activation_command_result_receipt_ttl_extension_recorded",
            "activation_command_result_receipt_garbage_collection_allowed",
            "activation_command_result_receipt_garbage_collection_scan_performed",
            "activation_command_result_receipt_garbage_collection_candidate_recorded",
            "activation_command_result_receipt_garbage_collection_decision_recorded",
            "activation_command_result_receipt_garbage_collection_persisted",
            "activation_command_result_receipt_delete_allowed",
            "activation_command_result_receipt_delete_performed",
            "activation_command_result_receipt_delete_marker_recorded",
            "activation_command_result_receipt_tombstone_recorded",
            "activation_command_result_receipt_sweep_allowed",
            "activation_command_result_receipt_sweep_performed",
            "activation_command_result_receipt_archive_allowed",
            "activation_command_result_receipt_archive_written",
            "activation_command_result_receipt_compaction_allowed",
            "activation_command_result_receipt_compaction_performed",
            "activation_command_result_receipt_compaction_artifact_written",
            "activation_command_result_receipt_ledger_retention_recorded",
            "activation_command_result_receipt_ledger_retention_persisted",
            "activation_command_result_receipt_index_retention_recorded",
            "activation_command_result_receipt_index_retention_persisted",
            "activation_command_result_receipt_delivery_retention_recorded",
            "activation_command_result_receipt_delivery_retention_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_retention_accepted",
            "operator_approval_from_expiry_accepted",
            "operator_approval_from_garbage_collection_accepted",
            "activation_allowed_by_result_receipt_retention",
            "activation_allowed_by_result_receipt_expiry",
            "activation_allowed_by_result_receipt_garbage_collection",
            "activation_allowed_by_result_receipt_audit_trail",
            "activation_allowed_by_result_receipt_immutable_evidence",
            "activation_allowed_by_result_receipt",
            "activation_command_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
        ] {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "retention_policy_recorded",
            "retention_policy_persisted",
            "retention_index_recorded",
            "expiry_recorded",
            "expiry_scheduler_registered",
            "expiry_timer_started",
            "ttl_update_recorded",
            "ttl_extension_recorded",
            "garbage_collection_scan_performed",
            "garbage_collection_candidate_recorded",
            "garbage_collection_decision_recorded",
            "delete_performed",
            "delete_marker_recorded",
            "tombstone_recorded",
            "sweep_performed",
            "archive_written",
            "compaction_performed",
            "ledger_retention_recorded",
            "index_retention_recorded",
            "delivery_retention_recorded",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_completion_ack_recorded",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "release_artifact_written",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_retention =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report();
    let source_bool = |key: &str| {
        source_retention
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_retention
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_retention
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_retention_expiry_gc_ready = source_status == "ready"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_enabled",
        )
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        )
        && source_retention
            .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status")
            .and_then(serde_json::Value::as_str)
            == Some("blocked")
        && source_u64("retention_expiry_garbage_collection_fixture_count") == 10
        && source_u64("blocked_retention_expiry_garbage_collection_fixture_count") == 10
        && source_u64("noop_retention_expiry_garbage_collection_fixture_count") == 10
        && source_u64("accepted_retention_expiry_garbage_collection_fixture_count") == 0
        && source_u64("retention_performed_count") == 0
        && source_u64("expiry_performed_count") == 0
        && source_u64("garbage_collection_performed_count") == 0
        && source_u64("delete_performed_count") == 0
        && source_u64("archive_written_count") == 0
        && source_u64("compaction_performed_count") == 0
        && !source_bool("activation_command_result_receipt_retention_policy_recorded")
        && !source_bool("activation_command_result_receipt_retention_index_recorded")
        && !source_bool("activation_command_result_receipt_expiry_scheduler_registered")
        && !source_bool("activation_command_result_receipt_expiry_timer_started")
        && !source_bool("activation_command_result_receipt_garbage_collection_scan_performed")
        && !source_bool("activation_command_result_receipt_delete_performed")
        && !source_bool("activation_command_result_receipt_tombstone_recorded")
        && !source_bool("activation_command_result_receipt_archive_written")
        && !source_bool("activation_command_result_receipt_compaction_performed")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_allowed_by_result_receipt_retention")
        && !source_bool("activation_allowed_by_result_receipt_expiry")
        && !source_bool("activation_allowed_by_result_receipt_garbage_collection")
        && !source_bool("activation_command_invoked")
        && !source_bool("activation_command_dispatched")
        && !source_bool("activation_request_accepted")
        && !source_bool("activation_request_executed")
        && source_u64("dispatch_performed_count") == 0
        && source_u64("execution_performed_count") == 0
        && source_u64("provider_invoked_count") == 0
        && source_u64("model_invoked_count") == 0
        && source_u64("memory_store_write_performed_count") == 0
        && source_u64("external_kg_adapter_read_performed_count") == 0
        && source_u64("live_kg_write_performed_count") == 0
        && source_u64("credential_read_count") == 0
        && source_u64("secret_file_read_count") == 0
        && source_u64("channel_send_performed_count") == 0
        && !source_bool("canary_harness_armed")
        && !source_bool("canary_harness_executable")
        && !source_bool("canary_live_enabled");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_retention_expiry_gc_ready;

    let export_query_observability_fixture =
        |fixture_id: &str, status: &str, denial_reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "export_query_observability_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(denial_reason.to_string()),
            );
            for key in [
                "source_retention_expiry_gc_present",
                "source_retention_expiry_gc_ready",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "export_requested",
                "query_requested",
                "observability_requested",
                "export_allowed",
                "export_request_accepted",
                "export_recorded",
                "export_persisted",
                "export_artifact_written",
                "export_stream_opened",
                "export_filesystem_written",
                "query_allowed",
                "query_registered",
                "query_endpoint_materialized",
                "query_index_recorded",
                "query_cache_written",
                "query_result_materialized",
                "observability_allowed",
                "observability_metric_emitted",
                "observability_log_recorded",
                "observability_trace_recorded",
                "observability_span_recorded",
                "observability_event_recorded",
                "observability_dashboard_materialized",
                "observability_alert_registered",
                "observability_slo_recorded",
                "ledger_observability_recorded",
                "index_observability_recorded",
                "delivery_observability_recorded",
                "activation_command_result_receipt_retention_policy_recorded",
                "activation_command_result_receipt_expiry_recorded",
                "activation_command_result_receipt_garbage_collection_scan_performed",
                "activation_command_result_receipt_audit_trail_recorded",
                "activation_command_result_receipt_immutable_evidence_recorded",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_result_receipt_ledger_written",
                "activation_command_result_receipt_indexed",
                "activation_command_result_receipt_enqueued",
                "activation_command_result_receipt_delivered",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "activation_command_completion_ack_delivered",
                "operator_approval_from_export_accepted",
                "operator_approval_from_query_accepted",
                "operator_approval_from_observability_accepted",
                "activation_from_export_allowed",
                "activation_from_query_allowed",
                "activation_from_observability_allowed",
                "activation_from_retention_allowed",
                "activation_from_expiry_allowed",
                "activation_from_garbage_collection_allowed",
                "activation_command_allowed",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "operator_approval_recorded",
                "dispatch_performed",
                "execution_performed",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "rollback_executed",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "active_binary_mutated",
                "upstream_fetch_performed",
                "upstream_merge_performed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let export_query_observability_fixtures = serde_json::Value::Array(vec![
        export_query_observability_fixture(
            "missing-source-retention-expiry-garbage-collection-report",
            "blocked_noop",
            "source_retention_expiry_garbage_collection_report_required",
            serde_json::json!({
                "source_retention_expiry_gc_present": false,
                "source_retention_expiry_gc_ready": false,
                "export_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "export-artifact-request",
            "blocked_export_noop",
            "export_artifact_write_denied",
            serde_json::json!({"export_requested": true, "export_file_requested": true}),
        ),
        export_query_observability_fixture(
            "export-stream-request",
            "blocked_export_noop",
            "export_stream_open_denied",
            serde_json::json!({"export_requested": true, "export_stream_requested": true}),
        ),
        export_query_observability_fixture(
            "query-endpoint-request",
            "blocked_query_noop",
            "query_endpoint_materialization_denied",
            serde_json::json!({"query_requested": true, "query_endpoint_requested": true}),
        ),
        export_query_observability_fixture(
            "query-index-cache-request",
            "blocked_query_noop",
            "query_index_cache_recording_denied",
            serde_json::json!({
                "query_requested": true,
                "query_index_requested": true,
                "query_cache_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "observability-metric-request",
            "blocked_observability_noop",
            "observability_metric_emission_denied",
            serde_json::json!({"observability_requested": true, "metric_requested": true}),
        ),
        export_query_observability_fixture(
            "observability-trace-log-event-request",
            "blocked_observability_noop",
            "trace_span_log_event_recording_denied",
            serde_json::json!({
                "observability_requested": true,
                "trace_requested": true,
                "span_requested": true,
                "log_requested": true,
                "event_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "dashboard-alert-slo-request",
            "blocked_observability_noop",
            "dashboard_alert_slo_materialization_denied",
            serde_json::json!({
                "observability_requested": true,
                "dashboard_requested": true,
                "alert_requested": true,
                "slo_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "activation-provider-memory-kg-observability",
            "blocked_observability_noop",
            "activation_provider_memory_kg_observability_denied",
            serde_json::json!({
                "observability_requested": true,
                "activation_from_observability_requested": true,
                "memory_store_observability_requested": true,
                "external_kg_observability_requested": true,
                "live_kg_observability_requested": true,
                "rollback_observability_requested": true,
                "secret_material_observability_requested": true,
                "provider_prompt_observability_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "ledger-index-delivery-external-public-install-observability",
            "blocked_observability_noop",
            "ledger_index_delivery_external_public_install_observability_denied",
            serde_json::json!({
                "observability_requested": true,
                "ledger_observability_requested": true,
                "index_observability_requested": true,
                "delivery_observability_requested": true,
                "external_send_observability_requested": true,
                "public_claim_observability_requested": true,
                "release_artifact_observability_requested": true,
                "install_observability_requested": true,
                "service_restart_observability_requested": true,
                "active_binary_observability_requested": true,
                "upstream_observability_requested": true,
            }),
        ),
    ]);
    let export_query_observability_fixture_count = export_query_observability_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let mut denials = source_retention
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_retention_expiry_garbage_collection_report_required",
        "export_request_acceptance_denied",
        "export_recording_denied",
        "export_persistence_denied",
        "export_artifact_write_denied",
        "export_stream_open_denied",
        "query_request_acceptance_denied",
        "query_registration_denied",
        "query_endpoint_materialization_denied",
        "query_index_recording_denied",
        "query_cache_write_denied",
        "query_result_materialization_denied",
        "observability_request_acceptance_denied",
        "metric_emission_denied",
        "log_recording_denied",
        "trace_recording_denied",
        "span_recording_denied",
        "event_recording_denied",
        "dashboard_materialization_denied",
        "alert_registration_denied",
        "slo_recording_denied",
        "ledger_observability_recording_denied",
        "index_observability_recording_denied",
        "delivery_observability_recording_denied",
        "operator_approval_from_export_query_observability_denied",
        "activation_from_export_query_observability_denied",
        "provider_model_memory_kg_observability_denied",
        "rollback_secret_external_public_install_restart_active_binary_observability_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_retention.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_ready": source_retention_expiry_gc_ready,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_no_export_no_query_no_metrics_logs_traces_events_dashboards_alerts_slo_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_v1",
            "export_query_observability_mode": "native_route_stdout_only_export_query_observability_denial_no_export_no_query_no_observability_no_authority_no_live",
            "export_query_observability_decision": "blocked_noop_activation_command_result_receipt_cannot_be_exported_queried_or_observed_into_authority",
            "source_retention_expiry_garbage_collection_fixture_count": source_u64("retention_expiry_garbage_collection_fixture_count"),
            "source_blocked_retention_expiry_garbage_collection_fixture_count": source_u64("blocked_retention_expiry_garbage_collection_fixture_count"),
            "source_accepted_retention_expiry_garbage_collection_fixture_count": source_u64("accepted_retention_expiry_garbage_collection_fixture_count"),
            "source_retention_performed_count": source_u64("retention_performed_count"),
            "source_expiry_performed_count": source_u64("expiry_performed_count"),
            "source_garbage_collection_performed_count": source_u64("garbage_collection_performed_count"),
            "export_query_observability_surface_count": 12,
            "export_query_observability_surface_ready_count": 12,
            "export_query_observability_side_effect_free_surface_count": 12,
            "export_query_observability_fixtures": export_query_observability_fixtures,
            "export_query_observability_fixture_count": export_query_observability_fixture_count,
            "blocked_export_query_observability_fixture_count": export_query_observability_fixture_count,
            "noop_export_query_observability_fixture_count": export_query_observability_fixture_count,
            "allowed_export_query_observability_fixture_count": 0,
            "accepted_export_query_observability_fixture_count": 0,
            "export_denied_count": export_query_observability_fixture_count,
            "query_denied_count": export_query_observability_fixture_count,
            "observability_denied_count": export_query_observability_fixture_count,
            "export_performed_count": 0,
            "query_performed_count": 0,
            "observability_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_export_allowed": false,
            "activation_command_result_receipt_export_request_accepted": false,
            "activation_command_result_receipt_export_recorded": false,
            "activation_command_result_receipt_export_persisted": false,
            "activation_command_result_receipt_export_artifact_written": false,
            "activation_command_result_receipt_export_stream_opened": false,
            "activation_command_result_receipt_export_filesystem_written": false,
            "activation_command_result_receipt_query_allowed": false,
            "activation_command_result_receipt_query_registered": false,
            "activation_command_result_receipt_query_endpoint_materialized": false,
            "activation_command_result_receipt_query_index_recorded": false,
            "activation_command_result_receipt_query_cache_written": false,
            "activation_command_result_receipt_query_result_materialized": false,
            "activation_command_result_receipt_observability_allowed": false,
            "activation_command_result_receipt_observability_metric_emitted": false,
            "activation_command_result_receipt_observability_log_recorded": false,
            "activation_command_result_receipt_observability_trace_recorded": false,
            "activation_command_result_receipt_observability_span_recorded": false,
            "activation_command_result_receipt_observability_event_recorded": false,
            "activation_command_result_receipt_observability_dashboard_materialized": false,
            "activation_command_result_receipt_observability_alert_registered": false,
            "activation_command_result_receipt_observability_slo_recorded": false,
            "activation_command_result_receipt_ledger_observability_recorded": false,
            "activation_command_result_receipt_index_observability_recorded": false,
            "activation_command_result_receipt_delivery_observability_recorded": false,
            "operator_approval_from_export_accepted": false,
            "operator_approval_from_query_accepted": false,
            "operator_approval_from_observability_accepted": false,
            "activation_allowed_by_result_receipt_export": false,
            "activation_allowed_by_result_receipt_query": false,
            "activation_allowed_by_result_receipt_observability": false,
            "activation_allowed_by_result_receipt": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_enabled": false,
            "activation_command_invoked": false,
            "activation_command_dispatched": false,
            "activation_request_accepted": false,
            "activation_request_recorded": false,
            "activation_request_persisted": false,
            "activation_request_executed": false,
            "operator_approval_recorded": false,
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "runtime_router_mutated_count": 0,
            "runtime_attachment_performed_count": 0,
            "live_context_attached_count": 0,
            "context_injection_performed_count": 0,
            "adapter_invoked_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_count": denied_count,
            "current_live_enabled_lane_count": 24,
            "enablement_lane_count": 27,
            "ready_enablement_lane_count": 27,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial",
                    "status": "allowed_report_only",
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                    "status": "allowed_report_only_next_slice",
                    "persists_summary": false,
                    "persists_briefing": false,
                    "delivers_summary": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "export_recorded",
            "export_persisted",
            "export_artifact_written",
            "export_stream_opened",
            "query_registered",
            "query_endpoint_materialized",
            "query_index_recorded",
            "query_cache_written",
            "observability_metric_emitted",
            "observability_log_recorded",
            "observability_trace_recorded",
            "observability_span_recorded",
            "observability_event_recorded",
            "observability_dashboard_materialized",
            "observability_alert_registered",
            "observability_slo_recorded",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_completion_ack_recorded",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "release_artifact_written",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_export =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_report();
    let source_bool = |key: &str| {
        source_export
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_export
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_export
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_export_query_observability_ready = source_status == "blocked"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_route_enabled",
        )
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready",
        )
        && source_u64("export_query_observability_fixture_count") == 10
        && source_u64("blocked_export_query_observability_fixture_count") == 10
        && source_u64("noop_export_query_observability_fixture_count") == 10
        && source_u64("accepted_export_query_observability_fixture_count") == 0
        && source_u64("export_performed_count") == 0
        && source_u64("query_performed_count") == 0
        && source_u64("observability_performed_count") == 0
        && !source_bool("activation_command_result_receipt_export_recorded")
        && !source_bool("activation_command_result_receipt_export_artifact_written")
        && !source_bool("activation_command_result_receipt_export_stream_opened")
        && !source_bool("activation_command_result_receipt_query_registered")
        && !source_bool("activation_command_result_receipt_query_endpoint_materialized")
        && !source_bool("activation_command_result_receipt_observability_metric_emitted")
        && !source_bool("activation_command_result_receipt_observability_log_recorded")
        && !source_bool("activation_command_result_receipt_observability_trace_recorded")
        && !source_bool("activation_command_result_receipt_observability_dashboard_materialized")
        && !source_bool("activation_command_result_receipt_observability_alert_registered")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_allowed_by_result_receipt_export")
        && !source_bool("activation_allowed_by_result_receipt_query")
        && !source_bool("activation_allowed_by_result_receipt_observability")
        && !source_bool("activation_command_invoked")
        && !source_bool("activation_command_dispatched")
        && source_u64("provider_invoked_count") == 0
        && source_u64("model_invoked_count") == 0
        && source_u64("memory_store_write_performed_count") == 0
        && source_u64("live_kg_write_performed_count") == 0
        && source_u64("credential_read_count") == 0
        && source_u64("secret_file_read_count") == 0
        && source_u64("channel_send_performed_count") == 0
        && source_u64("install_performed_count") == 0
        && source_u64("service_restarted_count") == 0
        && source_u64("active_binary_mutated_count") == 0;
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_export_query_observability_ready;

    let operator_summary_briefing_fixture =
        |fixture_id: &str, status: &str, denial_reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "operator_summary_briefing_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(denial_reason.to_string()),
            );
            for key in [
                "source_export_query_observability_present",
                "source_export_query_observability_ready",
                "summary_briefing_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "operator_summary_requested",
                "operator_briefing_requested",
                "operator_summary_allowed",
                "operator_summary_request_accepted",
                "operator_summary_recorded",
                "operator_summary_persisted",
                "operator_summary_materialized",
                "operator_summary_filesystem_written",
                "operator_summary_delivered",
                "operator_summary_channel_delivery_performed",
                "operator_briefing_allowed",
                "operator_briefing_request_accepted",
                "operator_briefing_recorded",
                "operator_briefing_persisted",
                "operator_briefing_materialized",
                "operator_briefing_filesystem_written",
                "operator_briefing_delivered",
                "operator_briefing_channel_delivery_performed",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_completion_ack_recorded",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "activation_activated",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "rollback_executed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let operator_summary_briefing_fixtures = serde_json::Value::Array(vec![
        operator_summary_briefing_fixture(
            "operator-summary-missing-source-export-query-observability",
            "blocked_noop",
            "source_export_query_observability_report_required",
            serde_json::json!({
                "source_export_query_observability_present": false,
                "source_export_query_observability_ready": false,
                "operator_summary_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-request",
            "blocked_summary_noop",
            "operator_summary_request_shape_denied",
            serde_json::json!({"operator_summary_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "operator-briefing-request",
            "blocked_briefing_noop",
            "operator_briefing_request_shape_denied",
            serde_json::json!({"operator_briefing_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-materialization-request",
            "blocked_summary_noop",
            "operator_summary_materialization_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_summary_materialization_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-briefing-materialization-request",
            "blocked_briefing_noop",
            "operator_briefing_materialization_denied",
            serde_json::json!({
                "operator_briefing_requested": true,
                "operator_briefing_materialization_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-persistence-filesystem-request",
            "blocked_summary_noop",
            "operator_summary_persistence_filesystem_write_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_summary_persistence_requested": true,
                "operator_summary_filesystem_write_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-briefing-persistence-filesystem-request",
            "blocked_briefing_noop",
            "operator_briefing_persistence_filesystem_write_denied",
            serde_json::json!({
                "operator_briefing_requested": true,
                "operator_briefing_persistence_requested": true,
                "operator_briefing_filesystem_write_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-briefing-channel-delivery-request",
            "blocked_delivery_noop",
            "operator_summary_briefing_channel_delivery_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "channel_delivery_requested": true,
                "telegram_send_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-briefing-activation-memory-kg-provider",
            "blocked_summary_noop",
            "activation_memory_kg_rollback_secret_provider_summary_briefing_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "activation_from_summary_briefing_requested": true,
                "memory_store_summary_requested": true,
                "live_kg_summary_requested": true,
                "rollback_summary_requested": true,
                "secret_material_summary_requested": true,
                "provider_prompt_summary_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-briefing-external-public-install",
            "blocked_delivery_noop",
            "external_public_install_restart_active_binary_summary_briefing_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "external_send_summary_requested": true,
                "public_claim_summary_requested": true,
                "release_artifact_summary_requested": true,
                "install_summary_requested": true,
                "service_restart_summary_requested": true,
                "active_binary_summary_requested": true,
            }),
        ),
    ]);
    let operator_summary_briefing_fixture_count = operator_summary_briefing_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let mut denials = source_export
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_export_query_observability_report_required",
        "operator_summary_request_acceptance_denied",
        "operator_briefing_request_acceptance_denied",
        "operator_summary_recording_denied",
        "operator_briefing_recording_denied",
        "operator_summary_persistence_denied",
        "operator_briefing_persistence_denied",
        "operator_summary_materialization_denied",
        "operator_briefing_materialization_denied",
        "operator_summary_filesystem_write_denied",
        "operator_briefing_filesystem_write_denied",
        "operator_summary_delivery_denied",
        "operator_briefing_delivery_denied",
        "telegram_send_denied",
        "channel_delivery_denied",
        "activation_from_summary_briefing_denied",
        "memory_kg_summary_briefing_denied",
        "rollback_summary_briefing_denied",
        "secret_material_summary_briefing_denied",
        "provider_prompt_summary_briefing_denied",
        "external_public_install_restart_active_binary_summary_briefing_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_export.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_ready": source_export_query_observability_ready,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_no_summary_no_briefing_no_delivery_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status": "blocked",
            "activation_command_result_receipt_operator_facing_summary_briefing_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
            "operator_facing_summary_briefing_mode": "native_route_stdout_only_operator_facing_summary_briefing_non_persistence_denial_no_summary_no_briefing_no_delivery_no_authority_no_live",
            "operator_facing_summary_briefing_decision": "blocked_noop_activation_command_result_receipt_cannot_be_summarized_briefed_delivered_or_promoted_into_authority",
            "source_export_query_observability_fixture_count": source_u64("export_query_observability_fixture_count"),
            "source_blocked_export_query_observability_fixture_count": source_u64("blocked_export_query_observability_fixture_count"),
            "source_accepted_export_query_observability_fixture_count": source_u64("accepted_export_query_observability_fixture_count"),
            "source_export_performed_count": source_u64("export_performed_count"),
            "source_query_performed_count": source_u64("query_performed_count"),
            "source_observability_performed_count": source_u64("observability_performed_count"),
            "operator_facing_summary_briefing_surface_count": 12,
            "operator_facing_summary_briefing_surface_ready_count": 12,
            "operator_facing_summary_briefing_side_effect_free_surface_count": 12,
            "operator_facing_summary_briefing_fixtures": operator_summary_briefing_fixtures,
            "operator_facing_summary_briefing_fixture_count": operator_summary_briefing_fixture_count,
            "blocked_operator_facing_summary_briefing_fixture_count": operator_summary_briefing_fixture_count,
            "noop_operator_facing_summary_briefing_fixture_count": operator_summary_briefing_fixture_count,
            "allowed_operator_facing_summary_briefing_fixture_count": 0,
            "accepted_operator_facing_summary_briefing_fixture_count": 0,
            "operator_summary_denied_count": operator_summary_briefing_fixture_count,
            "operator_briefing_denied_count": operator_summary_briefing_fixture_count,
            "operator_summary_performed_count": 0,
            "operator_briefing_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_operator_summary_allowed": false,
            "activation_command_result_receipt_operator_summary_request_accepted": false,
            "activation_command_result_receipt_operator_summary_recorded": false,
            "activation_command_result_receipt_operator_summary_persisted": false,
            "activation_command_result_receipt_operator_summary_materialized": false,
            "activation_command_result_receipt_operator_summary_filesystem_written": false,
            "activation_command_result_receipt_operator_summary_delivered": false,
            "activation_command_result_receipt_operator_summary_channel_delivery_performed": false,
            "activation_command_result_receipt_operator_briefing_allowed": false,
            "activation_command_result_receipt_operator_briefing_request_accepted": false,
            "activation_command_result_receipt_operator_briefing_recorded": false,
            "activation_command_result_receipt_operator_briefing_persisted": false,
            "activation_command_result_receipt_operator_briefing_materialized": false,
            "activation_command_result_receipt_operator_briefing_filesystem_written": false,
            "activation_command_result_receipt_operator_briefing_delivered": false,
            "activation_command_result_receipt_operator_briefing_channel_delivery_performed": false,
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed": false,
            "telegram_send_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_allowed_by_result_receipt_operator_summary": false,
            "activation_allowed_by_result_receipt_operator_briefing": false,
            "activation_allowed_by_result_receipt_summary_briefing": false,
            "activation_allowed_by_result_receipt": false,
            "activation_command_enabled": false,
            "activation_command_invoked": false,
            "activation_command_dispatched": false,
            "activation_activated": false,
            "runtime_router_mutated": false,
            "runtime_attachment_performed": false,
            "live_context_attached": false,
            "context_injection_performed": false,
            "adapter_invoked": false,
            "provider_invoked": false,
            "model_invoked": false,
            "provider_prompt_replayed": false,
            "auth_secret_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "usage_recorded": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "live_kg_write_performed": false,
            "readback_evidence_recorded": false,
            "readback_evidence_persisted": false,
            "router_handoff_recorded": false,
            "router_handoff_persisted": false,
            "rollback_executed": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restart_performed": false,
            "active_binary_mutated": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_operator_facing_summary_briefing": denials,
            "denied_by_operator_facing_summary_briefing_count": denied_count,
            "current_live_enabled_lane_count": 25,
            "enablement_lane_count": 28,
            "ready_enablement_lane_count": 28,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                    "status": "allowed_report_only",
                    "persists_summary": false,
                    "persists_briefing": false,
                    "delivers_summary": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_operator_acknowledgement": false,
                    "persists_acknowledgement": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_operator_summary_recorded",
            "activation_command_result_receipt_operator_summary_persisted",
            "activation_command_result_receipt_operator_summary_materialized",
            "activation_command_result_receipt_operator_summary_filesystem_written",
            "activation_command_result_receipt_operator_summary_delivered",
            "activation_command_result_receipt_operator_summary_channel_delivery_performed",
            "activation_command_result_receipt_operator_briefing_recorded",
            "activation_command_result_receipt_operator_briefing_persisted",
            "activation_command_result_receipt_operator_briefing_materialized",
            "activation_command_result_receipt_operator_briefing_filesystem_written",
            "activation_command_result_receipt_operator_briefing_delivered",
            "activation_command_result_receipt_operator_briefing_channel_delivery_performed",
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_activated",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "filesystem_written",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_summary =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report();
    let source_bool = |key: &str| {
        source_summary
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_summary
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_summary
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let source_ready = source_status == "blocked"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
        );
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let report_ready = source_ready
        && route_count_source_command_accepted
        && route_matrix.missing_route_count == 0;
    let source_report_sha256 = sha256_json_value(&source_summary);

    let final_ack_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert(
                "final_operator_acknowledgement_requested".to_string(),
                serde_json::json!(false),
            );
            fixture.insert(
                "final_operator_acknowledgement_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_summary_briefing_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_summary_briefing_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "acknowledgement_allowed",
                "acknowledgement_request_accepted",
                "acknowledgement_accepted",
                "acknowledgement_recorded",
                "acknowledgement_persisted",
                "acknowledgement_materialized",
                "acknowledgement_filesystem_written",
                "acknowledgement_delivered",
                "acknowledgement_channel_delivery_performed",
                "acknowledgement_identity_accepted",
                "acknowledgement_signature_accepted",
                "acknowledgement_timestamp_accepted",
                "acknowledgement_final_state_promoted",
                "acknowledgement_completion_promoted",
                "operator_final_acceptance_recorded",
                "operator_final_acceptance_persisted",
                "operator_final_acceptance_materialized",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "activation_command_completion_ack_delivered",
                "activation_allowed",
                "activation_performed",
                "live_mutation_execution_performed",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "rollback_executed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "final_acknowledgement_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };

    let final_acknowledgement_fixtures = serde_json::json!([
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-missing-source-summary-briefing",
            "blocked_noop",
            "source_summary_briefing_report_required",
            serde_json::json!({
                "source_summary_briefing_present": false,
                "source_summary_briefing_ready": false,
                "final_operator_acknowledgement_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_request_shape_denied",
            serde_json::json!({"final_operator_acknowledgement_requested": true}),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-acceptance-request",
            "blocked_acceptance_noop",
            "final_operator_acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_acceptance_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-recording-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_recording_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_recording_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-persistence-filesystem-write-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_persistence_filesystem_write_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_persistence_requested": true,
                "acknowledgement_filesystem_write_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-identity-signature-timestamp-request",
            "blocked_acceptance_noop",
            "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "operator_identity_acceptance_requested": true,
                "operator_signature_acceptance_requested": true,
                "operator_timestamp_acceptance_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-delivery-request",
            "blocked_delivery_noop",
            "final_operator_acknowledgement_delivery_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_delivery_requested": true,
                "telegram_send_requested": true,
                "channel_delivery_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-state-promotion-request",
            "blocked_promotion_noop",
            "final_state_completion_promotion_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "final_state_promotion_requested": true,
                "completion_promotion_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-activation-memory-kg-provider-request",
            "blocked_ack_noop",
            "activation_memory_kg_rollback_secret_provider_acknowledgement_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "activation_from_acknowledgement_requested": true,
                "memory_store_acknowledgement_requested": true,
                "live_kg_acknowledgement_requested": true,
                "rollback_acknowledgement_requested": true,
                "secret_material_acknowledgement_requested": true,
                "provider_prompt_acknowledgement_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-external-public-install-request",
            "blocked_delivery_noop",
            "external_public_install_restart_active_binary_acknowledgement_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "external_send_acknowledgement_requested": true,
                "public_claim_acknowledgement_requested": true,
                "release_artifact_acknowledgement_requested": true,
                "install_acknowledgement_requested": true,
                "service_restart_acknowledgement_requested": true,
                "active_binary_acknowledgement_requested": true,
            }),
        ),
    ]);
    let final_acknowledgement_fixture_count = final_acknowledgement_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&final_acknowledgement_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:ack=0:accept=0:persist=0:deliver=0:promote=0:live=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial:v1:no-ack-accept:no-ack-record:no-ack-persist:no-ack-deliver:no-final-state-promotion:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "final_operator_acknowledgement=false;acceptance=false;record=false;persist=false;deliver=false;promotion=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );
    let mut denials = source_summary
        .get("denied_by_operator_facing_summary_briefing")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_operator_facing_summary_briefing_report_required",
        "final_operator_acknowledgement_request_acceptance_denied",
        "final_operator_acknowledgement_acceptance_denied",
        "final_operator_acknowledgement_recording_denied",
        "final_operator_acknowledgement_persistence_denied",
        "final_operator_acknowledgement_materialization_denied",
        "final_operator_acknowledgement_filesystem_write_denied",
        "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
        "final_operator_acknowledgement_delivery_denied",
        "telegram_send_denied",
        "final_state_completion_promotion_denied",
        "activation_from_final_operator_acknowledgement_denied",
        "memory_kg_acknowledgement_denied",
        "rollback_acknowledgement_denied",
        "secret_material_acknowledgement_denied",
        "provider_prompt_acknowledgement_denied",
        "external_public_install_restart_active_binary_acknowledgement_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_summary.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_ready": source_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_report_sha256": source_report_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_no_ack_accept_no_record_no_persist_no_delivery_no_final_state_promotion_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status": "blocked",
            "activation_command_result_receipt_final_operator_acknowledgement_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
            "activation_command_result_receipt_final_operator_acknowledgement_mode": "native_route_stdout_only_final_operator_acknowledgement_non_acceptance_denial_no_ack_no_accept_no_record_no_deliver_no_authority_no_live",
            "activation_command_result_receipt_final_operator_acknowledgement_decision": "blocked_noop_activation_command_result_receipt_cannot_be_acknowledged_or_promoted_into_final_operator_authority",
            "source_operator_facing_summary_briefing_fixture_count": source_u64("operator_facing_summary_briefing_fixture_count"),
            "source_blocked_operator_facing_summary_briefing_fixture_count": source_u64("blocked_operator_facing_summary_briefing_fixture_count"),
            "source_accepted_operator_facing_summary_briefing_fixture_count": source_u64("accepted_operator_facing_summary_briefing_fixture_count"),
            "source_operator_summary_performed_count": source_u64("operator_summary_performed_count"),
            "source_operator_briefing_performed_count": source_u64("operator_briefing_performed_count"),
            "final_acknowledgement_fixtures_sha256": fixtures_sha256,
            "final_acknowledgement_contract_hash_sha256": contract_hash_sha256,
            "final_acknowledgement_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "required_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 10,
            "activation_command_result_receipt_final_operator_acknowledgement_fixtures": final_acknowledgement_fixtures,
            "activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 0,
            "accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_denied_count": final_acknowledgement_fixture_count,
            "activation_command_result_receipt_final_operator_acknowledgement_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_final_operator_acknowledgement_allowed": false,
            "activation_command_result_receipt_final_operator_acknowledgement_request_accepted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_accepted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_recorded": false,
            "activation_command_result_receipt_final_operator_acknowledgement_persisted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_materialized": false,
            "activation_command_result_receipt_final_operator_acknowledgement_filesystem_written": false,
            "activation_command_result_receipt_final_operator_acknowledgement_delivered": false,
            "activation_command_result_receipt_final_operator_acknowledgement_channel_delivery_performed": false,
            "activation_command_result_receipt_final_operator_acknowledgement_identity_accepted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_signature_accepted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_completion_promoted": false,
            "activation_command_result_receipt_operator_final_acceptance_recorded": false,
            "activation_command_result_receipt_operator_final_acceptance_persisted": false,
            "activation_command_result_receipt_operator_final_acceptance_materialized": false,
            "telegram_send_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "activation_allowed_by_result_receipt_final_operator_acknowledgement": false,
            "activation_allowed_by_result_receipt_summary_briefing": false,
            "activation_allowed_by_result_receipt": false,
            "activation_allowed": false,
            "activation_performed": false,
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "live_mutation_execution_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_activation_command_result_receipt_final_operator_acknowledgement": denials,
            "denied_by_activation_command_result_receipt_final_operator_acknowledgement_count": denied_count,
            "current_live_enabled_lane_count": 26,
            "enablement_lane_count": 29,
            "ready_enablement_lane_count": 29,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
                    "status": "allowed_report_only",
                    "accepts_operator_acknowledgement": false,
                    "persists_acknowledgement": false,
                    "delivers_acknowledgement": false,
                    "promotes_final_state": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_terminal_decision": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_final_operator_acknowledgement_recorded",
            "activation_command_result_receipt_final_operator_acknowledgement_persisted",
            "activation_command_result_receipt_final_operator_acknowledgement_materialized",
            "activation_command_result_receipt_final_operator_acknowledgement_filesystem_written",
            "activation_command_result_receipt_final_operator_acknowledgement_delivered",
            "activation_command_result_receipt_final_operator_acknowledgement_channel_delivery_performed",
            "activation_command_result_receipt_final_operator_acknowledgement_identity_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_signature_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted",
            "activation_command_result_receipt_final_operator_acknowledgement_completion_promoted",
            "activation_command_result_receipt_operator_final_acceptance_recorded",
            "activation_command_result_receipt_operator_final_acceptance_persisted",
            "activation_command_result_receipt_operator_final_acceptance_materialized",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_performed",
            "live_mutation_execution_performed",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "filesystem_written",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_final_ack =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report();
    let source_bool = |key: &str| {
        source_final_ack
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_final_ack
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_final_ack
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let source_ready = source_status == "blocked"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
        );
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let report_ready = source_ready
        && route_count_source_command_accepted
        && route_matrix.missing_route_count == 0;
    let source_report_sha256 = sha256_json_value(&source_final_ack);

    let terminal_decision_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::json!(id));
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert(
                "terminal_operator_decision_requested".to_string(),
                serde_json::json!(false),
            );
            fixture.insert(
                "terminal_operator_decision_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_final_acknowledgement_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_final_acknowledgement_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "terminal_decision_allowed",
                "terminal_decision_request_accepted",
                "terminal_decision_accepted",
                "terminal_decision_recorded",
                "terminal_decision_persisted",
                "terminal_decision_materialized",
                "terminal_decision_filesystem_written",
                "terminal_decision_delivered",
                "terminal_decision_channel_delivery_performed",
                "terminal_decision_identity_accepted",
                "terminal_decision_signature_accepted",
                "terminal_decision_timestamp_accepted",
                "terminal_decision_final_state_promoted",
                "terminal_decision_completion_promoted",
                "public_claim_requested",
                "public_claim_accepted",
                "public_claim_recorded",
                "public_claim_persisted",
                "public_claim_materialized",
                "public_claim_promoted",
                "public_ga_claimed",
                "public_release_published",
                "public_distribution_performed",
                "public_artifact_written",
                "release_artifact_written",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "receipt_materialized",
                "receipt_filesystem_written",
                "completion_ack_recorded",
                "completion_ack_persisted",
                "completion_ack_accepted",
                "completion_ack_delivered",
                "activation_allowed",
                "activation_performed",
                "live_mutation_execution_performed",
                "memory_write_execution_performed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "rollback_executed",
                "secret_material_read",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "terminal_operator_decision_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };

    let terminal_decision_public_claim_fixtures = serde_json::json!([
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-missing-final-ack",
            "blocked_noop",
            "source_final_operator_acknowledgement_report_required",
            serde_json::json!({
                "source_final_acknowledgement_present": false,
                "source_final_acknowledgement_ready": false,
                "terminal_operator_decision_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-request",
            "blocked_decision_noop",
            "terminal_operator_decision_request_shape_denied",
            serde_json::json!({"terminal_operator_decision_requested": true}),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-acceptance-request",
            "blocked_acceptance_noop",
            "terminal_operator_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_acceptance_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-recording-request",
            "blocked_decision_noop",
            "terminal_operator_decision_recording_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_recording_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-persistence-filesystem-write-request",
            "blocked_decision_noop",
            "terminal_operator_decision_persistence_filesystem_write_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_persistence_requested": true,
                "terminal_decision_filesystem_write_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-identity-signature-request",
            "blocked_acceptance_noop",
            "operator_identity_signature_terminal_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "operator_identity_acceptance_requested": true,
                "operator_signature_acceptance_requested": true,
                "operator_timestamp_acceptance_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-public-claim-request",
            "blocked_public_claim_noop",
            "public_claim_request_non_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_claim_requested": true,
                "public_claim_promotion_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-public-ga-release-request",
            "blocked_promotion_noop",
            "public_ga_release_publication_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_ga_claim_requested": true,
                "public_release_publish_requested": true,
                "public_distribution_requested": true,
                "release_artifact_write_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-activation-memory-provider-request",
            "blocked_decision_noop",
            "activation_memory_rollback_secret_provider_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "activation_from_terminal_decision_requested": true,
                "memory_write_terminal_decision_requested": true,
                "rollback_terminal_decision_requested": true,
                "secret_material_terminal_decision_requested": true,
                "provider_prompt_terminal_decision_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-external-public-install-request",
            "blocked_promotion_noop",
            "external_public_install_restart_active_binary_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "external_send_decision_requested": true,
                "public_claim_decision_requested": true,
                "release_artifact_decision_requested": true,
                "install_decision_requested": true,
                "service_restart_decision_requested": true,
                "active_binary_decision_requested": true,
            }),
        ),
    ]);
    let terminal_decision_public_claim_fixture_count = terminal_decision_public_claim_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&terminal_decision_public_claim_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:decision=0:public_claim=0:publish=0:artifact=0:live=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial:v1:no-terminal-decision-accept:no-public-claim:no-ga-release:no-artifact:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "terminal_decision=false;public_claim=false;public_release=false;artifact=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );
    let mut denials = source_final_ack
        .get("denied_by_activation_command_result_receipt_final_operator_acknowledgement")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_final_operator_acknowledgement_report_required",
        "terminal_operator_decision_request_acceptance_denied",
        "terminal_operator_decision_acceptance_denied",
        "terminal_operator_decision_recording_denied",
        "terminal_operator_decision_persistence_denied",
        "terminal_operator_decision_materialization_denied",
        "terminal_operator_decision_filesystem_write_denied",
        "operator_identity_signature_terminal_decision_acceptance_denied",
        "terminal_operator_decision_delivery_denied",
        "telegram_send_denied",
        "public_claim_non_promotion_denied",
        "public_ga_release_publication_promotion_denied",
        "activation_from_terminal_operator_decision_denied",
        "memory_write_terminal_decision_denied",
        "rollback_terminal_decision_denied",
        "secret_material_terminal_decision_denied",
        "provider_prompt_terminal_decision_denied",
        "external_public_install_restart_active_binary_terminal_decision_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_final_ack.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_ready": source_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_report_sha256": source_report_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_no_decision_accept_no_record_no_persist_no_delivery_no_public_claim_no_ga_release_no_artifact_no_context_memory_kg_provider_model_credential_channel_install_restart_binary",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status": "blocked",
            "activation_command_result_receipt_terminal_operator_decision_public_claim_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_v1",
            "activation_command_result_receipt_terminal_operator_decision_public_claim_mode": "native_route_stdout_only_terminal_operator_decision_public_claim_non_promotion_denial_no_decision_accept_no_public_claim_no_release_no_artifact_no_authority_no_live",
            "activation_command_result_receipt_terminal_operator_decision_public_claim_decision": "blocked_noop_activation_command_result_receipt_cannot_be_promoted_into_terminal_operator_decision_or_public_claim_authority",
            "source_final_operator_acknowledgement_fixture_count": source_u64("activation_command_result_receipt_final_operator_acknowledgement_fixture_count"),
            "source_blocked_final_operator_acknowledgement_fixture_count": source_u64("blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"),
            "source_accepted_final_operator_acknowledgement_fixture_count": source_u64("accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"),
            "source_final_operator_acknowledgement_performed_count": source_u64("activation_command_result_receipt_final_operator_acknowledgement_performed_count"),
            "terminal_operator_decision_public_claim_fixtures_sha256": fixtures_sha256,
            "terminal_operator_decision_public_claim_contract_hash_sha256": contract_hash_sha256,
            "terminal_operator_decision_public_claim_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "required_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count": 12,
            "ready_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count": 12,
            "required_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": 10,
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures": terminal_decision_public_claim_fixtures,
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "noop_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": 0,
            "accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": 0,
            "activation_command_result_receipt_terminal_operator_decision_performed_count": 0,
            "activation_command_result_receipt_public_claim_promotion_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_terminal_operator_decision_allowed": false,
            "activation_command_result_receipt_terminal_operator_decision_request_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_recorded": false,
            "activation_command_result_receipt_terminal_operator_decision_persisted": false,
            "activation_command_result_receipt_terminal_operator_decision_materialized": false,
            "activation_command_result_receipt_terminal_operator_decision_filesystem_written": false,
            "activation_command_result_receipt_terminal_operator_decision_delivered": false,
            "activation_command_result_receipt_terminal_operator_decision_channel_delivery_performed": false,
            "activation_command_result_receipt_terminal_operator_decision_identity_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_signature_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_timestamp_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_final_state_promoted": false,
            "activation_command_result_receipt_terminal_operator_decision_completion_promoted": false,
            "activation_command_result_receipt_public_claim_requested": false,
            "activation_command_result_receipt_public_claim_accepted": false,
            "activation_command_result_receipt_public_claim_recorded": false,
            "activation_command_result_receipt_public_claim_persisted": false,
            "activation_command_result_receipt_public_claim_materialized": false,
            "activation_command_result_receipt_public_claim_promoted": false,
            "activation_command_result_receipt_public_ga_claimed": false,
            "activation_command_result_receipt_public_release_published": false,
            "activation_command_result_receipt_public_distribution_performed": false,
            "activation_command_result_receipt_public_artifact_written": false,
            "telegram_send_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_allowed_by_result_receipt_terminal_operator_decision": false,
            "activation_allowed_by_result_receipt_final_operator_acknowledgement": false,
            "activation_allowed_by_result_receipt": false,
            "activation_allowed": false,
            "activation_performed": false,
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_write_execution_allowed": false,
            "memory_write_execution_ready": false,
            "memory_write_execution_performed": false,
            "memory_store_write_path_enabled": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_write_performed_count": 0,
            "memory_store_mutation_allowed": false,
            "memory_store_mutated": false,
            "rollback_execution_allowed": false,
            "rollback_executed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "secret_material_read": false,
            "provider_prompt_replay_enabled": false,
            "provider_invoked": false,
            "model_invoked": false,
            "public_release_published": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "public_distribution_performed": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "service_restart_performed": false,
            "active_binary_mutated": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_terminal_operator_decision_public_claim_surfaces": [
                "source_final_operator_acknowledgement_report_required",
                "terminal_operator_decision_request_shape_denied",
                "terminal_operator_decision_acceptance_denied",
                "terminal_operator_decision_recording_denied",
                "terminal_operator_decision_persistence_denied",
                "terminal_operator_decision_materialization_denied",
                "operator_identity_signature_terminal_decision_acceptance_denied",
                "terminal_operator_decision_delivery_denied",
                "public_claim_request_non_promotion_denied",
                "public_ga_release_publication_promotion_denied",
                "activation_from_terminal_operator_decision_denied",
                "external_public_install_restart_active_binary_terminal_decision_denied"
            ],
            "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim": denials,
            "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count": denied_count,
            "current_live_enabled_lane_count": 27,
            "enablement_lane_count": 30,
            "ready_enablement_lane_count": 30,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                    "status": "allowed_report_only",
                    "accepts_terminal_decision": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial",
                    "status": "allowed_report_only_next_slice",
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_terminal_operator_decision_recorded",
            "activation_command_result_receipt_terminal_operator_decision_persisted",
            "activation_command_result_receipt_terminal_operator_decision_materialized",
            "activation_command_result_receipt_terminal_operator_decision_filesystem_written",
            "activation_command_result_receipt_terminal_operator_decision_delivered",
            "activation_command_result_receipt_terminal_operator_decision_channel_delivery_performed",
            "activation_command_result_receipt_terminal_operator_decision_identity_accepted",
            "activation_command_result_receipt_terminal_operator_decision_signature_accepted",
            "activation_command_result_receipt_terminal_operator_decision_timestamp_accepted",
            "activation_command_result_receipt_terminal_operator_decision_final_state_promoted",
            "activation_command_result_receipt_terminal_operator_decision_completion_promoted",
            "activation_command_result_receipt_public_claim_recorded",
            "activation_command_result_receipt_public_claim_persisted",
            "activation_command_result_receipt_public_claim_materialized",
            "activation_command_result_receipt_public_claim_promoted",
            "activation_command_result_receipt_public_ga_claimed",
            "activation_command_result_receipt_public_release_published",
            "activation_command_result_receipt_public_distribution_performed",
            "activation_command_result_receipt_public_artifact_written",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_final_operator_acknowledgement_recorded",
            "activation_command_result_receipt_final_operator_acknowledgement_persisted",
            "activation_command_result_receipt_final_operator_acknowledgement_materialized",
            "activation_command_result_receipt_final_operator_acknowledgement_filesystem_written",
            "activation_command_result_receipt_final_operator_acknowledgement_delivered",
            "activation_command_result_receipt_operator_final_acceptance_recorded",
            "activation_command_result_receipt_operator_final_acceptance_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "raw_payload_plaintext_recorded",
            "raw_payload_plaintext_persisted",
            "secret_material_read",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "provider_prompt_replay_enabled",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "filesystem_written",
            "public_release_published",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "public_distribution_performed",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_terminal_decision = hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report();
    let source_ready = source_terminal_decision
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_status = source_terminal_decision
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let source_report_sha256 = sha256_json_value(&source_terminal_decision);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_u64 = |key: &str| -> u64 {
        source_terminal_decision
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let report_ready = source_ready
        && source_status == "ready"
        && route_count_source_command_accepted
        && source_u64(
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 10
        && source_u64(
            "blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 10
        && source_u64(
            "allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 0
        && source_u64(
            "activation_command_result_receipt_terminal_operator_decision_performed_count",
        ) == 0
        && source_u64("activation_command_result_receipt_public_claim_promotion_performed_count")
            == 0;

    let publication_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::json!(id));
            fixture.insert(
                "release_artifact_publication_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_terminal_operator_decision_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_terminal_operator_decision_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "release_artifact_publication_requested",
                "release_artifact_publication_allowed",
                "release_artifact_publication_accepted",
                "release_artifact_publication_recorded",
                "release_artifact_publication_persisted",
                "release_artifact_publication_materialized",
                "release_artifact_filesystem_written",
                "release_artifact_written",
                "public_artifact_written",
                "artifact_signature_accepted",
                "artifact_notarization_accepted",
                "publication_queue_enqueued",
                "publication_manifest_written",
                "public_distribution_performed",
                "public_release_published",
                "public_ga_claimed",
                "public_claim_promoted",
                "public_version_tag_created",
                "release_notes_materialized",
                "changelog_materialized",
                "terminal_operator_decision_promoted_to_release_approval",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "receipt_materialized",
                "completion_ack_recorded",
                "activation_allowed",
                "live_mutation_execution_performed",
                "memory_write_execution_performed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "rollback_executed",
                "secret_material_read",
                "credential_read",
                "secret_file_read",
                "provider_invoked",
                "model_invoked",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "release_artifact_publication_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };

    let release_artifact_publication_fixtures = serde_json::json!([
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-release-artifact-publication-missing-terminal-decision",
            "blocked_noop",
            "source_terminal_operator_decision_report_required",
            serde_json::json!({
                "source_terminal_operator_decision_present": false,
                "source_terminal_operator_decision_ready": false,
                "release_artifact_publication_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-release-artifact-write-request",
            "blocked_artifact_noop",
            "release_artifact_write_denied",
            serde_json::json!({
                "release_artifact_write_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-public-artifact-write-request",
            "blocked_artifact_noop",
            "public_artifact_write_denied",
            serde_json::json!({
                "public_artifact_write_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-artifact-signature-notarization-request",
            "blocked_artifact_noop",
            "artifact_signature_notarization_acceptance_denied",
            serde_json::json!({
                "artifact_signature_requested": true,
                "artifact_notarization_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-publication-queue-request",
            "blocked_publication_noop",
            "publication_queue_enqueue_denied",
            serde_json::json!({
                "publication_queue_enqueue_requested": true,
                "publication_manifest_write_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-distribution-channel-request",
            "blocked_distribution_noop",
            "public_distribution_channel_delivery_denied",
            serde_json::json!({
                "public_distribution_requested": true,
                "telegram_delivery_requested": true,
                "channel_delivery_requested": true,
                "external_delivery_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-public-version-tag-request",
            "blocked_release_noop",
            "public_version_tag_release_promotion_denied",
            serde_json::json!({
                "public_version_tag_requested": true,
                "public_release_publish_requested": true,
                "public_ga_claim_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-release-notes-changelog-request",
            "blocked_artifact_noop",
            "release_notes_changelog_materialization_denied",
            serde_json::json!({
                "release_notes_materialization_requested": true,
                "changelog_materialization_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-as-release-approval",
            "blocked_promotion_noop",
            "terminal_operator_decision_is_not_release_approval",
            serde_json::json!({
                "terminal_operator_decision_release_approval_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-release-publication-activation-memory-provider-install",
            "blocked_promotion_noop",
            "activation_memory_provider_install_restart_active_binary_publication_denied",
            serde_json::json!({
                "activation_from_release_publication_requested": true,
                "memory_write_publication_requested": true,
                "provider_prompt_publication_requested": true,
                "install_publication_requested": true,
                "service_restart_publication_requested": true,
                "active_binary_publication_requested": true,
            }),
        ),
    ]);
    let release_artifact_publication_fixture_count = release_artifact_publication_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&release_artifact_publication_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:publication=0:artifact=0:release=0:install=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial:v1:no-release-artifact:no-public-artifact:no-publication:no-distribution:no-install:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "release_artifact_publication=false;release_artifact=false;public_artifact=false;signature=false;notarization=false;publication_queue=false;public_release=false;public_ga=false;distribution=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );

    let mut denials = source_terminal_decision
        .get("denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_terminal_operator_decision_report_required",
        "release_artifact_write_denied",
        "public_artifact_write_denied",
        "artifact_signature_notarization_acceptance_denied",
        "publication_queue_enqueue_denied",
        "publication_manifest_write_denied",
        "public_distribution_channel_delivery_denied",
        "public_version_tag_release_promotion_denied",
        "release_notes_changelog_materialization_denied",
        "terminal_operator_decision_is_not_release_approval",
        "activation_from_release_artifact_publication_denied",
        "memory_write_publication_denied",
        "provider_prompt_publication_denied",
        "install_restart_active_binary_publication_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_terminal_decision.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_ready": source_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_report_sha256": source_report_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_no_artifact_no_publication_no_release_no_distribution_no_install_no_context_memory_kg_provider_model_credential_channel_restart_binary",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": true,
            "activation_command_result_receipt_release_artifact_publication_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_v1",
            "activation_command_result_receipt_release_artifact_publication_mode": "native_route_stdout_only_release_artifact_publication_denial_no_artifact_no_publication_no_release_no_distribution_no_install_no_live",
            "activation_command_result_receipt_release_artifact_publication_decision": "blocked_noop_terminal_operator_decision_cannot_be_promoted_into_release_artifact_publication_authority",
            "source_terminal_operator_decision_public_claim_fixture_count": source_u64("activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"),
            "source_blocked_terminal_operator_decision_public_claim_fixture_count": source_u64("blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"),
            "source_accepted_terminal_operator_decision_public_claim_fixture_count": source_u64("accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"),
            "source_terminal_operator_decision_performed_count": source_u64("activation_command_result_receipt_terminal_operator_decision_performed_count"),
            "source_public_claim_promotion_performed_count": source_u64("activation_command_result_receipt_public_claim_promotion_performed_count"),
            "release_artifact_publication_fixtures_sha256": fixtures_sha256,
            "release_artifact_publication_contract_hash_sha256": contract_hash_sha256,
            "release_artifact_publication_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "required_activation_command_result_receipt_release_artifact_publication_surface_count": 12,
            "ready_activation_command_result_receipt_release_artifact_publication_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count": 12,
            "required_activation_command_result_receipt_release_artifact_publication_fixture_count": 10,
            "activation_command_result_receipt_release_artifact_publication_fixtures": release_artifact_publication_fixtures,
            "activation_command_result_receipt_release_artifact_publication_fixture_count": release_artifact_publication_fixture_count,
            "blocked_activation_command_result_receipt_release_artifact_publication_fixture_count": release_artifact_publication_fixture_count,
            "noop_activation_command_result_receipt_release_artifact_publication_fixture_count": release_artifact_publication_fixture_count,
            "allowed_activation_command_result_receipt_release_artifact_publication_fixture_count": 0,
            "accepted_activation_command_result_receipt_release_artifact_publication_fixture_count": 0,
            "activation_command_result_receipt_release_artifact_publication_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_artifact_publication_allowed": false,
            "release_artifact_publication_requested": false,
            "release_artifact_publication_accepted": false,
            "release_artifact_publication_recorded": false,
            "release_artifact_publication_persisted": false,
            "release_artifact_publication_materialized": false,
            "release_artifact_filesystem_written": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "artifact_signature_accepted": false,
            "artifact_notarization_accepted": false,
            "publication_queue_enqueued": false,
            "publication_manifest_written": false,
            "public_distribution_performed": false,
            "public_release_published": false,
            "public_ga_claimed": false,
            "public_claim_promoted": false,
            "public_version_tag_created": false,
            "release_notes_materialized": false,
            "changelog_materialized": false,
            "terminal_operator_decision_promoted_to_release_approval": false,
            "telegram_send_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_recorded": false,
            "activation_command_result_receipt_persisted": false,
            "activation_command_result_receipt_accepted": false,
            "activation_command_result_receipt_materialized": false,
            "activation_command_completion_ack_recorded": false,
            "activation_allowed_by_release_artifact_publication": false,
            "activation_allowed_by_terminal_operator_decision": false,
            "activation_allowed_by_result_receipt": false,
            "activation_allowed": false,
            "activation_performed": false,
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_write_execution_allowed": false,
            "memory_write_execution_ready": false,
            "memory_write_execution_performed": false,
            "memory_store_write_path_enabled": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_write_performed_count": 0,
            "memory_store_mutation_allowed": false,
            "memory_store_mutated": false,
            "rollback_execution_allowed": false,
            "rollback_executed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "secret_material_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_prompt_replay_enabled": false,
            "provider_invoked": false,
            "model_invoked": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "service_restart_performed": false,
            "active_binary_mutated": false,
            "activation_command_result_receipt_release_artifact_publication_surfaces": [
                "source_terminal_operator_decision_report_required",
                "release_artifact_write_denied",
                "public_artifact_write_denied",
                "artifact_signature_notarization_acceptance_denied",
                "publication_queue_enqueue_denied",
                "publication_manifest_write_denied",
                "public_distribution_channel_delivery_denied",
                "public_version_tag_release_promotion_denied",
                "release_notes_changelog_materialization_denied",
                "terminal_operator_decision_is_not_release_approval",
                "activation_from_release_artifact_publication_denied",
                "external_public_install_restart_active_binary_publication_denied"
            ],
            "denied_by_activation_command_result_receipt_release_artifact_publication": denials,
            "denied_by_activation_command_result_receipt_release_artifact_publication_count": denied_count,
            "current_live_enabled_lane_count": 28,
            "enablement_lane_count": 31,
            "ready_enablement_lane_count": 31,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial",
                    "status": "allowed_report_only",
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "installs_or_restarts": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence",
                    "status": "allowed_report_only_next_slice",
                    "records_publication_receipt": false,
                    "persists_publication_receipt": false,
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "release_artifact_publication_recorded",
            "release_artifact_publication_persisted",
            "release_artifact_publication_materialized",
            "release_artifact_filesystem_written",
            "release_artifact_written",
            "public_artifact_written",
            "artifact_signature_accepted",
            "artifact_notarization_accepted",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "public_release_published",
            "public_release_claimed",
            "public_ga_claimed",
            "public_claim_promoted",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "terminal_operator_decision_promoted_to_release_approval",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_terminal_operator_decision_recorded",
            "activation_command_result_receipt_terminal_operator_decision_persisted",
            "activation_command_result_receipt_public_claim_recorded",
            "activation_command_result_receipt_public_claim_promoted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_completion_ack_recorded",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "raw_payload_plaintext_recorded",
            "raw_payload_plaintext_persisted",
            "secret_material_read",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "provider_prompt_replay_enabled",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "filesystem_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_publication = hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_report();
    let source_ready = source_publication
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_status = source_publication
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let source_report_sha256 = sha256_json_value(&source_publication);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_u64 = |key: &str| -> u64 {
        source_publication
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let report_ready = source_ready
        && source_status == "ready"
        && route_count_source_command_accepted
        && source_u64(
            "activation_command_result_receipt_release_artifact_publication_fixture_count",
        ) == 10
        && source_u64(
            "blocked_activation_command_result_receipt_release_artifact_publication_fixture_count",
        ) == 10
        && source_u64(
            "allowed_activation_command_result_receipt_release_artifact_publication_fixture_count",
        ) == 0
        && source_u64(
            "activation_command_result_receipt_release_artifact_publication_performed_count",
        ) == 0;

    let receipt_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::json!(id));
            fixture.insert(
                "publication_result_receipt_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_release_artifact_publication_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_release_artifact_publication_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "publication_result_receipt_requested",
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
                "release_artifact_publication_recorded",
                "release_artifact_publication_persisted",
                "release_artifact_publication_materialized",
                "release_artifact_filesystem_written",
                "release_artifact_written",
                "public_artifact_written",
                "publication_queue_enqueued",
                "publication_manifest_written",
                "public_distribution_performed",
                "public_release_published",
                "public_ga_claimed",
                "public_claim_promoted",
                "public_version_tag_created",
                "release_notes_materialized",
                "changelog_materialized",
                "terminal_operator_decision_promoted_to_release_approval",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "activation_allowed",
                "activation_performed",
                "live_mutation_execution_performed",
                "memory_write_execution_performed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "rollback_executed",
                "secret_material_read",
                "credential_read",
                "secret_file_read",
                "provider_invoked",
                "model_invoked",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "publication_result_receipt_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };

    let publication_result_receipt_fixtures = serde_json::json!([
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-missing-publication-report",
            "blocked_noop",
            "source_release_artifact_publication_report_required",
            serde_json::json!({
                "source_release_artifact_publication_present": false,
                "source_release_artifact_publication_ready": false,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-record-request",
            "blocked_record_noop",
            "publication_result_receipt_recording_denied",
            serde_json::json!({"publication_result_receipt_record_requested": true}),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-persist-request",
            "blocked_persist_noop",
            "publication_result_receipt_persistence_denied",
            serde_json::json!({"publication_result_receipt_persist_requested": true}),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-materialize-filesystem-request",
            "blocked_materialize_noop",
            "publication_result_receipt_materialization_filesystem_write_denied",
            serde_json::json!({
                "publication_result_receipt_materialize_requested": true,
                "publication_result_receipt_filesystem_write_requested": true,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-ledger-index-queue-request",
            "blocked_ledger_index_queue_noop",
            "publication_result_receipt_ledger_index_queue_denied",
            serde_json::json!({
                "publication_result_receipt_ledger_write_requested": true,
                "publication_result_receipt_index_requested": true,
                "publication_result_receipt_enqueue_requested": true,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-export-query-observability-request",
            "blocked_export_query_observability_noop",
            "publication_result_receipt_export_query_observability_denied",
            serde_json::json!({
                "publication_result_receipt_export_requested": true,
                "publication_result_receipt_query_requested": true,
                "publication_result_receipt_observability_requested": true,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-delivery-request",
            "blocked_delivery_noop",
            "publication_result_receipt_delivery_denied",
            serde_json::json!({
                "publication_result_receipt_delivery_requested": true,
                "telegram_delivery_requested": true,
                "channel_delivery_requested": true,
                "external_delivery_requested": true,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-status-signature-request",
            "blocked_acceptance_noop",
            "publication_result_receipt_status_signature_acceptance_denied",
            serde_json::json!({
                "publication_result_receipt_status_acceptance_requested": true,
                "publication_result_receipt_signature_acceptance_requested": true,
                "publication_result_receipt_timestamp_acceptance_requested": true,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-completion-ack-request",
            "blocked_ack_noop",
            "publication_completion_ack_denied",
            serde_json::json!({"publication_completion_ack_requested": true}),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-authority-request",
            "blocked_authority_noop",
            "publication_result_receipt_cannot_authorize_publication_activation_or_install",
            serde_json::json!({
                "publication_authority_requested": true,
                "public_release_publish_requested": true,
                "public_distribution_requested": true,
                "release_artifact_write_requested": true,
                "activation_from_publication_receipt_requested": true,
                "memory_write_publication_receipt_requested": true,
                "provider_prompt_publication_receipt_requested": true,
                "install_publication_receipt_requested": true,
                "service_restart_publication_receipt_requested": true,
                "active_binary_publication_receipt_requested": true,
            }),
        ),
    ]);
    let publication_result_receipt_fixture_count = publication_result_receipt_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&publication_result_receipt_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-release-artifact-publication-result-receipt-no-persistence:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:record=0:persist=0:deliver=0:authority=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-release-artifact-publication-result-receipt-no-persistence:v1:no-record:no-persist:no-deliver:no-authority:no-install:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "publication_result_receipt=false;completion_ack=false;release_artifact=false;public_release=false;delivery=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );

    let mut denials = source_publication
        .get("denied_by_activation_command_result_receipt_release_artifact_publication")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_release_artifact_publication_report_required",
        "publication_result_receipt_recording_denied",
        "publication_result_receipt_persistence_denied",
        "publication_result_receipt_materialization_denied",
        "publication_result_receipt_filesystem_write_denied",
        "publication_result_receipt_ledger_index_queue_denied",
        "publication_result_receipt_export_query_observability_denied",
        "publication_result_receipt_delivery_denied",
        "publication_result_receipt_status_signature_acceptance_denied",
        "publication_completion_ack_denied",
        "publication_result_receipt_publication_authority_denied",
        "publication_result_receipt_activation_authority_denied",
        "publication_result_receipt_memory_provider_install_restart_active_binary_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_publication.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_ready": source_ready,
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_status": source_status,
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_report_sha256": source_report_sha256,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_record_no_persist_no_materialize_no_deliver_no_authority_no_install_no_context_memory_kg_provider_model_credential_channel_restart_binary",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready": true,
            "activation_command_result_receipt_release_artifact_publication_result_receipt_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_v1",
            "activation_command_result_receipt_release_artifact_publication_result_receipt_mode": "native_route_stdout_only_publication_result_receipt_no_persistence_no_delivery_no_authority_no_install_no_live",
            "activation_command_result_receipt_release_artifact_publication_result_receipt_decision": "blocked_noop_release_artifact_publication_result_receipts_cannot_be_persisted_or_promoted_into_authority",
            "source_release_artifact_publication_fixture_count": source_u64("activation_command_result_receipt_release_artifact_publication_fixture_count"),
            "source_blocked_release_artifact_publication_fixture_count": source_u64("blocked_activation_command_result_receipt_release_artifact_publication_fixture_count"),
            "source_accepted_release_artifact_publication_fixture_count": source_u64("accepted_activation_command_result_receipt_release_artifact_publication_fixture_count"),
            "source_release_artifact_publication_performed_count": source_u64("activation_command_result_receipt_release_artifact_publication_performed_count"),
            "publication_result_receipt_fixtures_sha256": fixtures_sha256,
            "publication_result_receipt_contract_hash_sha256": contract_hash_sha256,
            "publication_result_receipt_policy_hash_sha256": policy_hash_sha256,
            "publication_result_receipt_side_effect_hash_sha256": side_effect_hash_sha256,
            "required_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count": 12,
            "ready_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count": 12,
            "required_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": 10,
            "activation_command_result_receipt_release_artifact_publication_result_receipt_fixtures": publication_result_receipt_fixtures,
            "activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": publication_result_receipt_fixture_count,
            "blocked_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": publication_result_receipt_fixture_count,
            "noop_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": publication_result_receipt_fixture_count,
            "allowed_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": 0,
            "accepted_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "publication_result_receipt_allowed": false,
            "publication_result_receipt_accepted": false,
            "publication_result_receipt_recorded": false,
            "publication_result_receipt_persisted": false,
            "publication_result_receipt_materialized": false,
            "publication_result_receipt_filesystem_written": false,
            "publication_result_receipt_ledger_written": false,
            "publication_result_receipt_indexed": false,
            "publication_result_receipt_enqueued": false,
            "publication_result_receipt_delivered": false,
            "publication_result_receipt_exported": false,
            "publication_result_receipt_query_registered": false,
            "publication_result_receipt_observability_recorded": false,
            "publication_result_receipt_hash_bound": false,
            "publication_result_receipt_signature_accepted": false,
            "publication_result_receipt_timestamp_accepted": false,
            "publication_result_receipt_status_accepted": false,
            "publication_completion_ack_recorded": false,
            "publication_completion_ack_persisted": false,
            "publication_completion_ack_accepted": false,
            "release_artifact_publication_recorded": false,
            "release_artifact_publication_persisted": false,
            "release_artifact_publication_materialized": false,
            "release_artifact_filesystem_written": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "publication_queue_enqueued": false,
            "publication_manifest_written": false,
            "public_distribution_performed": false,
            "public_release_published": false,
            "public_ga_claimed": false,
            "public_claim_promoted": false,
            "public_version_tag_created": false,
            "release_notes_materialized": false,
            "changelog_materialized": false,
            "terminal_operator_decision_promoted_to_release_approval": false,
            "telegram_send_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_allowed_by_publication_result_receipt": false,
            "activation_allowed_by_release_artifact_publication": false,
            "activation_allowed_by_terminal_operator_decision": false,
            "activation_allowed_by_result_receipt": false,
            "activation_allowed": false,
            "activation_performed": false,
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_write_execution_allowed": false,
            "memory_write_execution_ready": false,
            "memory_write_execution_performed": false,
            "memory_store_write_path_enabled": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_write_performed_count": 0,
            "memory_store_mutation_allowed": false,
            "memory_store_mutated": false,
            "rollback_execution_allowed": false,
            "rollback_executed": false,
            "secret_material_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_prompt_replay_enabled": false,
            "provider_invoked": false,
            "model_invoked": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "service_restart_performed": false,
            "active_binary_mutated": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_release_artifact_publication_result_receipt_surfaces": [
                "source_release_artifact_publication_report_required",
                "publication_result_receipt_recording_denied",
                "publication_result_receipt_persistence_denied",
                "publication_result_receipt_materialization_denied",
                "publication_result_receipt_filesystem_write_denied",
                "publication_result_receipt_ledger_index_queue_denied",
                "publication_result_receipt_export_query_observability_denied",
                "publication_result_receipt_delivery_denied",
                "publication_result_receipt_status_signature_acceptance_denied",
                "publication_completion_ack_denied",
                "publication_result_receipt_authority_denied",
                "publication_result_receipt_external_install_restart_active_binary_denied"
            ],
            "denied_by_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence": denials,
            "denied_by_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_count": denied_count,
            "current_live_enabled_lane_count": 29,
            "enablement_lane_count": 32,
            "ready_enablement_lane_count": 32,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence",
                    "status": "allowed_report_only",
                    "records_publication_receipt": false,
                    "persists_publication_receipt": false,
                    "delivers_publication_receipt": false,
                    "derives_activation_authority": false,
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "installs_or_restarts": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
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
            "release_artifact_publication_recorded",
            "release_artifact_publication_persisted",
            "release_artifact_publication_materialized",
            "release_artifact_filesystem_written",
            "release_artifact_written",
            "public_artifact_written",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "public_release_published",
            "public_ga_claimed",
            "public_claim_promoted",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "terminal_operator_decision_promoted_to_release_approval",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "secret_material_read",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "provider_invoked",
            "model_invoked",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
            "filesystem_written",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

