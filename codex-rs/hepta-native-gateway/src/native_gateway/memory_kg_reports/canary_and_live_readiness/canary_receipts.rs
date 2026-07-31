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
