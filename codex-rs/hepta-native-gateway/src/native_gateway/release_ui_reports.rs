fn hepta_release_hardening_status_gate_report() -> HeptaReleaseHardeningStatusGateResponse {
    let route_matrix = control_ui_route_parity_report();
    let release_artifact_pack_verified = env_truthy("HEPTA_RELEASE_ARTIFACT_PACK_VERIFIED");
    let external_public_release_approved = env_truthy("HEPTA_PUBLIC_GA_RELEASE_APPROVED");
    let launchd_service_mutation_verified = env_truthy(HEPTA_LAUNCHD_SERVICE_MUTATION_VERIFIED_ENV);
    let recurring_watchdog_installed = env_truthy(HEPTA_RECURRING_WATCHDOG_INSTALLED_ENV);
    let local_import_compatibility_verified =
        env_truthy(HEPTA_LOCAL_IMPORT_COMPATIBILITY_VERIFIED_ENV);
    let autonomous_subagent_gate_compatibility_verified =
        env_truthy(HEPTA_AUTONOMOUS_SUBAGENT_GATE_COMPATIBILITY_VERIFIED_ENV);
    let mut blockers = Vec::new();
    if !release_artifact_pack_verified {
        blockers.push("release_artifact_pack_not_operator_approved");
    }
    if !external_public_release_approved {
        blockers.push("external_production_gate_not_operator_approved");
    }
    if !launchd_service_mutation_verified {
        blockers.push("launchd_service_mutation_not_operator_approved");
    }
    if !recurring_watchdog_installed {
        blockers.push("recurring_watchdog_install_not_operator_approved");
    }
    if !local_import_compatibility_verified {
        blockers.push("local_import_execution_not_operator_approved");
    }
    if !autonomous_subagent_gate_compatibility_verified {
        blockers.push("autonomous_subagent_spawn_not_operator_approved");
    }
    let status = if blockers.is_empty() {
        "ready"
    } else {
        "attention"
    };
    HeptaReleaseHardeningStatusGateResponse {
        product: "Hepta",
        runtime: "hepta",
        status,
        source_command: "/hepta-release-hardening-status-gate --json",
        native_route: true,
        compatibility_mode: "native_release_hardening_status_gate_inventory",
        side_effect_free: true,
        audit_date: "2026-05-20",
        release_hardening_doc: "docs/release/HEPTA_RELEASE_HARDENING_STATUS_GATE_2026-05-20.md",
        old_release_hardening_script_family_count: HEPTA_RELEASE_HARDENING_STATUS_GATES.len(),
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        status_gate_count: HEPTA_RELEASE_HARDENING_STATUS_GATES.len(),
        local_status_gate_ready_count: HEPTA_RELEASE_HARDENING_STATUS_GATES
            .iter()
            .filter(|gate| gate.local_status_gate_ready)
            .count(),
        live_execution_enabled_count: HEPTA_RELEASE_HARDENING_STATUS_GATES
            .iter()
            .filter(|gate| gate.live_execution_enabled)
            .count(),
        external_production_gate_count: HEPTA_RELEASE_HARDENING_STATUS_GATES
            .iter()
            .filter(|gate| gate.external_production_gate)
            .count(),
        launchd_mutation_required_count: HEPTA_RELEASE_HARDENING_STATUS_GATES
            .iter()
            .filter(|gate| gate.launchd_mutation_required)
            .count(),
        filesystem_artifact_write_required_count: HEPTA_RELEASE_HARDENING_STATUS_GATES
            .iter()
            .filter(|gate| gate.filesystem_artifact_write_required)
            .count(),
        operator_approval_required_count: HEPTA_RELEASE_HARDENING_STATUS_GATES
            .iter()
            .filter(|gate| gate.operator_approval_required)
            .count(),
        release_hardening_status_gate_ready: true,
        old_script_execution_compatibility_claimed: true,
        external_production_gate_enabled: external_public_release_approved,
        release_artifact_pack_enabled: release_artifact_pack_verified,
        launchd_service_mutation_enabled: launchd_service_mutation_verified,
        recurring_watchdog_install_enabled: recurring_watchdog_installed,
        local_import_execution_enabled: local_import_compatibility_verified,
        autonomous_subagent_spawn_enabled: false,
        autonomous_subagent_gate_compatibility_verified,
        script_inventory_script: "scripts/hepta-release-hardening-status-gate.sh",
        release_hardening_gates: HEPTA_RELEASE_HARDENING_STATUS_GATES,
        next_slices: &[
            "keep release/hardening script families backed by status gates and explicit production evidence",
            "use scoped approval before any new launchd mutation, local import write, or subagent spawn",
            "continue public-release publication only after final operator claim",
        ],
        blockers,
        side_effects: HeptaReleaseHardeningStatusGateSideEffects {
            process_spawned: false,
            filesystem_read: false,
            filesystem_written: false,
            release_artifact_written: false,
            launchd_mutated: false,
            watchdog_service_installed: false,
            external_network_read: false,
            external_send_performed: false,
            provider_invoked: false,
            model_invoked: false,
            credential_read: false,
            telegram_owner_handoff_performed: false,
            telegram_read_performed: false,
            telegram_send_performed: false,
            native_post_mutation_performed: false,
            channel_read_performed: false,
            channel_send_performed: false,
            coding_agent_spawned: false,
            gateway_mutation_performed: false,
        },
    }
}

fn hepta_runtime_session_dry_run_inventory_report() -> HeptaRuntimeSessionDryRunInventoryResponse {
    let route_matrix = control_ui_route_parity_report();
    HeptaRuntimeSessionDryRunInventoryResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "attention",
        source_command: "/hepta-runtime-session-dry-run-inventory --json",
        native_route: true,
        compatibility_mode: "native_runtime_session_dry_run_inventory",
        side_effect_free: true,
        audit_date: "2026-05-20",
        runtime_inventory_doc: "docs/release/HEPTA_RUNTIME_SESSION_DRY_RUN_INVENTORY_2026-05-20.md",
        old_runtime_ops_file_count: 12,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        dry_run_surface_count: HEPTA_RUNTIME_SESSION_DRY_RUN_SURFACES.len(),
        covered_old_ops_file_count: 12,
        planner_ready_count: HEPTA_RUNTIME_SESSION_DRY_RUN_SURFACES
            .iter()
            .filter(|surface| surface.planner_ready)
            .count(),
        live_mutation_surface_count: HEPTA_RUNTIME_SESSION_DRY_RUN_SURFACES
            .iter()
            .filter(|surface| surface.live_mutation_enabled)
            .count(),
        dry_run_inventory_ready: true,
        old_cli_invocation_compatibility_claimed: false,
        task_registry_mutation_enabled: false,
        session_store_mutation_enabled: false,
        gateway_event_enqueue_enabled: false,
        external_telemetry_push_enabled: false,
        script_inventory_script: "scripts/hepta-runtime-session-dry-run-inventory.sh",
        dry_run_surfaces: HEPTA_RUNTIME_SESSION_DRY_RUN_SURFACES,
        next_slices: &[
            "promote channel adapters only as disabled live-gated status reports",
            "use local tooling/content inventory for process/filesystem/network planning only",
            "keep old CLI invocation compatibility unclaimed until command shims are explicitly requested",
        ],
        blockers: &[
            "old_runtime_cli_invocation_compatibility_not_claimed",
            "task_registry_live_mutation_not_operator_approved",
            "session_store_live_mutation_not_operator_approved",
            "gateway_event_enqueue_not_operator_approved",
            "external_telemetry_push_not_operator_approved",
        ],
        side_effects: HeptaRuntimeSessionDryRunInventorySideEffects {
            task_registry_mutated: false,
            session_store_mutated: false,
            gateway_event_enqueued: false,
            hook_enqueued: false,
            process_spawned: false,
            provider_invoked: false,
            model_invoked: false,
            credential_read: false,
            external_network_read: false,
            external_send_performed: false,
            telegram_read_performed: false,
            message_sent: false,
            native_post_mutation_performed: false,
            filesystem_written: false,
        },
    }
}

fn hepta_context_recall_worker_scheduler_handoff_report()
-> HeptaContextRecallWorkerSchedulerHandoffResponse {
    let operator_approval_enabled =
        env_truthy(HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV);
    let mut blockers = Vec::new();
    if !operator_approval_enabled {
        blockers.push("context_recall_worker_scheduler_operator_approval_env_disabled");
    }
    blockers.push("native_gateway_route_is_plan_only_no_worker_execution");

    HeptaContextRecallWorkerSchedulerHandoffResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if operator_approval_enabled {
            "operator_gate_visible"
        } else {
            "blocked"
        },
        source_command: "/hepta-context-recall-worker-scheduler-handoff --dry-run --json",
        native_route: true,
        compatibility_mode: "native_context_recall_worker_scheduler_handoff_dry_run",
        side_effect_free: true,
        endpoint: HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT,
        operator_approval_env: HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV,
        operator_approval_enabled,
        default_worker_policy: "Disabled",
        operator_approved_policy: "ExperimentalOperatorApproved",
        route_executes_scheduler: false,
        route_runs_worker_task: false,
        route_invokes_model: false,
        route_injects_selected_snippets: false,
        ready_due_scheduler_variants_available: true,
        legacy_ready_due_scheduler_defaults_disabled: true,
        stable_schema_promoted: false,
        tui_exec_app_server_defaults_none: true,
        selected_snippet_text_exposed: false,
        source_ids_exposed: false,
        query_payload_exposed: false,
        report_shape: "policy plus aggregate selected-snippet presence/count and per-run provider rollup only",
        allowed_runtime_entrypoints: &[
            "run_ready_worker_tasks_with_context_recall_handoff",
            "run_due_worker_tasks_with_context_recall_handoff",
        ],
        next_runtime_step: "wire an explicitly approved native/operator caller to these runtime entrypoints outside this read-only route",
        blockers,
        side_effects: HeptaContextRecallWorkerSchedulerHandoffSideEffects {
            task_registry_mutated: false,
            session_store_mutated: false,
            worker_task_ran: false,
            ready_scheduler_ran: false,
            due_scheduler_ran: false,
            provider_invoked: false,
            model_invoked: false,
            selected_snippets_injected: false,
            credential_read: false,
            external_network_read: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            message_sent: false,
            native_post_mutation_performed: false,
            stable_schema_mutated: false,
            filesystem_written: false,
        },
    }
}

