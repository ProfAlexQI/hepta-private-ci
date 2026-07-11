fn hepta_provider_channel_dry_run_plan_report() -> ProviderChannelDryRunPlanResponse {
    let route_matrix = control_ui_route_parity_report();
    crate::provider_domain::provider_channel_dry_run_plan_report(ProviderReportContext {
        current_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
    })
}

fn hepta_channel_adapter_status_inventory_report() -> HeptaChannelAdapterStatusInventoryResponse {
    let route_matrix = control_ui_route_parity_report();
    let channel_delivery_verified = env_truthy(HEPTA_CHANNEL_LIVE_DELIVERY_VERIFIED_ENV);
    let channel_read_verified =
        channel_delivery_verified || env_truthy(HEPTA_CHANNEL_LIVE_READ_VERIFIED_ENV);
    let channel_send_verified =
        channel_delivery_verified || env_truthy(HEPTA_CHANNEL_LIVE_SEND_VERIFIED_ENV);
    let live_adapter_enabled_count = if channel_read_verified && channel_send_verified {
        HEPTA_CHANNEL_ADAPTER_STATUS_ENTRIES.len()
    } else {
        HEPTA_CHANNEL_ADAPTER_STATUS_ENTRIES
            .iter()
            .filter(|adapter| adapter.live_read_enabled || adapter.live_send_enabled)
            .count()
    };
    HeptaChannelAdapterStatusInventoryResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if channel_read_verified && channel_send_verified {
            "ready"
        } else {
            "attention"
        },
        source_command: "/hepta-channel-adapter-status-inventory --json",
        native_route: true,
        compatibility_mode: "native_channel_adapter_disabled_status_inventory",
        side_effect_free: true,
        audit_date: "2026-05-20",
        channel_inventory_doc: "docs/release/HEPTA_CHANNEL_ADAPTER_STATUS_INVENTORY_2026-05-20.md",
        old_channel_ops_file_count: 13,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        adapter_count: HEPTA_CHANNEL_ADAPTER_STATUS_ENTRIES.len(),
        disabled_status_ready_count: HEPTA_CHANNEL_ADAPTER_STATUS_ENTRIES
            .iter()
            .filter(|adapter| adapter.disabled_status_ready)
            .count(),
        live_adapter_enabled_count,
        channel_status_inventory_ready: true,
        old_cli_invocation_compatibility_claimed: channel_delivery_verified,
        live_channel_read_enabled: channel_read_verified,
        live_channel_send_enabled: channel_send_verified,
        owner_handoff_performed: channel_delivery_verified,
        script_inventory_script: "scripts/hepta-channel-adapter-status-inventory.sh",
        channel_adapters: HEPTA_CHANNEL_ADAPTER_STATUS_ENTRIES,
        next_slices: &[
            "use local tooling/content inventory as the next no-side-effect planning slice",
            "keep Telegram owner handoff as separate explicit operator approval",
            "defer all channel live reads and sends until connector-specific approval",
        ],
        blockers: &[
            "channel_live_read_not_operator_approved",
            "channel_live_send_not_operator_approved",
            "telegram_owner_handoff_not_requested",
            "channel_credentials_not_read_by_inventory",
            "old_channel_cli_invocation_compatibility_not_claimed",
        ],
        side_effects: HeptaChannelAdapterStatusInventorySideEffects {
            channel_read_performed: false,
            channel_send_performed: false,
            credential_read: false,
            external_network_read: false,
            external_send_performed: false,
            gateway_mutation_performed: false,
            telegram_owner_handoff_performed: false,
            telegram_read_performed: false,
            telegram_send_performed: false,
            voice_call_performed: false,
            tts_audio_played: false,
            webhook_delivered: false,
            file_transfer_performed: false,
            native_post_mutation_performed: false,
            filesystem_written: false,
        },
    }
}

