fn hepta_memory_capability_absorption_inventory_report()
-> HeptaMemoryCapabilityAbsorptionInventoryResponse {
    let route_matrix = control_ui_route_parity_report();
    HeptaMemoryCapabilityAbsorptionInventoryResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "attention",
        source_command: "/hepta-memory-capability-absorption-inventory --json",
        native_route: true,
        compatibility_mode: "native_memory_capability_absorption_gap_inventory",
        side_effect_free: true,
        audit_date: "2026-05-20",
        memory_capability_inventory_doc: "docs/release/HEPTA_MEMORY_CAPABILITY_ABSORPTION_INVENTORY_2026-05-20.md",
        old_memory_capability_ops_file_count: 14,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        surface_count: HEPTA_MEMORY_CAPABILITY_ABSORPTION_SURFACES.len(),
        absorbed_or_represented_count: HEPTA_MEMORY_CAPABILITY_ABSORPTION_SURFACES
            .iter()
            .filter(|surface| surface.absorbed_or_represented)
            .count(),
        gap_report_ready_count: HEPTA_MEMORY_CAPABILITY_ABSORPTION_SURFACES
            .iter()
            .filter(|surface| surface.gap_report_ready)
            .count(),
        live_mutation_enabled_count: HEPTA_MEMORY_CAPABILITY_ABSORPTION_SURFACES
            .iter()
            .filter(|surface| surface.live_mutation_enabled)
            .count(),
        memory_capability_inventory_ready: true,
        old_cli_invocation_compatibility_claimed: false,
        memory_store_mutation_enabled: false,
        capability_registry_mutation_enabled: false,
        plugin_registry_mutation_enabled: false,
        coding_agent_spawn_enabled: false,
        search_provider_live_query_enabled: false,
        skill_workshop_write_enabled: false,
        script_inventory_script: "scripts/hepta-memory-capability-inventory.sh",
        memory_capability_surfaces: HEPTA_MEMORY_CAPABILITY_ABSORPTION_SURFACES,
        next_slices: &[
            "port remaining external release and hardening scripts as local-only status gates",
            "keep memory/plugin/capability writes disabled until explicit operator approval",
            "defer coding-agent spawn, search-provider query, and skill writes until scoped approval",
        ],
        blockers: &[
            "memory_store_mutation_not_operator_approved",
            "capability_registry_mutation_not_operator_approved",
            "plugin_registry_mutation_not_operator_approved",
            "coding_agent_spawn_not_operator_approved",
            "search_provider_live_query_not_operator_approved",
            "skill_workshop_write_not_operator_approved",
            "old_memory_capability_cli_invocation_compatibility_not_claimed",
        ],
        side_effects: HeptaMemoryCapabilityAbsorptionInventorySideEffects {
            memory_store_mutated: false,
            capability_registry_mutated: false,
            plugin_registry_mutated: false,
            coding_agent_spawned: false,
            skill_workshop_written: false,
            filesystem_read: false,
            filesystem_written: false,
            external_network_read: false,
            provider_invoked: false,
            model_invoked: false,
            credential_read: false,
            channel_read_performed: false,
            channel_send_performed: false,
            native_post_mutation_performed: false,
            gateway_mutation_performed: false,
            external_send_performed: false,
        },
    }
}

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_BLOCKED_ACTIONS: &[&str] = &[
    "memory_store_mutation",
    "hepta_intelligence_context_attachment",
    "kg_prompt_preview_execution",
    "kg_context_injection",
    "kg_external_adapter_read",
    "live_kg_write",
    "provider_model_invocation",
    "credential_read",
    "channel_delivery",
    "gateway_route_migration",
    "source_command_migration",
    "active_runtime_wiring",
    "service_restart",
    "release_or_public_ga_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_NEXT_SLICES: &[&str] = &[
    "turn memory live mutation from report-only to operator-approved staging fixture",
    "stage KG external adapter credential and rollback receipts without live writes",
    "accept bounded prompt-preview/context-handoff only after scoped operator packet",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_BLOCKED_ACTIONS:
    &[&str] = &[
    "invoke_shadow_execution_from_report_route",
    "enable_nonzero_traffic",
    "provider_model_invocation",
    "auth_secret_read",
    "credential_read",
    "external_network_call",
    "live_kg_write",
    "memory_store_write",
    "telegram_or_channel_delivery",
    "service_restart",
    "active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_NEXT_ACTIONS:
    &[&str] = &[
    "validate source route report on current-worktree native gateway",
    "install route through a separate live catch-up only after full preflight",
    "keep provider/model/KG/Memory effects disabled until an operator activation packet is accepted",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_BLOCKED_ACTIONS:
    &[&str] = &[
    "invoke_shadow_execution_from_report_route",
    "expose_live_activation_command",
    "enable_nonzero_traffic",
    "provider_model_invocation",
    "auth_secret_read",
    "credential_read",
    "external_network_call",
    "live_kg_write",
    "memory_store_write",
    "telegram_or_channel_delivery",
    "service_restart",
    "active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_NEXT_ACTIONS:
    &[&str] = &[
    "run controlled source gate with isolated fixture execution",
    "run full preflight before any live catch-up",
    "keep live report route side-effect-free until a separate operator activation packet is accepted",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_BLOCKED_ACTIONS:
    &[&str] = &[
    "accept_controlled_readback_receipt_as_evidence",
    "record_controlled_readback_receipt",
    "persist_controlled_readback_receipt",
    "materialize_controlled_readback_receipt",
    "write_controlled_readback_receipt_to_filesystem",
    "ledger_or_index_controlled_readback_receipt",
    "enqueue_or_deliver_controlled_readback_receipt",
    "export_query_or_observe_controlled_readback_receipt",
    "bind_hash_signature_timestamp_or_operator_identity",
    "record_completion_ack_from_readback_receipt",
    "derive_operator_approval_from_readback_receipt",
    "derive_activation_authority_from_readback_receipt",
    "promote_readback_receipt_to_public_claim",
    "invoke_shadow_execution_from_report_route",
    "expose_live_activation_command",
    "provider_model_invocation",
    "auth_secret_or_credential_read",
    "live_kg_or_memory_write",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_NEXT_ACTIONS:
    &[&str] = &[
    "run no-persistence source gate against the controlled route",
    "install read-only route through controlled live catch-up only after full preflight",
    "require a separate operator packet before any receipt can be accepted as activation evidence",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_BLOCKED_ACTIONS:
    &[&str] = &[
    "accept_readback_receipt_as_trusted_operator_record",
    "derive_operator_identity_from_readback_receipt",
    "derive_operator_intent_from_readback_receipt",
    "record_operator_approval_from_readback_receipt",
    "derive_activation_authority_from_readback_receipt",
    "enqueue_activation_request_from_readback_receipt",
    "expose_activation_command_from_readback_receipt",
    "enable_live_mutation_from_readback_receipt",
    "promote_public_claim_from_readback_receipt",
    "invoke_shadow_execution_from_report_route",
    "provider_model_invocation",
    "auth_secret_or_credential_read",
    "live_kg_or_memory_write",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_NEXT_ACTIONS:
    &[&str] = &[
    "run authority-denial source gate against the no-persistence route",
    "install read-only authority-denial route through controlled live catch-up only after full preflight",
    "keep receipt-derived authority inert until a separate trusted operator packet is accepted",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_BLOCKED_ACTIONS:
    &[&str] = &[
    "substitute_readback_receipt_for_trusted_operator_packet",
    "bind_readback_receipt_to_operator_packet",
    "extend_operator_packet_from_readback_receipt",
    "refresh_operator_packet_from_readback_receipt",
    "replay_operator_packet_from_readback_receipt",
    "materialize_trusted_operator_packet_from_readback_receipt",
    "accept_operator_identity_from_receipt_payload",
    "accept_operator_intent_from_receipt_payload",
    "accept_operator_approval_from_receipt_payload",
    "derive_activation_authority_from_receipt_payload",
    "enqueue_activation_request_from_receipt_payload",
    "expose_activation_command_from_receipt_payload",
    "enable_live_mutation_from_receipt_payload",
    "promote_public_claim_from_receipt_payload",
    "invoke_shadow_execution_from_report_route",
    "provider_model_invocation",
    "auth_secret_or_credential_read",
    "live_kg_or_memory_write",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_NEXT_ACTIONS:
    &[&str] = &[
    "run trusted-operator-packet-separation source gate against the authority-denial route",
    "install read-only packet-separation route through controlled live catch-up only after full preflight",
    "keep trusted operator packet acceptance on a separate explicit operator packet lane",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_BLOCKED_ACTIONS:
    &[&str] = &[
    "accept_operator_packet_without_identity",
    "accept_operator_packet_without_intent",
    "accept_operator_packet_without_signature",
    "accept_operator_packet_without_session",
    "accept_operator_packet_without_freshness",
    "accept_operator_packet_without_scope",
    "record_unverified_operator_packet",
    "persist_unverified_operator_packet",
    "derive_operator_approval_from_unverified_packet",
    "derive_activation_authority_from_unverified_packet",
    "enqueue_activation_request_from_unverified_packet",
    "expose_activation_command_from_unverified_packet",
    "enable_live_mutation_from_unverified_packet",
    "promote_public_claim_from_unverified_packet",
    "invoke_shadow_execution_from_report_route",
    "provider_model_invocation",
    "auth_secret_or_credential_read",
    "live_kg_or_memory_write",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_NEXT_ACTIONS:
    &[&str] = &[
    "run trusted-operator-packet-intake-precondition source gate against the packet-separation route",
    "install read-only intake-precondition route through controlled live catch-up only after full preflight",
    "accept no operator packet until identity, intent, signature, session, freshness, and scope all verify in a separate explicit lane",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_FIXTURES:
    &[TrustedOperatorPacketPartialPreconditionDenialFixture] = &[
    TrustedOperatorPacketPartialPreconditionDenialFixture {
        fixture_id: "partial_operator_packet_missing_identity",
        missing_precondition: "identity",
        verified_field_count: 5,
        missing_field_count: 1,
        packet_recorded: false,
        packet_persisted: false,
        packet_accepted: false,
        operator_approval_recorded: false,
        activation_authority_derived: false,
        activation_request_allowed: false,
        activation_command_exposed: false,
        live_mutation_allowed: false,
        public_claim_allowed: false,
    },
    TrustedOperatorPacketPartialPreconditionDenialFixture {
        fixture_id: "partial_operator_packet_missing_intent",
        missing_precondition: "intent",
        verified_field_count: 5,
        missing_field_count: 1,
        packet_recorded: false,
        packet_persisted: false,
        packet_accepted: false,
        operator_approval_recorded: false,
        activation_authority_derived: false,
        activation_request_allowed: false,
        activation_command_exposed: false,
        live_mutation_allowed: false,
        public_claim_allowed: false,
    },
    TrustedOperatorPacketPartialPreconditionDenialFixture {
        fixture_id: "partial_operator_packet_missing_signature",
        missing_precondition: "signature",
        verified_field_count: 5,
        missing_field_count: 1,
        packet_recorded: false,
        packet_persisted: false,
        packet_accepted: false,
        operator_approval_recorded: false,
        activation_authority_derived: false,
        activation_request_allowed: false,
        activation_command_exposed: false,
        live_mutation_allowed: false,
        public_claim_allowed: false,
    },
    TrustedOperatorPacketPartialPreconditionDenialFixture {
        fixture_id: "partial_operator_packet_missing_session",
        missing_precondition: "session",
        verified_field_count: 5,
        missing_field_count: 1,
        packet_recorded: false,
        packet_persisted: false,
        packet_accepted: false,
        operator_approval_recorded: false,
        activation_authority_derived: false,
        activation_request_allowed: false,
        activation_command_exposed: false,
        live_mutation_allowed: false,
        public_claim_allowed: false,
    },
    TrustedOperatorPacketPartialPreconditionDenialFixture {
        fixture_id: "partial_operator_packet_missing_freshness",
        missing_precondition: "freshness",
        verified_field_count: 5,
        missing_field_count: 1,
        packet_recorded: false,
        packet_persisted: false,
        packet_accepted: false,
        operator_approval_recorded: false,
        activation_authority_derived: false,
        activation_request_allowed: false,
        activation_command_exposed: false,
        live_mutation_allowed: false,
        public_claim_allowed: false,
    },
    TrustedOperatorPacketPartialPreconditionDenialFixture {
        fixture_id: "partial_operator_packet_missing_scope",
        missing_precondition: "scope",
        verified_field_count: 5,
        missing_field_count: 1,
        packet_recorded: false,
        packet_persisted: false,
        packet_accepted: false,
        operator_approval_recorded: false,
        activation_authority_derived: false,
        activation_request_allowed: false,
        activation_command_exposed: false,
        live_mutation_allowed: false,
        public_claim_allowed: false,
    },
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_BLOCKED_ACTIONS:
    &[&str] = &[
    "accept_partial_operator_packet_missing_identity",
    "accept_partial_operator_packet_missing_intent",
    "accept_partial_operator_packet_missing_signature",
    "accept_partial_operator_packet_missing_session",
    "accept_partial_operator_packet_missing_freshness",
    "accept_partial_operator_packet_missing_scope",
    "record_partial_operator_packet",
    "persist_partial_operator_packet",
    "derive_operator_approval_from_partial_operator_packet",
    "derive_activation_authority_from_partial_operator_packet",
    "enqueue_activation_request_from_partial_operator_packet",
    "expose_activation_command_from_partial_operator_packet",
    "enable_live_mutation_from_partial_operator_packet",
    "promote_public_claim_from_partial_operator_packet",
    "invoke_shadow_execution_from_report_route",
    "provider_model_invocation",
    "auth_secret_or_credential_read",
    "live_kg_or_memory_write",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_NEXT_ACTIONS:
    &[&str] = &[
    "run trusted-operator-packet-partial-precondition-denial-matrix source gate against the intake-precondition route",
    "install read-only partial-precondition denial matrix route through controlled live catch-up only after full preflight",
    "only consider complete-packet fixture validation after all six packet preconditions remain explicit and side-effect-free",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_FIXTURES:
    &[TrustedOperatorPacketCompletePreconditionAuthorityDenialFixture] = &[
    TrustedOperatorPacketCompletePreconditionAuthorityDenialFixture {
        fixture_id: "complete_operator_packet_all_preconditions_verified_authority_denied",
        verified_field_count: 6,
        missing_field_count: 0,
        identity_verified: true,
        intent_confirmed: true,
        signature_verified: true,
        session_bound: true,
        freshness_verified: true,
        scope_validated: true,
        acceptance_precondition_satisfied: true,
        packet_recorded: false,
        packet_persisted: false,
        packet_accepted: false,
        operator_approval_recorded: false,
        activation_authority_derived: false,
        activation_request_allowed: false,
        activation_command_exposed: false,
        live_mutation_allowed: false,
        public_claim_allowed: false,
    },
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_BLOCKED_ACTIONS:
    &[&str] = &[
    "record_complete_operator_packet_from_report_route",
    "persist_complete_operator_packet_from_report_route",
    "accept_complete_operator_packet_from_report_route",
    "record_operator_approval_from_complete_operator_packet",
    "derive_activation_authority_from_complete_operator_packet",
    "enqueue_activation_request_from_complete_operator_packet",
    "expose_activation_command_from_complete_operator_packet",
    "enable_live_mutation_from_complete_operator_packet",
    "promote_public_claim_from_complete_operator_packet",
    "invoke_shadow_execution_from_report_route",
    "provider_model_invocation",
    "auth_secret_or_credential_read",
    "live_kg_or_memory_write",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_NEXT_ACTIONS:
    &[&str] = &[
    "run trusted-operator-packet-complete-precondition-authority-denial source gate against the partial-precondition matrix route",
    "install read-only complete-precondition authority-denial route through controlled live catch-up only after full preflight",
    "keep complete packet acceptance separate from report routes until an operator-approved activation lane exists",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_BLOCKED_ACTIONS:
    &[&str] = &[
    "substitute_complete_packet_preconditions_for_operator_approval",
    "create_operator_approval_lane_from_complete_packet_fixture",
    "record_operator_approval_lane_from_report_route",
    "persist_operator_approval_lane_from_report_route",
    "enqueue_activation_lane_from_complete_packet_fixture",
    "derive_activation_authority_from_operator_approval_lane_report",
    "expose_activation_command_from_operator_approval_lane_report",
    "enable_live_mutation_from_operator_approval_lane_report",
    "promote_public_claim_from_operator_approval_lane_report",
    "invoke_shadow_execution_from_report_route",
    "provider_model_invocation",
    "auth_secret_or_credential_read",
    "live_kg_or_memory_write",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_NEXT_ACTIONS:
    &[&str] = &[
    "run trusted-operator-packet-complete-precondition-operator-approval-lane-separation source gate against the complete-precondition authority-denial route",
    "install read-only operator-approval lane separation route through controlled live catch-up only after full preflight",
    "spec an explicit operator-approved activation lane as a separate fail-closed scaffold before any accepting path exists",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "write_memory_from_report_route",
    "record_operator_approval_receipt_from_report_route",
    "persist_operator_approval_receipt_from_report_route",
    "expose_memory_write_execution_command_from_report_route",
    "run_post_write_validation_from_report_route",
    "attach_hepta_intelligence_context",
    "render_prompt_preview",
    "inject_context_into_prompt",
    "invoke_provider_or_model",
    "read_auth_secret_or_credential",
    "read_external_kg_adapter",
    "write_live_kg",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved memory live mutation durable lane source gate against the operator-approval lane separation route",
    "install memory live mutation durable lane route through controlled live catch-up after full preflight",
    "slice hepta-intelligence context attachment behind the enabled memory lane while keeping provider, KG write, and channel delivery disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "attach_unbounded_context",
    "attach_context_from_report_route",
    "render_prompt_preview_from_report_route",
    "materialize_prompt_payload_from_report_route",
    "inject_context_into_provider_prompt",
    "invoke_provider_or_model",
    "read_auth_secret_or_credential",
    "read_external_kg_adapter",
    "write_live_kg",
    "write_memory_from_report_route",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved Hepta Intelligence context attachment lane source gate against the memory durable mutation lane route",
    "install Hepta Intelligence context attachment lane route through controlled live catch-up after full preflight",
    "slice KG prompt-preview/read-only adapter lane while keeping KG live write, provider/model invocation, and channel delivery disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "render_prompt_preview_from_report_route",
    "materialize_prompt_payload_from_report_route",
    "attach_or_inject_context_from_report_route",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_live_kg",
    "write_memory_from_report_route",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved KG prompt-preview/read-only adapter lane source gate against the Hepta Intelligence context attachment lane route",
    "install KG prompt-preview/read-only adapter lane route through controlled live catch-up after full preflight",
    "slice explicit prompt-preview payload shape materialization while keeping provider/model invocation, KG live write, credential reads, and channel delivery disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "materialize_prompt_payload_from_report_route",
    "expose_raw_prompt_payload_from_report_route",
    "render_prompt_preview_from_report_route",
    "attach_or_inject_context_from_report_route",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_live_kg",
    "write_memory_from_report_route",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved KG prompt payload materialization lane source gate against the KG prompt-preview/read-only adapter lane route",
    "install KG prompt payload materialization lane route through controlled live catch-up after full preflight",
    "slice explicit redacted prompt payload acceptance receipt while keeping KG live write, provider/model invocation, credential reads, and channel delivery disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "record_prompt_payload_acceptance_receipt_from_report_route",
    "persist_prompt_payload_acceptance_receipt_from_report_route",
    "accept_prompt_payload_acceptance_receipt_from_report_route",
    "write_prompt_payload_acceptance_receipt_filesystem_artifact",
    "record_prompt_payload_acceptance_receipt_ledger_entry",
    "materialize_prompt_payload_from_report_route",
    "expose_raw_prompt_payload_from_report_route",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_live_kg",
    "promote_receipt_to_activation_authority",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_ACCEPTANCE_RECEIPT_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved KG prompt payload acceptance receipt lane source gate against the KG prompt payload materialization lane route",
    "install KG prompt payload acceptance receipt lane route through controlled live catch-up after full preflight",
    "slice explicit redacted payload readback audit receipt while keeping KG live write, provider/model invocation, credential reads, channel delivery, and report-route persistence disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "render_prompt_payload_readback_audit_receipt_from_report_route",
    "record_prompt_payload_readback_audit_receipt_from_report_route",
    "persist_prompt_payload_readback_audit_receipt_from_report_route",
    "accept_prompt_payload_readback_audit_receipt_from_report_route",
    "write_prompt_payload_readback_audit_receipt_filesystem_artifact",
    "record_prompt_payload_readback_audit_receipt_ledger_entry",
    "materialize_prompt_payload_from_report_route",
    "expose_raw_prompt_payload_from_report_route",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_live_kg",
    "promote_readback_audit_receipt_to_activation_authority",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_READBACK_AUDIT_RECEIPT_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved KG prompt payload readback audit receipt lane source gate against the KG prompt payload acceptance receipt lane route",
    "install KG prompt payload readback audit receipt lane route through controlled live catch-up after full preflight",
    "slice explicit context handoff acceptance lane while keeping context injection, KG live write, provider/model invocation, credential reads, channel delivery, and report-route persistence disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "attach_context_from_report_route",
    "inject_context_into_provider_prompt",
    "record_context_handoff_acceptance_from_report_route",
    "persist_context_handoff_acceptance_from_report_route",
    "accept_context_handoff_from_report_route",
    "write_context_handoff_filesystem_artifact",
    "record_context_handoff_ledger_entry",
    "render_prompt_preview_from_report_route",
    "materialize_prompt_payload_from_report_route",
    "expose_raw_prompt_payload_from_report_route",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_live_kg",
    "promote_context_handoff_acceptance_to_activation_authority",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved context handoff acceptance lane source gate against the KG prompt payload readback audit receipt lane route",
    "install context handoff acceptance lane route through controlled live catch-up after full preflight",
    "slice explicit context handoff receipt audit or bounded provider-router injection precondition while keeping actual context injection, KG live write, provider/model invocation, credential reads, channel delivery, and public release disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "attach_context_from_report_route",
    "inject_context_into_provider_prompt",
    "render_context_handoff_receipt_audit_from_report_route",
    "record_context_handoff_receipt_audit_from_report_route",
    "persist_context_handoff_receipt_audit_from_report_route",
    "accept_context_handoff_receipt_audit_from_report_route",
    "write_context_handoff_receipt_audit_filesystem_artifact",
    "record_context_handoff_receipt_audit_ledger_entry",
    "record_context_handoff_acceptance_from_report_route",
    "persist_context_handoff_acceptance_from_report_route",
    "accept_context_handoff_from_report_route",
    "render_prompt_preview_from_report_route",
    "materialize_prompt_payload_from_report_route",
    "expose_raw_context_or_prompt_payload_from_report_route",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_live_kg",
    "promote_context_handoff_receipt_audit_to_activation_authority",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved context handoff receipt audit lane source gate against the context handoff acceptance lane route",
    "install context handoff receipt audit lane route through controlled live catch-up after full preflight",
    "slice bounded provider-router injection precondition while keeping actual context injection, KG live write, provider/model invocation, credential reads, channel delivery, and public release disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "attach_context_from_report_route",
    "inject_context_into_provider_prompt",
    "mutate_provider_router_prompt_from_report_route",
    "materialize_raw_context_from_report_route",
    "render_provider_router_injection_payload_from_report_route",
    "record_provider_router_injection_precondition_from_report_route",
    "persist_provider_router_injection_precondition_from_report_route",
    "accept_provider_router_injection_precondition_from_report_route",
    "promote_provider_router_injection_precondition_to_activation_authority",
    "record_context_handoff_receipt_audit_from_report_route",
    "persist_context_handoff_receipt_audit_from_report_route",
    "accept_context_handoff_receipt_audit_from_report_route",
    "write_provider_router_injection_filesystem_artifact",
    "record_provider_router_injection_ledger_entry",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_live_kg",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved bounded provider-router injection precondition lane source gate against the context handoff receipt audit lane route",
    "install bounded provider-router injection precondition lane route through controlled live catch-up after full preflight",
    "slice a bounded provider-router injection dry-run envelope while keeping actual context injection, KG live write, provider/model invocation, credential reads, channel delivery, and public release disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "attach_context_from_report_route",
    "inject_context_into_provider_prompt",
    "mutate_provider_router_prompt_from_report_route",
    "materialize_raw_context_from_report_route",
    "construct_provider_router_injection_dry_run_envelope_from_report_route",
    "render_provider_router_injection_dry_run_envelope_from_report_route",
    "record_provider_router_injection_dry_run_envelope_from_report_route",
    "persist_provider_router_injection_dry_run_envelope_from_report_route",
    "accept_provider_router_injection_dry_run_envelope_from_report_route",
    "execute_provider_router_injection_dry_run_envelope_from_report_route",
    "promote_provider_router_injection_dry_run_envelope_to_activation_authority",
    "record_provider_router_injection_precondition_from_report_route",
    "persist_provider_router_injection_precondition_from_report_route",
    "accept_provider_router_injection_precondition_from_report_route",
    "write_provider_router_injection_dry_run_envelope_filesystem_artifact",
    "record_provider_router_injection_dry_run_envelope_ledger_entry",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_live_kg",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved bounded provider-router injection dry-run envelope lane source gate against the bounded provider-router injection precondition lane route",
    "install bounded provider-router injection dry-run envelope lane route through controlled live catch-up after full preflight",
    "slice a bounded provider-router injection dry-run envelope readback audit receipt while keeping actual context injection, KG live write, provider/model invocation, credential reads, channel delivery, and public release disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "attach_context_from_report_route",
    "inject_context_into_provider_prompt",
    "mutate_provider_router_prompt_from_report_route",
    "materialize_raw_context_from_report_route",
    "construct_provider_router_injection_dry_run_envelope_from_report_route",
    "render_provider_router_injection_dry_run_envelope_from_report_route",
    "record_provider_router_injection_dry_run_envelope_from_report_route",
    "persist_provider_router_injection_dry_run_envelope_from_report_route",
    "accept_provider_router_injection_dry_run_envelope_from_report_route",
    "execute_provider_router_injection_dry_run_envelope_from_report_route",
    "render_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route",
    "record_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route",
    "persist_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route",
    "accept_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route",
    "promote_provider_router_injection_dry_run_envelope_readback_audit_receipt_to_activation_authority",
    "write_provider_router_injection_dry_run_envelope_readback_audit_receipt_filesystem_artifact",
    "record_provider_router_injection_dry_run_envelope_readback_audit_receipt_ledger_entry",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_live_kg",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved bounded provider-router injection dry-run envelope readback audit receipt lane source gate against the dry-run envelope lane route",
    "install bounded provider-router injection dry-run envelope readback audit receipt lane route through controlled live catch-up after full preflight",
    "slice a bounded provider-router injection dry-run envelope receipt acceptance precondition while keeping actual context injection, KG live write, provider/model invocation, credential reads, channel delivery, and public release disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_BLOCKED_ACTIONS:
    &[&str] = &[
    "attach_context_from_report_route",
    "inject_context_into_provider_prompt",
    "mutate_provider_router_prompt_from_report_route",
    "materialize_raw_context_from_report_route",
    "construct_provider_router_injection_dry_run_envelope_from_report_route",
    "render_provider_router_injection_dry_run_envelope_from_report_route",
    "record_provider_router_injection_dry_run_envelope_from_report_route",
    "persist_provider_router_injection_dry_run_envelope_from_report_route",
    "accept_provider_router_injection_dry_run_envelope_from_report_route",
    "execute_provider_router_injection_dry_run_envelope_from_report_route",
    "render_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route",
    "record_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route",
    "persist_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route",
    "accept_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route",
    "acknowledge_provider_router_injection_dry_run_envelope_readback_audit_receipt_from_report_route",
    "perform_provider_router_injection_dry_run_envelope_readback_audit_receipt_no_op_handoff_from_report_route",
    "record_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_from_report_route",
    "persist_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_from_report_route",
    "accept_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_from_report_route",
    "promote_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_to_activation_authority",
    "write_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_filesystem_artifact",
    "record_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_ledger_entry",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_live_kg",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_NEXT_ACTIONS:
    &[&str] = &[
    "run operator-approved bounded provider-router injection dry-run envelope readback audit receipt acknowledgement no-op handoff lane source gate against the readback audit receipt lane route",
    "install bounded provider-router injection dry-run envelope readback audit receipt acknowledgement no-op handoff lane route through controlled live catch-up after full preflight",
    "slice a bounded provider-router injection dry-run envelope acknowledgement acceptance precondition while keeping actual acknowledgement, handoff, context injection, KG live write, provider/model invocation, credential reads, channel delivery, and public release disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_BLOCKED_ACTIONS:
    &[&str] = &[
    "accept_operator_canary_controlled_request_dispatch_budget_from_report_route",
    "consume_operator_canary_controlled_request_dispatch_budget_from_report_route",
    "dispatch_operator_canary_controlled_request_from_report_route",
    "execute_operator_canary_controlled_request_from_report_route",
    "record_operator_canary_controlled_request_noop_receipt_from_report_route",
    "persist_operator_canary_controlled_request_noop_receipt_from_report_route",
    "deliver_operator_canary_controlled_request_noop_receipt_from_report_route",
    "accept_operator_canary_controlled_request_noop_receipt_from_report_route",
    "materialize_operator_canary_controlled_request_noop_receipt_from_report_route",
    "materialize_operator_canary_controlled_request_payload_from_report_route",
    "write_operator_canary_controlled_request_payload_file",
    "inspect_operator_canary_controlled_request_raw_payload",
    "attach_context_from_report_route",
    "inject_context_into_provider_prompt",
    "mutate_provider_router_prompt_from_report_route",
    "read_kg_adapter_from_report_route",
    "construct_external_kg_adapter_client_from_report_route",
    "capture_kg_adapter_endpoint_or_credential_value",
    "read_auth_secret_or_credential",
    "write_memory_store",
    "write_live_kg",
    "invoke_provider_or_model",
    "telegram_or_channel_delivery",
    "service_restart_or_active_binary_mutation",
    "release_or_public_claim",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_NEXT_ACTIONS:
    &[&str] = &[
    "run operator canary controlled-request harness single-budget dispatch dry-run no-op receipt route gate against the acknowledgement no-op handoff route",
    "install canary controlled-request harness single-budget dispatch dry-run no-op receipt route through controlled live catch-up after full preflight",
    "slice operator-review readback index non-persistence while keeping budget acceptance, dispatch, execution, receipt persistence, context injection, Memory/KG writes, provider/model invocation, credential reads, channel delivery, and public release disabled",
];

const HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_SUPPORTED_KG_ADAPTERS: &[&str] =
    &["graphiti", "neo4j", "cocoindex"];

fn hepta_memory_intelligence_kg_full_enablement_runtime_readiness_report()
-> HeptaMemoryIntelligenceKgFullEnablementRuntimeReadinessResponse {
    let route_matrix = control_ui_route_parity_report();
    let memory = hepta_memory_capability_absorption_inventory_report();
    let core = hepta_gateway::hepta_core_fusion_readiness_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let readiness_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && memory.memory_capability_inventory_ready
        && memory.surface_count == 14
        && memory.absorbed_or_represented_count == 14
        && memory.gap_report_ready_count == 14
        && memory.live_mutation_enabled_count == 0
        && !memory.memory_store_mutation_enabled
        && core.full_fusion_complete
        && core.active_binary_package == "hepta-cli"
        && core.phase_5_engine_dependency_closure_remaining_dependency_count == 0
        && core.phase_5_engine_dependency_closure_blockers.is_empty();

    HeptaMemoryIntelligenceKgFullEnablementRuntimeReadinessResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if readiness_ready { "ready" } else { "blocked" },
        source_command: "/hepta-memory-intelligence-kg-full-enablement-runtime-readiness --json",
        native_route: true,
        compatibility_mode: "native_full_enablement_runtime_readiness_route_source_only",
        side_effect_free: true,
        audit_date: "2026-06-01",
        endpoint: HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_READINESS_ENDPOINT,
        readiness_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_READINESS_ROUTE.md",
        source_activation_readiness_gate: "hepta_memory_intelligence_kg_full_enablement_activation_readiness_gate",
        source_activation_readiness_script: "scripts/hepta-memory-intelligence-kg-full-enablement-activation-readiness-gate.sh",
        source_memory_inventory_endpoint: HEPTA_MEMORY_CAPABILITY_ABSORPTION_INVENTORY_ENDPOINT,
        source_core_fusion_endpoint: HEPTA_CORE_FUSION_READINESS_ENDPOINT,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        runtime_readiness_route_wired: true,
        runtime_readiness_route_active_install_performed_by_this_gate: false,
        full_enablement_activation_readiness_ready: readiness_ready,
        full_enablement_activation_readiness_status: "ready_for_operator_approved_activation_slicing",
        live_activation_status: "not_performed_by_this_route",
        core_full_fusion_complete: core.full_fusion_complete,
        active_binary_package: core.active_binary_package,
        remaining_direct_codex_dependency_count: core
            .phase_5_engine_dependency_closure_remaining_dependency_count,
        hepta_core_direct_memory_intelligence_dependency_count: 0,
        active_service_stack_consumes_memory_intelligence: true,
        memory_surface_count: memory.surface_count,
        absorbed_or_represented_count: memory.absorbed_or_represented_count,
        gap_report_ready_count: memory.gap_report_ready_count,
        live_mutation_enabled_count: memory.live_mutation_enabled_count,
        memory_store_mutation_enabled: memory.memory_store_mutation_enabled,
        kg_source_gate_count: 5,
        kg_ready_source_gate_count: 5,
        kg_blocked_source_gate_count: 5,
        kg_report_only_source_gate_count: 5,
        kg_required_total_preflight_requirement_count: 19,
        kg_missing_total_preflight_requirement_count: 19,
        enablement_lane_count: 6,
        ready_enablement_lane_count: 6,
        current_live_enabled_lane_count: 0,
        rust_contract_reference_count: 7,
        rust_contract_compile_checked_count: 7,
        operator_approval_required_before_activation: true,
        operator_activation_receipt_required: true,
        rollback_kill_switch_required: true,
        long_soak_required_before_mutation: true,
        context_handoff_acceptance_required: true,
        external_adapter_credentials_required_before_adapter_live: true,
        bounded_prompt_preview_scope_required: true,
        blocked_activation_actions: HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_BLOCKED_ACTIONS,
        next_slices: HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_NEXT_SLICES,
        side_effects: HeptaMemoryIntelligenceKgFullEnablementRuntimeReadinessSideEffects {
            full_live_enablement_performed: false,
            memory_store_mutated: false,
            capability_registry_mutated: false,
            plugin_registry_mutated: false,
            hepta_intelligence_context_attached: false,
            prompt_preview_rendered: false,
            prompt_payload_materialized: false,
            context_injection_performed: false,
            model_invoked: false,
            provider_invoked: false,
            external_kg_adapter_read_performed: false,
            graphiti_client_constructed: false,
            neo4j_client_constructed: false,
            cocoindex_client_constructed: false,
            network_call_performed: false,
            external_db_write_performed: false,
            live_kg_write_performed: false,
            credential_read: false,
            channel_send_performed: false,
            gateway_route_migration_performed: false,
            source_command_migration_performed: false,
            active_runtime_wired: false,
            service_restart_performed: false,
            active_binary_mutated: false,
            filesystem_written: false,
            release_artifact_written: false,
            public_release_claimed: false,
            public_ga_claimed: false,
        },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_readiness_report()
-> HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionReadinessResponse {
    let route_matrix = control_ui_route_parity_report();
    let runtime_readiness = hepta_memory_intelligence_kg_full_enablement_runtime_readiness_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && runtime_readiness.status == "ready"
        && runtime_readiness.full_enablement_activation_readiness_ready
        && runtime_readiness.live_mutation_enabled_count == 0
        && runtime_readiness.current_live_enabled_lane_count == 0
        && !runtime_readiness.side_effects.model_invoked
        && !runtime_readiness.side_effects.provider_invoked
        && !runtime_readiness.side_effects.credential_read
        && !runtime_readiness.side_effects.live_kg_write_performed
        && !runtime_readiness.side_effects.memory_store_mutated;

    HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionReadinessResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command: "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-readiness --json",
        native_route: true,
        compatibility_mode:
            "native_runtime_provider_router_shadow_context_activation_execution_readiness_route_source_only",
        side_effect_free: true,
        audit_date: "2026-06-11",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_READINESS_ENDPOINT,
        readiness_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_READINESS_ROUTE.md",
        source_execution_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-gate.sh",
        source_runtime_readiness_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_READINESS_ENDPOINT,
        source_runtime_execution_surface:
            "hepta-runtime-model-provider-memory-context-shadow-activation-execution-v1",
        source_runtime_execution_method: "execute_memory_context_activation_shadow",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        live_route_active_install_performed_by_this_gate: false,
        runtime_readiness_ready: runtime_readiness.full_enablement_activation_readiness_ready,
        runtime_readiness_status: runtime_readiness.status,
        operator_approved_shadow_context_activation_execution_report_ready: report_ready,
        runtime_owned_execution_surface_present: true,
        release_gate_required: true,
        operator_release_approval_required: true,
        canary_telemetry_required: true,
        rollback_kill_switch_required: true,
        post_activation_watchdog_soak_plan_required: true,
        idempotency_required: true,
        traffic_percent_ppm_required: 0,
        context_handoff_acceptance_required: true,
        shadow_context_attachment_supported_by_runtime: true,
        execution_invoked_by_report_route: false,
        live_route_exposes_activation_command: false,
        provider_invocation_allowed: false,
        provider_invocation_performed: false,
        model_invocation_allowed: false,
        model_invocation_performed: false,
        auth_secret_read_allowed: false,
        auth_secret_read_performed: false,
        credential_read_allowed: false,
        credential_read_performed: false,
        external_network_call_allowed: false,
        external_network_call_performed: false,
        live_kg_write_allowed: false,
        live_kg_write_performed: false,
        live_mutation_enabled_count: runtime_readiness.live_mutation_enabled_count,
        current_live_enabled_lane_count: runtime_readiness.current_live_enabled_lane_count,
        blocked_execution_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionSideEffects {
                report_route_invoked_runtime_execution: false,
                runtime_router_shadow_handoff_mutated_by_report_route: false,
                live_7373_router_mutated_by_report_route: false,
                feature_flag_mutated_in_live_7373_by_report_route: false,
                context_attached_to_live_7373_prompt_by_report_route: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                live_kg_write_performed: false,
                memory_store_mutated: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
            },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_report()
-> HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledResponse {
    let route_matrix = control_ui_route_parity_report();
    let readiness =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_readiness_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && readiness.status == "ready"
        && readiness.operator_approved_shadow_context_activation_execution_report_ready
        && readiness.runtime_owned_execution_surface_present
        && readiness.live_mutation_enabled_count == 0
        && readiness.current_live_enabled_lane_count == 0
        && !readiness.execution_invoked_by_report_route
        && !readiness.live_route_exposes_activation_command
        && !readiness.provider_invocation_performed
        && !readiness.model_invocation_performed
        && !readiness.auth_secret_read_performed
        && !readiness.credential_read_performed
        && !readiness.external_network_call_performed
        && !readiness.live_kg_write_performed
        && !readiness.side_effects.memory_store_mutated
        && !readiness.side_effects.service_restarted
        && !readiness.side_effects.active_binary_mutated;

    HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command: "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled --json",
        native_route: true,
        compatibility_mode:
            "native_runtime_provider_router_shadow_context_activation_execution_controlled_route_source_only",
        side_effect_free: true,
        audit_date: "2026-06-11",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_ENDPOINT,
        readiness_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_READINESS_ENDPOINT,
        readiness_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_READINESS_ROUTE.md",
        controlled_route_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_ROUTE.md",
        source_execution_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-gate.sh",
        source_controlled_route_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-route-gate.sh",
        source_runtime_execution_surface:
            "hepta-runtime-model-provider-memory-context-shadow-activation-execution-v1",
        source_runtime_execution_method: "execute_memory_context_activation_shadow",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        readiness_route_ready: readiness
            .operator_approved_shadow_context_activation_execution_report_ready,
        readiness_route_status: readiness.status,
        controlled_shadow_execution_report_ready: report_ready,
        runtime_owned_execution_surface_present: true,
        controlled_execution_contract:
            "hepta-runtime-provider-router-shadow-context-activation-controlled-report-v1",
        isolated_fixture_execution_required: true,
        isolated_fixture_execution_performed_by_source_gate: true,
        live_route_execution_invoked: false,
        report_route_exposes_activation_command: false,
        release_gate_required: true,
        operator_release_approval_required: true,
        canary_telemetry_required: true,
        rollback_kill_switch_required: true,
        post_activation_watchdog_soak_plan_required: true,
        idempotency_required: true,
        traffic_percent_ppm_required: 0,
        readback_receipt_required: true,
        audit_evidence_required: true,
        feature_flag_mutation_scope: "isolated_source_fixture_only",
        context_attachment_scope: "isolated_source_fixture_only",
        provider_invocation_allowed: false,
        provider_invocation_performed: false,
        model_invocation_allowed: false,
        model_invocation_performed: false,
        auth_secret_read_allowed: false,
        auth_secret_read_performed: false,
        credential_read_allowed: false,
        credential_read_performed: false,
        external_network_call_allowed: false,
        external_network_call_performed: false,
        live_kg_write_allowed: false,
        live_kg_write_performed: false,
        live_memory_write_allowed: false,
        live_memory_write_performed: false,
        live_mutation_enabled_count: readiness.live_mutation_enabled_count,
        current_live_enabled_lane_count: readiness.current_live_enabled_lane_count,
        blocked_execution_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledSideEffects {
                report_route_invoked_runtime_execution: false,
                source_gate_invokes_isolated_fixture_execution: true,
                isolated_fixture_router_mutated_by_source_gate: true,
                live_7373_router_mutated_by_report_route: false,
                feature_flag_mutated_in_live_7373_by_report_route: false,
                context_attached_to_live_7373_prompt_by_report_route: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                live_kg_write_performed: false,
                memory_store_mutated: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
            },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_no_persistence_report()
-> HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptNoPersistenceResponse{
    let route_matrix = control_ui_route_parity_report();
    let controlled =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && controlled.status == "ready"
        && controlled.controlled_shadow_execution_report_ready
        && controlled.readback_receipt_required
        && controlled.audit_evidence_required
        && controlled.live_mutation_enabled_count == 0
        && controlled.current_live_enabled_lane_count == 0
        && !controlled.live_route_execution_invoked
        && !controlled.report_route_exposes_activation_command
        && !controlled.provider_invocation_performed
        && !controlled.model_invocation_performed
        && !controlled.auth_secret_read_performed
        && !controlled.credential_read_performed
        && !controlled.external_network_call_performed
        && !controlled.live_kg_write_performed
        && !controlled.live_memory_write_performed
        && !controlled
            .side_effects
            .live_7373_router_mutated_by_report_route
        && !controlled.side_effects.memory_store_mutated
        && !controlled.side_effects.channel_send_performed
        && !controlled.side_effects.external_send_performed
        && !controlled.side_effects.service_restarted
        && !controlled.side_effects.active_binary_mutated
        && !controlled.side_effects.release_artifact_written
        && !controlled.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptNoPersistenceResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command: "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence --json",
        native_route: true,
        compatibility_mode:
            "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_no_persistence_route_source_only",
        side_effect_free: true,
        audit_date: "2026-06-11",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_ENDPOINT,
        controlled_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_ENDPOINT,
        controlled_route_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_ROUTE.md",
        readback_receipt_no_persistence_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_GATE.md",
        source_controlled_route_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-route-gate.sh",
        source_readback_receipt_no_persistence_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        controlled_route_ready: controlled.controlled_shadow_execution_report_ready,
        controlled_route_status: controlled.status,
        controlled_shadow_execution_report_ready: controlled.controlled_shadow_execution_report_ready,
        readback_receipt_no_persistence_ready: report_ready,
        readback_receipt_schema_declared: true,
        readback_receipt_requested: true,
        readback_receipt_allowed: false,
        readback_receipt_shape_accepted: false,
        readback_receipt_recorded: false,
        readback_receipt_persisted: false,
        readback_receipt_materialized: false,
        readback_receipt_filesystem_written: false,
        readback_receipt_ledger_written: false,
        readback_receipt_indexed: false,
        readback_receipt_enqueued: false,
        readback_receipt_delivered: false,
        readback_receipt_exported: false,
        readback_receipt_query_registered: false,
        readback_receipt_observability_recorded: false,
        readback_receipt_hash_bound: false,
        readback_receipt_signature_hash_recorded: false,
        readback_receipt_timestamp_recorded: false,
        readback_receipt_operator_identity_accepted: false,
        readback_receipt_status_accepted: false,
        completion_ack_recorded: false,
        completion_ack_persisted: false,
        completion_ack_accepted: false,
        operator_approval_from_receipt_accepted: false,
        activation_from_receipt_allowed: false,
        activation_authority_derived: false,
        public_claim_from_receipt_allowed: false,
        report_route_invokes_shadow_execution: false,
        report_route_exposes_activation_command: false,
        live_mutation_enabled_count: controlled.live_mutation_enabled_count,
        current_live_enabled_lane_count: controlled.current_live_enabled_lane_count,
        readback_receipt_surface_count: 10,
        blocked_readback_receipt_fixture_count: 10,
        allowed_readback_receipt_fixture_count: 0,
        blocked_readback_receipt_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptNoPersistenceSideEffects {
                report_route_invoked_runtime_execution: false,
                source_gate_invokes_isolated_fixture_execution: true,
                live_7373_router_mutated_by_report_route: false,
                readback_receipt_recorded: false,
                readback_receipt_persisted: false,
                readback_receipt_materialized: false,
                readback_receipt_filesystem_written: false,
                readback_receipt_exported: false,
                readback_receipt_query_registered: false,
                readback_receipt_observability_recorded: false,
                completion_ack_recorded: false,
                completion_ack_persisted: false,
                operator_approval_from_receipt_accepted: false,
                activation_from_receipt_allowed: false,
                activation_authority_derived: false,
                public_claim_from_receipt_allowed: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                live_kg_write_performed: false,
                memory_store_mutated: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
            },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_authority_denial_report()
-> HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptAuthorityDenialResponse{
    let route_matrix = control_ui_route_parity_report();
    let no_persistence =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_no_persistence_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && no_persistence.status == "ready"
        && no_persistence.readback_receipt_no_persistence_ready
        && no_persistence.readback_receipt_requested
        && !no_persistence.readback_receipt_allowed
        && !no_persistence.readback_receipt_recorded
        && !no_persistence.readback_receipt_persisted
        && !no_persistence.operator_approval_from_receipt_accepted
        && !no_persistence.activation_from_receipt_allowed
        && !no_persistence.activation_authority_derived
        && !no_persistence.public_claim_from_receipt_allowed
        && !no_persistence.report_route_invokes_shadow_execution
        && !no_persistence.report_route_exposes_activation_command
        && no_persistence.live_mutation_enabled_count == 0
        && no_persistence.current_live_enabled_lane_count == 0
        && !no_persistence.side_effects.provider_invoked
        && !no_persistence.side_effects.model_invoked
        && !no_persistence.side_effects.auth_secret_read
        && !no_persistence.side_effects.credential_read
        && !no_persistence.side_effects.live_kg_write_performed
        && !no_persistence.side_effects.memory_store_mutated
        && !no_persistence.side_effects.channel_send_performed
        && !no_persistence.side_effects.service_restarted
        && !no_persistence.side_effects.active_binary_mutated
        && !no_persistence.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptAuthorityDenialResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command: "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial --json",
        native_route: true,
        compatibility_mode:
            "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_authority_denial_route_source_only",
        side_effect_free: true,
        audit_date: "2026-06-11",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_ENDPOINT,
        no_persistence_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_ENDPOINT,
        no_persistence_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_GATE.md",
        authority_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_GATE.md",
        source_no_persistence_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence-gate.sh",
        source_authority_denial_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        no_persistence_route_ready: no_persistence.readback_receipt_no_persistence_ready,
        no_persistence_route_status: no_persistence.status,
        readback_receipt_no_persistence_ready: no_persistence.readback_receipt_no_persistence_ready,
        readback_receipt_authority_denial_ready: report_ready,
        readback_receipt_authority_boundary_declared: true,
        readback_receipt_shape_observed: true,
        readback_receipt_shape_accepted: false,
        trusted_operator_acceptance_record_required: true,
        trusted_operator_acceptance_record_present: false,
        trusted_operator_acceptance_record_accepted: false,
        operator_identity_verified_from_receipt: false,
        operator_intent_confirmed_from_receipt: false,
        operator_approval_from_receipt_accepted: false,
        activation_authority_derived: false,
        activation_request_from_receipt_allowed: false,
        activation_command_from_receipt_exposed: false,
        live_mutation_from_receipt_allowed: false,
        public_claim_from_receipt_allowed: false,
        public_release_from_receipt_allowed: false,
        report_route_invokes_shadow_execution: false,
        report_route_exposes_activation_command: false,
        live_mutation_enabled_count: no_persistence.live_mutation_enabled_count,
        current_live_enabled_lane_count: no_persistence.current_live_enabled_lane_count,
        receipt_authority_fixture_count: 8,
        blocked_receipt_authority_fixture_count: 8,
        allowed_receipt_authority_fixture_count: 0,
        blocked_receipt_authority_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptAuthorityDenialSideEffects {
                report_route_invoked_runtime_execution: false,
                source_gate_invokes_isolated_fixture_execution: true,
                live_7373_router_mutated_by_report_route: false,
                readback_receipt_recorded: false,
                readback_receipt_persisted: false,
                trusted_operator_acceptance_recorded: false,
                trusted_operator_acceptance_record_persisted: false,
                operator_identity_verified: false,
                operator_intent_confirmed: false,
                operator_approval_recorded: false,
                activation_authority_recorded: false,
                activation_request_enqueued: false,
                activation_command_exposed: false,
                public_claim_recorded: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                live_kg_write_performed: false,
                memory_store_mutated: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
            },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_separation_report()
-> HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketSeparationResponse{
    let route_matrix = control_ui_route_parity_report();
    let authority_denial =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_authority_denial_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && authority_denial.status == "ready"
        && authority_denial.readback_receipt_authority_denial_ready
        && authority_denial.readback_receipt_shape_observed
        && !authority_denial.readback_receipt_shape_accepted
        && authority_denial.trusted_operator_acceptance_record_required
        && !authority_denial.trusted_operator_acceptance_record_present
        && !authority_denial.trusted_operator_acceptance_record_accepted
        && !authority_denial.operator_identity_verified_from_receipt
        && !authority_denial.operator_intent_confirmed_from_receipt
        && !authority_denial.operator_approval_from_receipt_accepted
        && !authority_denial.activation_authority_derived
        && !authority_denial.activation_request_from_receipt_allowed
        && !authority_denial.activation_command_from_receipt_exposed
        && !authority_denial.live_mutation_from_receipt_allowed
        && !authority_denial.public_claim_from_receipt_allowed
        && !authority_denial.public_release_from_receipt_allowed
        && !authority_denial.report_route_invokes_shadow_execution
        && !authority_denial.report_route_exposes_activation_command
        && authority_denial.live_mutation_enabled_count == 0
        && authority_denial.current_live_enabled_lane_count == 0
        && !authority_denial.side_effects.provider_invoked
        && !authority_denial.side_effects.model_invoked
        && !authority_denial.side_effects.auth_secret_read
        && !authority_denial.side_effects.credential_read
        && !authority_denial.side_effects.live_kg_write_performed
        && !authority_denial.side_effects.memory_store_mutated
        && !authority_denial.side_effects.channel_send_performed
        && !authority_denial.side_effects.service_restarted
        && !authority_denial.side_effects.active_binary_mutated
        && !authority_denial.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketSeparationResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command: "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-separation --json",
        native_route: true,
        compatibility_mode:
            "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_separation_route_source_only",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_ENDPOINT,
        authority_denial_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_ENDPOINT,
        authority_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_GATE.md",
        trusted_operator_packet_separation_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_GATE.md",
        source_authority_denial_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial-gate.sh",
        source_trusted_operator_packet_separation_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-separation-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        authority_denial_route_ready: authority_denial.readback_receipt_authority_denial_ready,
        authority_denial_route_status: authority_denial.status,
        readback_receipt_authority_denial_ready: authority_denial.readback_receipt_authority_denial_ready,
        trusted_operator_packet_separation_ready: report_ready,
        readback_receipt_shape_observed: authority_denial.readback_receipt_shape_observed,
        readback_receipt_shape_accepted: false,
        independent_trusted_operator_packet_required: true,
        independent_trusted_operator_packet_present: false,
        independent_trusted_operator_packet_accepted: false,
        readback_receipt_can_substitute_operator_packet: false,
        readback_receipt_can_bind_operator_packet: false,
        readback_receipt_can_extend_operator_packet: false,
        readback_receipt_can_refresh_operator_packet: false,
        readback_receipt_can_replay_operator_packet: false,
        readback_receipt_can_materialize_operator_packet: false,
        operator_packet_identity_required: true,
        operator_packet_intent_required: true,
        operator_packet_signature_required: true,
        operator_packet_session_required: true,
        operator_packet_freshness_required: true,
        operator_packet_scope_required: true,
        operator_identity_verified_from_packet: false,
        operator_intent_confirmed_from_packet: false,
        operator_approval_from_packet_accepted: false,
        activation_authority_from_packet_derived: false,
        activation_request_from_packet_allowed: false,
        activation_command_from_packet_exposed: false,
        live_mutation_from_packet_allowed: false,
        public_claim_from_packet_allowed: false,
        report_route_invokes_shadow_execution: false,
        report_route_exposes_activation_command: false,
        live_mutation_enabled_count: authority_denial.live_mutation_enabled_count,
        current_live_enabled_lane_count: authority_denial.current_live_enabled_lane_count,
        packet_separation_fixture_count: 9,
        blocked_packet_substitution_fixture_count: 9,
        allowed_packet_substitution_fixture_count: 0,
        blocked_packet_separation_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketSeparationSideEffects {
                report_route_invoked_runtime_execution: false,
                source_gate_invokes_isolated_fixture_execution: true,
                live_7373_router_mutated_by_report_route: false,
                readback_receipt_recorded: false,
                readback_receipt_persisted: false,
                trusted_operator_packet_recorded: false,
                trusted_operator_packet_persisted: false,
                trusted_operator_packet_materialized: false,
                trusted_operator_packet_accepted: false,
                receipt_substituted_operator_packet: false,
                receipt_bound_operator_packet: false,
                receipt_extended_operator_packet: false,
                receipt_refreshed_operator_packet: false,
                receipt_replayed_operator_packet: false,
                operator_identity_verified: false,
                operator_intent_confirmed: false,
                operator_approval_recorded: false,
                activation_authority_recorded: false,
                activation_request_enqueued: false,
                activation_command_exposed: false,
                public_claim_recorded: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                live_kg_write_performed: false,
                memory_store_mutated: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
            },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_intake_precondition_report()
-> HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketIntakePreconditionResponse{
    let route_matrix = control_ui_route_parity_report();
    let packet_separation =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_separation_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && packet_separation.status == "ready"
        && packet_separation.trusted_operator_packet_separation_ready
        && packet_separation.independent_trusted_operator_packet_required
        && !packet_separation.independent_trusted_operator_packet_present
        && !packet_separation.independent_trusted_operator_packet_accepted
        && packet_separation.operator_packet_identity_required
        && packet_separation.operator_packet_intent_required
        && packet_separation.operator_packet_signature_required
        && packet_separation.operator_packet_session_required
        && packet_separation.operator_packet_freshness_required
        && packet_separation.operator_packet_scope_required
        && !packet_separation.operator_identity_verified_from_packet
        && !packet_separation.operator_intent_confirmed_from_packet
        && !packet_separation.operator_approval_from_packet_accepted
        && !packet_separation.activation_authority_from_packet_derived
        && !packet_separation.activation_request_from_packet_allowed
        && !packet_separation.activation_command_from_packet_exposed
        && !packet_separation.live_mutation_from_packet_allowed
        && !packet_separation.public_claim_from_packet_allowed
        && !packet_separation.report_route_invokes_shadow_execution
        && !packet_separation.report_route_exposes_activation_command
        && packet_separation.live_mutation_enabled_count == 0
        && packet_separation.current_live_enabled_lane_count == 0
        && !packet_separation.side_effects.provider_invoked
        && !packet_separation.side_effects.model_invoked
        && !packet_separation.side_effects.auth_secret_read
        && !packet_separation.side_effects.credential_read
        && !packet_separation.side_effects.live_kg_write_performed
        && !packet_separation.side_effects.memory_store_mutated
        && !packet_separation.side_effects.channel_send_performed
        && !packet_separation.side_effects.service_restarted
        && !packet_separation.side_effects.active_binary_mutated
        && !packet_separation.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketIntakePreconditionResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command: "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-intake-precondition --json",
        native_route: true,
        compatibility_mode:
            "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_intake_precondition_route_source_only",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_ENDPOINT,
        packet_separation_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_ENDPOINT,
        packet_separation_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_GATE.md",
        intake_precondition_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_GATE.md",
        source_packet_separation_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-separation-gate.sh",
        source_intake_precondition_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-intake-precondition-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        packet_separation_route_ready: packet_separation.trusted_operator_packet_separation_ready,
        packet_separation_route_status: packet_separation.status,
        trusted_operator_packet_separation_ready: packet_separation.trusted_operator_packet_separation_ready,
        trusted_operator_packet_intake_precondition_ready: report_ready,
        independent_trusted_operator_packet_required: true,
        independent_trusted_operator_packet_present: false,
        independent_trusted_operator_packet_shape_declared: true,
        operator_packet_identity_required: true,
        operator_packet_intent_required: true,
        operator_packet_signature_required: true,
        operator_packet_session_required: true,
        operator_packet_freshness_required: true,
        operator_packet_scope_required: true,
        operator_packet_required_field_count: 6,
        operator_packet_verified_field_count: 0,
        operator_packet_missing_field_count: 6,
        operator_packet_identity_verified: false,
        operator_packet_intent_confirmed: false,
        operator_packet_signature_verified: false,
        operator_packet_session_bound: false,
        operator_packet_freshness_verified: false,
        operator_packet_scope_validated: false,
        operator_packet_acceptance_precondition_satisfied: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        operator_approval_from_packet_accepted: false,
        activation_authority_from_packet_derived: false,
        activation_request_from_packet_allowed: false,
        activation_command_from_packet_exposed: false,
        live_mutation_from_packet_allowed: false,
        public_claim_from_packet_allowed: false,
        report_route_invokes_shadow_execution: false,
        report_route_exposes_activation_command: false,
        live_mutation_enabled_count: packet_separation.live_mutation_enabled_count,
        current_live_enabled_lane_count: packet_separation.current_live_enabled_lane_count,
        operator_packet_intake_fixture_count: 6,
        blocked_operator_packet_intake_fixture_count: 6,
        allowed_operator_packet_intake_fixture_count: 0,
        blocked_operator_packet_intake_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketIntakePreconditionSideEffects {
                report_route_invoked_runtime_execution: false,
                source_gate_invokes_isolated_fixture_execution: true,
                live_7373_router_mutated_by_report_route: false,
                operator_packet_recorded: false,
                operator_packet_persisted: false,
                operator_packet_materialized: false,
                operator_packet_accepted: false,
                operator_packet_identity_verified: false,
                operator_packet_intent_confirmed: false,
                operator_packet_signature_verified: false,
                operator_packet_session_bound: false,
                operator_packet_freshness_verified: false,
                operator_packet_scope_validated: false,
                operator_approval_recorded: false,
                activation_authority_recorded: false,
                activation_request_enqueued: false,
                activation_command_exposed: false,
                public_claim_recorded: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                live_kg_write_performed: false,
                memory_store_mutated: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
            },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_report()
-> HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketPartialPreconditionDenialMatrixResponse{
    let route_matrix = control_ui_route_parity_report();
    let intake_precondition =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_intake_precondition_report();
    let fixtures = HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_FIXTURES;
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let blocked_fixture_count = fixtures
        .iter()
        .filter(|fixture| {
            fixture.verified_field_count < 6
                && fixture.missing_field_count > 0
                && !fixture.packet_recorded
                && !fixture.packet_persisted
                && !fixture.packet_accepted
                && !fixture.operator_approval_recorded
                && !fixture.activation_authority_derived
                && !fixture.activation_request_allowed
                && !fixture.activation_command_exposed
                && !fixture.live_mutation_allowed
                && !fixture.public_claim_allowed
        })
        .count();
    let allowed_fixture_count = fixtures.len() - blocked_fixture_count;
    let partial_packet_max_verified_field_count = fixtures
        .iter()
        .map(|fixture| fixture.verified_field_count)
        .max()
        .unwrap_or(0);
    let partial_packet_min_missing_field_count = fixtures
        .iter()
        .map(|fixture| fixture.missing_field_count)
        .min()
        .unwrap_or(0);
    let partial_packet_recorded_count = fixtures
        .iter()
        .filter(|fixture| fixture.packet_recorded)
        .count();
    let partial_packet_persisted_count = fixtures
        .iter()
        .filter(|fixture| fixture.packet_persisted)
        .count();
    let partial_packet_accepted_count = fixtures
        .iter()
        .filter(|fixture| fixture.packet_accepted)
        .count();
    let partial_packet_activation_authority_count = fixtures
        .iter()
        .filter(|fixture| fixture.activation_authority_derived)
        .count();
    let partial_packet_activation_command_exposed_count = fixtures
        .iter()
        .filter(|fixture| fixture.activation_command_exposed)
        .count();
    let partial_packet_live_mutation_allowed_count = fixtures
        .iter()
        .filter(|fixture| fixture.live_mutation_allowed)
        .count();
    let partial_packet_public_claim_allowed_count = fixtures
        .iter()
        .filter(|fixture| fixture.public_claim_allowed)
        .count();
    let partial_packet_acceptance_precondition_satisfied_count = fixtures
        .iter()
        .filter(|fixture| fixture.verified_field_count == 6 && fixture.missing_field_count == 0)
        .count();
    let missing_identity_fixture_blocked = fixtures
        .iter()
        .any(|fixture| fixture.missing_precondition == "identity" && !fixture.packet_accepted);
    let missing_intent_fixture_blocked = fixtures
        .iter()
        .any(|fixture| fixture.missing_precondition == "intent" && !fixture.packet_accepted);
    let missing_signature_fixture_blocked = fixtures
        .iter()
        .any(|fixture| fixture.missing_precondition == "signature" && !fixture.packet_accepted);
    let missing_session_fixture_blocked = fixtures
        .iter()
        .any(|fixture| fixture.missing_precondition == "session" && !fixture.packet_accepted);
    let missing_freshness_fixture_blocked = fixtures
        .iter()
        .any(|fixture| fixture.missing_precondition == "freshness" && !fixture.packet_accepted);
    let missing_scope_fixture_blocked = fixtures
        .iter()
        .any(|fixture| fixture.missing_precondition == "scope" && !fixture.packet_accepted);
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && intake_precondition.status == "ready"
        && intake_precondition.trusted_operator_packet_intake_precondition_ready
        && intake_precondition.independent_trusted_operator_packet_required
        && intake_precondition.independent_trusted_operator_packet_shape_declared
        && intake_precondition.operator_packet_required_field_count == 6
        && intake_precondition.operator_packet_verified_field_count == 0
        && intake_precondition.operator_packet_missing_field_count == 6
        && !intake_precondition.operator_packet_acceptance_precondition_satisfied
        && !intake_precondition.operator_packet_recorded
        && !intake_precondition.operator_packet_persisted
        && !intake_precondition.operator_packet_accepted
        && !intake_precondition.operator_approval_from_packet_accepted
        && !intake_precondition.activation_authority_from_packet_derived
        && !intake_precondition.activation_request_from_packet_allowed
        && !intake_precondition.activation_command_from_packet_exposed
        && !intake_precondition.live_mutation_from_packet_allowed
        && !intake_precondition.public_claim_from_packet_allowed
        && !intake_precondition.report_route_invokes_shadow_execution
        && !intake_precondition.report_route_exposes_activation_command
        && intake_precondition.live_mutation_enabled_count == 0
        && intake_precondition.current_live_enabled_lane_count == 0
        && fixtures.len() == 6
        && blocked_fixture_count == 6
        && allowed_fixture_count == 0
        && partial_packet_max_verified_field_count == 5
        && partial_packet_min_missing_field_count == 1
        && partial_packet_acceptance_precondition_satisfied_count == 0
        && partial_packet_recorded_count == 0
        && partial_packet_persisted_count == 0
        && partial_packet_accepted_count == 0
        && partial_packet_activation_authority_count == 0
        && partial_packet_activation_command_exposed_count == 0
        && partial_packet_live_mutation_allowed_count == 0
        && partial_packet_public_claim_allowed_count == 0
        && missing_identity_fixture_blocked
        && missing_intent_fixture_blocked
        && missing_signature_fixture_blocked
        && missing_session_fixture_blocked
        && missing_freshness_fixture_blocked
        && missing_scope_fixture_blocked
        && !intake_precondition.side_effects.provider_invoked
        && !intake_precondition.side_effects.model_invoked
        && !intake_precondition.side_effects.auth_secret_read
        && !intake_precondition.side_effects.credential_read
        && !intake_precondition.side_effects.live_kg_write_performed
        && !intake_precondition.side_effects.memory_store_mutated
        && !intake_precondition.side_effects.channel_send_performed
        && !intake_precondition.side_effects.service_restarted
        && !intake_precondition.side_effects.active_binary_mutated
        && !intake_precondition.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketPartialPreconditionDenialMatrixResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command: "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix --json",
        native_route: true,
        compatibility_mode:
            "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_route_source_only",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_ENDPOINT,
        intake_precondition_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_ENDPOINT,
        intake_precondition_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_GATE.md",
        partial_precondition_denial_matrix_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_GATE.md",
        source_intake_precondition_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-intake-precondition-gate.sh",
        source_partial_precondition_denial_matrix_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        intake_precondition_route_ready:
            intake_precondition.trusted_operator_packet_intake_precondition_ready,
        intake_precondition_route_status: intake_precondition.status,
        trusted_operator_packet_intake_precondition_ready:
            intake_precondition.trusted_operator_packet_intake_precondition_ready,
        trusted_operator_packet_partial_precondition_denial_matrix_ready: report_ready,
        independent_trusted_operator_packet_required: true,
        independent_trusted_operator_packet_shape_declared: true,
        operator_packet_required_field_count: 6,
        operator_packet_complete_verified_field_count_required: 6,
        operator_packet_partial_fixture_count: fixtures.len(),
        blocked_operator_packet_partial_fixture_count: blocked_fixture_count,
        allowed_operator_packet_partial_fixture_count: allowed_fixture_count,
        partial_packet_max_verified_field_count,
        partial_packet_min_missing_field_count,
        partial_packet_acceptance_precondition_satisfied_count,
        partial_packet_recorded_count,
        partial_packet_persisted_count,
        partial_packet_accepted_count,
        partial_packet_activation_authority_count,
        partial_packet_activation_command_exposed_count,
        partial_packet_live_mutation_allowed_count,
        partial_packet_public_claim_allowed_count,
        missing_identity_fixture_blocked,
        missing_intent_fixture_blocked,
        missing_signature_fixture_blocked,
        missing_session_fixture_blocked,
        missing_freshness_fixture_blocked,
        missing_scope_fixture_blocked,
        report_route_invokes_shadow_execution: false,
        report_route_exposes_activation_command: false,
        live_mutation_enabled_count: intake_precondition.live_mutation_enabled_count,
        current_live_enabled_lane_count: intake_precondition.current_live_enabled_lane_count,
        partial_precondition_denial_fixtures: fixtures,
        blocked_operator_packet_partial_precondition_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketPartialPreconditionDenialMatrixSideEffects {
                report_route_invoked_runtime_execution: false,
                source_gate_invokes_isolated_fixture_execution: true,
                live_7373_router_mutated_by_report_route: false,
                partial_operator_packet_recorded: false,
                partial_operator_packet_persisted: false,
                partial_operator_packet_materialized: false,
                partial_operator_packet_accepted: false,
                partial_operator_packet_identity_verified: false,
                partial_operator_packet_intent_confirmed: false,
                partial_operator_packet_signature_verified: false,
                partial_operator_packet_session_bound: false,
                partial_operator_packet_freshness_verified: false,
                partial_operator_packet_scope_validated: false,
                partial_operator_packet_activation_authority_recorded: false,
                partial_operator_packet_activation_request_enqueued: false,
                partial_operator_packet_activation_command_exposed: false,
                partial_operator_packet_live_mutation_performed: false,
                partial_operator_packet_public_claim_recorded: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                live_kg_write_performed: false,
                memory_store_mutated: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
            },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_report()
-> HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketCompletePreconditionAuthorityDenialResponse{
    let route_matrix = control_ui_route_parity_report();
    let partial_matrix =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_report();
    let fixtures = HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_FIXTURES;
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let complete_precondition_authority_denied_fixture_count = fixtures
        .iter()
        .filter(|fixture| {
            fixture.verified_field_count == 6
                && fixture.missing_field_count == 0
                && fixture.identity_verified
                && fixture.intent_confirmed
                && fixture.signature_verified
                && fixture.session_bound
                && fixture.freshness_verified
                && fixture.scope_validated
                && fixture.acceptance_precondition_satisfied
                && !fixture.packet_recorded
                && !fixture.packet_persisted
                && !fixture.packet_accepted
                && !fixture.operator_approval_recorded
                && !fixture.activation_authority_derived
                && !fixture.activation_request_allowed
                && !fixture.activation_command_exposed
                && !fixture.live_mutation_allowed
                && !fixture.public_claim_allowed
        })
        .count();
    let complete_precondition_authority_allowed_fixture_count =
        fixtures.len() - complete_precondition_authority_denied_fixture_count;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && partial_matrix.status == "ready"
        && partial_matrix.trusted_operator_packet_partial_precondition_denial_matrix_ready
        && partial_matrix.independent_trusted_operator_packet_required
        && partial_matrix.independent_trusted_operator_packet_shape_declared
        && partial_matrix.operator_packet_required_field_count == 6
        && partial_matrix.operator_packet_complete_verified_field_count_required == 6
        && partial_matrix.operator_packet_partial_fixture_count == 6
        && partial_matrix.blocked_operator_packet_partial_fixture_count == 6
        && partial_matrix.allowed_operator_packet_partial_fixture_count == 0
        && partial_matrix.partial_packet_max_verified_field_count == 5
        && partial_matrix.partial_packet_min_missing_field_count == 1
        && partial_matrix.partial_packet_acceptance_precondition_satisfied_count == 0
        && partial_matrix.partial_packet_recorded_count == 0
        && partial_matrix.partial_packet_persisted_count == 0
        && partial_matrix.partial_packet_accepted_count == 0
        && partial_matrix.partial_packet_activation_authority_count == 0
        && partial_matrix.partial_packet_activation_command_exposed_count == 0
        && partial_matrix.partial_packet_live_mutation_allowed_count == 0
        && partial_matrix.partial_packet_public_claim_allowed_count == 0
        && !partial_matrix.report_route_invokes_shadow_execution
        && !partial_matrix.report_route_exposes_activation_command
        && partial_matrix.live_mutation_enabled_count == 0
        && partial_matrix.current_live_enabled_lane_count == 0
        && fixtures.len() == 1
        && complete_precondition_authority_denied_fixture_count == 1
        && complete_precondition_authority_allowed_fixture_count == 0
        && !partial_matrix.side_effects.provider_invoked
        && !partial_matrix.side_effects.model_invoked
        && !partial_matrix.side_effects.auth_secret_read
        && !partial_matrix.side_effects.credential_read
        && !partial_matrix.side_effects.live_kg_write_performed
        && !partial_matrix.side_effects.memory_store_mutated
        && !partial_matrix.side_effects.channel_send_performed
        && !partial_matrix.side_effects.service_restarted
        && !partial_matrix.side_effects.active_binary_mutated
        && !partial_matrix.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketCompletePreconditionAuthorityDenialResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command: "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial --json",
        native_route: true,
        compatibility_mode:
            "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_route_source_only",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_ENDPOINT,
        partial_precondition_denial_matrix_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_ENDPOINT,
        partial_precondition_denial_matrix_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_GATE.md",
        complete_precondition_authority_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_GATE.md",
        source_partial_precondition_denial_matrix_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix-gate.sh",
        source_complete_precondition_authority_denial_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        partial_precondition_denial_matrix_route_ready:
            partial_matrix.trusted_operator_packet_partial_precondition_denial_matrix_ready,
        partial_precondition_denial_matrix_route_status: partial_matrix.status,
        trusted_operator_packet_partial_precondition_denial_matrix_ready:
            partial_matrix.trusted_operator_packet_partial_precondition_denial_matrix_ready,
        trusted_operator_packet_complete_precondition_authority_denial_ready: report_ready,
        independent_trusted_operator_packet_required: true,
        independent_trusted_operator_packet_shape_declared: true,
        operator_packet_required_field_count: 6,
        operator_packet_verified_field_count: 6,
        operator_packet_missing_field_count: 0,
        operator_packet_identity_verified: true,
        operator_packet_intent_confirmed: true,
        operator_packet_signature_verified: true,
        operator_packet_session_bound: true,
        operator_packet_freshness_verified: true,
        operator_packet_scope_validated: true,
        operator_packet_acceptance_precondition_satisfied: true,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        operator_approval_from_packet_accepted: false,
        activation_authority_from_packet_derived: false,
        activation_request_from_packet_allowed: false,
        activation_command_from_packet_exposed: false,
        live_mutation_from_packet_allowed: false,
        public_claim_from_packet_allowed: false,
        report_route_invokes_shadow_execution: false,
        report_route_exposes_activation_command: false,
        live_mutation_enabled_count: partial_matrix.live_mutation_enabled_count,
        current_live_enabled_lane_count: partial_matrix.current_live_enabled_lane_count,
        complete_precondition_fixture_count: fixtures.len(),
        complete_precondition_authority_denied_fixture_count,
        complete_precondition_authority_allowed_fixture_count,
        complete_precondition_authority_denial_fixtures: fixtures,
        blocked_operator_packet_complete_precondition_authority_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketCompletePreconditionAuthorityDenialSideEffects {
                report_route_invoked_runtime_execution: false,
                source_gate_invokes_isolated_fixture_execution: true,
                live_7373_router_mutated_by_report_route: false,
                complete_operator_packet_recorded: false,
                complete_operator_packet_persisted: false,
                complete_operator_packet_materialized: false,
                complete_operator_packet_accepted: false,
                complete_operator_packet_operator_approval_recorded: false,
                complete_operator_packet_activation_authority_recorded: false,
                complete_operator_packet_activation_request_enqueued: false,
                complete_operator_packet_activation_command_exposed: false,
                complete_operator_packet_live_mutation_performed: false,
                complete_operator_packet_public_claim_recorded: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                live_kg_write_performed: false,
                memory_store_mutated: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
            },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_report()
-> HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketCompletePreconditionOperatorApprovalLaneSeparationResponse{
    let route_matrix = control_ui_route_parity_report();
    let complete_precondition =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && complete_precondition.status == "ready"
        && complete_precondition
            .trusted_operator_packet_complete_precondition_authority_denial_ready
        && complete_precondition.operator_packet_required_field_count == 6
        && complete_precondition.operator_packet_verified_field_count == 6
        && complete_precondition.operator_packet_missing_field_count == 0
        && complete_precondition.operator_packet_acceptance_precondition_satisfied
        && !complete_precondition.operator_packet_recorded
        && !complete_precondition.operator_packet_persisted
        && !complete_precondition.operator_packet_accepted
        && !complete_precondition.operator_approval_from_packet_accepted
        && !complete_precondition.activation_authority_from_packet_derived
        && !complete_precondition.activation_request_from_packet_allowed
        && !complete_precondition.activation_command_from_packet_exposed
        && !complete_precondition.live_mutation_from_packet_allowed
        && !complete_precondition.public_claim_from_packet_allowed
        && !complete_precondition.report_route_invokes_shadow_execution
        && !complete_precondition.report_route_exposes_activation_command
        && complete_precondition.live_mutation_enabled_count == 0
        && complete_precondition.current_live_enabled_lane_count == 0
        && !complete_precondition.side_effects.provider_invoked
        && !complete_precondition.side_effects.model_invoked
        && !complete_precondition.side_effects.auth_secret_read
        && !complete_precondition.side_effects.credential_read
        && !complete_precondition.side_effects.live_kg_write_performed
        && !complete_precondition.side_effects.memory_store_mutated
        && !complete_precondition.side_effects.channel_send_performed
        && !complete_precondition.side_effects.service_restarted
        && !complete_precondition.side_effects.active_binary_mutated
        && !complete_precondition.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketCompletePreconditionOperatorApprovalLaneSeparationResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command: "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation --json",
        native_route: true,
        compatibility_mode:
            "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_route_source_only",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_ENDPOINT,
        complete_precondition_authority_denial_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_ENDPOINT,
        complete_precondition_authority_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_GATE.md",
        operator_approval_lane_separation_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_GATE.md",
        source_complete_precondition_authority_denial_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial-gate.sh",
        source_operator_approval_lane_separation_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        complete_precondition_authority_denial_route_ready:
            complete_precondition.trusted_operator_packet_complete_precondition_authority_denial_ready,
        complete_precondition_authority_denial_route_status: complete_precondition.status,
        trusted_operator_packet_complete_precondition_authority_denial_ready:
            complete_precondition.trusted_operator_packet_complete_precondition_authority_denial_ready,
        trusted_operator_packet_complete_precondition_operator_approval_lane_separation_ready:
            report_ready,
        operator_packet_required_field_count: complete_precondition.operator_packet_required_field_count,
        operator_packet_verified_field_count: complete_precondition.operator_packet_verified_field_count,
        operator_packet_missing_field_count: complete_precondition.operator_packet_missing_field_count,
        operator_packet_acceptance_precondition_satisfied:
            complete_precondition.operator_packet_acceptance_precondition_satisfied,
        operator_packet_accepted: false,
        operator_approval_from_packet_accepted: false,
        complete_precondition_can_substitute_operator_approval: false,
        complete_precondition_can_create_activation_lane: false,
        operator_approved_activation_lane_required: true,
        operator_approved_activation_lane_present: false,
        activation_lane_acceptance_allowed: false,
        activation_lane_recorded: false,
        activation_lane_persisted: false,
        activation_lane_enqueued: false,
        activation_lane_effective: false,
        activation_authority_from_packet_derived: false,
        activation_request_from_packet_allowed: false,
        activation_command_from_packet_exposed: false,
        live_mutation_from_packet_allowed: false,
        public_claim_from_packet_allowed: false,
        report_route_invokes_shadow_execution: false,
        report_route_exposes_activation_command: false,
        live_mutation_enabled_count: complete_precondition.live_mutation_enabled_count,
        current_live_enabled_lane_count: complete_precondition.current_live_enabled_lane_count,
        operator_approval_receipt_required: true,
        rollback_kill_switch_required: true,
        post_activation_watchdog_soak_plan_required: true,
        blocked_operator_approval_lane_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_NEXT_ACTIONS,
        side_effects:
            HeptaMemoryIntelligenceKgFullEnablementRuntimeProviderRouterShadowExecutionControlledReadbackReceiptTrustedOperatorPacketCompletePreconditionOperatorApprovalLaneSeparationSideEffects {
                report_route_invoked_runtime_execution: false,
                source_gate_invokes_isolated_fixture_execution: true,
                live_7373_router_mutated_by_report_route: false,
                complete_operator_packet_recorded: false,
                complete_operator_packet_persisted: false,
                complete_operator_packet_accepted: false,
                complete_operator_packet_operator_approval_recorded: false,
                operator_approval_lane_recorded: false,
                operator_approval_lane_persisted: false,
                operator_approval_lane_materialized: false,
                operator_approval_lane_enqueued: false,
                operator_approval_lane_effective: false,
                activation_authority_recorded: false,
                activation_request_enqueued: false,
                activation_command_exposed: false,
                live_mutation_performed: false,
                public_claim_recorded: false,
                provider_invoked: false,
                model_invoked: false,
                auth_secret_read: false,
                credential_read: false,
                external_network_call_performed: false,
                live_kg_write_performed: false,
                memory_store_mutated: false,
                channel_send_performed: false,
                external_send_performed: false,
                gateway_route_migration_performed: false,
                source_command_migration_performed: false,
                service_restarted: false,
                active_binary_mutated: false,
                release_artifact_written: false,
                public_release_claimed: false,
            },
    }
}

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_memory_live_mutation_durable_lane_report()
-> HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedMemoryLiveMutationDurableLaneResponse {
    let route_matrix = control_ui_route_parity_report();
    let operator_lane =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && operator_lane.status == "ready"
        && operator_lane
            .trusted_operator_packet_complete_precondition_operator_approval_lane_separation_ready
        && operator_lane.operator_approved_activation_lane_required
        && !operator_lane.operator_approved_activation_lane_present
        && !operator_lane.activation_lane_acceptance_allowed
        && !operator_lane.activation_command_from_packet_exposed
        && !operator_lane.live_mutation_from_packet_allowed
        && operator_lane.live_mutation_enabled_count == 0
        && operator_lane.current_live_enabled_lane_count == 0
        && !operator_lane.side_effects.provider_invoked
        && !operator_lane.side_effects.model_invoked
        && !operator_lane.side_effects.auth_secret_read
        && !operator_lane.side_effects.credential_read
        && !operator_lane.side_effects.live_kg_write_performed
        && !operator_lane.side_effects.memory_store_mutated
        && !operator_lane.side_effects.channel_send_performed
        && !operator_lane.side_effects.service_restarted
        && !operator_lane.side_effects.active_binary_mutated
        && !operator_lane.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedMemoryLiveMutationDurableLaneResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command:
            "/hepta-memory-intelligence-kg-full-enablement-operator-approved-memory-live-mutation-durable-lane --json",
        native_route: true,
        compatibility_mode:
            "native_full_enablement_operator_approved_memory_live_mutation_durable_lane_status",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_ENDPOINT,
        operator_approval_lane_separation_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_ENDPOINT,
        operator_approval_lane_separation_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_APPROVED_SHADOW_CONTEXT_ACTIVATION_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_GATE.md",
        memory_live_mutation_durable_lane_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_GATE.md",
        source_operator_approval_lane_separation_gate: "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation-gate.sh",
        source_memory_live_mutation_durable_lane_gate:
            "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-memory-live-mutation-durable-lane-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        operator_approval_lane_separation_route_ready: operator_lane
            .trusted_operator_packet_complete_precondition_operator_approval_lane_separation_ready,
        operator_approval_lane_separation_route_status: operator_lane.status,
        trusted_operator_packet_complete_precondition_operator_approval_lane_separation_ready:
            operator_lane
                .trusted_operator_packet_complete_precondition_operator_approval_lane_separation_ready,
        operator_authorization_source: "telegram_direct_operator_authorization_2026_06_12_13_40_37_asia_shanghai",
        operator_authorization_scope:
            "memory_durable_mutation_lane_only_no_kg_provider_model_channel_or_public_release",
        operator_authorization_received: true,
        operator_approved_activation_lane_present: true,
        operator_approved_activation_lane_effective: true,
        operator_approval_receipt_required_for_write_execution: true,
        operator_approval_receipt_recorded_by_report_route: false,
        operator_approval_receipt_persisted_by_report_route: false,
        rollback_kill_switch_required: true,
        rollback_kill_switch_present: true,
        post_write_validation_required: true,
        post_write_validation_present: true,
        idempotency_required: true,
        idempotency_key_required_for_write_execution: true,
        memory_durable_mutation_lane_enabled: true,
        memory_store_write_path_enabled: true,
        memory_store_mutation_enabled: true,
        live_memory_write_allowed_by_lane: true,
        live_memory_write_performed_by_report_route: false,
        memory_write_execution_requires_explicit_command: true,
        memory_write_execution_command_exposed_by_report_route: false,
        memory_write_receipt_required: true,
        memory_write_receipt_recorded_by_report_route: false,
        kg_prompt_preview_lane_enabled: false,
        kg_external_adapter_read_lane_enabled: false,
        kg_live_write_lane_enabled: false,
        hepta_intelligence_context_attachment_lane_enabled: false,
        provider_model_invocation_lane_enabled: false,
        channel_delivery_lane_enabled: false,
        live_mutation_enabled_count: 1,
        current_live_enabled_lane_count: 1,
        enablement_lane_count: 6,
        ready_enablement_lane_count: 6,
        blocked_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_NEXT_ACTIONS,
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

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_report()
-> HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedHeptaIntelligenceContextAttachmentLaneResponse{
    let route_matrix = control_ui_route_parity_report();
    let memory_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_memory_live_mutation_durable_lane_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && memory_lane.status == "ready"
        && memory_lane.operator_approved_activation_lane_present
        && memory_lane.operator_approved_activation_lane_effective
        && memory_lane.memory_durable_mutation_lane_enabled
        && memory_lane.memory_store_write_path_enabled
        && memory_lane.memory_store_mutation_enabled
        && memory_lane.live_memory_write_allowed_by_lane
        && !memory_lane.live_memory_write_performed_by_report_route
        && !memory_lane.hepta_intelligence_context_attachment_lane_enabled
        && !memory_lane.kg_live_write_lane_enabled
        && !memory_lane.provider_model_invocation_lane_enabled
        && !memory_lane.channel_delivery_lane_enabled
        && memory_lane.live_mutation_enabled_count == 1
        && memory_lane.current_live_enabled_lane_count == 1
        && !memory_lane.side_effects.memory_store_mutated
        && !memory_lane.side_effects.memory_store_write_performed
        && !memory_lane.side_effects.hepta_intelligence_context_attached
        && !memory_lane.side_effects.prompt_preview_rendered
        && !memory_lane.side_effects.prompt_payload_materialized
        && !memory_lane.side_effects.context_injection_performed
        && !memory_lane.side_effects.provider_invoked
        && !memory_lane.side_effects.model_invoked
        && !memory_lane.side_effects.auth_secret_read
        && !memory_lane.side_effects.credential_read
        && !memory_lane.side_effects.external_kg_adapter_read_performed
        && !memory_lane.side_effects.live_kg_write_performed
        && !memory_lane.side_effects.channel_send_performed
        && !memory_lane.side_effects.service_restarted
        && !memory_lane.side_effects.active_binary_mutated
        && !memory_lane.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedHeptaIntelligenceContextAttachmentLaneResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command:
            "/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane --json",
        native_route: true,
        compatibility_mode:
            "native_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_status",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_ENDPOINT,
        memory_live_mutation_durable_lane_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_ENDPOINT,
        memory_live_mutation_durable_lane_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_GATE.md",
        intelligence_context_attachment_lane_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_GATE.md",
        source_memory_live_mutation_durable_lane_gate:
            "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-memory-live-mutation-durable-lane-gate.sh",
        source_intelligence_context_attachment_lane_gate:
            "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        memory_live_mutation_durable_lane_ready:
            memory_lane.memory_durable_mutation_lane_enabled,
        memory_live_mutation_durable_lane_status: memory_lane.status,
        operator_authorization_source: "telegram_direct_operator_authorization_2026_06_12_13_40_37_asia_shanghai",
        operator_authorization_scope:
            "hepta_intelligence_context_attachment_and_bounded_prompt_preview_lane_no_provider_model_kg_write_channel_or_public_release",
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
        prompt_payload_materialized_by_report_route: false,
        context_handoff_acceptance_required: true,
        context_attachment_requires_explicit_command: true,
        prompt_preview_requires_explicit_command: true,
        context_injection_allowed_by_lane: false,
        context_injection_performed_by_report_route: false,
        kg_prompt_preview_lane_enabled: false,
        kg_external_adapter_read_lane_enabled: false,
        kg_live_write_lane_enabled: false,
        provider_model_invocation_lane_enabled: false,
        channel_delivery_lane_enabled: false,
        live_mutation_enabled_count: 1,
        current_live_enabled_lane_count: 2,
        enablement_lane_count: 6,
        ready_enablement_lane_count: 6,
        blocked_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_NEXT_ACTIONS,
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

fn hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_preview_read_only_adapter_lane_report()
-> HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedKgPromptPreviewReadOnlyAdapterLaneResponse
{
    let route_matrix = control_ui_route_parity_report();
    let context_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && context_lane.status == "ready"
        && context_lane.operator_approved_activation_lane_present
        && context_lane.operator_approved_activation_lane_effective
        && context_lane.memory_durable_mutation_lane_enabled
        && context_lane.memory_store_write_path_enabled
        && context_lane.memory_store_mutation_enabled
        && context_lane.live_memory_write_allowed_by_lane
        && !context_lane.live_memory_write_performed_by_report_route
        && context_lane.hepta_intelligence_context_attachment_lane_enabled
        && context_lane.hepta_intelligence_context_attachment_allowed_by_lane
        && !context_lane.hepta_intelligence_context_attached_by_report_route
        && context_lane.bounded_prompt_preview_lane_enabled
        && context_lane.bounded_prompt_preview_allowed_by_lane
        && !context_lane.prompt_preview_rendered_by_report_route
        && !context_lane.prompt_payload_materialized_by_report_route
        && !context_lane.context_injection_allowed_by_lane
        && !context_lane.context_injection_performed_by_report_route
        && !context_lane.kg_prompt_preview_lane_enabled
        && !context_lane.kg_external_adapter_read_lane_enabled
        && !context_lane.kg_live_write_lane_enabled
        && !context_lane.provider_model_invocation_lane_enabled
        && !context_lane.channel_delivery_lane_enabled
        && context_lane.live_mutation_enabled_count == 1
        && context_lane.current_live_enabled_lane_count == 2
        && !context_lane.side_effects.memory_store_mutated
        && !context_lane.side_effects.memory_store_write_performed
        && !context_lane
            .side_effects
            .hepta_intelligence_context_attached
        && !context_lane.side_effects.prompt_preview_rendered
        && !context_lane.side_effects.prompt_payload_materialized
        && !context_lane.side_effects.context_injection_performed
        && !context_lane.side_effects.provider_invoked
        && !context_lane.side_effects.model_invoked
        && !context_lane.side_effects.auth_secret_read
        && !context_lane.side_effects.credential_read
        && !context_lane.side_effects.external_kg_adapter_read_performed
        && !context_lane.side_effects.live_kg_write_performed
        && !context_lane.side_effects.channel_send_performed
        && !context_lane.side_effects.service_restarted
        && !context_lane.side_effects.active_binary_mutated
        && !context_lane.side_effects.public_release_claimed;

    HeptaMemoryIntelligenceKgFullEnablementOperatorApprovedKgPromptPreviewReadOnlyAdapterLaneResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if report_ready { "ready" } else { "blocked" },
        source_command:
            "/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-preview-read-only-adapter-lane --json",
        native_route: true,
        compatibility_mode:
            "native_full_enablement_operator_approved_kg_prompt_preview_read_only_adapter_lane_status",
        side_effect_free: true,
        audit_date: "2026-06-12",
        endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_ENDPOINT,
        hepta_intelligence_context_attachment_lane_endpoint:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_ENDPOINT,
        hepta_intelligence_context_attachment_lane_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_GATE.md",
        kg_prompt_preview_read_only_adapter_lane_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_GATE.md",
        source_hepta_intelligence_context_attachment_lane_gate:
            "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane-gate.sh",
        source_kg_prompt_preview_read_only_adapter_lane_gate:
            "scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-preview-read-only-adapter-lane-gate.sh",
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        route_count_cutover_floor: NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        route_count_floor_preserved,
        route_count_source_command_accepted,
        source_route_wired: true,
        hepta_intelligence_context_attachment_lane_ready:
            context_lane.hepta_intelligence_context_attachment_lane_enabled,
        hepta_intelligence_context_attachment_lane_status: context_lane.status,
        operator_authorization_source: "telegram_direct_operator_authorization_2026_06_12_18_50_49_asia_shanghai",
        operator_authorization_scope:
            "kg_prompt_preview_read_only_adapter_lane_no_kg_live_write_provider_model_channel_or_public_release",
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
        prompt_payload_materialized_by_report_route: false,
        prompt_preview_requires_explicit_command: true,
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
        current_live_enabled_lane_count: 3,
        enablement_lane_count: 6,
        ready_enablement_lane_count: 6,
        blocked_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_BLOCKED_ACTIONS,
        allowed_next_actions:
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_NEXT_ACTIONS,
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

fn hepta_memory_intelligence_kg_activation_truth_index_report() -> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let memory_inventory = hepta_memory_capability_absorption_inventory_report();
    let runtime_readiness = hepta_memory_intelligence_kg_full_enablement_runtime_readiness_report();
    let memory_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_memory_live_mutation_durable_lane_report();
    let intelligence_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_report();
    let kg_preview_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_preview_read_only_adapter_lane_report();
    let readiness_index =
        hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_report();

    let source_full_live_activation_enabled = readiness_index
        .get("source_full_live_activation_enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let source_full_live_activation_status = readiness_index
        .get("source_full_live_activation_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let readiness_replay_allowed = readiness_index
        .get("replay_allowed")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let readiness_replay_accepted = readiness_index
        .get("replay_accepted")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let readiness_side_effects_all_false = readiness_index
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| effects.values().all(|value| value.as_bool() == Some(false)))
        .unwrap_or(false);
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let memory_core_connected = runtime_readiness.status == "ready"
        && runtime_readiness.core_full_fusion_complete
        && runtime_readiness.active_service_stack_consumes_memory_intelligence
        && runtime_readiness.memory_surface_count == 14
        && runtime_readiness.absorbed_or_represented_count == 14
        && runtime_readiness.gap_report_ready_count == 14
        && runtime_readiness.live_mutation_enabled_count == 0
        && !runtime_readiness.memory_store_mutation_enabled;
    let memory_lane_ready = memory_lane.status == "ready"
        && memory_lane.operator_approved_activation_lane_effective
        && memory_lane.memory_durable_mutation_lane_enabled
        && memory_lane.memory_store_write_path_enabled
        && memory_lane.memory_store_mutation_enabled
        && memory_lane.live_memory_write_allowed_by_lane
        && memory_lane.memory_write_execution_requires_explicit_command
        && !memory_lane.live_memory_write_performed_by_report_route
        && !memory_lane.memory_write_execution_command_exposed_by_report_route
        && !memory_lane.side_effects.memory_store_mutated
        && !memory_lane.side_effects.memory_store_write_performed;
    let intelligence_lane_ready = intelligence_lane.status == "ready"
        && intelligence_lane.hepta_intelligence_context_attachment_lane_enabled
        && intelligence_lane.hepta_intelligence_context_attachment_allowed_by_lane
        && intelligence_lane.context_attachment_requires_explicit_command
        && intelligence_lane.bounded_prompt_preview_lane_enabled
        && intelligence_lane.bounded_prompt_preview_allowed_by_lane
        && intelligence_lane.prompt_preview_requires_explicit_command
        && !intelligence_lane.hepta_intelligence_context_attached_by_report_route
        && !intelligence_lane.prompt_preview_rendered_by_report_route
        && !intelligence_lane.prompt_payload_materialized_by_report_route
        && !intelligence_lane.context_injection_allowed_by_lane
        && !intelligence_lane.context_injection_performed_by_report_route
        && !intelligence_lane
            .side_effects
            .hepta_intelligence_context_attached
        && !intelligence_lane.side_effects.prompt_preview_rendered
        && !intelligence_lane.side_effects.context_injection_performed;
    let kg_lane_ready = kg_preview_lane.status == "ready"
        && kg_preview_lane.kg_prompt_preview_lane_enabled
        && kg_preview_lane.kg_prompt_preview_allowed_by_lane
        && kg_preview_lane.kg_external_adapter_read_lane_enabled
        && kg_preview_lane.kg_external_adapter_read_allowed_by_lane
        && kg_preview_lane.kg_external_adapter_requires_explicit_command
        && kg_preview_lane.kg_external_adapter_credential_reference_required
        && !kg_preview_lane.kg_prompt_preview_rendered_by_report_route
        && !kg_preview_lane.kg_external_adapter_read_performed_by_report_route
        && !kg_preview_lane.kg_external_adapter_credential_read_allowed_by_lane
        && !kg_preview_lane.kg_external_adapter_credential_read_performed_by_report_route
        && !kg_preview_lane.kg_live_write_lane_enabled
        && !kg_preview_lane.kg_live_write_allowed_by_lane
        && !kg_preview_lane.kg_live_write_performed_by_report_route
        && !kg_preview_lane.provider_model_invocation_lane_enabled
        && !kg_preview_lane.provider_model_invocation_allowed_by_lane
        && !kg_preview_lane.channel_delivery_lane_enabled
        && !kg_preview_lane
            .side_effects
            .external_kg_adapter_read_performed
        && !kg_preview_lane.side_effects.credential_read
        && !kg_preview_lane.side_effects.live_kg_write_performed
        && !kg_preview_lane.side_effects.provider_invoked
        && !kg_preview_lane.side_effects.model_invoked;
    let operator_approved_lanes_ready =
        memory_lane_ready && intelligence_lane_ready && kg_lane_ready;
    let full_live_activation_blocked = !source_full_live_activation_enabled
        && source_full_live_activation_status == "blocked_report_only"
        && !readiness_replay_allowed
        && !readiness_replay_accepted
        && readiness_side_effects_all_false;
    let report_only_boundaries_intact = !memory_lane.side_effects.provider_invoked
        && !memory_lane.side_effects.model_invoked
        && !memory_lane.side_effects.credential_read
        && !memory_lane.side_effects.live_kg_write_performed
        && !memory_lane.side_effects.channel_send_performed
        && !memory_lane.side_effects.external_send_performed
        && !memory_lane.side_effects.service_restarted
        && !memory_lane.side_effects.active_binary_mutated
        && !memory_lane.side_effects.public_release_claimed
        && !intelligence_lane.side_effects.provider_invoked
        && !intelligence_lane.side_effects.model_invoked
        && !intelligence_lane.side_effects.credential_read
        && !intelligence_lane.side_effects.live_kg_write_performed
        && !intelligence_lane.side_effects.channel_send_performed
        && !kg_preview_lane.side_effects.provider_invoked
        && !kg_preview_lane.side_effects.model_invoked
        && !kg_preview_lane.side_effects.credential_read
        && !kg_preview_lane.side_effects.live_kg_write_performed
        && !kg_preview_lane.side_effects.channel_send_performed
        && !kg_preview_lane.side_effects.external_send_performed
        && !kg_preview_lane.side_effects.service_restarted
        && !kg_preview_lane.side_effects.active_binary_mutated
        && !kg_preview_lane.side_effects.public_release_claimed;
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && memory_inventory.memory_capability_inventory_ready
        && memory_core_connected
        && operator_approved_lanes_ready
        && full_live_activation_blocked
        && report_only_boundaries_intact;
    let truth_index_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-activation-truth-index-v1:route_count={}:memory_core_connected={}:operator_lanes_ready={}:full_live_blocked={}:readiness={}",
        route_matrix.route_count,
        memory_core_connected,
        operator_approved_lanes_ready,
        full_live_activation_blocked,
        sha256_json_value(&readiness_index),
    ));

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "source_command": "/hepta-memory-intelligence-kg-activation-truth-index --json",
        "native_route": true,
        "compatibility_mode": "native_memory_intelligence_kg_activation_truth_index_read_only",
        "side_effect_free": true,
        "audit_date": "2026-06-25",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_ACTIVATION_TRUTH_INDEX_ENDPOINT,
        "truth_index_schema_version": "memory_intelligence_kg_activation_truth_index_v1",
        "truth_index_hash_sha256": truth_index_hash_sha256,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        "route_count_floor_preserved": route_count_floor_preserved,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "source_memory_capability_inventory_endpoint": HEPTA_MEMORY_CAPABILITY_ABSORPTION_INVENTORY_ENDPOINT,
        "source_runtime_readiness_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_READINESS_ENDPOINT,
        "source_memory_live_mutation_durable_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_ENDPOINT,
        "source_hepta_intelligence_context_attachment_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_ENDPOINT,
        "source_kg_prompt_preview_read_only_adapter_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_ENDPOINT,
        "source_full_live_activation_readiness_index_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_READINESS_INDEX_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "hepta_core_connected": memory_core_connected,
            "hepta_core_full_fusion_complete": runtime_readiness.core_full_fusion_complete,
            "active_binary_package": runtime_readiness.active_binary_package,
            "remaining_direct_codex_dependency_count": runtime_readiness.remaining_direct_codex_dependency_count,
            "memory_capability_inventory_ready": memory_inventory.memory_capability_inventory_ready,
            "memory_surface_count": memory_inventory.surface_count,
            "absorbed_or_represented_count": memory_inventory.absorbed_or_represented_count,
            "gap_report_ready_count": memory_inventory.gap_report_ready_count,
            "baseline_live_mutation_enabled_count": memory_inventory.live_mutation_enabled_count,
            "baseline_memory_store_mutation_enabled": memory_inventory.memory_store_mutation_enabled,
            "operator_approved_lanes_ready": operator_approved_lanes_ready,
            "operator_approved_lane_count": 3,
            "ready_operator_approved_lane_count": if operator_approved_lanes_ready { 3 } else { 0 },
            "explicit_command_required_for_execution": true,
            "report_only_boundaries_intact": report_only_boundaries_intact,
            "full_live_activation_enabled": false,
            "full_live_activation_status": source_full_live_activation_status,
            "full_live_activation_blocked": full_live_activation_blocked,
            "live_activation_blocker_count": readiness_index["live_activation_blocker_count"].clone(),
            "readiness_surface_count": readiness_index["readiness_surface_count"].clone(),
            "replay_allowed": readiness_replay_allowed,
            "replay_accepted": readiness_replay_accepted,
            "readiness_index_side_effects_all_false": readiness_side_effects_all_false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_lane": {
                "status": memory_lane.status,
                "core_connected": memory_core_connected,
                "operator_approved_lane_ready": memory_lane_ready,
                "memory_durable_mutation_lane_enabled": memory_lane.memory_durable_mutation_lane_enabled,
                "memory_store_write_path_enabled": memory_lane.memory_store_write_path_enabled,
                "memory_store_mutation_enabled": memory_lane.memory_store_mutation_enabled,
                "live_memory_write_allowed_by_lane": memory_lane.live_memory_write_allowed_by_lane,
                "execution_requires_explicit_command": memory_lane.memory_write_execution_requires_explicit_command,
                "receipt_required": memory_lane.memory_write_receipt_required,
                "post_write_validation_required": memory_lane.post_write_validation_required,
                "report_route_write_performed": memory_lane.live_memory_write_performed_by_report_route,
                "report_route_exposes_execution_command": memory_lane.memory_write_execution_command_exposed_by_report_route,
                "side_effect_memory_store_mutated": memory_lane.side_effects.memory_store_mutated,
                "side_effect_memory_store_write_performed": memory_lane.side_effects.memory_store_write_performed
            },
            "hepta_intelligence_lane": {
                "status": intelligence_lane.status,
                "operator_approved_lane_ready": intelligence_lane_ready,
                "context_attachment_lane_enabled": intelligence_lane.hepta_intelligence_context_attachment_lane_enabled,
                "context_attachment_allowed_by_lane": intelligence_lane.hepta_intelligence_context_attachment_allowed_by_lane,
                "bounded_prompt_preview_lane_enabled": intelligence_lane.bounded_prompt_preview_lane_enabled,
                "bounded_prompt_preview_allowed_by_lane": intelligence_lane.bounded_prompt_preview_allowed_by_lane,
                "context_attachment_requires_explicit_command": intelligence_lane.context_attachment_requires_explicit_command,
                "prompt_preview_requires_explicit_command": intelligence_lane.prompt_preview_requires_explicit_command,
                "context_injection_allowed_by_lane": intelligence_lane.context_injection_allowed_by_lane,
                "report_route_context_attached": intelligence_lane.hepta_intelligence_context_attached_by_report_route,
                "report_route_prompt_preview_rendered": intelligence_lane.prompt_preview_rendered_by_report_route,
                "report_route_prompt_payload_materialized": intelligence_lane.prompt_payload_materialized_by_report_route,
                "report_route_context_injection_performed": intelligence_lane.context_injection_performed_by_report_route
            },
            "kg_lane": {
                "status": kg_preview_lane.status,
                "operator_approved_lane_ready": kg_lane_ready,
                "kg_prompt_preview_lane_enabled": kg_preview_lane.kg_prompt_preview_lane_enabled,
                "kg_prompt_preview_allowed_by_lane": kg_preview_lane.kg_prompt_preview_allowed_by_lane,
                "kg_external_adapter_read_lane_enabled": kg_preview_lane.kg_external_adapter_read_lane_enabled,
                "kg_external_adapter_read_allowed_by_lane": kg_preview_lane.kg_external_adapter_read_allowed_by_lane,
                "kg_external_adapter_requires_explicit_command": kg_preview_lane.kg_external_adapter_requires_explicit_command,
                "kg_external_adapter_credential_reference_required": kg_preview_lane.kg_external_adapter_credential_reference_required,
                "kg_external_adapter_credential_read_allowed_by_lane": kg_preview_lane.kg_external_adapter_credential_read_allowed_by_lane,
                "supported_kg_adapter_count": kg_preview_lane.supported_kg_adapter_count,
                "supported_kg_adapters": kg_preview_lane.supported_kg_adapters,
                "kg_live_write_lane_enabled": kg_preview_lane.kg_live_write_lane_enabled,
                "kg_live_write_allowed_by_lane": kg_preview_lane.kg_live_write_allowed_by_lane,
                "report_route_kg_prompt_preview_rendered": kg_preview_lane.kg_prompt_preview_rendered_by_report_route,
                "report_route_kg_adapter_read_performed": kg_preview_lane.kg_external_adapter_read_performed_by_report_route,
                "report_route_credential_read_performed": kg_preview_lane.kg_external_adapter_credential_read_performed_by_report_route,
                "report_route_kg_live_write_performed": kg_preview_lane.kg_live_write_performed_by_report_route
            }
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "truth_matrix": [
                {
                    "surface": "hepta_core",
                    "connected": memory_core_connected,
                    "operator_approved_lane_ready": true,
                    "explicit_command_required": false,
                    "report_route_execution_performed": false,
                    "full_live_unrestricted": false
                },
                {
                    "surface": "memory",
                    "connected": true,
                    "operator_approved_lane_ready": memory_lane_ready,
                    "explicit_command_required": true,
                    "report_route_execution_performed": memory_lane.live_memory_write_performed_by_report_route,
                    "full_live_unrestricted": false
                },
                {
                    "surface": "hepta_intelligence",
                    "connected": true,
                    "operator_approved_lane_ready": intelligence_lane_ready,
                    "explicit_command_required": true,
                    "report_route_execution_performed": intelligence_lane.hepta_intelligence_context_attached_by_report_route,
                    "full_live_unrestricted": false
                },
                {
                    "surface": "kg",
                    "connected": true,
                    "operator_approved_lane_ready": kg_lane_ready,
                    "explicit_command_required": true,
                    "report_route_execution_performed": kg_preview_lane.kg_external_adapter_read_performed_by_report_route,
                    "full_live_unrestricted": false
                },
                {
                    "surface": "provider_model",
                    "connected": false,
                    "operator_approved_lane_ready": false,
                    "explicit_command_required": true,
                    "report_route_execution_performed": false,
                    "full_live_unrestricted": false
                },
                {
                    "surface": "channel_public_release",
                    "connected": false,
                    "operator_approved_lane_ready": false,
                    "explicit_command_required": true,
                    "report_route_execution_performed": false,
                    "full_live_unrestricted": false
                }
            ],
            "blocked_actions": [
                "treat_lane_ready_as_full_live_activation",
                "write_memory_from_truth_index_report_route",
                "attach_or_inject_context_from_truth_index_report_route",
                "render_or_materialize_prompt_payload_from_truth_index_report_route",
                "read_kg_adapter_or_credential_from_truth_index_report_route",
                "write_live_kg_from_truth_index_report_route",
                "invoke_provider_or_model_from_truth_index_report_route",
                "telegram_or_channel_delivery_from_truth_index_report_route",
                "release_public_claim_from_truth_index_report_route",
                "service_restart_or_active_binary_mutation_from_truth_index_report_route"
            ],
            "allowed_next_actions": [
                {
                    "action": "continue_release_artifact_publication_denial_chain",
                    "status": "allowed_report_only_next_slice",
                    "mutates_memory": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "delivers_channel": false,
                    "claims_public_release": false
                },
                {
                    "action": "prepare_minimal_memory_canary_scoped_operator_packet",
                    "status": "blocked_until_release_and_separate_gate",
                    "requires_explicit_command": true,
                    "kg_allowed": false,
                    "provider_model_allowed": false,
                    "channel_delivery_allowed": false
                }
            ]
        }),
    );

    let mut side_effects = serde_json::Map::new();
    for key in [
        "upstream_fetch_performed",
        "upstream_merge_performed",
        "upstream_checkout_performed",
        "workspace_write",
        "active_binary_mutated",
        "active_service_restart",
        "launchd_mutated",
        "gateway_mutation_performed",
        "provider_invoked",
        "model_invoked",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_release_published",
        "public_ga_claimed",
        "evidence_persisted",
        "credential_value_read",
        "credential_read",
        "secret_file_read",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": {
                "truth_index_report_route_mutated_state": false,
                "full_live_enablement_performed": false,
                "memory_store_mutated": false,
                "memory_store_write_performed": false,
                "hepta_intelligence_context_attached": false,
                "prompt_preview_rendered": false,
                "prompt_payload_materialized": false,
                "context_injection_performed": false,
                "external_kg_adapter_read_performed": false,
                "external_adapter_client_constructed": false,
                "kg_credential_read": false,
                "auth_secret_read": false,
                "credential_read": false,
                "live_kg_write_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "network_call_performed": false,
                "external_db_write_performed": false,
                "channel_send_performed": false,
                "telegram_send_performed": false,
                "external_send_performed": false,
                "operator_acceptance_recorded": false,
                "operator_approval_recorded": false,
                "activation_authority_derived": false,
                "release_artifact_written": false,
                "public_release_claimed": false,
                "public_ga_claimed": false,
                "install_executed": false,
                "launchd_mutated": false,
                "service_restarted": false,
                "active_binary_mutated": false,
                "filesystem_written": false
            }
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready =
        route_matrix.ready && route_count_floor_preserved && route_count_source_command_accepted;

    let acknowledgement_fixture =
        |fixture_id: &str, status: &str, denial_reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "operator_acknowledgement_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(denial_reason.to_string()),
            );
            for key in [
                "source_operator_summary_non_persistence_present",
                "source_operator_summary_non_persistence_ready",
                "acknowledgement_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "operator_acknowledgement_allowed",
                "operator_acknowledgement_request_accepted",
                "operator_acknowledgement_recorded",
                "operator_acknowledgement_persisted",
                "operator_acknowledgement_materialized",
                "operator_acknowledgement_filesystem_written",
                "operator_acknowledgement_delivered",
                "operator_acknowledgement_accepted",
                "operator_identity_accepted",
                "operator_scope_accepted",
                "operator_activation_plan_accepted",
                "operator_summary_review_accepted",
                "operator_briefing_review_accepted",
                "receipt_acknowledgement_accepted",
                "runtime_attachment_acknowledged",
                "live_context_acknowledged",
                "memory_kg_acknowledged",
                "provider_secret_acknowledged",
                "operator_summary_recorded",
                "operator_summary_persisted",
                "operator_briefing_recorded",
                "operator_briefing_persisted",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "receipt_exported",
                "receipt_query_registered",
                "receipt_observability_recorded",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "runtime_router_mutated",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "rollback_executed",
                "public_release_claimed",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };

    let acknowledgement_fixtures = serde_json::Value::Array(vec![
        acknowledgement_fixture(
            "provider-router-operator-acknowledgement-missing-source-summary-non-persistence",
            "blocked_noop",
            "source_operator_summary_non_persistence_report_required",
            serde_json::json!({
                "source_operator_summary_non_persistence_present": false,
                "source_operator_summary_non_persistence_ready": false,
                "operator_acknowledgement_requested": true,
            }),
        ),
        acknowledgement_fixture(
            "provider-router-operator-acknowledgement-request",
            "blocked_acknowledgement_noop",
            "operator_acknowledgement_request_shape_denied",
            serde_json::json!({"operator_acknowledgement_requested": true}),
        ),
        acknowledgement_fixture(
            "provider-router-operator-identity-acknowledgement-request",
            "blocked_identity_noop",
            "operator_identity_acknowledgement_denied",
            serde_json::json!({
                "operator_acknowledgement_requested": true,
                "operator_identity_acknowledgement_requested": true,
            }),
        ),
        acknowledgement_fixture(
            "provider-router-operator-scope-acknowledgement-request",
            "blocked_scope_noop",
            "operator_scope_acknowledgement_denied",
            serde_json::json!({
                "operator_acknowledgement_requested": true,
                "operator_scope_acknowledgement_requested": true,
            }),
        ),
        acknowledgement_fixture(
            "provider-router-operator-activation-plan-acknowledgement-request",
            "blocked_activation_noop",
            "operator_activation_plan_acknowledgement_denied",
            serde_json::json!({
                "operator_acknowledgement_requested": true,
                "operator_activation_plan_acknowledgement_requested": true,
            }),
        ),
        acknowledgement_fixture(
            "provider-router-summary-review-acknowledgement-request",
            "blocked_review_noop",
            "operator_summary_review_acknowledgement_denied",
            serde_json::json!({
                "operator_acknowledgement_requested": true,
                "operator_summary_review_acknowledgement_requested": true,
                "operator_briefing_review_acknowledgement_requested": true,
            }),
        ),
        acknowledgement_fixture(
            "provider-router-receipt-export-query-observability-acknowledgement-request",
            "blocked_receipt_noop",
            "receipt_export_query_observability_acknowledgement_denied",
            serde_json::json!({
                "operator_acknowledgement_requested": true,
                "receipt_acknowledgement_requested": true,
                "receipt_export_acknowledgement_requested": true,
                "receipt_query_acknowledgement_requested": true,
                "receipt_observability_acknowledgement_requested": true,
            }),
        ),
        acknowledgement_fixture(
            "provider-router-runtime-attachment-live-context-acknowledgement-request",
            "blocked_runtime_noop",
            "runtime_attachment_live_context_acknowledgement_denied",
            serde_json::json!({
                "operator_acknowledgement_requested": true,
                "runtime_attachment_acknowledgement_requested": true,
                "live_context_acknowledgement_requested": true,
                "context_injection_acknowledgement_requested": true,
            }),
        ),
        acknowledgement_fixture(
            "provider-router-memory-kg-provider-secret-usage-acknowledgement-request",
            "blocked_memory_provider_noop",
            "memory_kg_provider_secret_usage_acknowledgement_denied",
            serde_json::json!({
                "operator_acknowledgement_requested": true,
                "memory_kg_acknowledgement_requested": true,
                "provider_secret_acknowledgement_requested": true,
                "usage_acknowledgement_requested": true,
            }),
        ),
        acknowledgement_fixture(
            "provider-router-external-public-install-restart-active-binary-acknowledgement-request",
            "blocked_external_noop",
            "external_public_install_restart_active_binary_acknowledgement_denied",
            serde_json::json!({
                "operator_acknowledgement_requested": true,
                "external_send_acknowledgement_requested": true,
                "public_claim_acknowledgement_requested": true,
                "release_artifact_acknowledgement_requested": true,
                "install_acknowledgement_requested": true,
                "service_restart_acknowledgement_requested": true,
                "active_binary_acknowledgement_requested": true,
            }),
        ),
    ]);
    let acknowledgement_fixture_count = acknowledgement_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let denials: Vec<serde_json::Value> = [
        "source_operator_summary_non_persistence_report_required",
        "operator_acknowledgement_request_acceptance_denied",
        "operator_acknowledgement_recording_denied",
        "operator_acknowledgement_persistence_denied",
        "operator_acknowledgement_materialization_denied",
        "operator_acknowledgement_filesystem_write_denied",
        "operator_acknowledgement_delivery_denied",
        "operator_acknowledgement_acceptance_denied",
        "operator_identity_acceptance_denied",
        "operator_scope_acceptance_denied",
        "operator_activation_plan_acceptance_denied",
        "operator_summary_review_acceptance_denied",
        "operator_briefing_review_acceptance_denied",
        "receipt_acknowledgement_acceptance_denied",
        "receipt_export_acknowledgement_denied",
        "receipt_query_acknowledgement_denied",
        "receipt_observability_acknowledgement_denied",
        "router_handoff_acknowledgement_denied",
        "readback_evidence_acknowledgement_denied",
        "runtime_attachment_acknowledgement_denied",
        "live_context_acknowledgement_denied",
        "context_injection_acknowledgement_denied",
        "memory_kg_acknowledgement_denied",
        "rollback_acknowledgement_denied",
        "secret_material_acknowledgement_denied",
        "provider_model_acknowledgement_denied",
        "external_public_install_restart_active_binary_acknowledgement_denied",
    ]
    .into_iter()
    .map(|item| serde_json::Value::String(item.to_string()))
    .collect();

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance --json",
        "native_route": true,
        "compatibility_mode": "native_runtime_provider_router_operator_acknowledgement_non_acceptance_status",
        "side_effect_free": true,
        "audit_date": "2026-06-29",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT,
        "source_runtime_provider_router_operator_acknowledgement_non_acceptance_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance-gate.sh",
        "source_runtime_provider_router_operator_acknowledgement_non_acceptance_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance-route-gate.sh",
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "source_operator_summary_non_persistence_gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_facing_summary_non_persistence_gate",
        "source_operator_summary_non_persistence_ready": true,
        "source_operator_summary_non_persistence_status": "blocked",
        "source_receipt_observability_denial_gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_receipt_observability_denial_gate",
        "source_receipt_observability_denial_ready": true,
        "source_receipt_observability_denial_status": "blocked",
        "source_runtime_model_provider_router": "runtime_provider_router",
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
        "route_count_floor_preserved": route_count_floor_preserved,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "runtime_provider_router_operator_acknowledgement_non_acceptance_route_enabled": true,
        "runtime_provider_router_operator_acknowledgement_non_acceptance_ready": true,
        "runtime_provider_router_operator_acknowledgement_non_acceptance_status": "blocked",
        "operator_acknowledgement_non_acceptance_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_v1",
        "operator_acknowledgement_non_acceptance_mode": "runtime_provider_router_operator_acknowledgement_non_acceptance_no_record_no_persist_no_activation",
        "operator_facing_summary_non_persistence_ready": true,
        "operator_facing_summary_non_persistence_status": "blocked",
        "receipt_observability_denial_ready": true,
        "receipt_observability_denial_status": "blocked",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "operator_facing_summary_fixture_count": 10,
        "blocked_operator_facing_summary_fixture_count": 10,
        "noop_operator_facing_summary_fixture_count": 10,
        "allowed_operator_facing_summary_fixture_count": 0,
        "accepted_operator_facing_summary_fixture_count": 0,
        "operator_summary_denied_count": 10,
        "operator_briefing_denied_count": 10,
        "operator_summary_performed_count": 0,
        "operator_briefing_performed_count": 0,
        "receipt_export_denied_count": 10,
        "receipt_query_denied_count": 10,
        "receipt_observability_denied_count": 10,
        "receipt_export_performed_count": 0,
        "receipt_query_performed_count": 0,
        "receipt_observability_performed_count": 0,
        "operator_acknowledgement_surface_count": 12,
        "operator_acknowledgement_surface_ready_count": 12,
        "operator_acknowledgement_side_effect_free_surface_count": 12,
        "operator_acknowledgement_fixtures": acknowledgement_fixtures,
        "operator_acknowledgement_fixture_count": acknowledgement_fixture_count,
        "blocked_operator_acknowledgement_fixture_count": acknowledgement_fixture_count,
        "noop_operator_acknowledgement_fixture_count": acknowledgement_fixture_count,
        "allowed_operator_acknowledgement_fixture_count": 0,
        "accepted_operator_acknowledgement_fixture_count": 0,
        "operator_acknowledgement_denied_count": acknowledgement_fixture_count,
        "operator_acknowledgement_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "operator_acknowledgement_allowed": false,
        "operator_acknowledgement_request_accepted": false,
        "operator_acknowledgement_recorded": false,
        "operator_acknowledgement_persisted": false,
        "operator_acknowledgement_materialized": false,
        "operator_acknowledgement_filesystem_written": false,
        "operator_acknowledgement_delivered": false,
        "operator_acknowledgement_accepted": false,
        "operator_identity_accepted": false,
        "operator_scope_accepted": false,
        "operator_activation_plan_accepted": false,
        "operator_summary_review_accepted": false,
        "operator_briefing_review_accepted": false,
        "receipt_acknowledgement_accepted": false,
        "runtime_attachment_acknowledged": false,
        "live_context_acknowledged": false,
        "memory_kg_acknowledged": false,
        "provider_secret_acknowledged": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "operator_summary_recorded": false,
        "operator_summary_persisted": false,
        "operator_summary_materialized": false,
        "operator_summary_filesystem_written": false,
        "operator_summary_delivered": false,
        "operator_briefing_recorded": false,
        "operator_briefing_persisted": false,
        "operator_briefing_materialized": false,
        "operator_briefing_filesystem_written": false,
        "operator_briefing_delivered": false,
        "operator_summary_briefing_channel_delivery_performed": false,
        "telegram_send_performed": false,
        "channel_send_performed": false,
        "external_send_performed": false,
        "receipt_export_allowed": false,
        "receipt_exported": false,
        "receipt_query_allowed": false,
        "receipt_query_registered": false,
        "receipt_observability_allowed": false,
        "receipt_observability_recorded": false,
        "receipt_recorded": false,
        "receipt_persisted": false,
        "receipt_accepted": false,
        "receipt_materialized": false,
        "receipt_filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "readback_evidence_recorded": false,
        "readback_evidence_persisted": false,
        "router_handoff_recorded": false,
        "router_handoff_persisted": false,
        "runtime_router_mutated": false,
        "live_context_attached": false,
        "context_injection_performed": false,
        "adapter_invoked": false,
        "provider_invoked": false,
        "model_invoked": false,
        "auth_secret_read": false,
        "credential_read": false,
        "secret_file_read": false,
        "usage_recorded": false,
        "memory_store_write_performed": false,
        "memory_store_mutated": false,
        "live_kg_write_performed": false,
        "rollback_executed": false,
        "public_release_claimed": false,
        "service_restart_performed": false,
        "active_binary_mutated": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "operator_acknowledgement_surfaces": [
            "source_operator_summary_non_persistence_report_required",
            "operator_acknowledgement_request_shape_denied",
            "operator_acknowledgement_recording_denied",
            "operator_acknowledgement_persistence_denied",
            "operator_identity_scope_activation_plan_acceptance_denied",
            "operator_summary_briefing_review_acceptance_denied",
            "receipt_export_query_observability_acknowledgement_denied",
            "router_handoff_readback_acknowledgement_denied",
            "runtime_attachment_live_context_acknowledgement_denied",
            "context_injection_acknowledgement_denied",
            "memory_kg_provider_secret_usage_acknowledgement_denied",
            "external_public_install_restart_active_binary_acknowledgement_denied"
        ],
        "denied_by_operator_acknowledgement_non_acceptance": denials,
        "denied_by_operator_acknowledgement_non_acceptance_count": 27,
        "allowed_next_actions": [
            {
                "action": "review_runtime_provider_router_operator_acknowledgement_non_acceptance",
                "status": "allowed_report_only",
                "accepts_acknowledgement": false,
                "records_acknowledgement": false,
                "persists_acknowledgement": false,
                "invokes_adapter": false,
                "invokes_model": false
            },
            {
                "action": "stage_runtime_provider_router_activation_request_denial_matrix",
                "status": "allowed_report_only_next_slice",
                "accepts_activation_request": false,
                "accepts_acknowledgement": false,
                "persists_summary": false,
                "exports_receipt": false,
                "invokes_adapter": false,
                "invokes_model": false
            }
        ],
        "source_operator_summary_non_persistence_report_required": true,
        "operator_acknowledgement_acceptance_forbidden": true,
        "operator_acknowledgement_recording_forbidden": true,
        "operator_acknowledgement_persistence_forbidden": true,
        "operator_acknowledgement_delivery_forbidden": true,
        "operator_identity_acceptance_forbidden": true,
        "operator_scope_acceptance_forbidden": true,
        "operator_activation_plan_acceptance_forbidden": true,
        "receipt_acknowledgement_acceptance_forbidden": true,
        "receipt_export_forbidden": true,
        "receipt_query_forbidden": true,
        "receipt_observability_forbidden": true,
        "operator_summary_persistence_forbidden": true,
        "operator_briefing_persistence_forbidden": true,
        "router_handoff_persistence_forbidden": true,
        "readback_evidence_persistence_forbidden": true,
        "live_context_attachment_forbidden": true,
        "adapter_invocation_forbidden": true,
        "provider_model_invocation_forbidden": true,
        "auth_secret_read_forbidden": true,
        "usage_recording_forbidden": true,
        }),
    );
    let mut acknowledgement_side_effects = serde_json::Map::new();
    for key in [
        "operator_acknowledgement_recorded",
        "operator_acknowledgement_persisted",
        "operator_acknowledgement_materialized",
        "operator_acknowledgement_filesystem_written",
        "operator_acknowledgement_delivered",
        "operator_acknowledgement_accepted",
        "operator_identity_accepted",
        "operator_scope_accepted",
        "operator_activation_plan_accepted",
        "operator_summary_review_accepted",
        "operator_briefing_review_accepted",
        "receipt_acknowledgement_accepted",
        "runtime_attachment_acknowledged",
        "live_context_acknowledged",
        "memory_kg_acknowledged",
        "provider_secret_acknowledged",
        "operator_summary_recorded",
        "operator_summary_persisted",
        "operator_summary_materialized",
        "operator_summary_filesystem_written",
        "operator_summary_delivered",
        "operator_briefing_recorded",
        "operator_briefing_persisted",
        "operator_briefing_materialized",
        "operator_briefing_filesystem_written",
        "operator_briefing_delivered",
        "operator_summary_briefing_channel_delivery_performed",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "receipt_exported",
        "receipt_query_registered",
        "receipt_observability_recorded",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "receipt_materialized",
        "receipt_filesystem_written",
        "readback_evidence_recorded",
        "readback_evidence_persisted",
        "router_handoff_recorded",
        "router_handoff_persisted",
        "runtime_router_mutated",
        "live_context_attached",
        "context_injection_performed",
        "adapter_invoked",
        "provider_invoked",
        "model_invoked",
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "usage_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "rollback_executed",
        "filesystem_written",
        "public_release_claimed",
        "launchd_mutated",
        "service_restart_performed",
        "active_binary_mutated",
    ] {
        acknowledgement_side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
    }
    if let Some(report) = report.as_object_mut() {
        report.insert(
            "side_effects".to_string(),
            serde_json::Value::Object(acknowledgement_side_effects),
        );
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_acknowledgement =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_report();
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
    let source_str = |key: &str| {
        source_acknowledgement
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_acknowledgement_ready = source_str("status") == "ready"
        && source_bool("runtime_provider_router_operator_acknowledgement_non_acceptance_ready")
        && source_str("runtime_provider_router_operator_acknowledgement_non_acceptance_status")
            == "blocked"
        && source_u64("operator_acknowledgement_fixture_count") == 10
        && source_u64("accepted_operator_acknowledgement_fixture_count") == 0
        && source_u64("operator_acknowledgement_performed_count") == 0
        && !source_bool("operator_acknowledgement_accepted")
        && !source_bool("operator_acknowledgement_recorded")
        && !source_bool("operator_acknowledgement_persisted")
        && !source_bool("runtime_router_mutated")
        && !source_bool("live_context_attached")
        && !source_bool("context_injection_performed")
        && !source_bool("adapter_invoked")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_acknowledgement_ready;

    let activation_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "activation_request_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_operator_acknowledgement_non_acceptance_present",
                "source_operator_acknowledgement_non_acceptance_ready",
                "activation_request_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            fixture.insert(
                "activation_request_requested".to_string(),
                serde_json::Value::Bool(false),
            );
            for key in [
                "activation_request_allowed",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_materialized",
                "activation_request_filesystem_written",
                "activation_request_delivered",
                "activation_request_executed",
                "activation_activated",
                "activation_nonce_accepted",
                "activation_generation_accepted",
                "operator_acknowledgement_accepted",
                "operator_identity_accepted",
                "operator_scope_accepted",
                "operator_activation_plan_accepted",
                "runtime_attachment_acknowledged",
                "live_context_acknowledged",
                "memory_kg_acknowledged",
                "provider_secret_acknowledged",
                "operator_summary_recorded",
                "operator_summary_persisted",
                "operator_summary_materialized",
                "operator_summary_filesystem_written",
                "operator_summary_delivered",
                "operator_briefing_recorded",
                "operator_briefing_persisted",
                "operator_briefing_materialized",
                "operator_briefing_filesystem_written",
                "operator_briefing_delivered",
                "receipt_exported",
                "receipt_query_registered",
                "receipt_observability_recorded",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "receipt_materialized",
                "receipt_filesystem_written",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "rollback_executed",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "public_release_claimed",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let activation_request_fixtures = serde_json::Value::Array(vec![
        activation_fixture(
            "provider-router-activation-request-missing-source-operator-acknowledgement-non-acceptance",
            "blocked_noop",
            "source_operator_acknowledgement_non_acceptance_report_required",
            serde_json::json!({
                "source_operator_acknowledgement_non_acceptance_present": false,
                "source_operator_acknowledgement_non_acceptance_ready": false,
                "activation_request_requested": true,
            }),
        ),
        activation_fixture(
            "provider-router-activation-request",
            "blocked_activation_noop",
            "activation_request_shape_denied",
            serde_json::json!({"activation_request_requested": true}),
        ),
        activation_fixture(
            "provider-router-activation-identity-scope-request",
            "blocked_identity_scope_noop",
            "activation_identity_scope_denied",
            serde_json::json!({
                "activation_request_requested": true,
                "activation_identity_requested": true,
                "activation_scope_requested": true,
            }),
        ),
        activation_fixture(
            "provider-router-activation-nonce-generation-request",
            "blocked_nonce_generation_noop",
            "activation_nonce_generation_denied",
            serde_json::json!({
                "activation_request_requested": true,
                "activation_nonce_requested": true,
                "activation_generation_requested": true,
            }),
        ),
        activation_fixture(
            "provider-router-runtime-attachment-activation-request",
            "blocked_runtime_noop",
            "runtime_attachment_activation_denied",
            serde_json::json!({
                "activation_request_requested": true,
                "runtime_attachment_requested": true,
                "runtime_router_mutation_requested": true,
            }),
        ),
        activation_fixture(
            "provider-router-live-context-activation-request",
            "blocked_context_noop",
            "live_context_context_injection_activation_denied",
            serde_json::json!({
                "activation_request_requested": true,
                "live_context_attachment_requested": true,
                "context_injection_requested": true,
            }),
        ),
        activation_fixture(
            "provider-router-adapter-provider-model-activation-request",
            "blocked_provider_noop",
            "adapter_provider_model_activation_denied",
            serde_json::json!({
                "activation_request_requested": true,
                "adapter_invocation_requested": true,
                "provider_invocation_requested": true,
                "model_invocation_requested": true,
            }),
        ),
        activation_fixture(
            "provider-router-memory-kg-activation-request",
            "blocked_memory_kg_noop",
            "memory_kg_activation_denied",
            serde_json::json!({
                "activation_request_requested": true,
                "memory_store_write_requested": true,
                "live_kg_write_requested": true,
            }),
        ),
        activation_fixture(
            "provider-router-receipt-readback-router-handoff-activation-request",
            "blocked_receipt_router_noop",
            "receipt_readback_router_handoff_activation_denied",
            serde_json::json!({
                "activation_request_requested": true,
                "receipt_record_requested": true,
                "receipt_persist_requested": true,
                "receipt_accept_requested": true,
                "readback_evidence_requested": true,
                "router_handoff_requested": true,
            }),
        ),
        activation_fixture(
            "provider-router-external-public-install-restart-active-binary-activation-request",
            "blocked_external_noop",
            "external_public_install_restart_active_binary_activation_denied",
            serde_json::json!({
                "activation_request_requested": true,
                "external_send_requested": true,
                "public_claim_requested": true,
                "release_artifact_requested": true,
                "install_requested": true,
                "service_restart_requested": true,
                "active_binary_mutation_requested": true,
            }),
        ),
    ]);
    let activation_request_fixture_count = activation_request_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let denials: Vec<serde_json::Value> = [
        "source_operator_acknowledgement_non_acceptance_report_required",
        "activation_request_acceptance_denied",
        "activation_request_recording_denied",
        "activation_request_persistence_denied",
        "activation_request_materialization_denied",
        "activation_request_filesystem_write_denied",
        "activation_request_delivery_denied",
        "activation_request_execution_denied",
        "activation_request_activation_denied",
        "operator_acknowledgement_acceptance_denied",
        "operator_identity_acceptance_denied",
        "operator_scope_acceptance_denied",
        "activation_nonce_acceptance_denied",
        "activation_generation_acceptance_denied",
        "runtime_attachment_denied",
        "live_context_attachment_denied",
        "context_injection_denied",
        "adapter_invocation_denied",
        "provider_model_invocation_denied",
        "memory_store_write_denied",
        "live_kg_write_denied",
        "receipt_record_persist_accept_denied",
        "receipt_export_query_observability_denied",
        "router_handoff_readback_persistence_denied",
        "usage_recording_denied",
        "secret_material_read_denied",
        "external_public_install_restart_active_binary_denied",
    ]
    .into_iter()
    .map(|item| serde_json::Value::String(item.to_string()))
    .collect();

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix --json",
        "native_route": true,
        "compatibility_mode": "native_runtime_provider_router_activation_request_denial_matrix_status",
        "side_effect_free": true,
        "audit_date": "2026-06-30",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_REQUEST_DENIAL_MATRIX_ENDPOINT,
        "source_activation_request_denial_matrix_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-gate.sh",
        "source_activation_request_denial_matrix_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-route-gate.sh",
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_acknowledgement_non_acceptance_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT,
            "source_operator_acknowledgement_non_acceptance_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance-route-gate.sh",
            "source_operator_acknowledgement_non_acceptance_ready": source_acknowledgement_ready,
            "source_operator_acknowledgement_non_acceptance_status": source_str("runtime_provider_router_operator_acknowledgement_non_acceptance_status"),
            "source_operator_acknowledgement_non_acceptance_report_sha256": sha256_json_value(&source_acknowledgement),
            "source_runtime_model_provider_router": source_str("source_runtime_model_provider_router"),
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_request_denial_matrix_route_enabled": true,
            "runtime_provider_router_activation_request_denial_matrix_ready": true,
            "runtime_provider_router_activation_request_denial_matrix_status": "blocked",
            "activation_request_denial_matrix_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_v1",
            "activation_request_denial_matrix_mode": "runtime_provider_router_activation_request_denial_matrix_no_accept_no_execute_no_activation",
            "operator_acknowledgement_non_acceptance_ready": source_acknowledgement_ready,
            "operator_acknowledgement_non_acceptance_status": source_str("runtime_provider_router_operator_acknowledgement_non_acceptance_status"),
            "operator_acknowledgement_fixture_count": source_u64("operator_acknowledgement_fixture_count"),
            "blocked_operator_acknowledgement_fixture_count": source_u64("blocked_operator_acknowledgement_fixture_count"),
            "noop_operator_acknowledgement_fixture_count": source_u64("noop_operator_acknowledgement_fixture_count"),
            "allowed_operator_acknowledgement_fixture_count": source_u64("allowed_operator_acknowledgement_fixture_count"),
            "accepted_operator_acknowledgement_fixture_count": source_u64("accepted_operator_acknowledgement_fixture_count"),
            "operator_acknowledgement_denied_count": source_u64("operator_acknowledgement_denied_count"),
            "operator_acknowledgement_performed_count": source_u64("operator_acknowledgement_performed_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_summary_denied_count": source_u64("operator_summary_denied_count"),
            "operator_briefing_denied_count": source_u64("operator_briefing_denied_count"),
            "operator_summary_performed_count": source_u64("operator_summary_performed_count"),
            "operator_briefing_performed_count": source_u64("operator_briefing_performed_count"),
            "receipt_export_denied_count": source_u64("receipt_export_denied_count"),
            "receipt_query_denied_count": source_u64("receipt_query_denied_count"),
            "receipt_observability_denied_count": source_u64("receipt_observability_denied_count"),
            "receipt_export_performed_count": source_u64("receipt_export_performed_count"),
            "receipt_query_performed_count": source_u64("receipt_query_performed_count"),
            "receipt_observability_performed_count": source_u64("receipt_observability_performed_count"),
            "activation_request_surface_count": 12,
            "activation_request_surface_ready_count": 12,
            "activation_request_side_effect_free_surface_count": 12,
            "activation_request_fixtures": activation_request_fixtures,
            "activation_request_fixture_count": activation_request_fixture_count,
            "blocked_activation_request_fixture_count": activation_request_fixture_count,
            "noop_activation_request_fixture_count": activation_request_fixture_count,
            "allowed_activation_request_fixture_count": 0,
            "accepted_activation_request_fixture_count": 0,
            "activation_request_denied_count": 10,
            "activation_request_performed_count": 0,
            "activation_execution_performed_count": 0,
        }),
    );
    for key in [
        "activation_request_allowed",
        "activation_request_accepted",
        "activation_request_recorded",
        "activation_request_persisted",
        "activation_request_materialized",
        "activation_request_filesystem_written",
        "activation_request_delivered",
        "activation_request_executed",
        "activation_activated",
        "activation_nonce_accepted",
        "activation_generation_accepted",
        "operator_acknowledgement_accepted",
        "operator_identity_accepted",
        "operator_scope_accepted",
        "operator_activation_plan_accepted",
        "runtime_attachment_acknowledged",
        "live_context_acknowledged",
        "memory_kg_acknowledged",
        "provider_secret_acknowledged",
        "operator_summary_recorded",
        "operator_summary_persisted",
        "operator_summary_materialized",
        "operator_summary_filesystem_written",
        "operator_summary_delivered",
        "operator_briefing_recorded",
        "operator_briefing_persisted",
        "operator_briefing_materialized",
        "operator_briefing_filesystem_written",
        "operator_briefing_delivered",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "receipt_export_allowed",
        "receipt_exported",
        "receipt_query_allowed",
        "receipt_query_registered",
        "receipt_observability_allowed",
        "receipt_observability_recorded",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "receipt_materialized",
        "receipt_filesystem_written",
        "readback_evidence_recorded",
        "readback_evidence_persisted",
        "router_handoff_recorded",
        "router_handoff_persisted",
        "runtime_router_mutated",
        "runtime_attachment_performed",
        "live_context_attached",
        "context_injection_performed",
        "adapter_invoked",
        "provider_invoked",
        "model_invoked",
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "usage_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "rollback_executed",
        "public_release_claimed",
        "service_restart_performed",
        "active_binary_mutated",
    ] {
        if let Some(report) = report.as_object_mut() {
            report.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_request_surfaces": [
                "source_operator_acknowledgement_non_acceptance_report_required",
                "activation_request_shape_denied",
                "activation_request_recording_denied",
                "activation_request_persistence_denied",
                "activation_identity_scope_denied",
                "activation_nonce_generation_denied",
                "runtime_attachment_activation_denied",
                "live_context_context_injection_activation_denied",
                "adapter_provider_model_activation_denied",
                "memory_kg_activation_denied",
                "receipt_readback_router_handoff_activation_denied",
                "external_public_install_restart_active_binary_activation_denied"
            ],
            "denied_by_activation_request_denial_matrix": denials,
            "denied_by_activation_request_denial_matrix_count": 27,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_request_denial_matrix",
                    "status": "allowed_report_only",
                    "accepts_activation_request": false,
                    "executes_activation": false,
                    "mutates_runtime": false,
                    "invokes_adapter": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_noop_handoff",
                    "status": "allowed_report_only_next_slice",
                    "accepts_activation_request": false,
                    "records_activation": false,
                    "executes_activation": false,
                    "attaches_live_context": false,
                    "invokes_model": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_acknowledgement_non_acceptance_report_required": true,
            "activation_request_acceptance_forbidden": true,
            "activation_request_recording_forbidden": true,
            "activation_request_persistence_forbidden": true,
            "activation_request_execution_forbidden": true,
            "activation_runtime_mutation_forbidden": true,
            "live_context_attachment_forbidden": true,
            "context_injection_forbidden": true,
            "adapter_invocation_forbidden": true,
            "provider_model_invocation_forbidden": true,
            "memory_kg_write_forbidden": true,
            "auth_secret_read_forbidden": true,
            "usage_recording_forbidden": true,
        }),
    );
    let mut side_effects = serde_json::Map::new();
    for key in [
        "activation_request_recorded",
        "activation_request_persisted",
        "activation_request_materialized",
        "activation_request_filesystem_written",
        "activation_request_delivered",
        "activation_request_executed",
        "activation_activated",
        "activation_nonce_accepted",
        "activation_generation_accepted",
        "operator_acknowledgement_accepted",
        "operator_identity_accepted",
        "operator_scope_accepted",
        "operator_activation_plan_accepted",
        "runtime_attachment_acknowledged",
        "live_context_acknowledged",
        "memory_kg_acknowledged",
        "provider_secret_acknowledged",
        "operator_summary_recorded",
        "operator_summary_persisted",
        "operator_summary_materialized",
        "operator_summary_filesystem_written",
        "operator_summary_delivered",
        "operator_briefing_recorded",
        "operator_briefing_persisted",
        "operator_briefing_materialized",
        "operator_briefing_filesystem_written",
        "operator_briefing_delivered",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "receipt_exported",
        "receipt_query_registered",
        "receipt_observability_recorded",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "receipt_materialized",
        "receipt_filesystem_written",
        "readback_evidence_recorded",
        "readback_evidence_persisted",
        "router_handoff_recorded",
        "router_handoff_persisted",
        "runtime_router_mutated",
        "runtime_attachment_performed",
        "live_context_attached",
        "context_injection_performed",
        "adapter_invoked",
        "provider_invoked",
        "model_invoked",
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "usage_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "rollback_executed",
        "filesystem_written",
        "public_release_claimed",
        "launchd_mutated",
        "service_restart_performed",
        "active_binary_mutated",
    ] {
        side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
    }
    if let Some(report) = report.as_object_mut() {
        report.insert(
            "side_effects".to_string(),
            serde_json::Value::Object(side_effects),
        );
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_activation_request =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_report();
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
    let source_str = |key: &str| {
        source_activation_request
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_activation_request_ready = source_str("status") == "ready"
        && source_bool("runtime_provider_router_activation_request_denial_matrix_ready")
        && source_str("runtime_provider_router_activation_request_denial_matrix_status")
            == "blocked"
        && source_u64("activation_request_fixture_count") == 10
        && source_u64("blocked_activation_request_fixture_count") == 10
        && source_u64("noop_activation_request_fixture_count") == 10
        && source_u64("accepted_activation_request_fixture_count") == 0
        && source_u64("activation_request_performed_count") == 0
        && source_u64("activation_execution_performed_count") == 0
        && !source_bool("activation_request_accepted")
        && !source_bool("activation_request_recorded")
        && !source_bool("activation_request_persisted")
        && !source_bool("activation_request_executed")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("runtime_attachment_performed")
        && !source_bool("live_context_attached")
        && !source_bool("context_injection_performed")
        && !source_bool("adapter_invoked")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("auth_secret_read")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("usage_recorded")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("receipt_recorded")
        && !source_bool("receipt_persisted")
        && !source_bool("receipt_accepted")
        && !source_bool("readback_evidence_recorded")
        && !source_bool("readback_evidence_persisted")
        && !source_bool("router_handoff_recorded")
        && !source_bool("router_handoff_persisted")
        && !source_bool("telegram_send_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_activation_request_ready;

    let activation_command_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::Value::String(id.to_string()));
            fixture.insert(
                "activation_command_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
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
                "activation_activated",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "receipt_exported",
                "receipt_query_registered",
                "receipt_observability_recorded",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "rollback_executed",
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
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let activation_command_fixtures = serde_json::Value::Array(vec![
        activation_command_fixture(
            "provider-router-activation-command-missing-source-activation-request-denial-matrix",
            "blocked_noop",
            "source_activation_request_denial_matrix_report_required",
            serde_json::json!({
                "source_activation_request_denial_matrix_present": false,
                "source_activation_request_denial_matrix_ready": false,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-handoff-request",
            "blocked_command_noop",
            "activation_command_handoff_shape_denied",
            serde_json::json!({}),
        ),
        activation_command_fixture(
            "provider-router-activation-command-registration-enable-request",
            "blocked_register_enable_noop",
            "activation_command_registration_enablement_denied",
            serde_json::json!({
                "activation_command_registration_requested": true,
                "activation_command_enable_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-direct-invocation-request",
            "blocked_invocation_noop",
            "activation_command_invocation_denied",
            serde_json::json!({"activation_command_invocation_requested": true}),
        ),
        activation_command_fixture(
            "provider-router-activation-command-runtime-router-dispatch-request",
            "blocked_dispatch_noop",
            "runtime_router_dispatch_denied",
            serde_json::json!({
                "runtime_router_dispatch_requested": true,
                "runtime_router_mutation_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-live-context-injection-request",
            "blocked_context_noop",
            "live_context_context_injection_command_denied",
            serde_json::json!({
                "live_context_attachment_requested": true,
                "context_injection_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-adapter-provider-model-request",
            "blocked_provider_noop",
            "adapter_provider_model_command_denied",
            serde_json::json!({
                "adapter_invocation_requested": true,
                "provider_invocation_requested": true,
                "model_invocation_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-memory-kg-request",
            "blocked_memory_kg_noop",
            "memory_kg_command_denied",
            serde_json::json!({
                "memory_store_write_requested": true,
                "live_kg_write_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-receipt-readback-router-handoff-request",
            "blocked_receipt_router_noop",
            "receipt_readback_router_handoff_command_denied",
            serde_json::json!({
                "receipt_record_requested": true,
                "receipt_persist_requested": true,
                "receipt_export_requested": true,
                "receipt_query_requested": true,
                "receipt_observability_requested": true,
                "readback_evidence_requested": true,
                "router_handoff_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-external-public-install-restart-active-binary-request",
            "blocked_external_noop",
            "external_public_install_restart_active_binary_command_denied",
            serde_json::json!({
                "external_send_requested": true,
                "public_claim_requested": true,
                "public_ga_claim_requested": true,
                "release_artifact_write_requested": true,
                "install_requested": true,
                "launchd_restart_requested": true,
                "service_restart_requested": true,
                "active_binary_mutation_requested": true,
            }),
        ),
    ]);
    let activation_command_fixture_count = activation_command_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let denials: Vec<serde_json::Value> = [
        "source_activation_request_denial_matrix_report_required",
        "activation_command_shape_registration_denied",
        "activation_command_acceptance_denied",
        "activation_command_enablement_denied",
        "activation_command_invocation_denied",
        "activation_command_dispatch_denied",
        "activation_command_noop_decision_recording_denied",
        "activation_command_noop_decision_persistence_denied",
        "activation_command_handoff_recording_denied",
        "activation_command_handoff_persistence_denied",
        "activation_command_handoff_acceptance_denied",
        "activation_command_handoff_materialization_denied",
        "activation_command_handoff_filesystem_write_denied",
        "activation_command_result_receipt_recording_denied",
        "activation_command_result_receipt_persistence_denied",
        "activation_request_acceptance_denied",
        "activation_execution_denied",
        "runtime_router_mutation_denied",
        "runtime_attachment_denied",
        "live_context_attachment_denied",
        "context_injection_denied",
        "adapter_invocation_denied",
        "provider_model_invocation_denied",
        "memory_store_write_denied",
        "live_kg_write_denied",
        "receipt_export_query_observability_denied",
        "router_handoff_readback_persistence_denied",
        "usage_recording_denied",
        "secret_material_read_denied",
        "external_public_install_restart_active_binary_denied",
    ]
    .into_iter()
    .map(|item| serde_json::Value::String(item.to_string()))
    .collect();
    let source_report_sha256 = sha256_json_value(&source_activation_request);
    let fixture_hash = sha256_json_value(&activation_command_fixtures);
    let contract_hash = sha256_text_value(&format!(
        "hepta-full-enablement-runtime-provider-router-activation-command-noop-handoff:native:source={source_report_sha256}:fixtures={fixture_hash}:route_count={}:command=0:dispatch=0:provider=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-noop-handoff:report-only:no-command-register:no-command-enable:no-command-invoke:no-dispatch:no-handoff-persist:no-provider:no-model:no-secret-read",
    );

    let mut report = source_activation_request.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff --json",
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_noop_handoff_status",
            "side_effect_free": true,
            "audit_date": "2026-06-30",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_NOOP_HANDOFF_ENDPOINT,
            "source_activation_request_denial_matrix_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_REQUEST_DENIAL_MATRIX_ENDPOINT,
            "source_activation_request_denial_matrix_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-gate.sh",
            "source_activation_request_denial_matrix_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-route-gate.sh",
            "source_activation_command_noop_handoff_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-gate.sh",
            "source_activation_command_noop_handoff_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-route-gate.sh",
            "source_activation_request_denial_matrix_report_sha256": source_report_sha256,
            "activation_command_fixtures_sha256": fixture_hash,
            "activation_command_contract_hash_sha256": contract_hash,
            "activation_command_policy_hash_sha256": policy_hash,
            "minimum_required_samples": 24,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_activation_request_denial_matrix_ready": source_activation_request_ready,
            "source_activation_request_denial_matrix_status": source_str("runtime_provider_router_activation_request_denial_matrix_status"),
            "source_runtime_model_provider_router": source_str("source_runtime_model_provider_router"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_noop_handoff_route_enabled": true,
            "runtime_provider_router_activation_command_noop_handoff_ready": true,
            "runtime_provider_router_activation_command_noop_handoff_status": "blocked",
            "activation_command_noop_handoff_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_v1",
            "activation_command_noop_handoff_mode": "runtime_provider_router_activation_command_noop_handoff_no_register_no_enable_no_invoke_no_dispatch",
            "activation_command_noop_handoff_decision": "runtime_provider_router_activation_request_denial_matrix_cannot_create_or_authorize_activation_commands",
            "runtime_provider_router_activation_request_denial_matrix_ready": source_bool("runtime_provider_router_activation_request_denial_matrix_ready"),
            "runtime_provider_router_activation_request_denial_matrix_status": source_str("runtime_provider_router_activation_request_denial_matrix_status"),
            "source_activation_request_fixture_count": source_u64("activation_request_fixture_count"),
            "source_blocked_activation_request_fixture_count": source_u64("blocked_activation_request_fixture_count"),
            "source_noop_activation_request_fixture_count": source_u64("noop_activation_request_fixture_count"),
            "source_accepted_activation_request_fixture_count": source_u64("accepted_activation_request_fixture_count"),
            "source_activation_request_performed_count": source_u64("activation_request_performed_count"),
            "activation_command_surface_count": 13,
            "activation_command_surface_ready_count": 13,
            "activation_command_side_effect_free_surface_count": 13,
            "activation_command_fixtures": activation_command_fixtures,
            "activation_command_fixture_count": activation_command_fixture_count,
            "activation_command_requested_fixture_count": activation_command_fixture_count,
            "blocked_activation_command_fixture_count": activation_command_fixture_count,
            "noop_activation_command_fixture_count": activation_command_fixture_count,
            "allowed_activation_command_fixture_count": 0,
            "accepted_activation_command_fixture_count": 0,
            "activation_command_denied_count": 10,
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
            "activation_command_noop_decision_accepted": false,
            "activation_command_handoff_recorded": false,
            "activation_command_handoff_persisted": false,
            "activation_command_handoff_accepted": false,
            "activation_command_handoff_materialized": false,
            "activation_command_handoff_filesystem_written": false,
            "activation_command_result_receipt_recorded": false,
            "activation_command_result_receipt_persisted": false,
            "activation_command_result_receipt_accepted": false,
            "activation_command_result_receipt_exported": false,
            "activation_command_result_receipt_query_registered": false,
            "activation_command_result_receipt_observability_recorded": false,
        }),
    );
    for key in [
        "activation_request_allowed",
        "activation_request_accepted",
        "activation_request_recorded",
        "activation_request_persisted",
        "activation_request_materialized",
        "activation_request_filesystem_written",
        "activation_request_delivered",
        "activation_request_executed",
        "activation_activated",
        "activation_nonce_accepted",
        "activation_generation_accepted",
        "runtime_router_mutated",
        "runtime_attachment_performed",
        "live_context_attached",
        "context_injection_performed",
        "adapter_invoked",
        "provider_invoked",
        "model_invoked",
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "usage_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "receipt_export_allowed",
        "receipt_exported",
        "receipt_query_allowed",
        "receipt_query_registered",
        "receipt_observability_allowed",
        "receipt_observability_recorded",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "receipt_materialized",
        "receipt_filesystem_written",
        "readback_evidence_recorded",
        "readback_evidence_persisted",
        "router_handoff_recorded",
        "router_handoff_persisted",
        "rollback_executed",
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
    ] {
        if let Some(report) = report.as_object_mut() {
            report.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_surfaces": [
                "source_activation_request_denial_matrix_report_required",
                "activation_command_handoff_shape_denied",
                "activation_command_registration_denied",
                "activation_command_enablement_denied",
                "activation_command_invocation_denied",
                "activation_command_dispatch_denied",
                "activation_command_handoff_record_persist_denied",
                "live_context_context_injection_command_denied",
                "adapter_provider_model_command_denied",
                "memory_kg_command_denied",
                "receipt_readback_router_handoff_command_denied",
                "command_result_receipt_export_query_observability_denied",
                "external_public_install_restart_active_binary_command_denied"
            ],
            "denied_by_activation_command_noop_handoff": denials,
            "denied_by_activation_command_noop_handoff_count": 30,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_noop_handoff",
                    "status": "allowed_report_only",
                    "registers_command": false,
                    "enables_command": false,
                    "invokes_command": false,
                    "dispatches_command": false,
                    "persists_handoff": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_no_persistence",
                    "status": "allowed_report_only_next_slice",
                    "records_command_result": false,
                    "persists_command_result": false,
                    "exports_receipt": false,
                    "registers_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "mutates_runtime": false,
                    "dispatches_command": false,
                    "attaches_live_context": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_activation_request_denial_matrix_report_required": true,
            "activation_command_registration_forbidden": true,
            "activation_command_enablement_forbidden": true,
            "activation_command_invocation_forbidden": true,
            "activation_command_dispatch_forbidden": true,
            "activation_command_handoff_persistence_forbidden": true,
            "activation_command_result_receipt_persistence_forbidden": true,
            "activation_request_acceptance_forbidden": true,
            "activation_request_execution_forbidden": true,
            "runtime_router_mutation_forbidden": true,
            "live_context_attachment_forbidden": true,
            "context_injection_forbidden": true,
            "adapter_invocation_forbidden": true,
            "provider_model_invocation_forbidden": true,
            "memory_kg_write_forbidden": true,
            "auth_secret_read_forbidden": true,
            "usage_recording_forbidden": true,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_shape_registered",
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
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_materialized",
            "activation_request_filesystem_written",
            "activation_request_delivered",
            "activation_request_executed",
            "activation_activated",
            "activation_nonce_accepted",
            "activation_generation_accepted",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "receipt_exported",
            "receipt_query_registered",
            "receipt_observability_recorded",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "receipt_filesystem_written",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "filesystem_written",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_noop_handoff =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_report();
    let source_bool = |key: &str| {
        source_noop_handoff
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_noop_handoff
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_noop_handoff
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_noop_handoff_ready = source_str("status") == "ready"
        && source_bool("runtime_provider_router_activation_command_noop_handoff_ready")
        && source_str("runtime_provider_router_activation_command_noop_handoff_status")
            == "blocked"
        && source_bool("runtime_provider_router_activation_request_denial_matrix_ready")
        && source_u64("activation_command_surface_count") == 13
        && source_u64("activation_command_surface_ready_count") == 13
        && source_u64("activation_command_fixture_count") == 10
        && source_u64("blocked_activation_command_fixture_count") == 10
        && source_u64("noop_activation_command_fixture_count") == 10
        && source_u64("allowed_activation_command_fixture_count") == 0
        && source_u64("accepted_activation_command_fixture_count") == 0
        && source_u64("activation_command_denied_count") == 10
        && source_u64("activation_command_performed_count") == 0
        && source_u64("activation_command_dispatch_performed_count") == 0
        && !source_bool("activation_command_shape_registered")
        && !source_bool("activation_command_allowed")
        && !source_bool("activation_command_accepted")
        && !source_bool("activation_command_enabled")
        && !source_bool("activation_command_invoked")
        && !source_bool("activation_command_dispatched")
        && !source_bool("activation_command_noop_decision_recorded")
        && !source_bool("activation_command_noop_decision_persisted")
        && !source_bool("activation_command_handoff_recorded")
        && !source_bool("activation_command_handoff_persisted")
        && !source_bool("activation_command_handoff_accepted")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_command_result_receipt_exported")
        && !source_bool("activation_command_result_receipt_query_registered")
        && !source_bool("activation_command_result_receipt_observability_recorded")
        && !source_bool("activation_request_accepted")
        && !source_bool("activation_request_recorded")
        && !source_bool("activation_request_persisted")
        && !source_bool("activation_request_executed")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("runtime_attachment_performed")
        && !source_bool("live_context_attached")
        && !source_bool("context_injection_performed")
        && !source_bool("adapter_invoked")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("auth_secret_read")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("usage_recorded")
        && !source_bool("memory_store_write_performed")
        && !source_bool("memory_store_mutated")
        && !source_bool("live_kg_write_performed")
        && !source_bool("receipt_recorded")
        && !source_bool("receipt_persisted")
        && !source_bool("receipt_accepted")
        && !source_bool("readback_evidence_recorded")
        && !source_bool("readback_evidence_persisted")
        && !source_bool("router_handoff_recorded")
        && !source_bool("router_handoff_persisted")
        && !source_bool("telegram_send_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("rollback_executed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_noop_handoff_ready;

    let result_receipt_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::Value::String(id.to_string()));
            fixture.insert(
                "activation_command_result_receipt_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_activation_command_noop_handoff_present",
                "source_activation_command_noop_handoff_ready",
                "activation_command_result_receipt_requested",
                "receipt_noop_confirmed",
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
                "activation_command_result_receipt_blocked_noop_status_accepted",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "activation_command_completion_ack_materialized",
                "activation_command_completion_ack_delivered",
                "operator_approval_from_receipt_accepted",
                "activation_from_receipt_allowed",
                "activation_command_shape_registered",
                "activation_command_allowed",
                "activation_command_accepted",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_command_dispatch_performed",
                "activation_command_noop_decision_recorded",
                "activation_command_noop_decision_persisted",
                "activation_command_handoff_recorded",
                "activation_command_handoff_persisted",
                "activation_command_handoff_accepted",
                "activation_command_handoff_materialized",
                "activation_command_handoff_filesystem_written",
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
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "receipt_exported",
                "receipt_query_registered",
                "receipt_observability_recorded",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "rollback_executed",
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
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let result_receipt_fixtures = serde_json::Value::Array(vec![
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-missing-source-noop-handoff",
            "blocked_noop",
            "source_activation_command_noop_handoff_report_required",
            serde_json::json!({
                "source_activation_command_noop_handoff_present": false,
                "source_activation_command_noop_handoff_ready": false,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-schema-registration-attempt",
            "blocked_schema_noop",
            "result_receipt_schema_registration_denied",
            serde_json::json!({"result_receipt_schema_registration_requested": true}),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-record-attempt",
            "blocked_record_noop",
            "result_receipt_recording_denied",
            serde_json::json!({"result_receipt_record_requested": true}),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-persist-attempt",
            "blocked_persist_noop",
            "result_receipt_persistence_denied",
            serde_json::json!({"result_receipt_persist_requested": true}),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-materialize-filesystem-attempt",
            "blocked_materialize_noop",
            "result_receipt_materialization_filesystem_write_denied",
            serde_json::json!({
                "result_receipt_materialize_requested": true,
                "result_receipt_filesystem_write_requested": true,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-ledger-index-queue-delivery-attempt",
            "blocked_ledger_index_delivery_noop",
            "result_receipt_ledger_index_queue_delivery_denied",
            serde_json::json!({
                "result_receipt_ledger_write_requested": true,
                "result_receipt_index_requested": true,
                "result_receipt_enqueue_requested": true,
                "result_receipt_delivery_requested": true,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-export-query-observability-attempt",
            "blocked_export_query_observability_noop",
            "result_receipt_export_query_observability_denied",
            serde_json::json!({
                "result_receipt_export_requested": true,
                "result_receipt_query_requested": true,
                "result_receipt_observability_requested": true,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-acceptance-completion-ack-attempt",
            "blocked_acceptance_ack_noop",
            "result_receipt_acceptance_completion_ack_denied",
            serde_json::json!({
                "result_receipt_acceptance_requested": true,
                "completion_ack_requested": true,
                "operator_approval_from_receipt_requested": true,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-runtime-context-provider-memory-kg-attempt",
            "blocked_runtime_provider_memory_kg_noop",
            "result_receipt_cannot_activate_runtime_provider_memory_or_kg",
            serde_json::json!({
                "result_receipt_status_requested": "completed",
                "activation_from_receipt_requested": true,
                "runtime_router_mutation_requested": true,
                "live_context_attachment_requested": true,
                "context_injection_requested": true,
                "provider_invocation_requested": true,
                "model_invocation_requested": true,
                "usage_record_requested": true,
                "memory_store_write_requested": true,
                "live_kg_write_requested": true,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-restart-active-binary-attempt",
            "blocked_external_noop",
            "result_receipt_cannot_send_publish_install_restart_or_mutate_active_binary",
            serde_json::json!({
                "external_send_requested": true,
                "public_claim_requested": true,
                "public_ga_claim_requested": true,
                "release_artifact_write_requested": true,
                "install_requested": true,
                "launchd_restart_requested": true,
                "service_restart_requested": true,
                "active_binary_mutation_requested": true,
            }),
        ),
    ]);
    let result_receipt_fixture_count = result_receipt_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let denials: Vec<serde_json::Value> = [
        "source_activation_command_noop_handoff_required",
        "activation_command_disabled_required",
        "activation_command_invocation_denied",
        "activation_command_dispatch_denied",
        "result_receipt_schema_registration_denied",
        "result_receipt_schema_acceptance_denied",
        "result_receipt_recording_denied",
        "result_receipt_persistence_denied",
        "result_receipt_acceptance_denied",
        "result_receipt_materialization_denied",
        "result_receipt_filesystem_write_denied",
        "result_receipt_ledger_write_denied",
        "result_receipt_indexing_denied",
        "result_receipt_queue_enqueue_denied",
        "result_receipt_delivery_denied",
        "result_receipt_export_denied",
        "result_receipt_query_registration_denied",
        "result_receipt_observability_recording_denied",
        "completion_ack_recording_denied",
        "completion_ack_persistence_denied",
        "completion_ack_acceptance_denied",
        "operator_approval_from_receipt_denied",
        "activation_from_receipt_denied",
        "runtime_router_mutation_denied",
        "live_context_attachment_denied",
        "context_injection_denied",
        "adapter_invocation_denied",
        "provider_model_invocation_denied",
        "usage_recording_denied",
        "memory_store_write_denied",
        "live_kg_write_denied",
        "secret_material_read_denied",
        "external_send_denied",
        "public_release_claim_denied",
        "install_restart_active_binary_mutation_denied",
    ]
    .into_iter()
    .map(|item| serde_json::Value::String(item.to_string()))
    .collect();
    let source_report_sha256 = sha256_json_value(&source_noop_handoff);
    let fixture_hash = sha256_json_value(&result_receipt_fixtures);
    let contract_hash = sha256_text_value(&format!(
        "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence:native:source={source_report_sha256}:fixtures={fixture_hash}:route_count={}:record=0:persist=0:export=0:query=0:observe=0:activation=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-no-persistence:report-only:no-receipt-record:no-receipt-persist:no-export:no-query:no-observability:no-activation:no-runtime:no-provider:no-model:no-secret-read",
    );

    let mut report = source_noop_handoff.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence --json",
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_no_persistence_status",
            "side_effect_free": true,
            "audit_date": "2026-06-30",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            "source_activation_command_noop_handoff_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_NOOP_HANDOFF_ENDPOINT,
            "source_activation_command_noop_handoff_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-gate.sh",
            "source_activation_command_noop_handoff_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-route-gate.sh",
            "source_activation_command_result_receipt_no_persistence_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-gate.sh",
            "source_activation_command_result_receipt_no_persistence_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-route-gate.sh",
            "source_activation_command_noop_handoff_report_sha256": source_report_sha256,
            "activation_command_result_receipt_fixtures_sha256": fixture_hash,
            "activation_command_result_receipt_contract_hash_sha256": contract_hash,
            "activation_command_result_receipt_policy_hash_sha256": policy_hash,
            "minimum_required_samples": 24,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_activation_command_noop_handoff_ready": source_noop_handoff_ready,
            "source_activation_command_noop_handoff_status": source_str("runtime_provider_router_activation_command_noop_handoff_status"),
            "source_activation_request_denial_matrix_ready": source_bool("runtime_provider_router_activation_request_denial_matrix_ready"),
            "source_runtime_model_provider_router": source_str("source_runtime_model_provider_router"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_no_persistence_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": true,
            "runtime_provider_router_activation_command_result_receipt_no_persistence_status": "blocked",
            "activation_command_result_receipt_no_persistence_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_v1",
            "activation_command_result_receipt_no_persistence_mode": "runtime_provider_router_activation_command_result_receipt_no_persistence_no_record_no_persist_no_export_no_query",
            "runtime_provider_router_activation_command_noop_handoff_ready": source_bool("runtime_provider_router_activation_command_noop_handoff_ready"),
            "runtime_provider_router_activation_command_noop_handoff_status": source_str("runtime_provider_router_activation_command_noop_handoff_status"),
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
            "activation_command_result_receipt_denied_count": 10,
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
            "activation_command_result_receipt_hash_bound": false,
            "activation_command_result_receipt_signature_hash_recorded": false,
            "activation_command_result_receipt_timestamp_recorded": false,
            "activation_command_result_receipt_operator_identity_accepted": false,
            "activation_command_result_receipt_status_accepted": false,
            "activation_command_result_receipt_blocked_noop_status_accepted": false,
            "activation_command_completion_ack_recorded": false,
            "activation_command_completion_ack_persisted": false,
            "activation_command_completion_ack_accepted": false,
            "activation_command_completion_ack_materialized": false,
            "activation_command_completion_ack_delivered": false,
            "operator_approval_from_receipt_accepted": false,
            "activation_from_receipt_allowed": false,
        }),
    );
    for key in [
        "activation_command_shape_registered",
        "activation_command_allowed",
        "activation_command_accepted",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_noop_decision_recorded",
        "activation_command_noop_decision_persisted",
        "activation_command_noop_decision_accepted",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_command_handoff_accepted",
        "activation_command_handoff_materialized",
        "activation_command_handoff_filesystem_written",
        "activation_request_allowed",
        "activation_request_accepted",
        "activation_request_recorded",
        "activation_request_persisted",
        "activation_request_materialized",
        "activation_request_filesystem_written",
        "activation_request_delivered",
        "activation_request_executed",
        "activation_activated",
        "runtime_router_mutated",
        "runtime_attachment_performed",
        "live_context_attached",
        "context_injection_performed",
        "adapter_invoked",
        "provider_invoked",
        "model_invoked",
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "usage_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "receipt_export_allowed",
        "receipt_exported",
        "receipt_query_allowed",
        "receipt_query_registered",
        "receipt_observability_allowed",
        "receipt_observability_recorded",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "receipt_materialized",
        "receipt_filesystem_written",
        "readback_evidence_recorded",
        "readback_evidence_persisted",
        "router_handoff_recorded",
        "router_handoff_persisted",
        "rollback_executed",
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
    ] {
        if let Some(report) = report.as_object_mut() {
            report.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_surfaces": [
                "source_activation_command_noop_handoff_report_required",
                "disabled_activation_command_noop_identity_required",
                "result_receipt_schema_registration_denied",
                "result_receipt_hash_signature_timestamp_binding_denied",
                "result_receipt_blocked_noop_status_acceptance_denied",
                "result_receipt_record_persist_materialize_denied",
                "result_receipt_filesystem_ledger_index_queue_delivery_denied",
                "result_receipt_export_query_observability_denied",
                "activation_command_completion_ack_denied",
                "operator_approval_and_activation_from_receipt_denied",
                "runtime_router_live_context_context_injection_denied",
                "adapter_provider_model_invocation_denied",
                "usage_memory_kg_write_denied",
                "external_public_install_restart_active_binary_denied"
            ],
            "denied_by_activation_command_result_receipt_no_persistence": denials,
            "denied_by_activation_command_result_receipt_no_persistence_count": 35,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_no_persistence",
                    "status": "allowed_report_only",
                    "records_command_result": false,
                    "persists_command_result": false,
                    "exports_receipt": false,
                    "registers_query": false,
                    "registers_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_duplicate_receipt": false,
                    "records_idempotency": false,
                    "persists_replay_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "records_command_result": false,
                    "persists_command_result": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_activation_command_noop_handoff_report_required": true,
            "result_receipt_schema_registration_forbidden": true,
            "result_receipt_recording_forbidden": true,
            "result_receipt_persistence_forbidden": true,
            "result_receipt_export_query_observability_forbidden": true,
            "result_receipt_activation_forbidden": true,
            "result_receipt_runtime_mutation_forbidden": true,
            "result_receipt_context_attachment_forbidden": true,
            "result_receipt_adapter_provider_model_invocation_forbidden": true,
            "result_receipt_memory_kg_write_forbidden": true,
            "result_receipt_secret_read_forbidden": true,
            "result_receipt_external_public_install_restart_active_binary_forbidden": true,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_shape_registered",
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
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "operator_approval_from_receipt_accepted",
            "activation_from_receipt_allowed",
            "activation_command_shape_registered",
            "activation_command_accepted",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
            "activation_command_noop_decision_recorded",
            "activation_command_noop_decision_persisted",
            "activation_command_handoff_recorded",
            "activation_command_handoff_persisted",
            "activation_command_handoff_materialized",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_materialized",
            "activation_request_filesystem_written",
            "activation_request_delivered",
            "activation_request_executed",
            "activation_activated",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "receipt_exported",
            "receipt_query_registered",
            "receipt_observability_recorded",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "receipt_filesystem_written",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "filesystem_written",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_no_persistence =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_report();
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
    let source_str = |key: &str| {
        source_no_persistence
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_no_persistence_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_no_persistence_status",
        ) == "blocked"
        && source_bool("runtime_provider_router_activation_command_noop_handoff_ready")
        && source_str("runtime_provider_router_activation_command_noop_handoff_status")
            == "blocked"
        && source_u64("activation_command_result_receipt_surface_count") == 14
        && source_u64("activation_command_result_receipt_surface_ready_count") == 14
        && source_u64("activation_command_result_receipt_fixture_count") == 10
        && source_u64("blocked_activation_command_result_receipt_fixture_count") == 10
        && source_u64("noop_activation_command_result_receipt_fixture_count") == 10
        && source_u64("allowed_activation_command_result_receipt_fixture_count") == 0
        && source_u64("accepted_activation_command_result_receipt_fixture_count") == 0
        && source_u64("activation_command_result_receipt_denied_count") == 10
        && source_u64("activation_command_result_receipt_performed_count") == 0
        && !source_bool("activation_command_result_receipt_shape_registered")
        && !source_bool("activation_command_result_receipt_allowed")
        && !source_bool("activation_command_result_receipt_schema_accepted")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_command_result_receipt_materialized")
        && !source_bool("activation_command_result_receipt_filesystem_written")
        && !source_bool("activation_command_result_receipt_ledger_written")
        && !source_bool("activation_command_result_receipt_indexed")
        && !source_bool("activation_command_result_receipt_enqueued")
        && !source_bool("activation_command_result_receipt_delivered")
        && !source_bool("activation_command_result_receipt_exported")
        && !source_bool("activation_command_result_receipt_query_registered")
        && !source_bool("activation_command_result_receipt_observability_recorded")
        && !source_bool("activation_command_completion_ack_recorded")
        && !source_bool("activation_command_completion_ack_persisted")
        && !source_bool("activation_command_completion_ack_accepted")
        && !source_bool("operator_approval_from_receipt_accepted")
        && !source_bool("activation_from_receipt_allowed")
        && !source_bool("activation_command_enabled")
        && !source_bool("activation_command_invoked")
        && !source_bool("activation_command_dispatched")
        && !source_bool("activation_request_accepted")
        && !source_bool("activation_request_recorded")
        && !source_bool("activation_request_executed")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("runtime_attachment_performed")
        && !source_bool("live_context_attached")
        && !source_bool("context_injection_performed")
        && !source_bool("adapter_invoked")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("auth_secret_read")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("usage_recorded")
        && !source_bool("memory_store_write_performed")
        && !source_bool("memory_store_mutated")
        && !source_bool("live_kg_write_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_no_persistence_ready;

    let replay_fixture = |id: &str, status: &str, reason: &str, extra: serde_json::Value| {
        let mut fixture = serde_json::Map::new();
        fixture.insert("id".to_string(), serde_json::Value::String(id.to_string()));
        fixture.insert(
            "replay_status".to_string(),
            serde_json::Value::String(status.to_string()),
        );
        fixture.insert(
            "reason".to_string(),
            serde_json::Value::String(reason.to_string()),
        );
        for key in [
            "source_no_persistence_present",
            "source_no_persistence_ready",
            "replay_requested",
            "canonical_noop_result_receipt_identity_required",
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
            "activation_command_completion_ack_materialized",
            "activation_command_completion_ack_delivered",
            "operator_approval_from_replay_accepted",
            "operator_approval_from_receipt_accepted",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_shape_registered",
            "activation_command_allowed",
            "activation_command_accepted",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
            "activation_command_noop_decision_recorded",
            "activation_command_noop_decision_persisted",
            "activation_command_handoff_recorded",
            "activation_command_handoff_persisted",
            "activation_request_allowed",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "activation_activated",
            "operator_approval_recorded",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "replay_ledger_written",
            "replay_indexed",
            "replay_query_registered",
            "replay_observability_recorded",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
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
        ] {
            fixture.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        let mut fixture = serde_json::Value::Object(fixture);
        extend_json_object(&mut fixture, extra);
        fixture
    };
    let replay_idempotency_fixtures = serde_json::Value::Array(vec![
        replay_fixture(
            "provider-router-activation-command-result-receipt-replay-missing-source-no-persistence-report",
            "blocked_noop",
            "source_result_receipt_no_persistence_report_required",
            serde_json::json!({
                "source_no_persistence_present": false,
                "source_no_persistence_ready": false,
            }),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-duplicate-identity-replay-attempt",
            "blocked_duplicate_noop",
            "duplicate_result_receipt_identity_replay_denied",
            serde_json::json!({"duplicate_result_receipt_identity_requested": true}),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-replay-acceptance-attempt",
            "blocked_replay_noop",
            "result_receipt_replay_acceptance_denied",
            serde_json::json!({"result_receipt_replay_acceptance_requested": true}),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-idempotency-key-recording-attempt",
            "blocked_idempotency_key_noop",
            "idempotency_key_recording_denied",
            serde_json::json!({
                "idempotency_key_acceptance_requested": true,
                "idempotency_key_recording_requested": true,
            }),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-idempotency-state-persistence-attempt",
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
            "provider-router-activation-command-result-receipt-cross-scope-reuse-attempt",
            "blocked_cross_scope_noop",
            "cross_scope_result_receipt_reuse_denied",
            serde_json::json!({"cross_scope_reuse_requested": true}),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-stale-nonce-out-of-order-replay-attempt",
            "blocked_nonce_order_noop",
            "stale_nonce_out_of_order_receipt_replay_denied",
            serde_json::json!({
                "stale_nonce_replay_requested": true,
                "out_of_order_replay_requested": true,
                "replay_nonce_acceptance_requested": true,
            }),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-completion-ledger-delivery-replay-attempt",
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
            "provider-router-activation-command-result-receipt-runtime-provider-memory-kg-replay-attempt",
            "blocked_runtime_provider_memory_kg_noop",
            "runtime_provider_memory_kg_replay_denied",
            serde_json::json!({
                "runtime_replay_requested": true,
                "provider_replay_requested": true,
                "model_replay_requested": true,
                "usage_replay_requested": true,
                "memory_store_replay_requested": true,
                "live_kg_replay_requested": true,
            }),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-restart-active-binary-replay-attempt",
            "blocked_external_noop",
            "external_public_install_restart_active_binary_replay_denied",
            serde_json::json!({
                "external_send_replay_requested": true,
                "public_claim_replay_requested": true,
                "public_ga_replay_requested": true,
                "release_artifact_replay_requested": true,
                "install_replay_requested": true,
                "launchd_restart_replay_requested": true,
                "service_restart_replay_requested": true,
                "active_binary_mutation_replay_requested": true,
            }),
        ),
    ]);
    let replay_idempotency_fixture_count = replay_idempotency_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let denials: Vec<serde_json::Value> = [
        "source_result_receipt_no_persistence_report_required",
        "canonical_noop_result_receipt_identity_required",
        "duplicate_result_receipt_identity_replay_denied",
        "result_receipt_replay_acceptance_denied",
        "idempotency_key_recording_denied",
        "idempotency_state_recording_denied",
        "idempotency_state_persistence_denied",
        "idempotency_state_materialization_denied",
        "idempotency_filesystem_write_denied",
        "cross_scope_result_receipt_reuse_denied",
        "stale_nonce_replay_denied",
        "out_of_order_receipt_replay_denied",
        "completion_ack_replay_denied",
        "activation_from_replay_denied",
        "runtime_router_replay_denied",
        "live_context_replay_denied",
        "context_injection_replay_denied",
        "adapter_invocation_replay_denied",
        "provider_model_replay_denied",
        "usage_record_replay_denied",
        "memory_store_replay_denied",
        "live_kg_replay_denied",
        "secret_material_replay_denied",
        "external_send_replay_denied",
        "public_claim_replay_denied",
        "install_restart_active_binary_replay_denied",
    ]
    .into_iter()
    .map(|item| serde_json::Value::String(item.to_string()))
    .collect();
    let denied_count = denials.len();
    let source_report_sha256 = sha256_json_value(&source_no_persistence);
    let fixture_hash = sha256_json_value(&replay_idempotency_fixtures);
    let contract_hash = sha256_text_value(&format!(
        "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial:native:source={source_report_sha256}:fixtures={fixture_hash}:route_count={}:replay=0:duplicate=0:idempotency=0:activation=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial:report-only:no-duplicate:no-replay:no-idempotency-record:no-persist:no-runtime:no-provider:no-model:no-secret-read",
    );

    let mut report = source_no_persistence.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial --json",
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-30",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_no_persistence_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            "source_activation_command_result_receipt_no_persistence_gate": source_str("gate"),
            "source_activation_command_result_receipt_no_persistence_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-route-gate.sh",
            "source_activation_command_result_receipt_replay_idempotency_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
            "source_activation_command_result_receipt_replay_idempotency_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-route-gate.sh",
            "source_activation_command_result_receipt_no_persistence_report_sha256": source_report_sha256,
            "replay_idempotency_fixtures_sha256": fixture_hash,
            "replay_idempotency_contract_hash_sha256": contract_hash,
            "replay_idempotency_policy_hash_sha256": policy_hash,
            "minimum_required_samples": 24,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_activation_command_result_receipt_no_persistence_ready": source_no_persistence_ready,
            "source_activation_command_result_receipt_no_persistence_status": source_str("runtime_provider_router_activation_command_result_receipt_no_persistence_status"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_status": source_str("runtime_provider_router_activation_command_result_receipt_no_persistence_status"),
            "runtime_provider_router_activation_command_noop_handoff_ready": source_bool("runtime_provider_router_activation_command_noop_handoff_ready"),
            "runtime_provider_router_activation_command_noop_handoff_status": source_str("runtime_provider_router_activation_command_noop_handoff_status"),
            "operator_authorization_source": "telegram_direct_operator_authorization_2026_06_30_14_26_asia_shanghai",
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status": "blocked",
            "activation_command_result_receipt_replay_idempotency_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_v1",
            "activation_command_result_receipt_replay_idempotency_mode": "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_no_duplicate_no_replay_no_idempotency_persist",
            "activation_command_result_receipt_replay_idempotency_decision": "runtime_provider_router_activation_command_result_receipt_cannot_be_replayed_duplicated_or_converted_into_idempotency_authority",
            "source_activation_command_result_receipt_fixture_count": source_u64("activation_command_result_receipt_fixture_count"),
            "source_blocked_activation_command_result_receipt_fixture_count": source_u64("blocked_activation_command_result_receipt_fixture_count"),
            "source_noop_activation_command_result_receipt_fixture_count": source_u64("noop_activation_command_result_receipt_fixture_count"),
            "source_accepted_activation_command_result_receipt_fixture_count": source_u64("accepted_activation_command_result_receipt_fixture_count"),
            "source_activation_command_result_receipt_performed_count": source_u64("activation_command_result_receipt_performed_count"),
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
            "receipt_replay_acceptance_fixture_count": 1,
            "idempotency_key_recording_fixture_count": 1,
            "idempotency_state_persistence_fixture_count": 1,
            "cross_scope_result_receipt_reuse_fixture_count": 1,
            "nonce_order_replay_fixture_count": 1,
            "completion_ack_replay_fixture_count": 1,
            "runtime_provider_memory_kg_replay_fixture_count": 1,
            "external_public_install_replay_fixture_count": 1,
            "replay_idempotency_denied_count": replay_idempotency_fixture_count,
            "duplicate_result_receipt_denied_count": replay_idempotency_fixture_count,
            "idempotency_state_denied_count": replay_idempotency_fixture_count,
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
            "operator_approval_from_replay_accepted": false,
            "activation_from_replay_allowed": false,
            "operator_approval_recorded": false,
        }),
    );
    for key in [
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
        "activation_command_completion_ack_materialized",
        "activation_command_completion_ack_delivered",
        "operator_approval_from_receipt_accepted",
        "activation_from_receipt_allowed",
        "activation_command_shape_registered",
        "activation_command_allowed",
        "activation_command_accepted",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_dispatch_performed",
        "activation_command_noop_decision_recorded",
        "activation_command_noop_decision_persisted",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_request_allowed",
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
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "usage_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "replay_ledger_written",
        "replay_indexed",
        "replay_query_registered",
        "replay_observability_recorded",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "readback_evidence_recorded",
        "readback_evidence_persisted",
        "router_handoff_recorded",
        "router_handoff_persisted",
        "rollback_executed",
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
    ] {
        if let Some(report) = report.as_object_mut() {
            report.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "replay_idempotency_surfaces": [
                "source_result_receipt_no_persistence_report_required",
                "canonical_noop_result_receipt_identity_required",
                "duplicate_receipt_rejection_required",
                "replay_request_rejection_required",
                "idempotency_key_state_recording_denied",
                "idempotency_persistence_materialization_denied",
                "cross_scope_receipt_reuse_denied",
                "nonce_order_freshness_replay_denied",
                "completion_ack_replay_denied",
                "activation_from_replay_denied",
                "runtime_router_live_context_replay_denied",
                "adapter_provider_model_replay_denied",
                "usage_memory_kg_replay_denied",
                "external_public_install_restart_active_binary_replay_denied"
            ],
            "denied_by_replay_idempotency": denials,
            "denied_by_replay_idempotency_count": denied_count,
            "denied_by_activation_command_result_receipt_replay_idempotency": denials,
            "denied_by_activation_command_result_receipt_replay_idempotency_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial",
                    "status": "allowed_report_only",
                    "accepts_duplicate_receipt": false,
                    "records_idempotency": false,
                    "persists_replay_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_out_of_order_receipt": false,
                    "records_monotonic_clock": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "accepts_duplicate_receipt": false,
                    "persists_replay_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_result_receipt_no_persistence_report_required": true,
            "duplicate_result_receipt_acceptance_forbidden": true,
            "result_receipt_replay_acceptance_forbidden": true,
            "idempotency_key_recording_forbidden": true,
            "idempotency_state_persistence_forbidden": true,
            "cross_scope_receipt_reuse_forbidden": true,
            "completion_ack_replay_forbidden": true,
            "activation_from_replay_forbidden": true,
            "runtime_provider_memory_kg_replay_forbidden": true,
            "secret_read_forbidden": true,
            "external_public_install_restart_active_binary_replay_forbidden": true,
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
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "replay_ledger_written",
            "replay_indexed",
            "replay_query_registered",
            "replay_observability_recorded",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
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
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_replay =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_report();
    let source_bool = |key: &str| {
        source_replay
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_replay
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_replay
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_replay_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status",
        ) == "blocked"
        && source_u64("accepted_replay_idempotency_fixture_count") == 0
        && source_u64("replay_idempotency_performed_count") == 0
        && source_u64("idempotency_state_recorded_count") == 0;
    let report_ready = source_replay_ready && route_count_source_command_accepted;

    let ordering_fixture = |fixture_id: &str,
                            status: &str,
                            reason: &str,
                            extra: serde_json::Value| {
        let mut fixture = serde_json::Map::new();
        fixture.insert(
            "fixture_id".to_string(),
            serde_json::Value::String(fixture_id.to_string()),
        );
        fixture.insert(
            "id".to_string(),
            serde_json::Value::String(fixture_id.to_string()),
        );
        fixture.insert(
            "ordering_monotonicity_status".to_string(),
            serde_json::Value::String(status.to_string()),
        );
        fixture.insert(
            "ordering_status".to_string(),
            serde_json::Value::String(status.to_string()),
        );
        fixture.insert(
            "denial_reason".to_string(),
            serde_json::Value::String(reason.to_string()),
        );
        for key in [
            "ordering_requested",
            "source_replay_idempotency_present",
            "source_replay_idempotency_ready",
            "canonical_noop_result_receipt_order_identity_required",
            "receipt_noop_confirmed",
        ] {
            fixture.insert(key.to_string(), serde_json::Value::Bool(true));
        }
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
            "activation_command_result_receipt_timestamp_ordering_accepted",
            "activation_command_result_receipt_epoch_ordering_accepted",
            "activation_command_result_receipt_stage_ordering_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_gap_fill_accepted",
            "activation_command_result_receipt_ack_before_noop_accepted",
            "activation_command_result_receipt_ledger_ordering_bypass_accepted",
            "activation_command_result_receipt_index_ordering_bypass_accepted",
            "activation_command_result_receipt_delivery_ordering_bypass_accepted",
            "activation_command_result_receipt_runtime_ordering_bypass_accepted",
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
            "activation_command_result_receipt_ledger_written",
            "activation_command_result_receipt_indexed",
            "activation_command_result_receipt_enqueued",
            "activation_command_result_receipt_delivered",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "operator_approval_from_ordering_accepted",
            "activation_from_ordering_allowed",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
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
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "ordering_ledger_written",
            "ordering_indexed",
            "ordering_query_registered",
            "ordering_observability_recorded",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
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
        ] {
            fixture.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        let mut fixture = serde_json::Value::Object(fixture);
        extend_json_object(&mut fixture, extra);
        fixture
    };
    let ordering_monotonicity_fixtures = serde_json::Value::Array(vec![
        ordering_fixture(
            "provider-router-activation-command-result-receipt-ordering-missing-source-replay-idempotency-report",
            "blocked_noop",
            "source_result_receipt_replay_idempotency_report_required",
            serde_json::json!({
                "source_replay_idempotency_present": false,
                "source_replay_idempotency_ready": false,
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-sequence-cursor-recording-attempt",
            "blocked_ordering_noop",
            "sequence_cursor_recording_denied",
            serde_json::json!({
                "sequence_cursor_recording_requested": true,
                "requested_sequence_cursor": "provider_router_activation_receipt_sequence_1",
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-out-of-order-sequence-attempt",
            "blocked_ordering_noop",
            "out_of_order_result_receipt_sequence_denied",
            serde_json::json!({
                "out_of_order_sequence_requested": true,
                "requested_sequence": 2,
                "observed_previous_sequence": 3,
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-sequence-gap-skip-attempt",
            "blocked_ordering_noop",
            "sequence_gap_or_skip_result_receipt_denied",
            serde_json::json!({
                "sequence_gap_requested": true,
                "requested_sequence": 5,
                "expected_next_sequence": 1,
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-timestamp-rollback-attempt",
            "blocked_ordering_noop",
            "timestamp_rollback_result_receipt_denied",
            serde_json::json!({
                "timestamp_rollback_requested": true,
                "requested_timestamp_order": "older_than_source_replay_idempotency_report",
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-epoch-rollback-attempt",
            "blocked_ordering_noop",
            "epoch_rollback_result_receipt_denied",
            serde_json::json!({
                "epoch_rollback_requested": true,
                "requested_epoch_order": "lower_than_current_activation_epoch",
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-same-sequence-different-hash-attempt",
            "blocked_ordering_noop",
            "same_sequence_different_hash_result_receipt_denied",
            serde_json::json!({
                "same_sequence_different_hash_requested": true,
                "requested_sequence": 1,
                "requested_hash_relation": "different_hash_for_same_sequence",
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-latest-wins-overwrite-attempt",
            "blocked_ordering_noop",
            "latest_wins_result_receipt_overwrite_denied",
            serde_json::json!({
                "latest_wins_overwrite_requested": true,
                "overwrite_existing_noop_requested": true,
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-stage-ledger-index-delivery-ordering-bypass-attempt",
            "blocked_ordering_noop",
            "stage_ledger_index_delivery_ordering_bypass_denied",
            serde_json::json!({
                "stage_transition_ordering_bypass_requested": true,
                "completion_ack_before_noop_requested": true,
                "ledger_ordering_bypass_requested": true,
                "index_ordering_bypass_requested": true,
                "delivery_ordering_bypass_requested": true,
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-runtime-provider-memory-kg-external-ordering-bypass-attempt",
            "blocked_ordering_noop",
            "runtime_provider_memory_kg_external_ordering_bypass_denied",
            serde_json::json!({
                "runtime_ordering_bypass_requested": true,
                "provider_ordering_bypass_requested": true,
                "model_ordering_bypass_requested": true,
                "memory_store_ordering_bypass_requested": true,
                "live_kg_ordering_bypass_requested": true,
                "external_send_ordering_bypass_requested": true,
                "public_claim_ordering_bypass_requested": true,
                "install_ordering_bypass_requested": true,
                "service_restart_ordering_bypass_requested": true,
                "active_binary_mutation_ordering_bypass_requested": true,
            }),
        ),
    ]);
    let ordering_monotonicity_fixture_count = ordering_monotonicity_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixture_hash = sha256_json_value(&ordering_monotonicity_fixtures);
    let source_replay_hash = sha256_json_value(&source_replay);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial:v1:source={source_replay_hash}:fixtures={fixture_hash}:ordering=0:cursor=0:monotonicity=0:persist=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial:v1:no-ordering:no-sequence-cursor:no-monotonicity-state:no-latest-wins:no-stage-bypass:no-runtime-provider-model-memory-kg-external-install-restart-binary-public-authority",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-ordering-monotonicity-side-effects=false;fixtures=10;ordering=0;cursor=0;monotonicity=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );
    let denials = vec![
        "source_result_receipt_replay_idempotency_report_required",
        "canonical_noop_result_receipt_order_identity_required",
        "sequence_cursor_acceptance_denied",
        "sequence_cursor_recording_denied",
        "sequence_cursor_persistence_denied",
        "monotonicity_state_recording_denied",
        "monotonicity_state_persistence_denied",
        "monotonicity_state_materialization_denied",
        "monotonicity_filesystem_write_denied",
        "out_of_order_sequence_denied",
        "sequence_gap_or_skip_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "completion_ack_before_noop_denied",
        "stage_transition_ordering_bypass_denied",
        "ledger_index_delivery_ordering_bypass_denied",
        "runtime_router_ordering_bypass_denied",
        "context_injection_ordering_bypass_denied",
        "provider_model_ordering_bypass_denied",
        "memory_kg_ordering_bypass_denied",
        "credential_secret_ordering_bypass_denied",
        "external_public_install_restart_ordering_bypass_denied",
        "active_binary_mutation_ordering_bypass_denied",
        "activation_from_ordering_denied",
    ];
    let denied_count = denials.len();

    let mut report = source_replay.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status",
            "side_effect_free": true,
            "source_activation_command_result_receipt_replay_idempotency_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_replay_idempotency_gate": source_str("gate"),
            "source_activation_command_result_receipt_replay_idempotency_ready": source_replay_ready,
            "source_activation_command_result_receipt_replay_idempotency_status": source_str("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status"),
            "source_activation_command_result_receipt_replay_idempotency_report_sha256": source_replay_hash,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status": "blocked",
            "activation_command_result_receipt_ordering_monotonicity_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_v1",
            "activation_command_result_receipt_ordering_monotonicity_mode": "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_no_ordering_no_monotonicity_persist",
            "activation_command_result_receipt_ordering_monotonicity_decision": "runtime_provider_router_activation_command_result_receipt_cannot_create_ordering_sequence_cursor_or_monotonicity_authority",
            "minimum_required_samples": 24,
            "ordering_monotonicity_fixtures_sha256": fixture_hash,
            "ordering_monotonicity_contract_hash_sha256": contract_hash,
            "ordering_monotonicity_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_replay_idempotency_fixture_count": source_u64("replay_idempotency_fixture_count"),
            "source_blocked_replay_idempotency_fixture_count": source_u64("blocked_replay_idempotency_fixture_count"),
            "source_accepted_replay_idempotency_fixture_count": source_u64("accepted_replay_idempotency_fixture_count"),
            "ordering_monotonicity_surface_count": 14,
            "ordering_monotonicity_surface_ready_count": 14,
            "ordering_monotonicity_side_effect_free_surface_count": 14,
            "ordering_monotonicity_fixtures": ordering_monotonicity_fixtures,
            "ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "blocked_ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "noop_ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "allowed_ordering_monotonicity_fixture_count": 0,
            "accepted_ordering_monotonicity_fixture_count": 0,
            "ordering_monotonicity_denied_count": ordering_monotonicity_fixture_count,
            "ordering_monotonicity_performed_count": 0,
            "sequence_cursor_accepted_count": 0,
            "sequence_cursor_recorded_count": 0,
            "sequence_cursor_persisted_count": 0,
            "monotonicity_state_recorded_count": 0,
            "monotonicity_state_persisted_count": 0,
            "denied_by_ordering_monotonicity": denials,
            "denied_by_ordering_monotonicity_count": denied_count,
            "denied_by_activation_command_result_receipt_ordering_monotonicity": denials,
            "denied_by_activation_command_result_receipt_ordering_monotonicity_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial",
                    "status": "allowed_report_only",
                    "accepts_out_of_order_receipt": false,
                    "records_monotonic_clock": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "persists_replacement_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "accepts_ordering": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
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
            "activation_command_result_receipt_timestamp_ordering_accepted",
            "activation_command_result_receipt_epoch_ordering_accepted",
            "activation_command_result_receipt_stage_ordering_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_gap_fill_accepted",
            "activation_command_result_receipt_ack_before_noop_accepted",
            "activation_command_result_receipt_ledger_ordering_bypass_accepted",
            "activation_command_result_receipt_index_ordering_bypass_accepted",
            "activation_command_result_receipt_delivery_ordering_bypass_accepted",
            "activation_command_result_receipt_runtime_ordering_bypass_accepted",
            "activation_command_result_receipt_provider_ordering_bypass_accepted",
            "activation_command_result_receipt_memory_kg_ordering_bypass_accepted",
            "activation_command_result_receipt_external_public_install_ordering_bypass_accepted",
            "operator_approval_from_ordering_accepted",
            "activation_from_ordering_allowed",
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
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "ordering_ledger_written",
            "ordering_indexed",
            "ordering_query_registered",
            "ordering_observability_recorded",
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
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_from_ordering_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_executed",
            "runtime_router_mutated",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "external_send_performed",
            "install_executed",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_ordering =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_report();
    let source_bool = |key: &str| {
        source_ordering
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_ordering
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_ordering
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_ordering_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status",
        ) == "blocked"
        && source_u64("accepted_ordering_monotonicity_fixture_count") == 0
        && source_u64("ordering_monotonicity_performed_count") == 0
        && source_u64("sequence_cursor_recorded_count") == 0
        && source_u64("monotonicity_state_recorded_count") == 0;
    let report_ready = source_ordering_ready && route_count_source_command_accepted;

    let cancellation_supersession_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "cancellation_supersession_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_ordering_monotonicity_present",
                "source_ordering_monotonicity_ready",
                "canonical_noop_result_receipt_lifecycle_identity_required",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "cancellation_requested",
                "supersession_requested",
                "replacement_receipt_requested",
                "replacement_hash_requested",
                "tombstone_requested",
                "delete_marker_requested",
                "completion_ack_cancellation_requested",
                "ledger_cancellation_requested",
                "index_cancellation_requested",
                "delivery_cancellation_requested",
                "export_cancellation_requested",
                "query_cancellation_requested",
                "observability_cancellation_requested",
                "runtime_router_supersession_requested",
                "provider_supersession_requested",
                "model_supersession_requested",
                "memory_store_supersession_requested",
                "live_kg_supersession_requested",
                "rollback_supersession_requested",
                "secret_material_supersession_requested",
                "external_send_supersession_requested",
                "public_claim_supersession_requested",
                "install_supersession_requested",
                "service_restart_supersession_requested",
                "active_binary_mutation_supersession_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            for key in [
                "activation_command_result_receipt_cancellation_allowed",
                "activation_command_result_receipt_cancellation_recorded",
                "activation_command_result_receipt_cancellation_persisted",
                "activation_command_result_receipt_cancellation_request_accepted",
                "activation_command_result_receipt_supersession_allowed",
                "activation_command_result_receipt_supersession_recorded",
                "activation_command_result_receipt_supersession_persisted",
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
                "activation_from_cancellation_allowed",
                "activation_from_supersession_allowed",
                "activation_from_ordering_allowed",
                "activation_from_replay_allowed",
                "activation_from_receipt_allowed",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_command_dispatch_performed",
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
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let cancellation_supersession_fixtures = serde_json::Value::Array(vec![
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-cancellation-missing-source-ordering-report",
            "blocked_noop",
            "source_ordering_monotonicity_report_required",
            serde_json::json!({
                "source_ordering_monotonicity_present": false,
                "source_ordering_monotonicity_ready": false,
                "cancellation_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-cancel-blocked-noop",
            "blocked_cancellation_noop",
            "cancel_after_blocked_noop_denied",
            serde_json::json!({
                "cancellation_requested": true,
                "cancel_after_blocked_noop_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-supersede-with-completed",
            "blocked_supersession_noop",
            "supersede_blocked_noop_with_completed_denied",
            serde_json::json!({
                "supersession_requested": true,
                "supersede_with_completed_receipt_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-replacement-hash",
            "blocked_replacement_noop",
            "replacement_receipt_hash_denied",
            serde_json::json!({
                "replacement_receipt_requested": true,
                "replacement_hash_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-tombstone-delete-marker",
            "blocked_tombstone_noop",
            "tombstone_delete_marker_denied",
            serde_json::json!({
                "tombstone_requested": true,
                "delete_marker_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-completion-ack-cancel",
            "blocked_cancellation_noop",
            "completion_ack_cancellation_denied",
            serde_json::json!({
                "cancellation_requested": true,
                "completion_ack_cancellation_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-ledger-index-delivery-export-cancel",
            "blocked_cancellation_noop",
            "ledger_index_delivery_export_cancellation_denied",
            serde_json::json!({
                "cancellation_requested": true,
                "ledger_cancellation_requested": true,
                "index_cancellation_requested": true,
                "delivery_cancellation_requested": true,
                "export_cancellation_requested": true,
                "query_cancellation_requested": true,
                "observability_cancellation_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-runtime-provider-model-supersede",
            "blocked_supersession_noop",
            "runtime_provider_model_supersession_denied",
            serde_json::json!({
                "supersession_requested": true,
                "runtime_router_supersession_requested": true,
                "provider_supersession_requested": true,
                "model_supersession_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-memory-kg-rollback-secret-supersede",
            "blocked_supersession_noop",
            "memory_kg_rollback_secret_supersession_denied",
            serde_json::json!({
                "supersession_requested": true,
                "memory_store_supersession_requested": true,
                "live_kg_supersession_requested": true,
                "rollback_supersession_requested": true,
                "secret_material_supersession_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-supersede",
            "blocked_supersession_noop",
            "external_public_install_supersession_denied",
            serde_json::json!({
                "supersession_requested": true,
                "external_send_supersession_requested": true,
                "public_claim_supersession_requested": true,
                "install_supersession_requested": true,
                "service_restart_supersession_requested": true,
                "active_binary_mutation_supersession_requested": true,
            }),
        ),
    ]);
    let cancellation_supersession_fixture_count = cancellation_supersession_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixture_hash = sha256_json_value(&cancellation_supersession_fixtures);
    let source_ordering_hash = sha256_json_value(&source_ordering);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial:v1:source={source_ordering_hash}:fixtures={fixture_hash}:cancel=0:supersede=0:replacement=0:tombstone=0:delete=0:persist=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial:v1:no-cancel:no-supersede:no-replacement:no-tombstone:no-delete:no-ack-cancel:no-ledger-index-delivery-export-query-observe:no-runtime-provider-model-memory-kg-external-install-restart-binary-public-authority",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-cancellation-supersession-side-effects=false;fixtures=10;cancel=0;supersede=0;replacement=0;tombstone=0;delete=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );
    let denials = vec![
        "source_ordering_monotonicity_report_required",
        "cancellation_request_acceptance_denied",
        "cancellation_recording_denied",
        "cancellation_persistence_denied",
        "supersession_request_acceptance_denied",
        "supersession_recording_denied",
        "supersession_persistence_denied",
        "replacement_receipt_acceptance_denied",
        "replacement_receipt_recording_denied",
        "replacement_receipt_persistence_denied",
        "replacement_hash_acceptance_denied",
        "tombstone_recording_denied",
        "delete_marker_recording_denied",
        "cancel_after_blocked_noop_denied",
        "supersede_blocked_noop_with_completed_denied",
        "completion_ack_cancellation_denied",
        "ledger_cancellation_denied",
        "index_cancellation_denied",
        "delivery_cancellation_denied",
        "export_query_observability_cancellation_denied",
        "runtime_router_supersession_denied",
        "live_context_supersession_denied",
        "adapter_provider_model_supersession_denied",
        "usage_memory_kg_supersession_denied",
        "rollback_secret_material_supersession_denied",
        "external_public_release_supersession_denied",
        "install_restart_active_binary_supersession_denied",
    ];
    let denied_count = denials.len();
    let denials_value = serde_json::Value::Array(
        denials
            .iter()
            .map(|denial| serde_json::Value::String((*denial).to_string()))
            .collect(),
    );

    let mut report = source_ordering.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status",
            "side_effect_free": true,
            "source_activation_command_result_receipt_ordering_monotonicity_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_ordering_monotonicity_gate": source_str("gate"),
            "source_activation_command_result_receipt_ordering_monotonicity_ready": source_ordering_ready,
            "source_activation_command_result_receipt_ordering_monotonicity_status": source_str("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status"),
            "source_activation_command_result_receipt_ordering_monotonicity_report_sha256": source_ordering_hash,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status": "blocked",
            "activation_command_result_receipt_cancellation_supersession_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_v1",
            "activation_command_result_receipt_cancellation_supersession_mode": "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_no_cancel_no_supersede_no_replacement_persist",
            "activation_command_result_receipt_cancellation_supersession_decision": "runtime_provider_router_activation_command_result_receipt_cannot_cancel_supersede_replace_tombstone_delete_or_derive_activation_authority",
            "minimum_required_samples": 24,
            "cancellation_supersession_fixtures_sha256": fixture_hash,
            "cancellation_supersession_contract_hash_sha256": contract_hash,
            "cancellation_supersession_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_ordering_monotonicity_fixture_count": source_u64("ordering_monotonicity_fixture_count"),
            "source_blocked_ordering_monotonicity_fixture_count": source_u64("blocked_ordering_monotonicity_fixture_count"),
            "source_accepted_ordering_monotonicity_fixture_count": source_u64("accepted_ordering_monotonicity_fixture_count"),
            "cancellation_supersession_surface_count": 14,
            "cancellation_supersession_surface_ready_count": 14,
            "cancellation_supersession_side_effect_free_surface_count": 14,
            "cancellation_supersession_surfaces": [
                "source_ordering_monotonicity_report_required",
                "cancellation_request_shape_denied",
                "supersession_request_shape_denied",
                "replacement_receipt_hash_denied",
                "tombstone_or_delete_marker_denied",
                "cancel_after_blocked_noop_denied",
                "supersede_blocked_noop_with_completed_denied",
                "acknowledgement_cancellation_denied",
                "ledger_index_delivery_export_query_observability_cancellation_denied",
                "runtime_router_live_context_supersession_denied",
                "adapter_provider_model_usage_supersession_denied",
                "memory_kg_rollback_secret_supersession_denied",
                "external_public_install_restart_active_binary_supersession_denied",
                "activation_authority_from_cancellation_supersession_denied"
            ],
            "cancellation_supersession_fixtures": cancellation_supersession_fixtures,
            "cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "blocked_cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "noop_cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "allowed_cancellation_supersession_fixture_count": 0,
            "accepted_cancellation_supersession_fixture_count": 0,
            "cancellation_fixture_count": 5,
            "supersession_fixture_count": 5,
            "cancellation_denied_count": 5,
            "supersession_denied_count": 5,
            "cancellation_performed_count": 0,
            "supersession_performed_count": 0,
            "replacement_receipt_accepted_count": 0,
            "replacement_receipt_recorded_count": 0,
            "replacement_receipt_persisted_count": 0,
            "tombstone_recorded_count": 0,
            "delete_marker_recorded_count": 0,
            "denied_by_cancellation_supersession": denials_value,
            "denied_by_cancellation_supersession_count": denied_count,
            "denied_by_activation_command_result_receipt_cancellation_supersession": denials_value,
            "denied_by_activation_command_result_receipt_cancellation_supersession_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial",
                    "status": "allowed_report_only",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "persists_replacement_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "persists_replacement_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_cancellation_allowed",
            "activation_command_result_receipt_cancellation_recorded",
            "activation_command_result_receipt_cancellation_persisted",
            "activation_command_result_receipt_cancellation_request_accepted",
            "activation_command_result_receipt_supersession_allowed",
            "activation_command_result_receipt_supersession_recorded",
            "activation_command_result_receipt_supersession_persisted",
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
            "activation_from_cancellation_allowed",
            "activation_from_supersession_allowed",
            "activation_from_ordering_allowed",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
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
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_cancellation_recorded",
            "activation_command_result_receipt_cancellation_persisted",
            "activation_command_result_receipt_supersession_recorded",
            "activation_command_result_receipt_supersession_persisted",
            "activation_command_result_receipt_replacement_receipt_recorded",
            "activation_command_result_receipt_replacement_receipt_persisted",
            "activation_command_result_receipt_tombstone_recorded",
            "activation_command_result_receipt_delete_marker_recorded",
            "activation_from_cancellation_allowed",
            "activation_from_supersession_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_executed",
            "runtime_router_mutated",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "external_send_performed",
            "install_executed",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_cancellation =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_report();
    let source_bool = |key: &str| {
        source_cancellation
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_cancellation
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_cancellation
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_cancellation_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status",
        ) == "blocked"
        && source_u64("accepted_cancellation_supersession_fixture_count") == 0
        && source_u64("cancellation_performed_count") == 0
        && source_u64("supersession_performed_count") == 0
        && source_u64("replacement_receipt_recorded_count") == 0
        && source_u64("replacement_receipt_persisted_count") == 0
        && source_u64("tombstone_recorded_count") == 0
        && source_u64("delete_marker_recorded_count") == 0;
    let report_ready = source_cancellation_ready && route_count_source_command_accepted;

    let audit_trail_immutable_evidence_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "audit_evidence_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_cancellation_supersession_present",
                "source_cancellation_supersession_ready",
                "audit_trail_requested",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "immutable_evidence_requested",
                "hash_chain_requested",
                "merkle_root_requested",
                "attestation_requested",
                "witness_requested",
                "notary_requested",
                "audit_trail_materialization_requested",
                "audit_trail_filesystem_write_requested",
                "ledger_evidence_requested",
                "index_evidence_requested",
                "delivery_evidence_requested",
                "activation_from_audit_evidence_requested",
                "memory_store_evidence_requested",
                "live_kg_evidence_requested",
                "rollback_evidence_requested",
                "secret_material_evidence_requested",
                "provider_prompt_evidence_requested",
                "external_send_evidence_requested",
                "public_claim_evidence_requested",
                "release_artifact_evidence_requested",
                "install_evidence_requested",
                "service_restart_evidence_requested",
                "active_binary_mutation_evidence_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
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
                "audit_trail_exported",
                "immutable_evidence_exported",
                "audit_evidence_query_registered",
                "audit_evidence_observability_recorded",
                "activation_command_result_receipt_cancellation_allowed",
                "activation_command_result_receipt_cancellation_recorded",
                "activation_command_result_receipt_cancellation_persisted",
                "activation_command_result_receipt_supersession_allowed",
                "activation_command_result_receipt_supersession_recorded",
                "activation_command_result_receipt_supersession_persisted",
                "replacement_receipt_accepted",
                "replacement_receipt_recorded",
                "replacement_receipt_persisted",
                "tombstone_recorded",
                "delete_marker_recorded",
                "activation_command_result_receipt_ordering_allowed",
                "activation_command_result_receipt_ordering_recorded",
                "activation_command_result_receipt_ordering_persisted",
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
                "operator_approval_from_audit_trail_accepted",
                "operator_approval_from_immutable_evidence_accepted",
                "activation_from_audit_trail_allowed",
                "activation_from_immutable_evidence_allowed",
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
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let audit_trail_immutable_evidence_fixtures = serde_json::Value::Array(vec![
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-audit-missing-source-cancellation-supersession-report",
            "blocked_noop",
            "source_cancellation_supersession_report_required",
            serde_json::json!({
                "source_cancellation_supersession_present": false,
                "source_cancellation_supersession_ready": false,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-audit-trail-append-request",
            "blocked_noop",
            "audit_trail_append_request_denied",
            serde_json::json!({
                "audit_trail_request_shape": "append_blocked_noop_result_receipt_audit_trail",
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-immutable-evidence-packet",
            "blocked_evidence_noop",
            "immutable_evidence_packet_request_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "immutable_evidence_request_shape": "seal_blocked_noop_result_receipt_as_immutable_evidence",
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-hash-chain-merkle-root",
            "blocked_evidence_noop",
            "hash_chain_merkle_root_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "hash_chain_requested": true,
                "merkle_root_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-attestation-witness-notary",
            "blocked_evidence_noop",
            "attestation_witness_notary_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "attestation_requested": true,
                "witness_requested": true,
                "notary_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-audit-trail-materialization",
            "blocked_noop",
            "audit_trail_materialization_filesystem_denied",
            serde_json::json!({
                "audit_trail_materialization_requested": true,
                "audit_trail_filesystem_write_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-ledger-index-delivery-evidence",
            "blocked_noop",
            "ledger_index_delivery_evidence_denied",
            serde_json::json!({
                "ledger_evidence_requested": true,
                "index_evidence_requested": true,
                "delivery_evidence_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-activation-from-audit-evidence",
            "blocked_evidence_noop",
            "activation_from_audit_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "activation_from_audit_evidence_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-memory-kg-rollback-secret-provider-evidence",
            "blocked_evidence_noop",
            "memory_kg_rollback_secret_provider_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "memory_store_evidence_requested": true,
                "live_kg_evidence_requested": true,
                "rollback_evidence_requested": true,
                "secret_material_evidence_requested": true,
                "provider_prompt_evidence_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-evidence",
            "blocked_evidence_noop",
            "external_public_install_restart_active_binary_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "external_send_evidence_requested": true,
                "public_claim_evidence_requested": true,
                "release_artifact_evidence_requested": true,
                "install_evidence_requested": true,
                "service_restart_evidence_requested": true,
                "active_binary_mutation_evidence_requested": true,
            }),
        ),
    ]);
    let audit_fixture_count = audit_trail_immutable_evidence_fixtures
        .as_array()
        .map(std::vec::Vec::len)
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
    let fixtures_hash = sha256_json_value(&audit_trail_immutable_evidence_fixtures);
    let source_cancellation_hash = sha256_json_value(&source_cancellation);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial:v1:source={source_cancellation_hash}:fixtures={fixtures_hash}:audit=0:evidence=0:hash=0:attestation=0:record=0:persist=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial:v1:no-audit-write:no-evidence-persist:no-hash-chain:no-merkle-root:no-attestation:no-witness:no-notary:no-ledger-index-delivery:no-provider-model-memory-kg-secret-external-install-restart-binary-public-authority",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-audit-trail-immutable-evidence-side-effects=false;fixtures=10;audit=0;evidence=0;hash=0;attestation=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0;external=0;install=0",
    );
    let denials = vec![
        "source_cancellation_supersession_report_required",
        "audit_trail_request_acceptance_denied",
        "audit_trail_recording_denied",
        "audit_trail_persistence_denied",
        "audit_trail_materialization_denied",
        "immutable_evidence_request_acceptance_denied",
        "immutable_evidence_recording_denied",
        "immutable_evidence_persistence_denied",
        "immutable_evidence_materialization_denied",
        "hash_chain_recording_denied",
        "merkle_root_recording_denied",
        "attestation_recording_denied",
        "witness_recording_denied",
        "notary_recording_denied",
        "ledger_evidence_recording_denied",
        "index_evidence_recording_denied",
        "delivery_evidence_recording_denied",
        "activation_from_audit_evidence_denied",
        "memory_store_evidence_denied",
        "live_kg_evidence_denied",
        "rollback_evidence_denied",
        "secret_material_evidence_denied",
        "provider_prompt_evidence_denied",
        "external_public_install_restart_active_binary_evidence_denied",
    ];
    let denied_count = denials.len();
    let denials_value = serde_json::Value::Array(
        denials
            .iter()
            .map(|denial| serde_json::Value::String((*denial).to_string()))
            .collect(),
    );

    let mut report = source_cancellation.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status",
            "side_effect_free": true,
            "source_activation_command_result_receipt_cancellation_supersession_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_cancellation_supersession_gate": source_str("gate"),
            "source_activation_command_result_receipt_cancellation_supersession_ready": source_cancellation_ready,
            "source_activation_command_result_receipt_cancellation_supersession_status": source_str("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status"),
            "source_activation_command_result_receipt_cancellation_supersession_report_sha256": source_cancellation_hash,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_audit_trail_immutable_evidence_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_v1",
            "activation_command_result_receipt_audit_trail_immutable_evidence_mode": "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_no_audit_write_no_evidence_persist",
            "activation_command_result_receipt_audit_trail_immutable_evidence_decision": "runtime_provider_router_activation_command_result_receipt_cannot_be_wrapped_as_audit_trail_or_immutable_evidence_authority",
            "minimum_required_samples": 24,
            "audit_trail_immutable_evidence_fixtures_sha256": fixtures_hash,
            "audit_trail_immutable_evidence_contract_hash_sha256": contract_hash,
            "audit_trail_immutable_evidence_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_cancellation_supersession_fixture_count": source_u64("cancellation_supersession_fixture_count"),
            "source_blocked_cancellation_supersession_fixture_count": source_u64("blocked_cancellation_supersession_fixture_count"),
            "source_noop_cancellation_supersession_fixture_count": source_u64("noop_cancellation_supersession_fixture_count"),
            "source_accepted_cancellation_supersession_fixture_count": source_u64("accepted_cancellation_supersession_fixture_count"),
            "source_cancellation_performed_count": source_u64("cancellation_performed_count"),
            "source_supersession_performed_count": source_u64("supersession_performed_count"),
            "source_replacement_receipt_recorded_count": source_u64("replacement_receipt_recorded_count"),
            "source_replacement_receipt_persisted_count": source_u64("replacement_receipt_persisted_count"),
            "source_tombstone_recorded_count": source_u64("tombstone_recorded_count"),
            "source_delete_marker_recorded_count": source_u64("delete_marker_recorded_count"),
            "cancellation_supersession_surface_count": source_u64("cancellation_supersession_surface_count"),
            "cancellation_supersession_surface_ready_count": source_u64("cancellation_supersession_surface_ready_count"),
            "audit_trail_immutable_evidence_surface_count": 12,
            "audit_trail_immutable_evidence_surface_ready_count": 12,
            "audit_trail_immutable_evidence_side_effect_free_surface_count": 12,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "audit_trail_immutable_evidence_surfaces": [
                "source_cancellation_supersession_report_required",
                "audit_trail_request_shape_denied",
                "immutable_evidence_request_shape_denied",
                "append_only_audit_log_recording_denied",
                "evidence_hash_chain_recording_denied",
                "attestation_witness_notary_recording_denied",
                "audit_trail_materialization_denied",
                "immutable_evidence_persistence_denied",
                "ledger_index_delivery_evidence_denied",
                "activation_from_audit_evidence_denied",
                "memory_kg_rollback_secret_provider_evidence_denied",
                "external_public_install_restart_active_binary_evidence_denied"
            ],
            "audit_trail_immutable_evidence_fixtures": audit_trail_immutable_evidence_fixtures,
            "audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "blocked_audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "noop_audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "allowed_audit_trail_immutable_evidence_fixture_count": 0,
            "accepted_audit_trail_immutable_evidence_fixture_count": 0,
            "audit_trail_denied_count": audit_fixture_count,
            "immutable_evidence_denied_count": immutable_evidence_denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "audit_trail_performed_count": 0,
            "immutable_evidence_performed_count": 0,
            "audit_trail_recorded_count": 0,
            "audit_trail_persisted_count": 0,
            "immutable_evidence_recorded_count": 0,
            "immutable_evidence_persisted_count": 0,
            "hash_chain_recorded_count": 0,
            "merkle_root_recorded_count": 0,
            "attestation_recorded_count": 0,
            "witness_recorded_count": 0,
            "notary_recorded_count": 0,
            "ledger_evidence_recorded_count": 0,
            "index_evidence_recorded_count": 0,
            "delivery_evidence_recorded_count": 0,
            "denied_by_audit_trail_immutable_evidence": denials_value,
            "denied_by_audit_trail_immutable_evidence_count": denied_count,
            "denied_by_activation_command_result_receipt_audit_trail_immutable_evidence": denials_value,
            "denied_by_activation_command_result_receipt_audit_trail_immutable_evidence_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
                    "status": "allowed_report_only",
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
                    "status": "allowed_report_only_next_slice",
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "performs_retention": false,
                    "performs_gc": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_cancellation_supersession_report_required": true,
            "audit_trail_acceptance_forbidden": true,
            "audit_trail_recording_forbidden": true,
            "audit_trail_persistence_forbidden": true,
            "immutable_evidence_acceptance_forbidden": true,
            "immutable_evidence_recording_forbidden": true,
            "immutable_evidence_persistence_forbidden": true,
            "hash_chain_or_merkle_root_recording_forbidden": true,
            "attestation_witness_notary_recording_forbidden": true,
            "runtime_provider_memory_kg_evidence_forbidden": true,
            "secret_read_forbidden": true,
            "external_public_install_restart_active_binary_evidence_forbidden": true,
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
            "operator_approval_from_audit_trail_accepted",
            "operator_approval_from_immutable_evidence_accepted",
            "activation_from_audit_trail_allowed",
            "activation_from_immutable_evidence_allowed",
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
            "activation_command_dispatch_performed",
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
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        if !report_object
            .get("side_effects")
            .is_some_and(serde_json::Value::is_object)
        {
            report_object.insert("side_effects".to_string(), serde_json::json!({}));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
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
            "activation_command_result_receipt_ledger_evidence_persisted",
            "activation_command_result_receipt_index_evidence_recorded",
            "activation_command_result_receipt_index_evidence_persisted",
            "activation_command_result_receipt_delivery_evidence_recorded",
            "activation_command_result_receipt_delivery_evidence_persisted",
            "activation_from_audit_trail_allowed",
            "activation_from_immutable_evidence_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
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
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
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
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report();
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let source_hash_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status",
        ) == "blocked"
        && source_u64("accepted_audit_trail_immutable_evidence_fixture_count") == 0
        && source_u64("audit_trail_performed_count") == 0
        && source_u64("immutable_evidence_performed_count") == 0
        && source_u64("audit_trail_recorded_count") == 0
        && source_u64("audit_trail_persisted_count") == 0
        && source_u64("immutable_evidence_recorded_count") == 0
        && source_u64("immutable_evidence_persisted_count") == 0
        && source_u64("hash_chain_recorded_count") == 0
        && source_u64("merkle_root_recorded_count") == 0
        && source_u64("attestation_recorded_count") == 0
        && source_u64("witness_recorded_count") == 0
        && source_u64("notary_recorded_count") == 0;
    let report_ready = source_ready && route_count_source_command_accepted;

    let retention_gc_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "retention_gc_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_audit_evidence_present",
                "source_audit_evidence_ready",
                "source_audit_trail_immutable_evidence_present",
                "source_audit_trail_immutable_evidence_ready",
                "retention_requested",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "expiry_requested",
                "garbage_collection_requested",
                "retention_index_requested",
                "expiry_schedule_requested",
                "expiry_timer_requested",
                "ttl_update_requested",
                "ttl_extension_requested",
                "garbage_collection_scan_requested",
                "delete_requested",
                "tombstone_requested",
                "sweep_requested",
                "archive_requested",
                "compaction_requested",
                "activation_from_retention_gc_requested",
                "memory_store_gc_evidence_requested",
                "live_kg_gc_evidence_requested",
                "rollback_gc_evidence_requested",
                "secret_material_gc_evidence_requested",
                "provider_prompt_gc_evidence_requested",
                "ledger_retention_requested",
                "index_retention_requested",
                "delivery_retention_requested",
                "external_send_gc_evidence_requested",
                "public_claim_gc_evidence_requested",
                "release_artifact_gc_evidence_requested",
                "install_gc_evidence_requested",
                "service_restart_gc_evidence_requested",
                "active_binary_gc_evidence_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
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
                "activation_command_result_receipt_audit_trail_recorded",
                "activation_command_result_receipt_audit_trail_persisted",
                "activation_command_result_receipt_immutable_evidence_recorded",
                "activation_command_result_receipt_immutable_evidence_persisted",
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
                "operator_approval_from_retention_accepted",
                "operator_approval_from_expiry_accepted",
                "operator_approval_from_garbage_collection_accepted",
                "activation_from_retention_allowed",
                "activation_from_expiry_allowed",
                "activation_from_garbage_collection_allowed",
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
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };

    let retention_expiry_garbage_collection_fixtures = serde_json::Value::Array(vec![
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-retention-missing-source-audit-evidence",
            "blocked_noop",
            "source_audit_trail_immutable_evidence_report_required",
            serde_json::json!({
                "source_audit_evidence_present": false,
                "source_audit_evidence_ready": false,
                "source_audit_trail_immutable_evidence_present": false,
                "source_audit_trail_immutable_evidence_ready": false,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-retention-policy-write-request",
            "blocked_noop",
            "retention_policy_write_request_denied",
            serde_json::json!({
                "retention_policy_request_shape": "record_blocked_noop_receipt_retention_policy",
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-retention-index-record",
            "blocked_noop",
            "retention_index_recording_denied",
            serde_json::json!({
                "retention_index_requested": true,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-expiry-scheduler-timer",
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
            "provider-router-activation-command-result-receipt-ttl-update-extension",
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
            "provider-router-activation-command-result-receipt-garbage-collection-scan",
            "blocked_gc_noop",
            "garbage_collection_scan_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "garbage_collection_scan_requested": true,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-delete-tombstone-sweep",
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
            "provider-router-activation-command-result-receipt-archive-compaction",
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
            "provider-router-activation-command-result-receipt-activation-memory-kg-provider-retention-gc",
            "blocked_gc_noop",
            "activation_memory_kg_provider_retention_gc_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "garbage_collection_requested": true,
                "activation_from_retention_gc_requested": true,
                "memory_store_gc_evidence_requested": true,
                "live_kg_gc_evidence_requested": true,
                "rollback_gc_evidence_requested": true,
                "secret_material_gc_evidence_requested": true,
                "provider_prompt_gc_evidence_requested": true,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-retention-gc",
            "blocked_gc_noop",
            "external_public_install_restart_active_binary_retention_gc_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "garbage_collection_requested": true,
                "ledger_retention_requested": true,
                "index_retention_requested": true,
                "delivery_retention_requested": true,
                "external_send_gc_evidence_requested": true,
                "public_claim_gc_evidence_requested": true,
                "release_artifact_gc_evidence_requested": true,
                "install_gc_evidence_requested": true,
                "service_restart_gc_evidence_requested": true,
                "active_binary_gc_evidence_requested": true,
            }),
        ),
    ]);
    let retention_gc_fixture_count = retention_expiry_garbage_collection_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_hash = sha256_json_value(&retention_expiry_garbage_collection_fixtures);
    let source_report_sha256 = sha256_json_value(&source);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial:v1:source={source_report_sha256}:fixtures={fixtures_hash}:retention=0:expiry=0:gc=0:delete=0:archive=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial:v1:no-retention:no-expiry:no-gc:no-delete:no-tombstone:no-sweep:no-archive:no-compaction:no-provider-model-memory-kg-secret-external-install-restart-binary-public-authority",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-retention-expiry-garbage-collection-side-effects=false;fixtures=10;retention=0;expiry=0;gc=0;delete=0;archive=0;compaction=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0;external=0;install=0",
    );
    let denials = vec![
        "source_audit_trail_immutable_evidence_report_required",
        "retention_policy_request_acceptance_denied",
        "retention_policy_recording_denied",
        "retention_policy_persistence_denied",
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
        "delete_marker_recording_denied",
        "tombstone_recording_denied",
        "sweep_execution_denied",
        "archive_write_denied",
        "compaction_execution_denied",
        "ledger_retention_recording_denied",
        "index_retention_recording_denied",
        "delivery_retention_recording_denied",
        "activation_from_retention_expiry_gc_denied",
        "memory_kg_gc_denied",
        "rollback_gc_denied",
        "secret_material_gc_denied",
        "provider_prompt_gc_denied",
        "external_public_install_restart_active_binary_gc_denied",
    ];
    let denied_count = denials.len();
    let denials_value = serde_json::Value::Array(
        denials
            .iter()
            .map(|denial| serde_json::Value::String((*denial).to_string()))
            .collect(),
    );

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status",
            "side_effect_free": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_gate": source_str("gate"),
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_ready": source_ready,
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_status": source_str("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status"),
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256": source_report_sha256,
            "source_audit_trail_immutable_evidence_contract_hash_sha256": source_hash_str("audit_trail_immutable_evidence_contract_hash_sha256"),
            "source_audit_trail_immutable_evidence_policy_hash_sha256": source_hash_str("audit_trail_immutable_evidence_policy_hash_sha256"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_retention_expiry_garbage_collection_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_v1",
            "activation_command_result_receipt_retention_expiry_garbage_collection_mode": "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_no_retention_no_expiry_no_gc",
            "activation_command_result_receipt_retention_expiry_garbage_collection_decision": "runtime_provider_router_activation_command_result_receipt_cannot_be_retained_expired_garbage_collected_deleted_archived_or_compacted_into_authority",
            "minimum_required_samples": 24,
            "retention_expiry_garbage_collection_fixtures_sha256": fixtures_hash,
            "retention_expiry_garbage_collection_contract_hash_sha256": contract_hash,
            "retention_expiry_garbage_collection_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_audit_trail_immutable_evidence_fixture_count": source_u64("audit_trail_immutable_evidence_fixture_count"),
            "source_blocked_audit_trail_immutable_evidence_fixture_count": source_u64("blocked_audit_trail_immutable_evidence_fixture_count"),
            "source_accepted_audit_trail_immutable_evidence_fixture_count": source_u64("accepted_audit_trail_immutable_evidence_fixture_count"),
            "source_audit_trail_performed_count": source_u64("audit_trail_performed_count"),
            "source_immutable_evidence_performed_count": source_u64("immutable_evidence_performed_count"),
            "source_hash_chain_recorded_count": source_u64("hash_chain_recorded_count"),
            "source_merkle_root_recorded_count": source_u64("merkle_root_recorded_count"),
            "source_attestation_recorded_count": source_u64("attestation_recorded_count"),
            "audit_trail_immutable_evidence_surface_count": source_u64("audit_trail_immutable_evidence_surface_count"),
            "audit_trail_immutable_evidence_fixture_count": source_u64("audit_trail_immutable_evidence_fixture_count"),
            "retention_expiry_garbage_collection_surface_count": 12,
            "retention_expiry_garbage_collection_surface_ready_count": 12,
            "retention_expiry_garbage_collection_side_effect_free_surface_count": 12,
            "retention_expiry_garbage_collection_surfaces": [
                "source_audit_trail_immutable_evidence_report_required",
                "retention_policy_request_shape_denied",
                "retention_index_recording_denied",
                "expiry_scheduler_registration_denied",
                "ttl_update_extension_denied",
                "garbage_collection_scan_denied",
                "delete_tombstone_sweep_denied",
                "archive_compaction_denied",
                "ledger_index_delivery_retention_evidence_denied",
                "activation_from_retention_expiry_gc_denied",
                "memory_kg_rollback_secret_provider_gc_denied",
                "external_public_install_restart_active_binary_gc_denied"
            ],
            "retention_expiry_garbage_collection_fixtures": retention_expiry_garbage_collection_fixtures,
            "retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "blocked_retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "noop_retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "allowed_retention_expiry_garbage_collection_fixture_count": 0,
            "accepted_retention_expiry_garbage_collection_fixture_count": 0,
            "retention_denied_count": retention_gc_fixture_count,
            "expiry_denied_count": retention_gc_fixture_count,
            "garbage_collection_denied_count": retention_gc_fixture_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "retention_performed_count": 0,
            "expiry_performed_count": 0,
            "garbage_collection_performed_count": 0,
            "delete_performed_count": 0,
            "archive_written_count": 0,
            "compaction_performed_count": 0,
            "retention_policy_recorded_count": 0,
            "retention_policy_persisted_count": 0,
            "retention_index_recorded_count": 0,
            "retention_index_persisted_count": 0,
            "expiry_recorded_count": 0,
            "expiry_persisted_count": 0,
            "expiry_scheduler_registered_count": 0,
            "expiry_timer_started_count": 0,
            "ttl_update_recorded_count": 0,
            "ttl_extension_recorded_count": 0,
            "garbage_collection_scan_performed_count": 0,
            "garbage_collection_candidate_recorded_count": 0,
            "garbage_collection_decision_recorded_count": 0,
            "delete_marker_recorded_count": 0,
            "tombstone_recorded_count": 0,
            "sweep_performed_count": 0,
            "ledger_retention_recorded_count": 0,
            "index_retention_recorded_count": 0,
            "delivery_retention_recorded_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_retention_expiry_garbage_collection": denials_value,
            "denied_by_retention_expiry_garbage_collection_count": denied_count,
            "denied_by_activation_command_result_receipt_retention_expiry_garbage_collection": denials_value,
            "denied_by_activation_command_result_receipt_retention_expiry_garbage_collection_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
                    "status": "allowed_report_only",
                    "performs_retention": false,
                    "performs_expiry": false,
                    "performs_gc": false,
                    "deletes_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial",
                    "status": "allowed_report_only_next_slice",
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "performs_retention": false,
                    "performs_gc": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_audit_trail_immutable_evidence_report_required": true,
            "retention_acceptance_forbidden": true,
            "retention_recording_forbidden": true,
            "retention_persistence_forbidden": true,
            "expiry_acceptance_forbidden": true,
            "expiry_scheduler_registration_forbidden": true,
            "ttl_update_forbidden": true,
            "garbage_collection_forbidden": true,
            "delete_tombstone_sweep_forbidden": true,
            "archive_compaction_forbidden": true,
            "runtime_provider_memory_kg_gc_evidence_forbidden": true,
            "secret_read_forbidden": true,
            "external_public_install_restart_active_binary_gc_forbidden": true,
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
            "activation_command_result_receipt_ledger_written",
            "activation_command_result_receipt_indexed",
            "activation_command_result_receipt_enqueued",
            "activation_command_result_receipt_delivered",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "activation_allowed_by_result_receipt_retention",
            "activation_allowed_by_result_receipt_expiry",
            "activation_allowed_by_result_receipt_garbage_collection",
            "activation_allowed_by_result_receipt_audit_trail",
            "activation_allowed_by_result_receipt_immutable_evidence",
            "activation_allowed_by_result_receipt",
            "operator_approval_from_retention_accepted",
            "operator_approval_from_expiry_accepted",
            "operator_approval_from_garbage_collection_accepted",
            "activation_from_retention_allowed",
            "activation_from_expiry_allowed",
            "activation_from_garbage_collection_allowed",
            "activation_command_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
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
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        if !report_object
            .get("side_effects")
            .is_some_and(serde_json::Value::is_object)
        {
            report_object.insert("side_effects".to_string(), serde_json::json!({}));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_retention_policy_recorded",
            "activation_command_result_receipt_retention_policy_persisted",
            "activation_command_result_receipt_retention_policy_materialized",
            "activation_command_result_receipt_retention_policy_filesystem_written",
            "activation_command_result_receipt_retention_index_recorded",
            "activation_command_result_receipt_retention_index_persisted",
            "activation_command_result_receipt_expiry_recorded",
            "activation_command_result_receipt_expiry_persisted",
            "activation_command_result_receipt_expiry_scheduler_registered",
            "activation_command_result_receipt_expiry_timer_started",
            "activation_command_result_receipt_ttl_update_recorded",
            "activation_command_result_receipt_ttl_extension_recorded",
            "activation_command_result_receipt_garbage_collection_scan_performed",
            "activation_command_result_receipt_garbage_collection_candidate_recorded",
            "activation_command_result_receipt_garbage_collection_decision_recorded",
            "activation_command_result_receipt_garbage_collection_persisted",
            "activation_command_result_receipt_delete_performed",
            "activation_command_result_receipt_delete_marker_recorded",
            "activation_command_result_receipt_tombstone_recorded",
            "activation_command_result_receipt_sweep_performed",
            "activation_command_result_receipt_archive_written",
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
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
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
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
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
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report();
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let source_hash_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status",
        ) == "blocked"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready",
        )
        && source_u64("retention_expiry_garbage_collection_fixture_count") == 10
        && source_u64("accepted_retention_expiry_garbage_collection_fixture_count") == 0
        && source_u64("retention_performed_count") == 0
        && source_u64("expiry_performed_count") == 0
        && source_u64("garbage_collection_performed_count") == 0
        && !source_bool("activation_command_result_receipt_retention_policy_recorded")
        && !source_bool("activation_command_result_receipt_expiry_recorded")
        && !source_bool("activation_command_result_receipt_garbage_collection_scan_performed")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_allowed_by_result_receipt")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("external_send_performed")
        && !source_bool("install_executed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = source_ready && route_count_source_command_accepted;

    let export_query_observability_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "export_query_observability_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(reason.to_string()),
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
                "export_file_requested",
                "export_stream_requested",
                "query_endpoint_requested",
                "query_index_requested",
                "query_cache_requested",
                "metric_requested",
                "trace_requested",
                "span_requested",
                "log_requested",
                "event_requested",
                "dashboard_requested",
                "alert_requested",
                "slo_requested",
                "activation_from_observability_requested",
                "memory_store_observability_requested",
                "live_kg_observability_requested",
                "rollback_observability_requested",
                "secret_material_observability_requested",
                "provider_prompt_observability_requested",
                "ledger_observability_requested",
                "index_observability_requested",
                "delivery_observability_requested",
                "external_send_observability_requested",
                "public_claim_observability_requested",
                "release_artifact_observability_requested",
                "install_observability_requested",
                "service_restart_observability_requested",
                "active_binary_observability_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            for key in [
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
                "activation_from_export_allowed",
                "activation_from_query_allowed",
                "activation_from_observability_allowed",
                "activation_from_retention_allowed",
                "activation_from_expiry_allowed",
                "activation_from_garbage_collection_allowed",
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
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };

    let export_query_observability_fixtures = serde_json::Value::Array(vec![
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-export-missing-source-retention-gc",
            "blocked_noop",
            "source_retention_expiry_garbage_collection_report_required",
            serde_json::json!({
                "source_retention_expiry_gc_present": false,
                "source_retention_expiry_gc_ready": false,
                "export_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-export-artifact-request",
            "blocked_export_noop",
            "export_artifact_write_denied",
            serde_json::json!({
                "export_requested": true,
                "export_file_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-export-stream-request",
            "blocked_export_noop",
            "export_stream_open_denied",
            serde_json::json!({
                "export_requested": true,
                "export_stream_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-query-endpoint-request",
            "blocked_query_noop",
            "query_endpoint_materialization_denied",
            serde_json::json!({
                "query_requested": true,
                "query_endpoint_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-query-index-cache-request",
            "blocked_query_noop",
            "query_index_cache_recording_denied",
            serde_json::json!({
                "query_requested": true,
                "query_index_requested": true,
                "query_cache_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-observability-metric-request",
            "blocked_observability_noop",
            "observability_metric_emission_denied",
            serde_json::json!({
                "observability_requested": true,
                "metric_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-observability-trace-log-event-request",
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
            "provider-router-activation-command-result-receipt-dashboard-alert-slo-request",
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
            "provider-router-activation-command-result-receipt-activation-memory-kg-provider-observability",
            "blocked_observability_noop",
            "activation_memory_kg_provider_observability_denied",
            serde_json::json!({
                "observability_requested": true,
                "activation_from_observability_requested": true,
                "memory_store_observability_requested": true,
                "live_kg_observability_requested": true,
                "rollback_observability_requested": true,
                "secret_material_observability_requested": true,
                "provider_prompt_observability_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-observability",
            "blocked_observability_noop",
            "external_public_install_restart_active_binary_observability_denied",
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
            }),
        ),
    ]);
    let export_query_observability_fixture_count = export_query_observability_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_hash = sha256_json_value(&export_query_observability_fixtures);
    let source_report_sha256 = sha256_json_value(&source);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-export-query-observability-denial:v1:source={source_report_sha256}:fixtures={fixtures_hash}:export=0:query=0:observability=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-export-query-observability-denial:v1:no-export:no-query:no-observability:no-provider-model-memory-kg-secret-external-install-restart-binary-public-authority",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-export-query-observability-side-effects=false;fixtures=10;export=0;query=0;observability=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0;external=0;install=0",
    );
    let denials = vec![
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
        "activation_from_export_query_observability_denied",
        "memory_kg_observability_denied",
        "rollback_observability_denied",
        "secret_material_observability_denied",
        "provider_prompt_observability_denied",
        "external_public_install_restart_active_binary_observability_denied",
    ];
    let denied_count = denials.len();
    let denials_value = serde_json::Value::Array(
        denials
            .iter()
            .map(|denial| serde_json::Value::String((*denial).to_string()))
            .collect(),
    );

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status",
            "side_effect_free": true,
            "base_url": "native",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_activation_command_result_receipt_retention_expiry_garbage_collection_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_retention_expiry_garbage_collection_gate": source_str("gate"),
            "source_activation_command_result_receipt_retention_expiry_garbage_collection_ready": source_ready,
            "source_activation_command_result_receipt_retention_expiry_garbage_collection_status": source_str("runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status"),
            "source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256": source_report_sha256,
            "source_retention_expiry_garbage_collection_contract_hash_sha256": source_hash_str("retention_expiry_garbage_collection_contract_hash_sha256"),
            "source_retention_expiry_garbage_collection_policy_hash_sha256": source_hash_str("retention_expiry_garbage_collection_policy_hash_sha256"),
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256": source_hash_str("source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256"),
            "source_activation_command_result_receipt_cancellation_supersession_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "source_activation_command_result_receipt_cancellation_supersession_report_sha256": source_hash_str("source_activation_command_result_receipt_cancellation_supersession_report_sha256"),
            "source_activation_command_result_receipt_ordering_monotonicity_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "source_activation_command_result_receipt_ordering_monotonicity_report_sha256": source_hash_str("source_activation_command_result_receipt_ordering_monotonicity_report_sha256"),
            "source_activation_command_result_receipt_replay_idempotency_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "source_activation_command_result_receipt_replay_idempotency_report_sha256": source_hash_str("source_activation_command_result_receipt_replay_idempotency_report_sha256"),
            "source_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "source_activation_command_result_receipt_no_persistence_report_sha256": source_hash_str("source_activation_command_result_receipt_no_persistence_report_sha256"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_export_query_observability_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_v1",
            "activation_command_result_receipt_export_query_observability_mode": "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_no_export_no_query_no_observability",
            "activation_command_result_receipt_export_query_observability_decision": "runtime_provider_router_activation_command_result_receipt_cannot_be_exported_queried_observed_or_promoted_into_authority",
            "minimum_required_samples": 24,
            "export_query_observability_fixtures_sha256": fixtures_hash,
            "export_query_observability_contract_hash_sha256": contract_hash,
            "export_query_observability_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "retention_expiry_garbage_collection_surface_count": source_u64("retention_expiry_garbage_collection_surface_count"),
            "retention_expiry_garbage_collection_fixture_count": source_u64("retention_expiry_garbage_collection_fixture_count"),
            "source_blocked_retention_expiry_garbage_collection_fixture_count": source_u64("blocked_retention_expiry_garbage_collection_fixture_count"),
            "source_accepted_retention_expiry_garbage_collection_fixture_count": source_u64("accepted_retention_expiry_garbage_collection_fixture_count"),
            "source_retention_performed_count": source_u64("retention_performed_count"),
            "source_expiry_performed_count": source_u64("expiry_performed_count"),
            "source_garbage_collection_performed_count": source_u64("garbage_collection_performed_count"),
            "export_query_observability_surface_count": 12,
            "export_query_observability_surface_ready_count": 12,
            "export_query_observability_side_effect_free_surface_count": 12,
            "export_query_observability_surfaces": [
                "source_retention_expiry_garbage_collection_report_required",
                "export_request_shape_denied",
                "export_artifact_write_denied",
                "export_stream_open_denied",
                "query_endpoint_materialization_denied",
                "query_index_cache_recording_denied",
                "observability_metric_emission_denied",
                "trace_span_log_event_recording_denied",
                "dashboard_alert_slo_materialization_denied",
                "ledger_index_delivery_observability_evidence_denied",
                "activation_memory_kg_provider_observability_denied",
                "external_public_install_restart_active_binary_observability_denied"
            ],
            "export_query_observability_fixtures": export_query_observability_fixtures,
            "export_query_observability_fixture_count": export_query_observability_fixture_count,
            "blocked_export_query_observability_fixture_count": export_query_observability_fixture_count,
            "noop_export_query_observability_fixture_count": export_query_observability_fixture_count,
            "allowed_export_query_observability_fixture_count": 0,
            "accepted_export_query_observability_fixture_count": 0,
            "export_denied_count": export_query_observability_fixture_count,
            "query_denied_count": export_query_observability_fixture_count,
            "observability_denied_count": export_query_observability_fixture_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "export_performed_count": 0,
            "query_performed_count": 0,
            "observability_performed_count": 0,
            "export_recorded_count": 0,
            "export_persisted_count": 0,
            "export_artifact_written_count": 0,
            "export_stream_opened_count": 0,
            "query_registered_count": 0,
            "query_endpoint_materialized_count": 0,
            "query_index_recorded_count": 0,
            "query_cache_written_count": 0,
            "query_result_materialized_count": 0,
            "observability_metric_emitted_count": 0,
            "observability_log_recorded_count": 0,
            "observability_trace_recorded_count": 0,
            "observability_span_recorded_count": 0,
            "observability_event_recorded_count": 0,
            "observability_dashboard_materialized_count": 0,
            "observability_alert_registered_count": 0,
            "observability_slo_recorded_count": 0,
            "ledger_observability_recorded_count": 0,
            "index_observability_recorded_count": 0,
            "delivery_observability_recorded_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_export_query_observability": denials_value,
            "denied_by_export_query_observability_count": denied_count,
            "denied_by_activation_command_result_receipt_export_query_observability": denials_value,
            "denied_by_activation_command_result_receipt_export_query_observability_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial",
                    "status": "allowed_report_only",
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                    "status": "allowed_report_only_next_slice",
                    "persists_summary": false,
                    "persists_briefing": false,
                    "delivers_summary": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_retention_expiry_garbage_collection_report_required": true,
            "export_acceptance_forbidden": true,
            "export_recording_forbidden": true,
            "export_persistence_forbidden": true,
            "export_artifact_write_forbidden": true,
            "export_stream_forbidden": true,
            "query_registration_forbidden": true,
            "query_endpoint_materialization_forbidden": true,
            "query_index_cache_forbidden": true,
            "observability_metric_forbidden": true,
            "observability_trace_log_event_forbidden": true,
            "dashboard_alert_slo_forbidden": true,
            "activation_from_export_query_observability_forbidden": true,
            "runtime_provider_memory_kg_observability_forbidden": true,
            "secret_read_forbidden": true,
            "external_public_install_restart_active_binary_observability_forbidden": true,
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_export_allowed",
            "activation_command_result_receipt_export_request_accepted",
            "activation_command_result_receipt_export_recorded",
            "activation_command_result_receipt_export_persisted",
            "activation_command_result_receipt_export_artifact_written",
            "activation_command_result_receipt_export_stream_opened",
            "activation_command_result_receipt_export_filesystem_written",
            "activation_command_result_receipt_query_allowed",
            "activation_command_result_receipt_query_registered",
            "activation_command_result_receipt_query_endpoint_materialized",
            "activation_command_result_receipt_query_index_recorded",
            "activation_command_result_receipt_query_cache_written",
            "activation_command_result_receipt_query_result_materialized",
            "activation_command_result_receipt_observability_allowed",
            "activation_command_result_receipt_observability_metric_emitted",
            "activation_command_result_receipt_observability_log_recorded",
            "activation_command_result_receipt_observability_trace_recorded",
            "activation_command_result_receipt_observability_span_recorded",
            "activation_command_result_receipt_observability_event_recorded",
            "activation_command_result_receipt_observability_dashboard_materialized",
            "activation_command_result_receipt_observability_alert_registered",
            "activation_command_result_receipt_observability_slo_recorded",
            "activation_command_result_receipt_ledger_observability_recorded",
            "activation_command_result_receipt_index_observability_recorded",
            "activation_command_result_receipt_delivery_observability_recorded",
            "activation_command_result_receipt_retention_policy_recorded",
            "activation_command_result_receipt_retention_index_recorded",
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
            "activation_allowed_by_result_receipt_export",
            "activation_allowed_by_result_receipt_query",
            "activation_allowed_by_result_receipt_observability",
            "activation_allowed_by_result_receipt_retention",
            "activation_allowed_by_result_receipt_expiry",
            "activation_allowed_by_result_receipt_garbage_collection",
            "activation_allowed_by_result_receipt_audit_trail",
            "activation_allowed_by_result_receipt_immutable_evidence",
            "activation_allowed_by_result_receipt",
            "activation_from_export_allowed",
            "activation_from_query_allowed",
            "activation_from_observability_allowed",
            "activation_command_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
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
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        if !report_object
            .get("side_effects")
            .is_some_and(serde_json::Value::is_object)
        {
            report_object.insert("side_effects".to_string(), serde_json::json!({}));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_export_recorded",
            "activation_command_result_receipt_export_persisted",
            "activation_command_result_receipt_export_artifact_written",
            "activation_command_result_receipt_export_stream_opened",
            "activation_command_result_receipt_export_filesystem_written",
            "activation_command_result_receipt_query_registered",
            "activation_command_result_receipt_query_endpoint_materialized",
            "activation_command_result_receipt_query_index_recorded",
            "activation_command_result_receipt_query_cache_written",
            "activation_command_result_receipt_query_result_materialized",
            "activation_command_result_receipt_observability_metric_emitted",
            "activation_command_result_receipt_observability_log_recorded",
            "activation_command_result_receipt_observability_trace_recorded",
            "activation_command_result_receipt_observability_span_recorded",
            "activation_command_result_receipt_observability_event_recorded",
            "activation_command_result_receipt_observability_dashboard_materialized",
            "activation_command_result_receipt_observability_alert_registered",
            "activation_command_result_receipt_observability_slo_recorded",
            "activation_command_result_receipt_ledger_observability_recorded",
            "activation_command_result_receipt_index_observability_recorded",
            "activation_command_result_receipt_delivery_observability_recorded",
            "activation_command_result_receipt_retention_policy_recorded",
            "activation_command_result_receipt_retention_index_recorded",
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
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
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
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_report();
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let source_hash_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status",
        ) == "blocked"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready",
        )
        && source_u64("export_query_observability_surface_count") == 12
        && source_u64("export_query_observability_fixture_count") == 10
        && source_u64("accepted_export_query_observability_fixture_count") == 0
        && source_u64("export_performed_count") == 0
        && source_u64("query_performed_count") == 0
        && source_u64("observability_performed_count") == 0
        && !source_bool("activation_command_result_receipt_export_recorded")
        && !source_bool("activation_command_result_receipt_query_registered")
        && !source_bool("activation_command_result_receipt_observability_metric_emitted")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_allowed_by_result_receipt")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("install_executed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = source_ready && route_count_source_command_accepted;

    let operator_summary_briefing_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "operator_summary_briefing_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "operator_facing_summary_briefing_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_export_query_observability_present",
                "source_export_query_observability_ready",
                "summary_briefing_noop_confirmed",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "operator_summary_requested",
                "operator_briefing_requested",
                "operator_summary_materialization_requested",
                "operator_briefing_materialization_requested",
                "operator_summary_persistence_requested",
                "operator_briefing_persistence_requested",
                "operator_summary_filesystem_write_requested",
                "operator_briefing_filesystem_write_requested",
                "channel_delivery_requested",
                "telegram_send_requested",
                "activation_from_summary_briefing_requested",
                "memory_store_summary_requested",
                "live_kg_summary_requested",
                "rollback_summary_requested",
                "secret_material_summary_requested",
                "provider_prompt_summary_requested",
                "external_send_summary_requested",
                "public_claim_summary_requested",
                "release_artifact_summary_requested",
                "install_summary_requested",
                "service_restart_summary_requested",
                "active_binary_summary_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            for key in [
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
                "activation_command_result_receipt_ledger_written",
                "activation_command_result_receipt_indexed",
                "activation_command_result_receipt_enqueued",
                "activation_command_result_receipt_delivered",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "activation_command_completion_ack_delivered",
                "activation_allowed_by_result_receipt_operator_summary",
                "activation_allowed_by_result_receipt_operator_briefing",
                "activation_allowed_by_result_receipt_summary_briefing",
                "activation_allowed_by_result_receipt",
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
            "provider-router-activation-command-result-receipt-operator-summary-missing-source-export-query-observability",
            "blocked_noop",
            "source_export_query_observability_report_required",
            serde_json::json!({
                "source_export_query_observability_present": false,
                "source_export_query_observability_ready": false,
                "operator_summary_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-request",
            "blocked_summary_noop",
            "operator_summary_request_shape_denied",
            serde_json::json!({"operator_summary_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-briefing-request",
            "blocked_briefing_noop",
            "operator_briefing_request_shape_denied",
            serde_json::json!({"operator_briefing_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-materialization-request",
            "blocked_summary_noop",
            "operator_summary_materialization_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_summary_materialization_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-briefing-materialization-request",
            "blocked_briefing_noop",
            "operator_briefing_materialization_denied",
            serde_json::json!({
                "operator_briefing_requested": true,
                "operator_briefing_materialization_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-persistence-filesystem-request",
            "blocked_summary_noop",
            "operator_summary_persistence_filesystem_write_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_summary_persistence_requested": true,
                "operator_summary_filesystem_write_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-briefing-persistence-filesystem-request",
            "blocked_briefing_noop",
            "operator_briefing_persistence_filesystem_write_denied",
            serde_json::json!({
                "operator_briefing_requested": true,
                "operator_briefing_persistence_requested": true,
                "operator_briefing_filesystem_write_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-briefing-channel-delivery-request",
            "blocked_delivery_noop",
            "operator_summary_briefing_channel_delivery_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "channel_delivery_requested": true,
                "telegram_send_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-briefing-activation-memory-kg-provider",
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
                "provider_prompt_summary_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-briefing-external-public-install",
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
                "active_binary_summary_requested": true
            }),
        ),
    ]);
    let operator_summary_briefing_fixture_count = operator_summary_briefing_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash = sha256_json_value(&operator_summary_briefing_fixtures);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial:v1:source={source_report_sha256}:fixtures={fixtures_hash}:summary=0:briefing=0:delivery=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial:v1:no-summary:no-briefing:no-record:no-persist:no-materialize:no-delivery:no-authority:no-provider-model-memory-kg-secret-external-install-restart-binary",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-operator-facing-summary-briefing-side-effects=false;summary=0;briefing=0;delivery=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0;external=0;install=0",
    );
    let denials = vec![
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
    ];
    let denied_count = denials.len();
    let denials_value = serde_json::Value::Array(
        denials
            .iter()
            .map(|denial| serde_json::Value::String((*denial).to_string()))
            .collect(),
    );

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status",
            "side_effect_free": true,
            "base_url": "native",
            "source_activation_command_result_receipt_export_query_observability_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_export_query_observability_gate": source_str("gate"),
            "source_activation_command_result_receipt_export_query_observability_ready": source_ready,
            "source_activation_command_result_receipt_export_query_observability_status": source_str("runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status"),
            "source_activation_command_result_receipt_export_query_observability_report_sha256": source_report_sha256,
            "source_export_query_observability_contract_hash_sha256": source_hash_str("export_query_observability_contract_hash_sha256"),
            "source_export_query_observability_policy_hash_sha256": source_hash_str("export_query_observability_policy_hash_sha256"),
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_operator_facing_summary_briefing_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
            "activation_command_result_receipt_operator_facing_summary_briefing_mode": "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_no_summary_no_briefing_no_delivery",
            "activation_command_result_receipt_operator_facing_summary_briefing_decision": "runtime_provider_router_activation_command_result_receipt_cannot_record_persist_materialize_deliver_or_promote_operator_summary_briefing_into_authority",
            "minimum_required_samples": 24,
            "operator_summary_briefing_fixtures_sha256": fixtures_hash,
            "operator_summary_briefing_contract_hash_sha256": contract_hash,
            "operator_summary_briefing_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
            "export_query_observability_surface_count": source_u64("export_query_observability_surface_count"),
            "export_query_observability_fixture_count": source_u64("export_query_observability_fixture_count"),
            "operator_facing_summary_briefing_surface_count": 12,
            "operator_facing_summary_briefing_surface_ready_count": 12,
            "operator_facing_summary_briefing_side_effect_free_surface_count": 12,
            "operator_facing_summary_briefing_surfaces": [
                "source_export_query_observability_report_required",
                "operator_summary_request_shape_denied",
                "operator_briefing_request_shape_denied",
                "summary_materialization_denied",
                "briefing_materialization_denied",
                "summary_persistence_denied",
                "briefing_persistence_denied",
                "summary_delivery_denied",
                "briefing_delivery_denied",
                "activation_from_summary_briefing_denied",
                "memory_kg_rollback_secret_provider_summary_briefing_denied",
                "external_public_install_restart_active_binary_summary_briefing_denied"
            ],
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
            "operator_summary_briefing_delivery_performed_count": 0,
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
            "operator_summary_recorded_count": 0,
            "operator_summary_persisted_count": 0,
            "operator_briefing_recorded_count": 0,
            "operator_briefing_persisted_count": 0,
            "operator_summary_delivered_count": 0,
            "operator_briefing_delivered_count": 0,
            "operator_summary_briefing_channel_delivery_count": 0,
            "operator_summary_briefing_external_send_count": 0,
            "operator_summary_briefing_telegram_send_count": 0,
            "operator_summary_briefing_activation_authority_derived_count": 0,
            "operator_summary_briefing_live_execution_allowed_count": 0,
            "activation_allowed_by_result_receipt_operator_summary": false,
            "activation_allowed_by_result_receipt_operator_briefing": false,
            "activation_allowed_by_result_receipt_summary_briefing": false,
            "activation_from_summary_briefing_forbidden": true,
            "runtime_provider_memory_kg_summary_briefing_forbidden": true,
            "external_public_install_restart_active_binary_summary_briefing_forbidden": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_operator_facing_summary_briefing": denials_value,
            "denied_by_operator_facing_summary_briefing_count": denied_count,
            "denied_by_activation_command_result_receipt_operator_facing_summary_briefing": denials_value,
            "denied_by_activation_command_result_receipt_operator_facing_summary_briefing_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                    "status": "allowed_report_only",
                    "persists_summary": false,
                    "persists_briefing": false,
                    "delivers_summary": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_operator_acknowledgement": false,
                    "persists_acknowledgement": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "persists_summary": false,
                    "persists_briefing": false,
                    "delivers_summary": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_export_query_observability_report_required": true,
            "operator_summary_acceptance_forbidden": true,
            "operator_briefing_acceptance_forbidden": true,
            "operator_summary_recording_forbidden": true,
            "operator_briefing_recording_forbidden": true,
            "operator_summary_persistence_forbidden": true,
            "operator_briefing_persistence_forbidden": true,
            "operator_summary_materialization_forbidden": true,
            "operator_briefing_materialization_forbidden": true,
            "operator_summary_filesystem_write_forbidden": true,
            "operator_briefing_filesystem_write_forbidden": true,
            "operator_summary_delivery_forbidden": true,
            "operator_briefing_delivery_forbidden": true,
            "telegram_send_forbidden": true,
            "channel_delivery_forbidden": true,
            "secret_read_forbidden": true,
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_operator_summary_allowed",
            "activation_command_result_receipt_operator_summary_request_accepted",
            "activation_command_result_receipt_operator_summary_recorded",
            "activation_command_result_receipt_operator_summary_persisted",
            "activation_command_result_receipt_operator_summary_materialized",
            "activation_command_result_receipt_operator_summary_filesystem_written",
            "activation_command_result_receipt_operator_summary_delivered",
            "activation_command_result_receipt_operator_summary_channel_delivery_performed",
            "activation_command_result_receipt_operator_briefing_allowed",
            "activation_command_result_receipt_operator_briefing_request_accepted",
            "activation_command_result_receipt_operator_briefing_recorded",
            "activation_command_result_receipt_operator_briefing_persisted",
            "activation_command_result_receipt_operator_briefing_materialized",
            "activation_command_result_receipt_operator_briefing_filesystem_written",
            "activation_command_result_receipt_operator_briefing_delivered",
            "activation_command_result_receipt_operator_briefing_channel_delivery_performed",
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
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
            "activation_allowed_by_result_receipt_operator_summary",
            "activation_allowed_by_result_receipt_operator_briefing",
            "activation_allowed_by_result_receipt_summary_briefing",
            "activation_allowed_by_result_receipt",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
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
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        if !report_object
            .get("side_effects")
            .is_some_and(serde_json::Value::is_object)
        {
            report_object.insert("side_effects".to_string(), serde_json::json!({}));
        }
    }
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
            "activation_command_result_receipt_operator_briefing_recorded",
            "activation_command_result_receipt_operator_briefing_persisted",
            "activation_command_result_receipt_operator_briefing_materialized",
            "activation_command_result_receipt_operator_briefing_filesystem_written",
            "activation_command_result_receipt_operator_briefing_delivered",
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_activated",
            "runtime_router_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "memory_store_mutated",
            "live_kg_write_performed",
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
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_summary =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report();
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
    let source_str = |key: &str| {
        source_summary
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let source_hash_str = |key: &str| {
        source_summary
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let source_status = source_str(
        "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status",
    );
    let source_ready = source_str("status") == "ready"
        && source_status == "blocked"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
        )
        && source_u64("operator_facing_summary_briefing_surface_count") == 12
        && source_u64("operator_facing_summary_briefing_fixture_count") == 10
        && source_u64("accepted_operator_facing_summary_briefing_fixture_count") == 0
        && source_u64("operator_summary_performed_count") == 0
        && source_u64("operator_briefing_performed_count") == 0
        && source_u64("operator_summary_briefing_delivery_performed_count") == 0
        && !source_bool("activation_command_result_receipt_operator_summary_recorded")
        && !source_bool("activation_command_result_receipt_operator_summary_persisted")
        && !source_bool("activation_command_result_receipt_operator_summary_delivered")
        && !source_bool("activation_command_result_receipt_operator_briefing_recorded")
        && !source_bool("activation_command_result_receipt_operator_briefing_persisted")
        && !source_bool("activation_command_result_receipt_operator_briefing_delivered")
        && !source_bool(
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
        )
        && !source_bool("telegram_send_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_allowed_by_result_receipt_summary_briefing")
        && !source_bool("activation_allowed_by_result_receipt")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("memory_store_write_performed")
        && !source_bool("memory_store_mutated")
        && !source_bool("live_kg_write_performed")
        && !source_bool("install_executed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = source_ready && route_count_source_command_accepted;
    let source_report_sha256 = sha256_json_value(&source_summary);

    let final_ack_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "final_operator_acknowledgement_requested".to_string(),
                serde_json::Value::Bool(false),
            );
            fixture.insert(
                "final_operator_acknowledgement_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_summary_briefing_present",
                "source_summary_briefing_ready",
                "final_acknowledgement_noop_confirmed",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "acknowledgement_acceptance_requested",
                "acknowledgement_recording_requested",
                "acknowledgement_persistence_requested",
                "acknowledgement_filesystem_write_requested",
                "operator_identity_acceptance_requested",
                "operator_signature_acceptance_requested",
                "operator_timestamp_acceptance_requested",
                "acknowledgement_delivery_requested",
                "telegram_send_requested",
                "channel_delivery_requested",
                "final_state_promotion_requested",
                "completion_promotion_requested",
                "activation_from_acknowledgement_requested",
                "memory_store_acknowledgement_requested",
                "live_kg_acknowledgement_requested",
                "rollback_acknowledgement_requested",
                "secret_material_acknowledgement_requested",
                "provider_prompt_acknowledgement_requested",
                "external_send_acknowledgement_requested",
                "public_claim_acknowledgement_requested",
                "release_artifact_acknowledgement_requested",
                "install_acknowledgement_requested",
                "service_restart_acknowledgement_requested",
                "active_binary_acknowledgement_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
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
                "secret_material_read",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "rollback_executed",
                "public_release_claimed",
                "public_release_published",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "service_restarted",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };

    let final_acknowledgement_fixtures = serde_json::Value::Array(vec![
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-missing-source-summary-briefing",
            "blocked_noop",
            "source_summary_briefing_report_required",
            serde_json::json!({
                "source_summary_briefing_present": false,
                "source_summary_briefing_ready": false,
                "final_operator_acknowledgement_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_request_shape_denied",
            serde_json::json!({"final_operator_acknowledgement_requested": true}),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-acceptance-request",
            "blocked_acceptance_noop",
            "final_operator_acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_acceptance_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-recording-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_recording_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_recording_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-persistence-filesystem-write-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_persistence_filesystem_write_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_persistence_requested": true,
                "acknowledgement_filesystem_write_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-identity-signature-timestamp-request",
            "blocked_acceptance_noop",
            "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "operator_identity_acceptance_requested": true,
                "operator_signature_acceptance_requested": true,
                "operator_timestamp_acceptance_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-delivery-request",
            "blocked_delivery_noop",
            "final_operator_acknowledgement_delivery_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_delivery_requested": true,
                "telegram_send_requested": true,
                "channel_delivery_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-state-promotion-request",
            "blocked_promotion_noop",
            "final_state_completion_promotion_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "final_state_promotion_requested": true,
                "completion_promotion_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-activation-memory-kg-provider-request",
            "blocked_ack_noop",
            "activation_memory_kg_rollback_secret_provider_acknowledgement_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "activation_from_acknowledgement_requested": true,
                "memory_store_acknowledgement_requested": true,
                "live_kg_acknowledgement_requested": true,
                "rollback_acknowledgement_requested": true,
                "secret_material_acknowledgement_requested": true,
                "provider_prompt_acknowledgement_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-external-public-install-request",
            "blocked_delivery_noop",
            "external_public_install_restart_active_binary_acknowledgement_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "external_send_acknowledgement_requested": true,
                "public_claim_acknowledgement_requested": true,
                "release_artifact_acknowledgement_requested": true,
                "install_acknowledgement_requested": true,
                "service_restart_acknowledgement_requested": true,
                "active_binary_acknowledgement_requested": true
            }),
        ),
    ]);
    let final_acknowledgement_fixture_count = final_acknowledgement_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_hash = sha256_json_value(&final_acknowledgement_fixtures);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial:v1:source={source_report_sha256}:fixtures={fixtures_hash}:ack=0:accept=0:record=0:persist=0:deliver=0:promote=0:live=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial:v1:no-ack:no-accept:no-record:no-persist:no-materialize:no-deliver:no-final-state:no-provider-model-memory-kg-secret-external-install-restart-binary-public-claim",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-final-operator-acknowledgement-side-effects=false;ack=0;accept=0;record=0;persist=0;deliver=0;promotion=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0;external=0;install=0",
    );
    let mut denials = source_summary
        .get("denied_by_activation_command_result_receipt_operator_facing_summary_briefing")
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
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status",
            "side_effect_free": true,
            "base_url": "native",
            "source_activation_command_result_receipt_operator_facing_summary_briefing_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_operator_facing_summary_briefing_gate": source_str("gate"),
            "source_activation_command_result_receipt_operator_facing_summary_briefing_ready": source_ready,
            "source_activation_command_result_receipt_operator_facing_summary_briefing_status": source_status,
            "source_activation_command_result_receipt_operator_facing_summary_briefing_report_sha256": source_report_sha256,
            "source_operator_summary_briefing_contract_hash_sha256": source_hash_str("operator_summary_briefing_contract_hash_sha256"),
            "source_operator_summary_briefing_policy_hash_sha256": source_hash_str("operator_summary_briefing_policy_hash_sha256"),
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_final_operator_acknowledgement_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
            "activation_command_result_receipt_final_operator_acknowledgement_mode": "native_route_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_no_ack_no_accept_no_record_no_deliver_no_authority_no_live",
            "activation_command_result_receipt_final_operator_acknowledgement_decision": "runtime_provider_router_activation_command_result_receipt_cannot_accept_record_persist_deliver_or_promote_final_operator_acknowledgement_into_authority",
            "source_operator_facing_summary_briefing_fixture_count": source_u64("operator_facing_summary_briefing_fixture_count"),
            "source_blocked_operator_facing_summary_briefing_fixture_count": source_u64("blocked_operator_facing_summary_briefing_fixture_count"),
            "source_accepted_operator_facing_summary_briefing_fixture_count": source_u64("accepted_operator_facing_summary_briefing_fixture_count"),
            "source_operator_summary_performed_count": source_u64("operator_summary_performed_count"),
            "source_operator_briefing_performed_count": source_u64("operator_briefing_performed_count"),
            "final_acknowledgement_fixtures_sha256": fixtures_hash,
            "final_acknowledgement_contract_hash_sha256": contract_hash,
            "final_acknowledgement_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
            "required_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 10,
            "activation_command_result_receipt_final_operator_acknowledgement_surfaces": [
                "source_operator_facing_summary_briefing_report_required",
                "final_operator_acknowledgement_request_shape_denied",
                "final_operator_acknowledgement_acceptance_denied",
                "final_operator_acknowledgement_recording_denied",
                "final_operator_acknowledgement_persistence_denied",
                "final_operator_acknowledgement_materialization_denied",
                "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
                "final_operator_acknowledgement_delivery_denied",
                "final_state_completion_promotion_denied",
                "activation_from_final_operator_acknowledgement_denied",
                "memory_kg_rollback_secret_provider_acknowledgement_denied",
                "external_public_install_restart_active_binary_acknowledgement_denied"
            ],
            "activation_command_result_receipt_final_operator_acknowledgement_fixtures": final_acknowledgement_fixtures,
            "activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 0,
            "accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_denied_count": final_acknowledgement_fixture_count,
            "activation_command_result_receipt_final_operator_acknowledgement_performed_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_recorded_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_persisted_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_delivered_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "memory_write_execution_allowed": false,
            "memory_write_execution_ready": false,
            "memory_store_write_path_enabled": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed_count": 0,
            "memory_store_mutation_allowed": false,
            "rollback_execution_allowed": false,
            "raw_payload_plaintext_recorded": false,
            "raw_payload_plaintext_persisted": false,
            "provider_prompt_replay_enabled": false,
            "external_send_enabled": false,
            "public_claim_or_release_artifact_write_enabled": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_activation_command_result_receipt_final_operator_acknowledgement": denials,
            "denied_by_activation_command_result_receipt_final_operator_acknowledgement_count": denied_count,
            "source_operator_facing_summary_briefing_denial_count": source_u64("denied_by_activation_command_result_receipt_operator_facing_summary_briefing_count"),
            "final_operator_acknowledgement_acceptance_forbidden": true,
            "final_operator_acknowledgement_recording_forbidden": true,
            "final_operator_acknowledgement_persistence_forbidden": true,
            "final_operator_acknowledgement_delivery_forbidden": true,
            "final_operator_acknowledgement_promotion_forbidden": true,
            "activation_from_final_operator_acknowledgement_forbidden": true,
            "runtime_provider_memory_kg_final_operator_acknowledgement_forbidden": true,
            "external_public_install_restart_active_binary_final_operator_acknowledgement_forbidden": true,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
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
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_terminal_decision": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "accepts_operator_acknowledgement": false,
                    "promotes_final_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_final_operator_acknowledgement_allowed",
            "activation_command_result_receipt_final_operator_acknowledgement_request_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_accepted",
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
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "activation_allowed_by_result_receipt_final_operator_acknowledgement",
            "activation_allowed_by_result_receipt_summary_briefing",
            "activation_allowed_by_result_receipt",
            "activation_allowed",
            "activation_performed",
            "activation_activated",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "secret_material_read",
            "provider_invoked",
            "model_invoked",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_release_published",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "service_restarted",
            "active_binary_mutated",
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        if !report_object
            .get("side_effects")
            .is_some_and(serde_json::Value::is_object)
        {
            report_object.insert("side_effects".to_string(), serde_json::json!({}));
        }
    }
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
            "activation_command_result_receipt_operator_summary_recorded",
            "activation_command_result_receipt_operator_summary_persisted",
            "activation_command_result_receipt_operator_summary_materialized",
            "activation_command_result_receipt_operator_summary_filesystem_written",
            "activation_command_result_receipt_operator_summary_delivered",
            "activation_command_result_receipt_operator_briefing_recorded",
            "activation_command_result_receipt_operator_briefing_persisted",
            "activation_command_result_receipt_operator_briefing_materialized",
            "activation_command_result_receipt_operator_briefing_filesystem_written",
            "activation_command_result_receipt_operator_briefing_delivered",
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
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
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
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
            "secret_material_read",
            "filesystem_written",
            "public_release_claimed",
            "public_release_published",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "service_restarted",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_final_ack =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report();
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
    let source_str = |key: &str| {
        source_final_ack
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let source_status = source_str(
        "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status",
    );
    let source_ready = source_status == "blocked"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
        );
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let report_ready = source_ready
        && route_count_source_command_accepted
        && route_matrix.missing_route_count == 0
        && source_u64(
            "activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        ) == 10
        && source_u64(
            "accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        ) == 0
        && source_u64(
            "activation_command_result_receipt_final_operator_acknowledgement_performed_count",
        ) == 0
        && !source_bool("activation_allowed_by_result_receipt_final_operator_acknowledgement")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("public_release_published")
        && !source_bool("release_artifact_written")
        && !source_bool("install_executed")
        && !source_bool("active_binary_mutated");
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
                "live_kg_write_performed",
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
            "provider-router-activation-result-receipt-terminal-decision-missing-final-ack",
            "blocked_noop",
            "source_final_operator_acknowledgement_report_required",
            serde_json::json!({
                "source_final_acknowledgement_present": false,
                "source_final_acknowledgement_ready": false,
                "terminal_operator_decision_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-request",
            "blocked_decision_noop",
            "terminal_operator_decision_request_shape_denied",
            serde_json::json!({"terminal_operator_decision_requested": true}),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-acceptance-request",
            "blocked_acceptance_noop",
            "terminal_operator_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_acceptance_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-recording-request",
            "blocked_decision_noop",
            "terminal_operator_decision_recording_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_recording_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-persistence-filesystem-write-request",
            "blocked_decision_noop",
            "terminal_operator_decision_persistence_filesystem_write_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_persistence_requested": true,
                "terminal_decision_filesystem_write_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-identity-signature-request",
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
            "provider-router-activation-result-receipt-terminal-decision-public-claim-request",
            "blocked_public_claim_noop",
            "public_claim_request_non_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_claim_requested": true,
                "public_claim_promotion_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-public-ga-release-request",
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
            "provider-router-activation-result-receipt-terminal-decision-activation-memory-provider-request",
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
            "provider-router-activation-result-receipt-terminal-decision-external-public-install-request",
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
        "runtime-provider-router-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:decision=0:public_claim=0:publish=0:artifact=0:live=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial:v1:no-terminal-decision-accept:no-public-claim:no-ga-release:no-artifact:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "runtime-provider-router-terminal-decision=false;public_claim=false;public_release=false;artifact=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
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
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status",
            "side_effect_free": true,
            "base_url": "native",
            "source_activation_command_result_receipt_final_operator_acknowledgement_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_final_operator_acknowledgement_gate": source_str("gate"),
            "source_activation_command_result_receipt_final_operator_acknowledgement_ready": source_ready,
            "source_activation_command_result_receipt_final_operator_acknowledgement_status": source_status,
            "source_activation_command_result_receipt_final_operator_acknowledgement_report_sha256": source_report_sha256,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_terminal_operator_decision_public_claim_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_v1",
            "activation_command_result_receipt_terminal_operator_decision_public_claim_mode": "native_route_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_no_decision_no_public_claim_no_release_no_artifact_no_authority_no_live",
            "activation_command_result_receipt_terminal_operator_decision_public_claim_decision": "runtime_provider_router_activation_command_result_receipt_cannot_promote_final_acknowledgement_or_receipt_into_terminal_operator_decision_or_public_claim_authority",
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
            "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim": denials,
            "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count": denied_count,
            "source_final_operator_acknowledgement_denial_count": source_u64("denied_by_activation_command_result_receipt_final_operator_acknowledgement_count"),
            "terminal_operator_decision_acceptance_forbidden": true,
            "terminal_operator_decision_recording_forbidden": true,
            "terminal_operator_decision_persistence_forbidden": true,
            "terminal_operator_decision_delivery_forbidden": true,
            "terminal_operator_decision_promotion_forbidden": true,
            "public_claim_promotion_forbidden": true,
            "public_release_publication_forbidden": true,
            "release_artifact_publication_forbidden": true,
            "activation_from_terminal_operator_decision_forbidden": true,
            "runtime_provider_memory_kg_terminal_operator_decision_forbidden": true,
            "external_public_install_restart_active_binary_terminal_operator_decision_forbidden": true,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                    "status": "allowed_report_only",
                    "accepts_terminal_decision": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial",
                    "status": "allowed_report_only_next_slice",
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "accepts_terminal_decision": false,
                    "claims_public_release": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_terminal_operator_decision_allowed",
            "activation_command_result_receipt_terminal_operator_decision_request_accepted",
            "activation_command_result_receipt_terminal_operator_decision_accepted",
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
            "activation_command_result_receipt_public_claim_requested",
            "activation_command_result_receipt_public_claim_accepted",
            "activation_command_result_receipt_public_claim_recorded",
            "activation_command_result_receipt_public_claim_persisted",
            "activation_command_result_receipt_public_claim_materialized",
            "activation_command_result_receipt_public_claim_promoted",
            "activation_command_result_receipt_public_ga_claimed",
            "activation_command_result_receipt_public_release_published",
            "activation_command_result_receipt_public_distribution_performed",
            "activation_command_result_receipt_public_artifact_written",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "activation_allowed_by_result_receipt_terminal_operator_decision",
            "activation_allowed_by_result_receipt_final_operator_acknowledgement",
            "activation_allowed_by_result_receipt",
            "activation_allowed",
            "activation_performed",
            "activation_activated",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "secret_material_read",
            "provider_invoked",
            "model_invoked",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_release_published",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "public_distribution_performed",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "service_restarted",
            "active_binary_mutated",
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        if !report_object
            .get("side_effects")
            .is_some_and(serde_json::Value::is_object)
        {
            report_object.insert("side_effects".to_string(), serde_json::json!({}));
        }
    }
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

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_terminal =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report();
    let source_bool = |key: &str| {
        source_terminal
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_terminal
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_terminal
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let source_status = source_str(
        "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status",
    );
    let source_ready = source_status == "blocked"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready",
        );
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let report_ready = source_ready
        && route_count_source_command_accepted
        && route_matrix.missing_route_count == 0
        && source_u64(
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 10
        && source_u64(
            "accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 0
        && source_u64(
            "activation_command_result_receipt_terminal_operator_decision_performed_count",
        ) == 0
        && source_u64("activation_command_result_receipt_public_claim_promotion_performed_count")
            == 0
        && !source_bool("activation_command_result_receipt_public_claim_promoted")
        && !source_bool("public_release_published")
        && !source_bool("release_artifact_written")
        && !source_bool("public_artifact_written")
        && !source_bool("public_distribution_performed")
        && !source_bool("activation_allowed")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("install_executed")
        && !source_bool("active_binary_mutated");
    let source_report_sha256 = sha256_json_value(&source_terminal);

    let release_publication_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::json!(id));
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
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
                "provider_prompt_replayed",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
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

    let release_publication_fixtures = serde_json::json!([
        release_publication_fixture(
            "provider-router-activation-result-receipt-release-artifact-publication-missing-terminal-decision",
            "blocked_noop",
            "source_terminal_operator_decision_report_required",
            serde_json::json!({
                "source_terminal_operator_decision_present": false,
                "source_terminal_operator_decision_ready": false,
                "release_artifact_publication_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-release-artifact-write-request",
            "blocked_artifact_noop",
            "release_artifact_write_denied",
            serde_json::json!({
                "release_artifact_write_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-public-artifact-write-request",
            "blocked_artifact_noop",
            "public_artifact_write_denied",
            serde_json::json!({
                "public_artifact_write_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-artifact-signature-notarization-request",
            "blocked_artifact_noop",
            "artifact_signature_notarization_acceptance_denied",
            serde_json::json!({
                "artifact_signature_requested": true,
                "artifact_notarization_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-publication-queue-request",
            "blocked_publication_noop",
            "publication_queue_enqueue_denied",
            serde_json::json!({
                "publication_queue_enqueue_requested": true,
                "publication_manifest_write_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-distribution-channel-request",
            "blocked_distribution_noop",
            "public_distribution_channel_delivery_denied",
            serde_json::json!({
                "public_distribution_requested": true,
                "telegram_delivery_requested": true,
                "channel_delivery_requested": true,
                "external_delivery_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-public-version-tag-request",
            "blocked_release_noop",
            "public_version_tag_release_promotion_denied",
            serde_json::json!({
                "public_version_tag_requested": true,
                "public_release_publish_requested": true,
                "public_ga_claim_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-release-notes-changelog-request",
            "blocked_artifact_noop",
            "release_notes_changelog_materialization_denied",
            serde_json::json!({
                "release_notes_materialization_requested": true,
                "changelog_materialization_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-terminal-decision-as-release-approval",
            "blocked_promotion_noop",
            "terminal_operator_decision_is_not_release_approval",
            serde_json::json!({
                "terminal_operator_decision_release_approval_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-release-publication-activation-memory-provider-install",
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
    let release_publication_fixture_count = release_publication_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&release_publication_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:publication=0:artifact=0:claim=0:distribution=0:install=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial:v1:no-artifact-write:no-public-artifact:no-signing:no-notarization:no-publication:no-release-claim:no-distribution",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "runtime-provider-router-release-artifact-publication=false;artifact=false;signature=false;notarization=false;queue=false;manifest=false;public_release=false;distribution=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );
    let mut denials = source_terminal
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

    let mut report = source_terminal.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_status",
            "side_effect_free": true,
            "base_url": "native",
            "source_activation_command_result_receipt_terminal_operator_decision_public_claim_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_terminal_operator_decision_public_claim_gate": source_str("gate"),
            "source_activation_command_result_receipt_terminal_operator_decision_public_claim_ready": source_ready,
            "source_activation_command_result_receipt_terminal_operator_decision_public_claim_status": source_status,
            "source_activation_command_result_receipt_terminal_operator_decision_public_claim_report_sha256": source_report_sha256,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_release_artifact_publication_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_v1",
            "activation_command_result_receipt_release_artifact_publication_mode": "native_route_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_no_artifact_no_publication_no_claim_no_distribution_no_authority_no_live",
            "activation_command_result_receipt_release_artifact_publication_decision": "runtime_provider_router_activation_command_result_receipt_cannot_promote_terminal_operator_decision_or_public_claim_denial_into_release_artifact_publication_authority",
            "source_terminal_operator_decision_public_claim_fixture_count": source_u64("activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"),
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
            "activation_command_result_receipt_release_artifact_publication_fixtures": release_publication_fixtures,
            "activation_command_result_receipt_release_artifact_publication_fixture_count": release_publication_fixture_count,
            "blocked_activation_command_result_receipt_release_artifact_publication_fixture_count": release_publication_fixture_count,
            "noop_activation_command_result_receipt_release_artifact_publication_fixture_count": release_publication_fixture_count,
            "allowed_activation_command_result_receipt_release_artifact_publication_fixture_count": 0,
            "accepted_activation_command_result_receipt_release_artifact_publication_fixture_count": 0,
            "activation_command_result_receipt_release_artifact_publication_performed_count": 0,
            "release_artifact_written_count": 0,
            "public_artifact_written_count": 0,
            "public_distribution_performed_count": 0,
            "publication_manifest_written_count": 0,
            "publication_queue_enqueued_count": 0,
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
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_release_artifact_publication_allowed": false,
            "activation_command_result_receipt_release_artifact_publication_requested": false,
            "activation_command_result_receipt_release_artifact_publication_accepted": false,
            "activation_command_result_receipt_release_artifact_publication_recorded": false,
            "activation_command_result_receipt_release_artifact_publication_persisted": false,
            "activation_command_result_receipt_release_artifact_publication_materialized": false,
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
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
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
            "raw_payload_plaintext_recorded": false,
            "raw_payload_plaintext_persisted": false,
            "secret_material_read": false,
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
            "denied_by_activation_command_result_receipt_release_artifact_publication": denials,
            "denied_by_activation_command_result_receipt_release_artifact_publication_count": denied_count,
            "source_terminal_operator_decision_public_claim_denial_count": source_u64("denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count"),
            "release_artifact_publication_forbidden": true,
            "release_artifact_write_forbidden": true,
            "public_artifact_write_forbidden": true,
            "artifact_signature_notarization_forbidden": true,
            "publication_queue_forbidden": true,
            "publication_manifest_forbidden": true,
            "public_distribution_forbidden": true,
            "public_release_publication_forbidden": true,
            "public_ga_claim_forbidden": true,
            "terminal_operator_decision_release_approval_forbidden": true,
            "activation_from_release_artifact_publication_forbidden": true,
            "runtime_provider_memory_kg_release_artifact_publication_forbidden": true,
            "external_public_install_restart_active_binary_publication_forbidden": true,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial",
                    "status": "allowed_report_only",
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_consolidated_memory_intelligence_kg_audit",
                    "status": "allowed_read_only_next_slice",
                    "writes_release_artifact": false,
                    "claims_public_release": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "publishes_release_artifact": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_release_artifact_publication_allowed",
            "activation_command_result_receipt_release_artifact_publication_requested",
            "activation_command_result_receipt_release_artifact_publication_accepted",
            "activation_command_result_receipt_release_artifact_publication_recorded",
            "activation_command_result_receipt_release_artifact_publication_persisted",
            "activation_command_result_receipt_release_artifact_publication_materialized",
            "release_artifact_publication_allowed",
            "release_artifact_publication_requested",
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
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_allowed_by_release_artifact_publication",
            "activation_allowed_by_terminal_operator_decision",
            "activation_allowed_by_result_receipt",
            "activation_allowed",
            "activation_performed",
            "activation_activated",
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
            "service_restart_performed",
            "service_restarted",
            "active_binary_mutated",
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        if !report_object
            .get("side_effects")
            .is_some_and(serde_json::Value::is_object)
        {
            report_object.insert("side_effects".to_string(), serde_json::json!({}));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_release_artifact_publication_recorded",
            "activation_command_result_receipt_release_artifact_publication_persisted",
            "activation_command_result_receipt_release_artifact_publication_materialized",
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
            "activation_command_result_receipt_terminal_operator_decision_recorded",
            "activation_command_result_receipt_terminal_operator_decision_persisted",
            "activation_command_result_receipt_terminal_operator_decision_materialized",
            "activation_command_result_receipt_terminal_operator_decision_filesystem_written",
            "activation_command_result_receipt_public_claim_recorded",
            "activation_command_result_receipt_public_claim_persisted",
            "activation_command_result_receipt_public_claim_materialized",
            "activation_command_result_receipt_public_claim_promoted",
            "activation_command_result_receipt_public_release_published",
            "activation_command_result_receipt_public_distribution_performed",
            "activation_command_result_receipt_public_artifact_written",
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