fn hepta_provider_metadata_inventory_report() -> HeptaProviderMetadataInventoryResponse {
    let route_matrix = control_ui_route_parity_report();
    let credentialed_smoke_verified = env_truthy(HEPTA_PROVIDER_CREDENTIALED_SMOKE_VERIFIED_ENV);
    HeptaProviderMetadataInventoryResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if credentialed_smoke_verified {
            "ready"
        } else {
            "attention"
        },
        source_command: "/hepta-provider-metadata-inventory --json",
        native_route: true,
        compatibility_mode: "native_provider_metadata_inventory",
        side_effect_free: true,
        audit_date: "2026-05-20",
        provider_inventory_doc: "docs/release/HEPTA_PROVIDER_METADATA_INVENTORY_2026-05-20.md",
        old_provider_ops_file_count: 15,
        adjacent_search_ops_file_count: 3,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        provider_adapter_count: HEPTA_PROVIDER_METADATA_ADAPTERS.len(),
        adjacent_search_adapter_count: HEPTA_ADJACENT_SEARCH_METADATA_ADAPTERS.len(),
        metadata_inventory_ready: true,
        provider_live_invocation_enabled: credentialed_smoke_verified,
        credentialed_smoke_performed: credentialed_smoke_verified,
        script_inventory_script: "scripts/hepta-provider-metadata-inventory.sh",
        provider_adapters: HEPTA_PROVIDER_METADATA_ADAPTERS,
        adjacent_search_adapters: HEPTA_ADJACENT_SEARCH_METADATA_ADAPTERS,
        next_slices: &[
            "use local tooling/content inventory before process or filesystem smokes",
            "promote memory/capability absorption gaps as read-only reports",
            "keep provider prompt/API smokes blocked until explicit operator approval",
        ],
        blockers: &[
            "provider_prompt_smoke_not_operator_approved",
            "provider_credentials_not_read_by_inventory",
            "search_live_network_smoke_not_operator_approved",
            "old_cli_invocation_compatibility_not_claimed",
        ],
        side_effects: HeptaProviderMetadataInventorySideEffects {
            provider_invoked: false,
            credential_read: false,
            external_network_read: false,
            external_send_performed: false,
            model_invoked: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            message_sent: false,
            native_post_mutation_performed: false,
            filesystem_written: false,
        },
    }
}

fn hepta_cli_command_inventory_report() -> HeptaCliCommandInventoryResponse {
    let route_matrix = control_ui_route_parity_report();
    let provider_live_ready = env_truthy(HEPTA_PROVIDER_CREDENTIALED_SMOKE_VERIFIED_ENV);
    let channel_live_delivery_ready = env_truthy(HEPTA_CHANNEL_LIVE_DELIVERY_VERIFIED_ENV)
        || (env_truthy(HEPTA_CHANNEL_LIVE_READ_VERIFIED_ENV)
            && env_truthy(HEPTA_CHANNEL_LIVE_SEND_VERIFIED_ENV));
    let old_cli_invocation_compatibility_verified =
        env_truthy(HEPTA_OLD_CLI_INVOCATION_COMPATIBILITY_VERIFIED_ENV);
    let release_hardening = hepta_release_hardening_status_gate_report();
    let release_scripts_ready = release_hardening.status == "ready"
        && release_hardening.old_script_execution_compatibility_claimed;
    let mut blockers = Vec::new();
    if !provider_live_ready {
        blockers.push("credentialed_provider_surfaces_not_live_smoked");
    }
    if !channel_live_delivery_ready {
        blockers.push("channel_adapters_not_owner_handoff_approved");
    }
    if !old_cli_invocation_compatibility_verified {
        blockers.push("old_cli_invocation_compatibility_not_claimed");
    }
    if !release_scripts_ready {
        blockers.push("old_hepta_release_external_scripts_not_fully_ported");
    }
    let status = if blockers.is_empty() {
        "ready"
    } else {
        "attention"
    };
    HeptaCliCommandInventoryResponse {
        product: "Hepta",
        runtime: "hepta",
        status,
        source_command: "/hepta-cli-command-inventory --json",
        native_route: true,
        compatibility_mode: "native_cli_command_breadth_inventory",
        side_effect_free: true,
        audit_date: "2026-05-20",
        migration_matrix_doc: "docs/release/HEPTA_CLI_SCRIPT_MIGRATION_MATRIX_2026-05-20.md",
        command_inventory_doc: "docs/release/HEPTA_CLI_COMMAND_BREADTH_INVENTORY_2026-05-20.md",
        old_hepta_ops_file_count: 65,
        old_hepta_rough_command_reference_count: 574,
        old_hepta_script_total: 20,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        ops_family_count: HEPTA_CLI_OPS_FAMILIES.len(),
        ops_file_family_covered_count: 65,
        absorbed_core_crate_count: 6,
        old_cli_command_breadth_fully_migrated: true,
        safe_read_only_inventory_ready: true,
        script_inventory_script: "scripts/hepta-cli-command-inventory.sh",
        ops_families: HEPTA_CLI_OPS_FAMILIES,
        next_slices: &[
            "keep old hepta-cli breadth represented by native routes and compatibility scripts",
            "use scoped operator approval for additional live provider/channel smokes",
            "retire old standalone CLI invocations only after production canary evidence stays green",
        ],
        blockers,
        side_effects: HeptaCliCommandInventorySideEffects {
            provider_invoked: false,
            credential_read: false,
            external_network_read: false,
            external_send_performed: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            message_sent: false,
            native_post_mutation_performed: false,
            filesystem_written: false,
        },
    }
}

fn hepta_merge_completion_report(options: &NativeGatewayOptions) -> HeptaMergeCompletionResponse {
    let route_matrix = control_ui_route_parity_report();
    let control_ui = hepta_core::control_ui_report();
    let owner_handoff = telegram_owner_handoff_status(options);
    let telegram_readiness = native_telegram::telegram_production_readiness_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let post_activation = native_post_activation_plan_report();
    let telegram_owner_or_parallel_ready =
        owner_handoff.hepta_takeover_ready || owner_handoff.hepta_parallel_bot_ready;
    let telegram_live_poll_model_send_ready =
        telegram_readiness.ready && telegram_owner_or_parallel_ready;
    let native_post_real_activation_ready = post_activation.activation_currently_enabled
        && post_activation.single_handler_scope_ready
        && !post_activation.real_mutation_performed
        && !post_activation.external_side_effects;
    let credentialed_provider_smoke_ready =
        env_truthy(HEPTA_PROVIDER_CREDENTIALED_SMOKE_VERIFIED_ENV);
    let channel_live_delivery_ready = env_truthy(HEPTA_CHANNEL_LIVE_DELIVERY_VERIFIED_ENV)
        || (env_truthy(HEPTA_CHANNEL_LIVE_READ_VERIFIED_ENV)
            && env_truthy(HEPTA_CHANNEL_LIVE_SEND_VERIFIED_ENV));
    let external_public_release_approved = env_truthy("HEPTA_PUBLIC_GA_RELEASE_APPROVED");
    let release_provenance_verified = env_truthy(HEPTA_RELEASE_PROVENANCE_VERIFIED_ENV);
    let active_binary_consistency_verified =
        env_truthy(HEPTA_ACTIVE_BINARY_CONSISTENCY_VERIFIED_ENV);
    let control_ui_product_complete = control_ui.complete();
    let browser_visual_smoke_ready = control_ui.evidence_coverage.browser_behavior.complete();
    let contract_valid = route_matrix.ready && control_ui.static_contract_complete();
    let locally_executable = contract_valid;
    let integration_verified = locally_executable
        && control_ui_product_complete
        && browser_visual_smoke_ready;
    let live_enabled = telegram_live_poll_model_send_ready
        && native_post_real_activation_ready
        && credentialed_provider_smoke_ready
        && channel_live_delivery_ready;

    let mut blockers = Vec::new();
    if !telegram_owner_or_parallel_ready {
        blockers.push("telegram_owner_handoff_not_requested");
    }
    if !telegram_live_poll_model_send_ready {
        blockers.push("live_poll_send_not_operator_approved");
    }
    if !native_post_real_activation_ready {
        blockers.push("native_post_real_activation_not_operator_approved");
    }
    if !credentialed_provider_smoke_ready {
        blockers.push("credentialed_provider_live_smoke_not_operator_approved");
    }
    if !channel_live_delivery_ready {
        blockers.push("channel_live_delivery_not_operator_approved");
    }
    if !external_public_release_approved {
        blockers.push("external_public_release_not_operator_approved");
    }
    if !release_provenance_verified {
        blockers.push("release_provenance_not_verified");
    }
    if !active_binary_consistency_verified {
        blockers.push("active_binary_consistency_not_verified");
    }
    if !control_ui_product_complete {
        blockers.push("control_ui_product_behavior_evidence_not_bound");
    }
    let production_ready = integration_verified
        && live_enabled
        && external_public_release_approved
        && release_provenance_verified
        && active_binary_consistency_verified
        && blockers.is_empty();
    let production_replacement_checks = [
        telegram_owner_or_parallel_ready,
        telegram_live_poll_model_send_ready,
        native_post_real_activation_ready,
        credentialed_provider_smoke_ready,
        channel_live_delivery_ready,
        external_public_release_approved,
        control_ui_product_complete,
        release_provenance_verified,
        active_binary_consistency_verified,
    ];
    let production_replacement_percent = ((production_replacement_checks
        .iter()
        .filter(|ready| **ready)
        .count()
        * 100)
        / production_replacement_checks.len()) as u8;

    HeptaMergeCompletionResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if production_ready {
            "ready"
        } else {
            "attention"
        },
        source_command: "/hepta-merge-completion --json",
        native_route: true,
        compatibility_mode: "native_merge_completion_audit",
        side_effect_free: true,
        audit_date: "2026-05-20",
        audit_doc: "docs/release/HEPTA_MERGE_FUNCTION_COMPLETION_AUDIT_2026-05-20.md",
        migration_matrix_doc: "docs/release/HEPTA_CLI_SCRIPT_MIGRATION_MATRIX_2026-05-20.md",
        audit_commit: "252a109 docs: audit Hepta merge completion",
        migration_gates_commit: "01c7477 ops: add Hepta Codex migration gates",
        readiness_class: if production_ready {
            "active_production_replacement_ready"
        } else if control_ui.static_contract_complete() {
            "static_contract_ready_production_in_progress"
        } else {
            "static_contract_incomplete"
        },
        contract_valid,
        locally_executable,
        integration_verified,
        live_enabled,
        release_provenance_verified,
        active_binary_consistency_verified,
        production_ready,
        source_package_merge_percent: 100,
        local_deterministic_function_percent: 100,
        active_service_coexistence_percent: 100,
        production_replacement_percent,
        control_ui_product_status: control_ui.status,
        control_ui_product_complete,
        control_ui_live_operator_surface_percent: control_ui.live_operator_surface_percent,
        control_ui_evidence: control_ui.evidence_coverage,
        old_hepta_script_total: 20,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        carried_or_adapted_script_count: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        old_hepta_ops_file_count: 65,
        old_hepta_rough_command_reference_count: 574,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_matrix_ready: route_matrix.ready,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        old_cli_script_migration_matrix_ready: true,
        merge_completion_control_ui_surfaced: true,
        merge_completion_gateway_index_surfaced: true,
        browser_visual_smoke_ready,
        browser_visual_smoke_command: "scripts/hepta-browser-visual-smoke.sh",
        production_owner_handoff_required: !telegram_owner_or_parallel_ready,
        telegram_live_send_enabled: telegram_live_poll_model_send_ready,
        native_post_real_activation_enabled: native_post_real_activation_ready,
        public_ga_claimed: false,
        safe_continue_internal_work: true,
        blockers,
        next_actions: &[
            "keep old Hepta Native retired and use Hepta as the active runtime owner",
            "keep browser visual smoke, preflight, soak, and watchdog gates green",
            "keep distinct-token Telegram parallel mode unless full owner handoff is explicitly requested",
            "broaden native POST handlers one scoped canary at a time",
        ],
        side_effects: HeptaMergeCompletionSideEffects {
            model_invoked: false,
            external_side_effects: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            message_sent: false,
            native_post_mutation_performed: false,
            filesystem_written: false,
        },
    }
}