fn hepta_local_tooling_content_inventory_report() -> HeptaLocalToolingContentInventoryResponse {
    let route_matrix = control_ui_route_parity_report();
    HeptaLocalToolingContentInventoryResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "attention",
        source_command: "/hepta-local-tooling-content-inventory --json",
        native_route: true,
        compatibility_mode: "native_local_tooling_content_planning_inventory",
        side_effect_free: true,
        audit_date: "2026-05-20",
        local_tooling_inventory_doc: "docs/release/HEPTA_LOCAL_TOOLING_CONTENT_INVENTORY_2026-05-20.md",
        old_local_tooling_ops_file_count: 11,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        surface_count: HEPTA_LOCAL_TOOLING_CONTENT_SURFACES.len(),
        planner_ready_count: HEPTA_LOCAL_TOOLING_CONTENT_SURFACES
            .iter()
            .filter(|surface| surface.planner_ready)
            .count(),
        live_process_enabled_count: HEPTA_LOCAL_TOOLING_CONTENT_SURFACES
            .iter()
            .filter(|surface| surface.process_execution_enabled)
            .count(),
        filesystem_touch_enabled_count: HEPTA_LOCAL_TOOLING_CONTENT_SURFACES
            .iter()
            .filter(|surface| surface.filesystem_touch_enabled)
            .count(),
        network_read_enabled_count: HEPTA_LOCAL_TOOLING_CONTENT_SURFACES
            .iter()
            .filter(|surface| surface.network_read_enabled)
            .count(),
        tool_invocation_enabled_count: HEPTA_LOCAL_TOOLING_CONTENT_SURFACES
            .iter()
            .filter(|surface| surface.tool_invocation_enabled)
            .count(),
        local_tooling_inventory_ready: true,
        old_cli_invocation_compatibility_claimed: false,
        process_execution_enabled: false,
        filesystem_read_enabled: false,
        filesystem_write_enabled: false,
        network_read_enabled: false,
        tool_invocation_enabled: false,
        script_inventory_script: "scripts/hepta-local-tooling-content-inventory.sh",
        local_tooling_surfaces: HEPTA_LOCAL_TOOLING_CONTENT_SURFACES,
        next_slices: &[
            "use memory/capability absorption inventory for gap reporting only",
            "require operator approval before temp workspace process or filesystem smokes",
            "keep network fetches and tool invocation disabled until explicit scoped approval",
        ],
        blockers: &[
            "process_execution_not_operator_approved",
            "filesystem_read_not_operator_approved",
            "filesystem_write_not_operator_approved",
            "network_fetch_not_operator_approved",
            "tool_invocation_not_operator_approved",
            "old_local_tooling_cli_invocation_compatibility_not_claimed",
        ],
        side_effects: HeptaLocalToolingContentInventorySideEffects {
            process_spawned: false,
            filesystem_read: false,
            filesystem_written: false,
            external_network_read: false,
            tool_invoked: false,
            provider_invoked: false,
            model_invoked: false,
            credential_read: false,
            channel_read_performed: false,
            channel_send_performed: false,
            gateway_mutation_performed: false,
            native_post_mutation_performed: false,
            external_send_performed: false,
        },
    }
}

fn hepta_systems_tool_registry_inventory_report() -> HeptaSystemsToolRegistryInventoryResponse {
    let route_matrix = control_ui_route_parity_report();
    HeptaSystemsToolRegistryInventoryResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        source_command: "/hepta-systems-tool-registry-inventory --json",
        native_route: true,
        compatibility_mode: "native_systems_tool_registry_inventory_report",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint: HEPTA_SYSTEMS_TOOL_REGISTRY_INVENTORY_ENDPOINT,
        systems_plan_doc: "docs/architecture/HEPTA_SYSTEMS_PLUGINS_TOOLS_WORKFLOW_PLAN_2026-06-12.md",
        canonical_matrix_doc: "docs/architecture/HEPTA_SYSTEMS_CANONICAL_GATE_MATRIX_2026-06-12.md",
        inventory_report_script: "scripts/hepta-systems-tool-registry-inventory-report.sh",
        inventory_gate_script: "scripts/hepta-systems-tool-registry-inventory-gate.sh",
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        source_kind_count: HEPTA_SYSTEMS_TOOL_REGISTRY_INVENTORY_SOURCE_KINDS.len(),
        source_kinds: HEPTA_SYSTEMS_TOOL_REGISTRY_INVENTORY_SOURCE_KINDS,
        inventory_entry_field_count: HEPTA_SYSTEMS_TOOL_REGISTRY_INVENTORY_ENTRY_FIELDS.len(),
        inventory_entry_fields: HEPTA_SYSTEMS_TOOL_REGISTRY_INVENTORY_ENTRY_FIELDS,
        next_absorption_target: "native_systems_cockpit_read_only_tool_registry_view",
        tool_registry_inventory_ready: true,
        tool_invocation_enabled: false,
        mcp_server_start_enabled: false,
        plugin_install_enabled: false,
        connector_install_enabled: false,
        side_effects: HeptaSystemsToolRegistryInventorySideEffects {
            tool_invoked: false,
            mcp_server_started: false,
            plugin_installed: false,
            connector_installed: false,
            credential_read: false,
            provider_invoked: false,
            model_invoked: false,
            channel_send_performed: false,
            gateway_or_auth_mutated: false,
            native_post_mutation_performed: false,
        },
    }
}

