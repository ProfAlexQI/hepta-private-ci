fn route_native_gateway_request_after_preflight(
    method: &str,
    path: &str,
    options: &NativeGatewayOptions,
    request_body: Option<&str>,
    preflight: &RuntimeRequestPreflightReceipt,
) -> (&'static str, &'static str, String) {
    let telegram_plugin = native_telegram::telegram_plugin_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    if method == "GET" {
        macro_rules! dispatch_registered_native_reports {
            ($($pattern:pat $(if $guard:expr)? => $body:block)*) => {
                match path {
                    $($pattern $(if $guard)? => $body)*
                    _ => {}
                }
            };
        }
        include!("report_registry.rs");

        if let Some(query) = path
            .strip_prefix("/api/query-transcript/")
            .filter(|query| !query.is_empty())
        {
            return (
                "200 OK",
                "application/json; charset=utf-8",
                native_transcript_json(Some(query)),
            );
        }

        if let Some(cursor) = path
            .strip_prefix("/api/live-events/")
            .filter(|cursor| !cursor.is_empty())
        {
            return (
                "200 OK",
                "application/json; charset=utf-8",
                native_events_json(NativeEventSurface::LiveEvents, Some(cursor)),
            );
        }

        for spec in NATIVE_TASK_ARTIFACT_ROUTE_SPECS {
            if let Some(task_id) = path
                .strip_prefix(spec.prefix)
                .filter(|task_id| !task_id.is_empty())
            {
                return (
                    "200 OK",
                    "application/json; charset=utf-8",
                    native_task_artifact_json(spec, task_id),
                );
            }
        }
    }

    let native_post_gates = preflight.native_post_gate_inputs(
        env_truthy(NATIVE_POST_REAL_HANDLERS_ENV),
        env_truthy(NATIVE_POST_REAL_HANDLER_APPROVAL_ENV),
    );
    if let Some(report) = hepta_gateway::native_post_dispatch_plan_report(
        method,
        path,
        request_body,
        native_post_gates.real_handler_enabled,
        native_post_gates.operator_approval_enabled,
        native_post_real_handler_scope_from_env().as_deref(),
        &native_post_execution_store_root(),
        NativePostExecutionStoreLimits {
            max_store_bytes: native_post_store_max_bytes(),
            max_store_lines: native_post_store_max_lines(),
            rate_limit_window_ms: native_post_rate_limit_window_ms(),
        },
    ) {
        return (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&report),
        );
    }

    if let Some(body) = control_ui_route_response(method, path) {
        return ("200 OK", "application/json; charset=utf-8", body);
    }

    if method != "GET" {
        (
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            "method not allowed; supported POST endpoints are /api/actions/<action> and native POST route specs".to_string(),
        )
    } else {
        (
            "404 Not Found",
            "text/plain; charset=utf-8",
            "not found".to_string(),
        )
    }
}
pub(crate) fn native_gateway_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    let gateway_replacement_readiness = gateway_replacement_readiness(options, telegram_plugin);
    let control_ui_route_parity = control_ui_route_parity_report();
    let telegram_owner_handoff_status = telegram_owner_handoff_status(options);
    let active_gateway_replacement_ready = gateway_replacement_readiness.ready;
    let replacement_blocker = gateway_replacement_readiness.blockers.first().copied();
    json_or_error(&NativeGatewayResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        migration_mode: "codex_fork_native",
        bind_addr: &options.bind_addr,
        launchd_entrypoint_compatible: true,
        active_gateway_replacement_ready,
        replacement_blocker,
        gateway_replacement_readiness_endpoint: GATEWAY_REPLACEMENT_READINESS_ENDPOINT,
        gateway_replacement_readiness,
        gateway_route_core_status: native_gateway_route_core_status(),
        gateway_live_activation_plan_endpoint: GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT,
        gateway_live_activation_plan: gateway_live_activation_plan(options, telegram_plugin),
        control_ui_route_parity_endpoint: CONTROL_UI_ROUTE_PARITY_ENDPOINT,
        control_ui_route_parity_ready: control_ui_route_parity.ready,
        control_ui_route_parity,
        hepta_merge_completion_endpoint: HEPTA_MERGE_COMPLETION_ENDPOINT,
        hepta_native_packaging_gate_endpoint: HEPTA_NATIVE_PACKAGING_GATE_ENDPOINT,
        hepta_legacy_compatibility_closure_endpoint: HEPTA_LEGACY_COMPATIBILITY_CLOSURE_ENDPOINT,
        hepta_public_ga_operator_approval_packet_endpoint:
            HEPTA_PUBLIC_GA_OPERATOR_APPROVAL_PACKET_ENDPOINT,
        hepta_public_ga_readiness_endpoint: HEPTA_PUBLIC_GA_READINESS_ENDPOINT,
        hepta_core_fusion_readiness_endpoint: HEPTA_CORE_FUSION_READINESS_ENDPOINT,
        hepta_name_repository_closure_endpoint: HEPTA_NAME_REPOSITORY_CLOSURE_ENDPOINT,
        hepta_engine_adapter_boundary_endpoint: HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT,
        hepta_codex_engine_adapter_boundary_endpoint: HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT,
        telegram_plugin_requested: options.with_telegram_plugin,
        telegram_plugin_status: telegram_plugin.status,
        telegram_plugin_native_supervisor_ready: telegram_plugin.in_process_supervisor_ready,
        telegram_plugin_reply_loop_ready: telegram_plugin.in_process_reply_loop_ready,
        telegram_plugin_poll_ms: options.telegram_plugin_poll_ms,
        telegram_receive_once_endpoint: "/api/telegram-receive-once",
        telegram_model_turn_plan_endpoint: "/api/telegram-model-turn-plan",
        telegram_model_bridge_endpoint: "/api/telegram-model-bridge",
        telegram_send_plan_endpoint: "/api/telegram-send-plan",
        telegram_drain_once_endpoint: "/api/telegram-drain-once",
        telegram_poll_loop_endpoint: "/api/telegram-poll-loop",
        telegram_live_soak_endpoint: TELEGRAM_LIVE_SOAK_ENDPOINT,
        telegram_live_soak_status_endpoint: TELEGRAM_LIVE_SOAK_STATUS_ENDPOINT,
        telegram_production_readiness_endpoint: TELEGRAM_PRODUCTION_READINESS_ENDPOINT,
        telegram_production_readiness_status: native_telegram::telegram_production_readiness_status(
            options.with_telegram_plugin,
            options.telegram_plugin_poll_ms,
        ),
        telegram_delivery_ledger_endpoint: TELEGRAM_DELIVERY_LEDGER_ENDPOINT,
        telegram_delivery_ledger_status: native_telegram::telegram_delivery_ledger_status(
            options.with_telegram_plugin,
        ),
        telegram_owner_handoff_endpoint: TELEGRAM_OWNER_HANDOFF_ENDPOINT,
        telegram_owner_handoff_status,
        telegram_cursor_endpoint: "/api/telegram-cursor",
        telegram_gate_summary: native_telegram::telegram_gateway_gate_summary(),
        telegram_poll_loop_status: native_telegram::telegram_poll_loop_status(
            options.with_telegram_plugin,
            options.telegram_plugin_poll_ms,
        ),
        telegram_live_soak_status: native_telegram::telegram_live_soak_status(
            options.with_telegram_plugin,
            options.telegram_plugin_poll_ms,
        ),
        telegram_readiness_summary_side_effect_free: true,
        telegram_plugin,
        migrated_surfaces: &[
            "--serve-ui entrypoint",
            "loopback guard",
            "native gateway readiness JSON",
            "health endpoint",
            "Control UI shell",
            "Telegram plugin redacted config contract",
            "Telegram native supervisor readiness surface",
            "Telegram gated one-shot receive surface",
            "Telegram redacted model-turn planning surface",
            "Telegram gated model bridge skeleton",
            "Telegram gated send plan surface",
            "Telegram gated drain-once pipeline surface",
            "Telegram gated poll-loop supervisor surface",
            "Telegram live soak guard and observation surface",
            "Telegram production readiness guard surface",
            "Telegram durable delivery ledger surface",
            "Telegram owner handoff conflict guard",
            "Telegram cursor state surface",
            "Control UI route parity report",
            "Control UI side-effect-free compatibility endpoints",
            "Gateway live activation side-effect-free plan",
            "Native redacted session inventory",
            "Native redacted transcript preview/search",
            "Native redacted task artifact surfaces",
            "Native redacted events/activity surfaces",
            "Hepta merge and functional completion audit surface",
            "Hepta CLI command breadth read-only inventory",
            "Hepta provider/search metadata inventory",
            "Hepta runtime/task/session dry-run inventory",
            "Hepta channel adapter disabled status inventory",
            "Hepta local tooling/content planning inventory",
            "Hepta memory/capability absorption gap inventory",
            "Hepta release/hardening status gate",
            "Hepta provider/channel/runtime dry-run plan",
            "Hepta Native packaging readiness gate",
            "Hepta legacy compatibility closure gate",
            "Hepta public GA readiness gate",
        ],
        next_migration_slice: "keep public GA readiness blocked until explicit operator-approved live handoff, mutation, provider, channel, and release gates are satisfied",
    })
}