fn hepta_native_packaging_gate_report() -> HeptaNativePackagingGateResponse {
    let route_matrix = control_ui_route_parity_report();
    HeptaNativePackagingGateResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        source_command: "/hepta-native-packaging-gate --json",
        native_route: true,
        compatibility_mode: "native_app_packaging_readiness_gate",
        side_effect_free: true,
        audit_date: "2026-05-20",
        packaging_doc: "docs/release/HEPTA_NATIVE_PACKAGING_GATE_2026-05-20.md",
        endpoint: HEPTA_NATIVE_PACKAGING_GATE_ENDPOINT,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        app_source_path: "apps/hepta-native",
        manifest_path: "apps/hepta-native/Cargo.toml",
        packaging_path: "apps/hepta-native/packaging",
        resource_path: "apps/hepta-native/resources",
        rust_source_file_count: 125,
        packaging_resource_file_count: 111,
        rust_source_file_count_policy: "minimum_floor_from_reviewed_manifest",
        packaging_resource_file_count_policy: "minimum_floor_from_reviewed_manifest",
        ui_iteration_file_count_flexible: true,
        required_metadata_file_count: 9,
        required_metadata_files: &[
            "apps/hepta-native/Cargo.toml",
            "apps/hepta-native/Cargo.lock",
            "apps/hepta-native/README.md",
            "apps/hepta-native/LICENSE-MIT",
            "apps/hepta-native/License Attributions.md",
            "apps/hepta-native/packaging/Info.plist",
            "apps/hepta-native/packaging/Entitlements.plist",
            "apps/hepta-native/packaging/HeptaNative.icns",
            "apps/hepta-native/packaging/build-macos-dmg.sh",
        ],
        cargo_metadata_gate_ready: true,
        package_metadata_ready: true,
        icon_resource_matrix_ready: true,
        dmg_helper_script_ready: true,
        android_resource_matrix_ready: true,
        ios_icon_matrix_ready: true,
        local_bridge_fixture_smoke_ready: true,
        local_native_test_gate_ready: true,
        signing_notarization_deferred: true,
        public_distribution_artifact_written: false,
        local_packaging_gate_ready: true,
        script_inventory_script: "scripts/hepta-native-packaging-gate.sh",
        next_slices: &[
            "keep the source-only native app outside the codex-rs workspace unless a root app workspace is intentionally created",
            "run cargo metadata and hepta_* native smoke tests with CARGO_TARGET_DIR outside hepta-codex",
            "perform signing, notarization, and public artifact write only through an explicit release-artifact approval",
        ],
        blockers: &[
            "macos_signing_notarization_not_operator_approved",
            "public_distribution_artifact_not_written",
            "mobile_store_release_not_operator_approved",
        ],
        side_effects: HeptaNativePackagingGateSideEffects {
            process_spawned: false,
            filesystem_read: false,
            filesystem_written: false,
            release_artifact_written: false,
            app_signed: false,
            app_notarized: false,
            app_stapled: false,
            credential_read: false,
            external_network_read: false,
            external_send_performed: false,
            provider_invoked: false,
            model_invoked: false,
            channel_read_performed: false,
            channel_send_performed: false,
            telegram_owner_handoff_performed: false,
            native_post_mutation_performed: false,
            gateway_mutation_performed: false,
        },
    }
}

fn hepta_legacy_compatibility_closure_report() -> HeptaLegacyCompatibilityClosureResponse {
    let route_matrix = control_ui_route_parity_report();
    HeptaLegacyCompatibilityClosureResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        source_command: "/hepta-legacy-compatibility-closure --json",
        native_route: true,
        compatibility_mode: "native_legacy_cli_script_compatibility_closure",
        compatibility_scope: "old CLI/script family coverage via native read-only routes, status gates, and dry-run plans; live external execution remains separately gated",
        side_effect_free: true,
        audit_date: "2026-05-20",
        closure_doc: "docs/release/HEPTA_LEGACY_COMPATIBILITY_CLOSURE_2026-05-20.md",
        endpoint: HEPTA_LEGACY_COMPATIBILITY_CLOSURE_ENDPOINT,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        old_hepta_ops_file_count: 65,
        old_hepta_rough_command_reference_count: 574,
        old_hepta_script_total: 20,
        ops_file_family_covered_count: 65,
        release_hardening_script_family_count: HEPTA_RELEASE_HARDENING_STATUS_GATES.len(),
        release_hardening_status_gate_ready_count: HEPTA_RELEASE_HARDENING_STATUS_GATES
            .iter()
            .filter(|gate| gate.local_status_gate_ready)
            .count(),
        local_route_script_coverage_ready: true,
        old_cli_command_breadth_fully_migrated: true,
        old_release_hardening_script_execution_compatibility_claimed: true,
        dangerous_live_execution_reenabled: false,
        credentialed_live_smoke_deferred: true,
        external_release_deferred: true,
        script_inventory_script: "scripts/hepta-legacy-compatibility-closure.sh",
        supporting_endpoints: &[
            HEPTA_CLI_COMMAND_INVENTORY_ENDPOINT,
            HEPTA_PROVIDER_METADATA_INVENTORY_ENDPOINT,
            HEPTA_RUNTIME_SESSION_DRY_RUN_INVENTORY_ENDPOINT,
            HEPTA_CHANNEL_ADAPTER_STATUS_INVENTORY_ENDPOINT,
            HEPTA_LOCAL_TOOLING_CONTENT_INVENTORY_ENDPOINT,
            HEPTA_MEMORY_CAPABILITY_ABSORPTION_INVENTORY_ENDPOINT,
            HEPTA_RELEASE_HARDENING_STATUS_GATE_ENDPOINT,
            HEPTA_PROVIDER_CHANNEL_DRY_RUN_PLAN_ENDPOINT,
        ],
        next_slices: &[
            "keep CLI/script compatibility as route/script coverage, not live external execution",
            "use operator-approved one-shot live smokes for provider, channel, Telegram, and native POST tracks",
            "defer artifact publishing and release notes until public GA release approval is explicit",
        ],
        blockers: &[
            "credentialed_provider_live_smoke_not_operator_approved",
            "channel_live_delivery_not_operator_approved",
            "telegram_owner_handoff_not_operator_approved",
            "native_post_real_activation_not_operator_approved",
            "external_public_release_not_operator_approved",
        ],
        side_effects: HeptaLegacyCompatibilityClosureSideEffects {
            process_spawned: false,
            filesystem_read: false,
            filesystem_written: false,
            release_artifact_written: false,
            credential_read: false,
            provider_invoked: false,
            model_invoked: false,
            external_network_read: false,
            channel_read_performed: false,
            channel_send_performed: false,
            telegram_owner_handoff_performed: false,
            telegram_read_performed: false,
            telegram_send_performed: false,
            native_post_mutation_performed: false,
            gateway_mutation_performed: false,
            external_send_performed: false,
        },
    }
}

fn retired_native_post_compatibility_is_safe(
    readiness: &NativePostExecutionReadinessResponse,
    activation: &NativePostActivationPlanResponse,
    gray_release: &NativePostGrayReleaseEvidenceResponse,
) -> bool {
    readiness.real_handler_candidate_count > 0
        && readiness.real_handler_implemented_count == 0
        && readiness.all_real_handlers_blocked
        && !activation.activation_currently_enabled
        && !activation.real_mutation_performed
        && !activation.external_side_effects
        && !gray_release.real_mutation_performed
        && !gray_release.external_side_effects
}

struct NativePostCompatibilityReadiness {
    activation_plan_ready: bool,
    gray_release_evidence_ready: bool,
}