fn hepta_systems_workflow_definition_registry_report()
-> HeptaSystemsWorkflowDefinitionRegistryResponse {
    let route_matrix = control_ui_route_parity_report();
    HeptaSystemsWorkflowDefinitionRegistryResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        source_command: "/hepta-systems-workflow-definition-registry --json",
        native_route: true,
        compatibility_mode: "native_systems_workflow_definition_registry_report",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_ENDPOINT,
        systems_plan_doc: "docs/architecture/HEPTA_SYSTEMS_PLUGINS_TOOLS_WORKFLOW_PLAN_2026-06-12.md",
        canonical_matrix_doc: "docs/architecture/HEPTA_SYSTEMS_CANONICAL_GATE_MATRIX_2026-06-12.md",
        definition_registry_report_script: "scripts/hepta-systems-workflow-definition-registry-report.sh",
        definition_registry_gate_script: "scripts/hepta-systems-workflow-definition-registry-gate.sh",
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        step_kind_count: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_STEP_KINDS.len(),
        step_kinds: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_STEP_KINDS,
        definition_entry_field_count: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_DEFINITION_FIELDS
            .len(),
        definition_entry_fields: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_DEFINITION_FIELDS,
        step_entry_field_count: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_STEP_FIELDS.len(),
        step_entry_fields: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_STEP_FIELDS,
        start_plan_field_count: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_START_PLAN_FIELDS.len(),
        start_plan_fields: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_START_PLAN_FIELDS,
        ready_to_append_start_event_requires_approval: true,
        start_plan_appends_event: false,
        step_projection_field_count:
            HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_STEP_PROJECTION_FIELDS.len(),
        step_projection_fields: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_STEP_PROJECTION_FIELDS,
        step_projection_event_type_count:
            HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_STEP_PROJECTION_EVENT_TYPES.len(),
        step_projection_event_types:
            HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_STEP_PROJECTION_EVENT_TYPES,
        step_projection_appends_events: false,
        pending_plan_field_count: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_PENDING_PLAN_FIELDS
            .len(),
        pending_plan_fields: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_PENDING_PLAN_FIELDS,
        pending_plan_mutates_event_log: false,
        write_proposal_field_count:
            HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_WRITE_PROPOSAL_FIELDS.len(),
        write_proposal_fields: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_WRITE_PROPOSAL_FIELDS,
        write_proposal_commits_event_log: false,
        write_validation_field_count:
            HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_WRITE_VALIDATION_FIELDS.len(),
        write_validation_fields: HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_WRITE_VALIDATION_FIELDS,
        write_validation_commits_event_log: false,
        next_absorption_target: "native_systems_cockpit_read_only_workflow_definition_registry",
        workflow_definition_registry_ready: true,
        workflow_activity_execution_enabled: false,
        tool_invocation_enabled: false,
        approval_resolution_enabled: false,
        delivery_send_enabled: false,
        ledger_mutation_enabled: false,
        side_effects: HeptaSystemsWorkflowDefinitionRegistrySideEffects {
            workflow_activity_executed: false,
            tool_invoked: false,
            approval_resolved: false,
            delivery_send_performed: false,
            ledger_mutated: false,
            credential_read: false,
            provider_invoked: false,
            model_invoked: false,
            channel_send_performed: false,
            gateway_or_auth_mutated: false,
            native_post_mutation_performed: false,
        },
    }
}