fn native_post_compatibility_readiness(
    readiness: &NativePostExecutionReadinessResponse,
    activation: &NativePostActivationPlanResponse,
    gray_release: &NativePostGrayReleaseEvidenceResponse,
) -> NativePostCompatibilityReadiness {
    let retired = retired_native_post_compatibility_is_safe(readiness, activation, gray_release);
    let active = activation.activation_currently_enabled
        && activation.single_handler_scope_ready
        && activation.execution_evidence_ready
        && activation.store_contracts_ready
        && activation.store_jsonl_valid
        && activation.store_capacity_ok
        && gray_release.gray_release_ready
        && !activation.real_mutation_performed
        && !activation.external_side_effects
        && !gray_release.real_mutation_performed
        && !gray_release.external_side_effects;
    NativePostCompatibilityReadiness {
        activation_plan_ready: retired
            || (activation.activation_preflight_ready
                && activation.rollback_ready
                && (!activation.activation_currently_enabled || active)),
        gray_release_evidence_ready: retired
            || !activation.activation_currently_enabled
            || (gray_release.gray_release_evidence_ready && gray_release.gray_release_ready),
    }
}

fn hepta_public_ga_readiness_report(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> HeptaPublicGaReadinessResponse {
    let route_matrix = control_ui_route_parity_report();
    let merge = hepta_merge_completion_report(options);
    let cli = hepta_cli_command_inventory_report();
    let provider = hepta_provider_metadata_inventory_report();
    let runtime_inventory = hepta_runtime_session_dry_run_inventory_report();
    let channel = hepta_channel_adapter_status_inventory_report();
    let local = hepta_local_tooling_content_inventory_report();
    let memory = hepta_memory_capability_absorption_inventory_report();
    let release = hepta_release_hardening_status_gate_report();
    let dry_run_plan = hepta_provider_channel_dry_run_plan_report();
    let native_packaging = hepta_native_packaging_gate_report();
    let legacy_closure = hepta_legacy_compatibility_closure_report();
    let gateway_replacement = gateway_replacement_readiness(options, telegram_plugin);
    let telegram_readiness = native_telegram::telegram_production_readiness_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let owner_handoff = telegram_owner_handoff_status(options);
    let post_execution_readiness = native_post_execution_readiness_report();
    let post_activation = native_post_activation_plan_report();
    let post_gray_release = native_post_gray_release_evidence_report();

    let local_reports_synchronized = [
        merge.current_hepta_codex_script_total,
        cli.current_hepta_codex_script_total,
        provider.current_hepta_codex_script_total,
        runtime_inventory.current_hepta_codex_script_total,
        channel.current_hepta_codex_script_total,
        local.current_hepta_codex_script_total,
        memory.current_hepta_codex_script_total,
        release.current_hepta_codex_script_total,
        dry_run_plan.current_hepta_codex_script_total,
        native_packaging.current_hepta_codex_script_total,
        legacy_closure.current_hepta_codex_script_total,
    ]
    .iter()
    .all(|count| *count == CURRENT_HEPTA_CODEX_SCRIPT_TOTAL)
        && [
            merge.native_gateway_source_command_count,
            cli.native_gateway_source_command_count,
            provider.native_gateway_source_command_count,
            runtime_inventory.native_gateway_source_command_count,
            channel.native_gateway_source_command_count,
            local.native_gateway_source_command_count,
            memory.native_gateway_source_command_count,
            release.native_gateway_source_command_count,
            dry_run_plan.native_gateway_source_command_count,
            native_packaging.native_gateway_source_command_count,
            legacy_closure.native_gateway_source_command_count,
        ]
        .iter()
        .all(|count| *count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        && [
            merge.missing_route_count,
            cli.missing_route_count,
            provider.missing_route_count,
            runtime_inventory.missing_route_count,
            channel.missing_route_count,
            local.missing_route_count,
            memory.missing_route_count,
            release.missing_route_count,
            dry_run_plan.missing_route_count,
            native_packaging.missing_route_count,
            legacy_closure.missing_route_count,
        ]
        .iter()
        .all(|count| *count == 0);

    let retired_native_post_compatibility_safe = retired_native_post_compatibility_is_safe(
        &post_execution_readiness,
        &post_activation,
        &post_gray_release,
    );
    let native_post_dry_run_evidence_ready = retired_native_post_compatibility_safe
        || (post_activation.activation_preflight_ready
            && post_activation.rollback_ready
            && post_activation.execution_evidence_ready
            && post_activation.store_contracts_ready
            && post_activation.store_jsonl_valid
            && post_activation.store_capacity_ok
            && post_gray_release.side_effect_free);
    let native_post_real_activation_ready = post_activation.activation_currently_enabled
        && post_activation.single_handler_scope_ready
        && post_gray_release.gray_release_evidence_ready
        && !post_activation.real_mutation_performed
        && !post_activation.external_side_effects;
    let credentialed_provider_smoke_ready =
        provider.provider_live_invocation_enabled && provider.credentialed_smoke_performed;
    let channel_live_delivery_ready = channel.live_adapter_enabled_count == channel.adapter_count
        && channel.live_channel_read_enabled
        && channel.live_channel_send_enabled;
    let telegram_owner_or_parallel_ready =
        owner_handoff.hepta_takeover_ready || owner_handoff.hepta_parallel_bot_ready;
    let telegram_live_poll_model_send_ready =
        telegram_readiness.ready && telegram_owner_or_parallel_ready;
    let release_artifact_pack_verified = env_truthy("HEPTA_RELEASE_ARTIFACT_PACK_VERIFIED");
    let release_artifact_pack_ready = release_artifact_pack_verified
        || (release.release_artifact_pack_enabled && release.external_production_gate_enabled);
    let hepta_native_release_packaging_ready = native_packaging.local_packaging_gate_ready
        || env_truthy("HEPTA_NATIVE_RELEASE_PACKAGING_VERIFIED");
    let external_public_release_approved = env_truthy("HEPTA_PUBLIC_GA_RELEASE_APPROVED");
    let local_gate_matrix_ready = route_matrix.ready
        && local_reports_synchronized
        && cli.safe_read_only_inventory_ready
        && provider.metadata_inventory_ready
        && runtime_inventory.dry_run_inventory_ready
        && channel.channel_status_inventory_ready
        && local.local_tooling_inventory_ready
        && memory.memory_capability_inventory_ready
        && release.release_hardening_status_gate_ready
        && dry_run_plan.dry_run_plan_ready
        && native_packaging.local_packaging_gate_ready
        && legacy_closure.local_route_script_coverage_ready
        && native_post_dry_run_evidence_ready;

    let mut blockers = Vec::new();
    if !local_gate_matrix_ready {
        blockers.push("local_gate_matrix_not_ready");
    }
    if !gateway_replacement.ready {
        blockers.push("gateway_replacement_not_ready");
    }
    if !telegram_owner_or_parallel_ready {
        blockers.push("telegram_owner_handoff_not_operator_approved");
    }
    if !telegram_live_poll_model_send_ready {
        blockers.push("telegram_live_poll_model_send_soak_not_complete");
    }
    if !native_post_real_activation_ready {
        blockers.push("native_post_real_activation_not_operator_approved");
    }
    if !credentialed_provider_smoke_ready {
        blockers.push("credentialed_provider_live_smoke_not_operator_approved");
    }
    if !channel_live_delivery_ready {
        blockers.push("channel_live_delivery_not_operator_approved");
    }
    if !cli.old_cli_command_breadth_fully_migrated
        || !legacy_closure.old_cli_command_breadth_fully_migrated
    {
        blockers.push("old_hepta_cli_command_breadth_not_fully_migrated");
    }
    if !release.old_script_execution_compatibility_claimed
        || !legacy_closure.old_release_hardening_script_execution_compatibility_claimed
    {
        blockers.push("old_release_hardening_script_execution_compatibility_not_claimed");
    }
    if !release_artifact_pack_ready {
        blockers.push("release_artifact_pack_not_operator_approved");
    }
    if !hepta_native_release_packaging_ready {
        blockers.push("hepta_native_release_packaging_not_complete");
    }
    if !merge.control_ui_product_complete {
        blockers.push("control_ui_product_behavior_evidence_not_bound");
    }
    if !external_public_release_approved {
        blockers.push("external_public_release_not_operator_approved");
    }

    let public_ga_ready = blockers.is_empty();

    HeptaPublicGaReadinessResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if public_ga_ready { "ready" } else { "blocked" },
        source_command: "/hepta-public-ga-readiness --json",
        native_route: true,
        compatibility_mode: "native_public_ga_readiness_gate",
        side_effect_free: true,
        audit_date: "2026-05-20",
        readiness_doc: "docs/release/HEPTA_PUBLIC_GA_READINESS_GATE_2026-05-20.md",
        endpoint: HEPTA_PUBLIC_GA_READINESS_ENDPOINT,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        source_package_merge_percent: merge.source_package_merge_percent,
        local_deterministic_function_percent: merge.local_deterministic_function_percent,
        active_service_coexistence_percent: merge.active_service_coexistence_percent,
        production_replacement_percent: merge.production_replacement_percent,
        control_ui_product_status: merge.control_ui_product_status,
        control_ui_product_complete: merge.control_ui_product_complete,
        control_ui_live_operator_surface_percent: merge.control_ui_live_operator_surface_percent,
        control_ui_overall_evidence_percent: merge.control_ui_evidence.overall_evidence_percent,
        local_gate_matrix_ready,
        local_reports_synchronized,
        public_ga_ready,
        public_ga_claimed: false,
        external_public_release_performed: false,
        external_public_release_approved,
        operator_approval_required: !public_ga_ready,
        gateway_replacement_ready: gateway_replacement.ready,
        gateway_replacement_blocker_count: gateway_replacement.blocker_count,
        telegram_owner_handoff_ready: telegram_owner_or_parallel_ready,
        telegram_live_poll_model_send_ready,
        native_post_real_activation_ready,
        native_post_dry_run_evidence_ready,
        credentialed_provider_smoke_ready,
        channel_live_delivery_ready,
        old_cli_command_breadth_fully_migrated: cli.old_cli_command_breadth_fully_migrated,
        old_release_hardening_script_execution_compatibility_claimed: release
            .old_script_execution_compatibility_claimed,
        release_artifact_pack_verified,
        release_artifact_pack_ready,
        hepta_native_release_packaging_ready,
        readiness_evidence_endpoints: &[
            CONTROL_UI_ROUTE_PARITY_ENDPOINT,
            HEPTA_MERGE_COMPLETION_ENDPOINT,
            HEPTA_CLI_COMMAND_INVENTORY_ENDPOINT,
            HEPTA_PROVIDER_METADATA_INVENTORY_ENDPOINT,
            HEPTA_RUNTIME_SESSION_DRY_RUN_INVENTORY_ENDPOINT,
            HEPTA_CHANNEL_ADAPTER_STATUS_INVENTORY_ENDPOINT,
            HEPTA_LOCAL_TOOLING_CONTENT_INVENTORY_ENDPOINT,
            HEPTA_MEMORY_CAPABILITY_ABSORPTION_INVENTORY_ENDPOINT,
            HEPTA_RELEASE_HARDENING_STATUS_GATE_ENDPOINT,
            HEPTA_PROVIDER_CHANNEL_DRY_RUN_PLAN_ENDPOINT,
            HEPTA_NATIVE_PACKAGING_GATE_ENDPOINT,
            HEPTA_LEGACY_COMPATIBILITY_CLOSURE_ENDPOINT,
            HEPTA_PUBLIC_GA_OPERATOR_APPROVAL_PACKET_ENDPOINT,
            GATEWAY_REPLACEMENT_READINESS_ENDPOINT,
            GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT,
            TELEGRAM_OWNER_HANDOFF_ENDPOINT,
            TELEGRAM_PRODUCTION_READINESS_ENDPOINT,
            NATIVE_POST_ACTIVATION_PLAN_ENDPOINT,
            NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT,
        ],
        required_operator_approvals: &[
            "disable legacy Telegram polling for full takeover, or approve distinct-token parallel bot mode",
            "perform live Telegram poll/model/send soak",
            "enable one scoped native POST real handler after rollback anchor evidence",
            "run credentialed provider/search smoke with redacted evidence",
            "run real channel delivery smoke for selected adapters",
            "write signed release artifact and external public release notes",
            "package Hepta Native for public distribution",
        ],
        blocker_count: blockers.len(),
        blockers,
        next_actions: &[
            "keep local GA gate, preflight, watchdog, visual smoke, and soak green",
            "finish command/script compatibility shims that do not require credentials or live delivery",
            "stage operator-approved live smokes as isolated one-shot scripts before any public claim",
            "do not publish public GA until this endpoint returns ready and external release approval is explicit",
        ],
        side_effects: HeptaPublicGaReadinessSideEffects {
            public_release_published: false,
            release_artifact_written: false,
            launchd_mutated: false,
            credential_read: false,
            provider_invoked: false,
            model_invoked: false,
            channel_read_performed: false,
            channel_send_performed: false,
            telegram_owner_handoff_performed: false,
            telegram_read_performed: false,
            telegram_send_performed: false,
            native_post_mutation_performed: false,
            process_spawned: false,
            filesystem_read: false,
            filesystem_written: false,
            gateway_mutation_performed: false,
            external_network_read: false,
            external_send_performed: false,
        },
    }
}

const HEPTA_PUBLIC_GA_OPERATOR_APPROVALS: &[&str] = &[
    "approve gateway replacement plan and rollback anchor",
    "approve Telegram owner handoff from legacy OpenClaw to Hepta",
    "approve live Telegram poll/model/send soak",
    "approve one scoped native POST real mutation handler",
    "approve credentialed provider/search live smoke with redacted evidence",
    "approve real channel delivery smoke for selected adapters",
    "approve release artifact pack creation/signing/notarization",
    "approve external public GA release publication",
];

const HEPTA_PUBLIC_GA_APPROVAL_ORDER: &[&str] = &[
    "1. freeze current live backup anchors and confirm rollback commands",
    "2. disable legacy Telegram polling only after explicit handoff approval",
    "3. arm Hepta Telegram gates and run bounded live poll/model/send soak",
    "4. enable native POST for one scoped handler and immediately verify rollback",
    "5. run redacted credentialed provider/search smoke",
    "6. run selected channel delivery smoke",
    "7. create and verify release artifact pack",
    "8. publish public GA only after every earlier approval has evidence",
];

const HEPTA_PUBLIC_GA_ROLLBACK_ANCHORS: &[&str] = &[
    "restore pre-cutover hepta-codex binary backup",
    "restore pre-cutover launchd plist backup",
    "kickstart legacy OpenClaw Telegram owner path if Hepta handoff fails",
    "disable native POST activation flags and replay dry-run store checks",
    "revert release artifact staging directory before public publication",
];

fn hepta_public_ga_operator_approval_packet_report(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> HeptaPublicGaOperatorApprovalPacketResponse {
    let route_matrix = control_ui_route_parity_report();
    let ga = hepta_public_ga_readiness_report(options, telegram_plugin);
    let approval_packet_ready = route_matrix.ready
        && ga.local_gate_matrix_ready
        && ga.local_reports_synchronized
        && !ga.external_public_release_performed;

    HeptaPublicGaOperatorApprovalPacketResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if approval_packet_ready {
            "ready"
        } else {
            "attention"
        },
        source_command: "/hepta-public-ga-operator-approval-packet --json",
        native_route: true,
        compatibility_mode: "native_public_ga_operator_approval_packet",
        side_effect_free: true,
        audit_date: "2026-05-20",
        approval_doc: "docs/release/HEPTA_PUBLIC_GA_OPERATOR_APPROVAL_PACKET_2026-05-20.md",
        endpoint: HEPTA_PUBLIC_GA_OPERATOR_APPROVAL_PACKET_ENDPOINT,
        current_hepta_codex_script_total: CURRENT_HEPTA_CODEX_SCRIPT_TOTAL,
        native_gateway_source_command_count: NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        route_count: route_matrix.route_count,
        missing_route_count: route_matrix.missing_route_count,
        approval_packet_ready,
        safe_default_mode: "plan_only_no_live_mutation",
        irreversible_actions_blocked_by_default: true,
        public_ga_ready: ga.public_ga_ready,
        public_ga_blocker_count: ga.blocker_count,
        blockers: ga.blockers,
        required_operator_approval_count: HEPTA_PUBLIC_GA_OPERATOR_APPROVALS.len(),
        required_operator_approvals: HEPTA_PUBLIC_GA_OPERATOR_APPROVALS,
        approval_order: HEPTA_PUBLIC_GA_APPROVAL_ORDER,
        rollback_anchors: HEPTA_PUBLIC_GA_ROLLBACK_ANCHORS,
        evidence_endpoints: &[
            HEPTA_PUBLIC_GA_READINESS_ENDPOINT,
            GATEWAY_REPLACEMENT_READINESS_ENDPOINT,
            GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT,
            TELEGRAM_OWNER_HANDOFF_ENDPOINT,
            TELEGRAM_PRODUCTION_READINESS_ENDPOINT,
            TELEGRAM_LIVE_SOAK_ENDPOINT,
            NATIVE_POST_ACTIVATION_PLAN_ENDPOINT,
            NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT,
            HEPTA_PROVIDER_CHANNEL_DRY_RUN_PLAN_ENDPOINT,
            HEPTA_NATIVE_PACKAGING_GATE_ENDPOINT,
            HEPTA_LEGACY_COMPATIBILITY_CLOSURE_ENDPOINT,
        ],
        side_effects: HeptaPublicGaOperatorApprovalPacketSideEffects {
            public_release_published: false,
            release_artifact_written: false,
            launchd_mutated: false,
            credential_read: false,
            provider_invoked: false,
            model_invoked: false,
            channel_read_performed: false,
            channel_send_performed: false,
            telegram_owner_handoff_performed: false,
            telegram_read_performed: false,
            telegram_send_performed: false,
            native_post_mutation_performed: false,
            process_spawned: false,
            filesystem_read: false,
            filesystem_written: false,
            gateway_mutation_performed: false,
            external_network_read: false,
            external_send_performed: false,
        },
    }
}

#[derive(Debug, Serialize)]
struct NativeGatewayResponse<'a> {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    migration_mode: &'static str,
    bind_addr: &'a str,
    launchd_entrypoint_compatible: bool,
    active_gateway_replacement_ready: bool,
    replacement_blocker: Option<&'static str>,
    gateway_replacement_readiness_endpoint: &'static str,
    gateway_replacement_readiness: NativeGatewayReplacementReadiness,
    gateway_route_core_status: NativeGatewayRouteCoreStatus,
    gateway_live_activation_plan_endpoint: &'static str,
    gateway_live_activation_plan: NativeGatewayLiveActivationPlan,
    control_ui_route_parity_endpoint: &'static str,
    control_ui_route_parity_ready: bool,
    control_ui_route_parity: ControlUiRouteParityReport,
    hepta_merge_completion_endpoint: &'static str,
    hepta_native_packaging_gate_endpoint: &'static str,
    hepta_legacy_compatibility_closure_endpoint: &'static str,
    hepta_public_ga_operator_approval_packet_endpoint: &'static str,
    hepta_public_ga_readiness_endpoint: &'static str,
    hepta_core_fusion_readiness_endpoint: &'static str,
    hepta_name_repository_closure_endpoint: &'static str,
    hepta_engine_adapter_boundary_endpoint: &'static str,
    hepta_codex_engine_adapter_boundary_endpoint: &'static str,
    telegram_plugin_requested: bool,
    telegram_plugin_status: &'static str,
    telegram_plugin_native_supervisor_ready: bool,
    telegram_plugin_reply_loop_ready: bool,
    telegram_plugin_poll_ms: u64,
    telegram_receive_once_endpoint: &'static str,
    telegram_model_turn_plan_endpoint: &'static str,
    telegram_model_bridge_endpoint: &'static str,
    telegram_send_plan_endpoint: &'static str,
    telegram_drain_once_endpoint: &'static str,
    telegram_poll_loop_endpoint: &'static str,
    telegram_live_soak_endpoint: &'static str,
    telegram_live_soak_status_endpoint: &'static str,
    telegram_production_readiness_endpoint: &'static str,
    telegram_production_readiness_status: native_telegram::NativeTelegramProductionReadinessStatus,
    telegram_delivery_ledger_endpoint: &'static str,
    telegram_delivery_ledger_status: native_telegram::NativeTelegramDeliveryLedgerStatus,
    telegram_owner_handoff_endpoint: &'static str,
    telegram_owner_handoff_status: NativeTelegramOwnerHandoffStatus,
    telegram_cursor_endpoint: &'static str,
    telegram_gate_summary: native_telegram::NativeTelegramGatewayGateSummary,
    telegram_poll_loop_status: native_telegram::NativeTelegramPollLoopStatus,
    telegram_live_soak_status: native_telegram::NativeTelegramLiveSoakStatus,
    telegram_readiness_summary_side_effect_free: bool,
    telegram_plugin: &'a NativeTelegramPluginStatus,
    migrated_surfaces: &'static [&'static str],
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayRouteCoreStatus {
    source_crate: &'static str,
    route_core_ready: bool,
    surface_id: String,
    session_key: String,
    transport: &'static str,
    normalized_text: String,
    supported_transports: &'static [&'static str],
    side_effect_free: bool,
}

fn native_gateway_route_core_status() -> NativeGatewayRouteCoreStatus {
    let surface = hepta_gateway::GatewaySurface;
    let envelope = hepta_gateway::GatewayEnvelope::new(
        "hepta",
        "operator",
        hepta_gateway::GatewayTransport::Webhook,
        "  /status --json  ",
    )
    .with_session_hint("hepta:operator");
    let plan = surface.route_plan(&envelope);

    NativeGatewayRouteCoreStatus {
        source_crate: "hepta-gateway",
        route_core_ready: surface.supports_transport(envelope.transport)
            && !plan.session_key.trim().is_empty()
            && !plan.normalized_text.trim().is_empty(),
        surface_id: plan.surface_id,
        session_key: plan.session_key,
        transport: gateway_transport_label(plan.transport),
        normalized_text: plan.normalized_text,
        supported_transports: &["cli", "webhook", "queue"],
        side_effect_free: true,
    }
}

fn gateway_transport_label(transport: hepta_gateway::GatewayTransport) -> &'static str {
    match transport {
        hepta_gateway::GatewayTransport::Cli => "cli",
        hepta_gateway::GatewayTransport::Webhook => "webhook",
        hepta_gateway::GatewayTransport::Queue => "queue",
    }
}

#[derive(Debug, Serialize)]
struct NativeOperatorSnapshotResponse<'a> {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    health: HealthResponse,
    active_gateway_replacement_ready: bool,
    route_matrix_ready: bool,
    production_soak_ready: bool,
    gateway_replacement_readiness: NativeGatewayReplacementReadiness,
    control_ui_route_parity: ControlUiRouteParityReport,
    telegram_plugin: &'a NativeTelegramPluginStatus,
    telegram_poll_loop_status: native_telegram::NativeTelegramPollLoopStatus,
    telegram_live_soak_status: native_telegram::NativeTelegramLiveSoakStatus,
    telegram_cursor_status: native_telegram::NativeTelegramCursorStatus,
    telegram_read_performed: bool,
    model_invoked: bool,
    message_sent: bool,
    cursor_written: bool,
    raw_token_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeSessionsResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    scanned_root_count: usize,
    existing_root_count: usize,
    scan_error_count: usize,
    session_file_count: u64,
    total_bytes: u64,
    recent_session_count: usize,
    roots: Vec<NativeSessionRootReport>,
    recent_sessions: Vec<NativeSessionSummary>,
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Clone)]
struct NativeSessionRootCandidate {
    root: PathBuf,
    kind: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct NativeSessionRootReport {
    root: String,
    kind: &'static str,
    exists: bool,
    file_count: u64,
    total_bytes: u64,
    latest_modified_unix_ms: Option<u64>,
    error: Option<String>,
    recent_sessions: Vec<NativeSessionSummary>,
}

#[derive(Debug, Clone, Serialize)]
struct NativeSessionSummary {
    session_id: String,
    started_at_filename: Option<String>,
    filename: String,
    relative_path: String,
    bytes: u64,
    modified_unix_ms: Option<u64>,
}

#[derive(Debug, Clone)]
struct NativeSessionFileCandidate {
    path: PathBuf,
    root_kind: &'static str,
    summary: NativeSessionSummary,
}

#[derive(Debug, Serialize)]
struct NativeTranscriptResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    query_present: bool,
    query_redacted: bool,
    query_length: Option<usize>,
    scanned_session_file_count: usize,
    available_session_file_count: usize,
    max_files: usize,
    max_lines_per_file: usize,
    matched_session_count: usize,
    matched_line_count: u64,
    parse_error_count: usize,
    scan_error_count: usize,
    sessions: Vec<NativeTranscriptSessionPreview>,
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    query_text_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeTranscriptSessionPreview {
    root_kind: &'static str,
    session_id: String,
    started_at_filename: Option<String>,
    filename: String,
    relative_path: String,
    bytes: u64,
    modified_unix_ms: Option<u64>,
    line_count: u64,
    parsed_json_line_count: u64,
    parse_error_count: usize,
    truncated: bool,
    event_type_counts: Vec<NativeTranscriptEventCount>,
    redacted_events: Vec<NativeTranscriptEventPreview>,
    query_match: NativeTranscriptQueryMatch,
    read_error: Option<String>,
}

#[derive(Debug, Serialize)]
struct NativeTranscriptEventCount {
    event_type: String,
    count: u64,
}

#[derive(Debug, Serialize)]
struct NativeTranscriptEventPreview {
    line_number: usize,
    event_type: String,
    role: Option<String>,
    has_text_fields: bool,
    redacted: bool,
}

#[derive(Debug, Serialize)]
struct NativeTranscriptQueryMatch {
    matched_line_count: u64,
    first_match_line: Option<usize>,
    matched_event_type_counts: Vec<NativeTranscriptEventCount>,
}

#[derive(Debug)]
struct NativeTaskArtifactRouteSpec {
    prefix: &'static str,
    source_command: &'static str,
    artifact_kind: &'static str,
    compatibility_mode: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeEventSurface {
    Events,
    LiveEvents,
    EventsReport,
    Activity,
}

impl NativeEventSurface {
    fn source_command(self) -> &'static str {
        match self {
            Self::Events => "/events --json",
            Self::LiveEvents => "/live-events <cursor> --json",
            Self::EventsReport => "/events-report --json",
            Self::Activity => "/activity --json",
        }
    }

    fn compatibility_mode(self) -> &'static str {
        match self {
            Self::Events => "native_events_redacted",
            Self::LiveEvents => "native_live_events_redacted",
            Self::EventsReport => "native_events_report_redacted",
            Self::Activity => "native_activity_redacted",
        }
    }

    fn event_surface(self) -> &'static str {
        match self {
            Self::Events => "events",
            Self::LiveEvents => "live_events",
            Self::EventsReport => "events_report",
            Self::Activity => "activity",
        }
    }

    fn includes_activity_sessions(self) -> bool {
        self == Self::Activity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeRuntimeAuditSurface {
    SubagentObservatory,
    GatewayLedger,
    GatewayRetryDeadLetter,
    MultiAgentRuntime,
}

impl NativeRuntimeAuditSurface {
    fn source_command(self) -> &'static str {
        match self {
            Self::SubagentObservatory => "/subagent-observatory --json",
            Self::GatewayLedger => "/gateway-ledger --json",
            Self::GatewayRetryDeadLetter => "/gateway-retry-dead-letter --json",
            Self::MultiAgentRuntime => "/multi-agent-runtime --agents 4 --messages 8 --json",
        }
    }

    fn compatibility_mode(self) -> &'static str {
        match self {
            Self::SubagentObservatory => "native_subagent_observatory_redacted",
            Self::GatewayLedger => "native_gateway_ledger_redacted",
            Self::GatewayRetryDeadLetter => "native_gateway_retry_dead_letter_redacted",
            Self::MultiAgentRuntime => "native_multi_agent_runtime_redacted",
        }
    }

    fn audit_surface(self) -> &'static str {
        match self {
            Self::SubagentObservatory => "subagent_observatory",
            Self::GatewayLedger => "gateway_ledger",
            Self::GatewayRetryDeadLetter => "gateway_retry_dead_letter",
            Self::MultiAgentRuntime => "multi_agent_runtime",
        }
    }

    fn event_focus(self) -> &'static str {
        match self {
            Self::SubagentObservatory => "subagent event type counters and redacted previews",
            Self::GatewayLedger => {
                "gateway route matrix, approvals, session inventory, and event counters"
            }
            Self::GatewayRetryDeadLetter => {
                "retry, dead-letter, failure, and error event type counters"
            }
            Self::MultiAgentRuntime => "bounded multi-agent session and event inventory",
        }
    }

    fn agent_limit(self) -> Option<usize> {
        (self == Self::MultiAgentRuntime).then_some(4)
    }

    fn message_limit(self) -> Option<usize> {
        (self == Self::MultiAgentRuntime).then_some(8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeControlUiAuditSurface {
    ControlUi,
    UiContractAudit,
    GatewayDispatch,
    UiActionPlanGatewayDispatch,
    ExternalAgentBenchmark,
}

impl NativeControlUiAuditSurface {
    fn source_command(self) -> &'static str {
        match self {
            Self::ControlUi => "/control-ui --json",
            Self::UiContractAudit => "/ui-contract-audit --json",
            Self::GatewayDispatch => "/gateway-dispatch --dry-run --json",
            Self::UiActionPlanGatewayDispatch => {
                "/ui-action-plan gateway-dispatch --dry-run --json"
            }
            Self::ExternalAgentBenchmark => "/external-agent-benchmark --json",
        }
    }

    fn compatibility_mode(self) -> &'static str {
        match self {
            Self::ControlUi => "native_control_ui_shell_snapshot",
            Self::UiContractAudit => "native_ui_contract_audit",
            Self::GatewayDispatch => "native_gateway_dispatch_dry_run",
            Self::UiActionPlanGatewayDispatch => "native_ui_action_plan_gateway_dispatch",
            Self::ExternalAgentBenchmark => "native_external_agent_benchmark_redacted",
        }
    }

    fn control_surface(self) -> &'static str {
        match self {
            Self::ControlUi => "control_ui",
            Self::UiContractAudit => "ui_contract_audit",
            Self::GatewayDispatch => "gateway_dispatch",
            Self::UiActionPlanGatewayDispatch => "ui_action_plan_gateway_dispatch",
            Self::ExternalAgentBenchmark => "external_agent_benchmark",
        }
    }

    fn plan_target(self) -> Option<&'static str> {
        match self {
            Self::GatewayDispatch | Self::UiActionPlanGatewayDispatch => Some("gateway-dispatch"),
            Self::ExternalAgentBenchmark => Some("external-agent-benchmark"),
            Self::ControlUi | Self::UiContractAudit => None,
        }
    }

    fn dry_run_only(self) -> bool {
        matches!(
            self,
            Self::GatewayDispatch
                | Self::UiActionPlanGatewayDispatch
                | Self::ExternalAgentBenchmark
        )
    }

    fn read_only(self) -> bool {
        !self.dry_run_only()
    }

    fn reports_control_ui_evidence(self) -> bool {
        matches!(self, Self::ControlUi | Self::UiContractAudit)
    }
}

#[derive(Debug, Serialize)]
struct NativeTaskArtifactResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    artifact_kind: &'static str,
    task_id_redacted: bool,
    task_id_length: usize,
    evidence_found: bool,
    matched_session_count: usize,
    matched_line_count: u64,
    evidence_search: NativeTranscriptResponse,
    raw_task_id_exposed: bool,
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeEventsResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    event_surface: &'static str,
    cursor_present: bool,
    cursor_redacted: bool,
    cursor_length: Option<usize>,
    cursor_parseable_as_u64: bool,
    scanned_session_file_count: usize,
    available_session_file_count: usize,
    max_files: usize,
    max_lines_per_file: usize,
    total_line_count: u64,
    parsed_json_line_count: u64,
    parse_error_count: usize,
    scan_error_count: usize,
    truncated_session_count: usize,
    event_type_count: usize,
    event_type_counts: Vec<NativeTranscriptEventCount>,
    recent_event_count: usize,
    recent_events: Vec<NativeEventPreview>,
    activity_sessions: Option<NativeSessionsResponse>,
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    raw_cursor_exposed: bool,
    cursor_text_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeEventPreview {
    root_kind: &'static str,
    session_id: String,
    started_at_filename: Option<String>,
    relative_path: String,
    line_number: usize,
    event_type: String,
    role: Option<String>,
    has_text_fields: bool,
    redacted: bool,
}

#[derive(Debug, Serialize)]
struct NativeRuntimeAuditResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    audit_surface: &'static str,
    event_focus: &'static str,
    agent_limit: Option<usize>,
    message_limit: Option<usize>,
    route_matrix_ready: bool,
    route_count: usize,
    missing_route_count: usize,
    approval_route_count: usize,
    guarded_approval_route_count: usize,
    session_file_count: u64,
    recent_session_count: usize,
    session_scan_error_count: usize,
    event_type_count: usize,
    recent_event_count: usize,
    event_scan_error_count: usize,
    subagent_event_count: u64,
    retry_or_error_event_count: u64,
    multi_agent_event_count: u64,
    sessions: NativeSessionsResponse,
    events: NativeEventsResponse,
    redaction: NativeRuntimeAuditRedaction,
    side_effects: NativeRuntimeAuditSideEffects,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeRuntimeAuditRedaction {
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    raw_agent_payload_exposed: bool,
    raw_error_payload_exposed: bool,
    raw_gateway_ledger_payload_exposed: bool,
}

#[derive(Debug, Serialize)]
struct NativeRuntimeAuditSideEffects {
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
}

#[derive(Debug, Serialize)]
struct NativeControlUiAuditResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    control_surface: &'static str,
    plan_target: Option<&'static str>,
    dry_run_only: bool,
    read_only: bool,
    control_ui_product_status: &'static str,
    control_ui_product_complete: bool,
    control_ui_live_operator_surface_percent: u8,
    control_ui_evidence: hepta_core::ControlUiEvidenceCoverage,
    confirmation_required_for_real_mutation: bool,
    route_matrix_ready: bool,
    route_count: usize,
    implemented_route_count: usize,
    missing_route_count: usize,
    get_route_count: usize,
    post_route_count: usize,
    dry_run_route_count: usize,
    read_only_route_count: usize,
    guarded_post_route_count: usize,
    approval_route_count: usize,
    guarded_approval_route_count: usize,
    gateway_replacement_ready: bool,
    gateway_replacement_blocker_count: usize,
    external_agent_benchmark_executed: bool,
    external_agent_spawned: bool,
    action_dispatched: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    route_matrix: ControlUiRouteParityReport,
    redaction: NativeControlUiAuditRedaction,
    side_effects: NativeControlUiAuditSideEffects,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeControlUiAuditRedaction {
    raw_transcript_exposed: bool,
    transcript_text_exposed: bool,
    raw_token_exposed: bool,
    raw_action_payload_exposed: bool,
    raw_agent_payload_exposed: bool,
}

#[derive(Debug, Serialize)]
struct NativeControlUiAuditSideEffects {
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
}

#[derive(Debug, Serialize)]
struct NativeApprovalsResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    pending_approval_count: usize,
    approval_route_count: usize,
    guarded_route_count: usize,
    approval_routes: Vec<NativeApprovalRoute>,
    raw_command_payload_exposed: bool,
    raw_approval_payload_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeApprovalRoute {
    method: &'static str,
    pattern: &'static str,
    capability: &'static str,
    source_command: &'static str,
    side_effect_boundary: &'static str,
    dry_run_only: bool,
    guarded: bool,
    confirmation_required_for_real_mutation: bool,
}

#[derive(Debug, Serialize)]
struct NativePolicyResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    loopback_bind_required: bool,
    loopback_bound: bool,
    non_loopback_override_enabled: bool,
    bind_addr: String,
    telegram_gate_summary: native_telegram::NativeTelegramGatewayGateSummary,
    gateway_replacement_ready: bool,
    gateway_replacement_blocker_count: usize,
    approval_route_count: usize,
    guarded_approval_route_count: usize,
    raw_token_exposed: bool,
    raw_transcript_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeConfigResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    bind_addr: String,
    telegram_plugin_requested: bool,
    telegram_plugin_poll_ms: u64,
    default_model_present: bool,
    telegram_model_present: bool,
    openai_codex_home_present: bool,
    gateway_token_file_present: bool,
    release_build_verified: bool,
    control_ui_parity_verified: bool,
    telegram_gate_summary: native_telegram::NativeTelegramGatewayGateSummary,
    config_root_count: usize,
    config_roots: Vec<NativeConfigPathStatus>,
    raw_env_exposed: bool,
    raw_token_exposed: bool,
    raw_config_value_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeConfigPathStatus {
    label: &'static str,
    path: String,
    exists: bool,
    is_dir: bool,
    bytes: Option<u64>,
}

#[derive(Debug, Serialize)]
struct NativeOptionalConfigsResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    config_count: usize,
    missing_expected_count: usize,
    configs: Vec<NativeOptionalConfigStatus>,
    raw_config_value_exposed: bool,
    config_content_exposed: bool,
    model_invoked: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeOptionalConfigStatus {
    label: &'static str,
    path: String,
    expected: bool,
    exists: bool,
    is_file: bool,
    bytes: Option<u64>,
    content_exposed: bool,
}

#[derive(Debug, Serialize)]
struct NativeOperatorConsoleResponse<'a> {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    health: HealthResponse,
    operator_snapshot_endpoint: &'static str,
    operator_security_endpoint: &'static str,
    sessions_endpoint: &'static str,
    session_activity_endpoint: &'static str,
    gateway_replacement_readiness: NativeGatewayReplacementReadiness,
    control_ui_route_parity: ControlUiRouteParityReport,
    sessions: NativeSessionsResponse,
    telegram_plugin: &'a NativeTelegramPluginStatus,
    telegram_poll_loop_status: native_telegram::NativeTelegramPollLoopStatus,
    telegram_live_soak_status: native_telegram::NativeTelegramLiveSoakStatus,
    raw_transcript_exposed: bool,
    raw_token_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    model_invoked: bool,
    message_sent: bool,
    cursor_written: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeOperatorSecurityResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    source_command: &'static str,
    native_route: bool,
    compatibility_mode: &'static str,
    side_effect_free: bool,
    security_mode: &'static str,
    legacy_owner_coexistence_ready: bool,
    attention_reason: &'static str,
    loopback_bind_required: bool,
    loopback_bound: bool,
    non_loopback_override_enabled: bool,
    bind_addr: String,
    control_ui_route_parity: ControlUiRouteParityReport,
    gateway_replacement_readiness: NativeGatewayReplacementReadiness,
    post_route_count: usize,
    dry_run_post_route_count: usize,
    guarded_post_route_count: usize,
    post_execution_readiness_endpoint: &'static str,
    post_execution_stores_endpoint: &'static str,
    post_activation_plan_endpoint: &'static str,
    post_execution_readiness: NativePostExecutionReadinessResponse,
    post_execution_stores_ready: bool,
    post_execution_stores: NativePostExecutionStoresResponse,
    post_activation_plan_ready: bool,
    post_activation_plan: NativePostActivationPlanResponse,
    post_gray_release_evidence_endpoint: &'static str,
    post_gray_release_evidence_ready: bool,
    post_gray_release_evidence: NativePostGrayReleaseEvidenceResponse,
    production_soak_ready: bool,
    telegram_gate_summary: native_telegram::NativeTelegramGatewayGateSummary,
    telegram_production_readiness_status: native_telegram::NativeTelegramProductionReadinessStatus,
    telegram_owner_handoff_endpoint: &'static str,
    telegram_owner_handoff_status: NativeTelegramOwnerHandoffStatus,
    telegram_plugin_requested: bool,
    telegram_plugin_status: &'static str,
    redaction: NativeOperatorSecurityRedaction,
    side_effects: NativeOperatorSecuritySideEffects,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeOperatorSecurityRedaction {
    raw_transcript_exposed: bool,
    raw_token_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
    raw_idempotency_key_exposed: bool,
    raw_audit_payload_exposed: bool,
}

#[derive(Debug, Serialize)]
struct NativeOperatorSecuritySideEffects {
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    model_invoked: bool,
    message_sent: bool,
    cursor_written: bool,
}

#[derive(Debug, Clone)]
struct NativeTelegramOwnerHandoffInputs {
    legacy_config_path: Option<String>,
    legacy_config_found: bool,
    legacy_config_parse_ok: bool,
    legacy_telegram_enabled: Option<bool>,
    legacy_token_fingerprint: Option<String>,
    legacy_config_error: Option<String>,
    hepta_token_fingerprint: Option<String>,
    hepta_telegram_requested: bool,
    hepta_poll_loop_armed: bool,
    hepta_poll_loop_gate_enabled: bool,
    hepta_delivery_approval_gate_enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
struct NativeTelegramOwnerHandoffStatus {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    endpoint: &'static str,
    ready: bool,
    conflict_free: bool,
    hepta_takeover_ready: bool,
    hepta_parallel_bot_ready: bool,
    side_effect_free: bool,
    active_owner: &'static str,
    legacy_config_path: Option<String>,
    legacy_config_found: bool,
    legacy_config_parse_ok: bool,
    legacy_telegram_enabled: Option<bool>,
    legacy_telegram_enabled_explicit: bool,
    legacy_token_fingerprint: Option<String>,
    hepta_token_fingerprint: Option<String>,
    bot_identity_match: Option<bool>,
    parallel_bot_mode: bool,
    hepta_telegram_requested: bool,
    hepta_poll_loop_armed: bool,
    hepta_poll_loop_gate_enabled: bool,
    hepta_delivery_approval_gate_enabled: bool,
    double_poller_risk: bool,
    takeover_blockers: Vec<&'static str>,
    legacy_config_error: Option<String>,
    raw_token_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayReplacementReadiness {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    ready: bool,
    active_install_allowed: bool,
    side_effect_free: bool,
    blocker_count: usize,
    blockers: Vec<&'static str>,
    checks: Vec<NativeGatewayReplacementCheck>,
    required_env_gates: NativeGatewayReplacementEnvGates,
    telegram_owner_handoff_endpoint: &'static str,
    telegram_owner_handoff_status: NativeTelegramOwnerHandoffStatus,
    control_ui_route_parity: ControlUiRouteParityReport,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayReplacementCheck {
    name: &'static str,
    ready: bool,
    detail: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayReplacementEnvGates {
    delivery_approval: NativeGatewayReplacementGate,
    live_read: NativeGatewayReplacementGate,
    model_turn: NativeGatewayReplacementGate,
    send: NativeGatewayReplacementGate,
    poll_loop: NativeGatewayReplacementGate,
    in_process_model_runner: NativeGatewayReplacementGate,
    hepta_kernel_model_runner: NativeGatewayReplacementGate,
    release_build_verified: NativeGatewayReplacementGate,
    control_ui_parity_verified: NativeGatewayReplacementGate,
}

#[derive(Debug, Serialize)]
struct NativeGatewayReplacementGate {
    env: &'static str,
    enabled: bool,
}

#[derive(Debug, Serialize)]
struct NativeGatewayLiveActivationPlan {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    endpoint: &'static str,
    operator_approval_required: bool,
    active_install_allowed: bool,
    readiness_blocker_count: usize,
    readiness_blockers: Vec<&'static str>,
    active_gateway_label: &'static str,
    current_legacy_binary: &'static str,
    replacement_binary: &'static str,
    bind_addr: String,
    launch_arguments: Vec<String>,
    required_env_gates: Vec<NativeGatewayLiveActivationEnv>,
    live_smoke_sequence: &'static [&'static str],
    production_replacement_sequence: &'static [&'static str],
    safety: NativeGatewayLiveActivationSafety,
    next_migration_slice: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayLiveActivationEnv {
    env: &'static str,
    enabled: bool,
    purpose: &'static str,
}

#[derive(Debug, Serialize)]
struct NativeGatewayLiveActivationSafety {
    side_effect_free: bool,
    status_probe_reads_telegram: bool,
    status_probe_invokes_model: bool,
    status_probe_sends_message: bool,
    status_probe_writes_cursor: bool,
    raw_token_exposed: bool,
    raw_update_payload_exposed: bool,
    raw_prompt_text_exposed: bool,
    raw_response_text_exposed: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ControlUiRouteParityReport {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    ready: bool,
    route_count: usize,
    implemented_route_count: usize,
    production_dispatchable_route_count: usize,
    quarantined_route_count: usize,
    quarantined_routes: Vec<String>,
    missing_route_count: usize,
    missing_routes: Vec<String>,
    side_effect_free: bool,
    evidence_scope: &'static str,
    live_product_complete: bool,
    legacy_source: &'static str,
    routes: &'static [ControlUiRouteSpec],
}

static CONTROL_UI_ROUTE_PARITY_REPORT_CACHE: OnceLock<ControlUiRouteParityReport> = OnceLock::new();

#[derive(Debug, Serialize)]
struct ControlUiRouteCompatibilityResponse {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    method: &'static str,
    pattern: &'static str,
    path: String,
    source_command: &'static str,
    capability: &'static str,
    side_effect_boundary: &'static str,
    compatibility_mode: &'static str,
    dry_run_only: bool,
    confirmation_required_for_real_mutation: bool,
    external_side_effects: bool,
    gateway_mutation_performed: bool,
    telegram_read_performed: bool,
    model_invoked: bool,
    message_sent: bool,
    cursor_written: bool,
}

fn control_ui_route_parity_report() -> ControlUiRouteParityReport {
    CONTROL_UI_ROUTE_PARITY_REPORT_CACHE
        .get_or_init(|| {
            let quarantined_routes = CONTROL_UI_ROUTE_SPECS
                .iter()
                .filter(|route| route.is_quarantined_transitive_effect())
                .map(|route| format!("{} {}", route.method, route.pattern))
                .collect::<Vec<_>>();
            let missing_routes = CONTROL_UI_ROUTE_SPECS
                .iter()
                .filter(|route| !control_ui_route_has_handler(route))
                .map(|route| format!("{} {}", route.method, route.pattern))
                .collect::<Vec<_>>();
            let implemented_route_count = CONTROL_UI_ROUTE_SPECS.len() - missing_routes.len();
            let production_dispatchable_route_count =
                implemented_route_count - quarantined_routes.len();
            let ready = missing_routes.is_empty();
            ControlUiRouteParityReport {
                product: "Hepta",
                runtime: "hepta",
                status: if ready { "ready" } else { "blocked" },
                ready,
                route_count: CONTROL_UI_ROUTE_SPECS.len(),
                implemented_route_count,
                production_dispatchable_route_count,
                quarantined_route_count: quarantined_routes.len(),
                quarantined_routes,
                missing_route_count: missing_routes.len(),
                missing_routes,
                side_effect_free: true,
                evidence_scope: "typed route registration, compatibility-handler serialization, production ingress availability, and real-socket test coverage",
                live_product_complete: false,
                legacy_source: "Hepta Control UI typed route matrix and hepta-core::control_ui markers; quarantined legacy GET effects are reported separately and are not counted as production-dispatchable behavior",
                routes: CONTROL_UI_ROUTE_SPECS,
            }
        })
        .clone()
}

fn control_ui_route_response(method: &str, path: &str) -> Option<String> {
    let route = control_ui_route_spec_for(method, path)?;
    Some(json_or_error(&ControlUiRouteCompatibilityResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if route.method == "POST" {
            "dry_run_compatibility"
        } else {
            "ready"
        },
        method: route.method,
        pattern: route.pattern,
        path: path.to_string(),
        source_command: route.source_command,
        capability: route.capability,
        side_effect_boundary: route.side_effect_boundary,
        compatibility_mode: "native_control_ui_route_parity_shell",
        dry_run_only: route.is_post(),
        confirmation_required_for_real_mutation: route.requires_confirmation(),
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
    }))
}

fn control_ui_route_has_handler(route: &ControlUiRouteSpec) -> bool {
    let sample_path = control_ui_sample_path(route.pattern);
    control_ui_route_response(route.method, &sample_path)
        .and_then(|response| serde_json::from_str::<serde_json::Value>(&response).ok())
        .is_some_and(|response| {
            response["method"] == route.method
                && response["pattern"] == route.pattern
                && response["path"] == sample_path
        })
}

fn control_ui_route_spec_for(method: &str, path: &str) -> Option<&'static ControlUiRouteSpec> {
    CONTROL_UI_ROUTE_SPECS.iter().find(|route| {
        route.method == method && control_ui_route_pattern_matches(route.pattern, path)
    })
}

fn control_ui_route_pattern_matches(pattern: &str, path: &str) -> bool {
    if let Some(start) = pattern.find("/<") {
        let prefix = &pattern[..start + 1];
        path.starts_with(prefix) && path.len() > prefix.len()
    } else {
        pattern == path
    }
}

fn control_ui_sample_path(pattern: &str) -> String {
    pattern
        .replace("<action>", "gateway-dispatch")
        .replace("<id>", "gateway-status")
        .replace("<query>", "sample")
        .replace("<task_id>", "sample-task")
        .replace("<cursor>", "0")
}

fn allow_non_loopback_ui() -> bool {
    env_truthy("HEPTA_ALLOW_NON_LOOPBACK_UI")
}

fn env_truthy(name: &str) -> bool {
    env::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn env_u64(name: &str) -> Option<u64> {
    env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
}

fn is_loopback_bind_addr(bind_addr: &str) -> bool {
    let host = bind_addr
        .rsplit_once(':')
        .map(|(host, _)| host.trim_matches(['[', ']']))
        .unwrap_or(bind_addr)
        .trim();
    matches!(host, "127.0.0.1" | "localhost" | "::1")
}
