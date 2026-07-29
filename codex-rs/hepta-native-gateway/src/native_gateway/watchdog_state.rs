use super::*;

const WATCHDOG_STATE_SCHEMA: &str = "hepta_watchdog_state_v1";

#[derive(Debug, Serialize)]
struct WatchdogStateResponse {
    schema_version: &'static str,
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    side_effect_free: bool,
    route: WatchdogRouteState,
    operator: WatchdogOperatorState,
    owner: NativeTelegramOwnerHandoffStatus,
    poll: native_telegram::NativeTelegramPollLoopStatus,
    production: native_telegram::NativeTelegramProductionReadinessStatus,
    native_post: WatchdogNativePostState,
    architecture: WatchdogArchitectureState,
    product_boundary: WatchdogProductBoundaryState,
}

#[derive(Debug, Serialize)]
struct WatchdogRouteState {
    status: &'static str,
    route_count: usize,
    implemented_route_count: usize,
    missing_route_count: usize,
    manifest_schema_version: &'static str,
    manifest_sha256: String,
}

#[derive(Debug, Serialize)]
struct WatchdogOperatorState {
    status: &'static str,
    security_mode: &'static str,
    legacy_owner_coexistence_ready: bool,
    attention_reason: &'static str,
    loopback_bound: bool,
}

#[derive(Debug, Serialize)]
struct WatchdogNativePostState {
    activation: NativePostActivationPlanResponse,
    stores: NativePostExecutionStoresResponse,
}

#[derive(Debug, Serialize)]
struct WatchdogArchitectureState {
    status: &'static str,
    all_ready: bool,
    adapter_status: &'static str,
    adapter_alias_status: &'static str,
    adapter_surface_count: usize,
    adapter_parity_evidence_count: usize,
    adapter_parity_complete: bool,
    adapter_parity_promotion_ready: bool,
    adapter_parity_completion_gate_ready: bool,
    adapter_parity_completion_gate_status: &'static str,
    adapter_parity_completion_gate_allows_promotion: bool,
    adapter_shadow_replay_required_surface_count: usize,
    adapter_shadow_replay_covered_surface_count: usize,
    adapter_shadow_replay_remaining_surface_count: usize,
    adapter_parity_promotion_blocker_count: usize,
    core_status: &'static str,
    core_phase: &'static str,
    phase_3_binary_package_inversion_ready: bool,
    binary_package_inversion_gate_status: &'static str,
    binary_package_inversion_blocker_count: usize,
    active_binary_package: &'static str,
    intended_binary_package: &'static str,
    installed_service_binary: &'static str,
    phase_4_name_repository_closure_gate_status: &'static str,
    phase_4_name_repository_closure_ready: bool,
    phase_4_name_repository_closure_remaining_surface_count: usize,
    name_repository_closure_status: &'static str,
    name_repository_closure_gate_status: &'static str,
    name_repository_closure_remaining_surface_count: usize,
    name_repository_closure_blocker_count: usize,
    phase_5_engine_dependency_closure_gate_status: &'static str,
    phase_5_engine_dependency_closure_ready: bool,
    phase_5_engine_dependency_closure_remaining_dependency_count: usize,
    engine_dependency_closure_status: &'static str,
    engine_dependency_closure_gate_status: &'static str,
    engine_dependency_closure_remaining_dependency_count: usize,
    engine_dependency_closure_blocker_count: usize,
    full_fusion_complete: bool,
    forbidden_side_effects_clear: bool,
    adapter: hepta_gateway::HeptaCodexEngineAdapterBoundaryResponse,
    core: hepta_gateway::HeptaCoreFusionReadinessResponse,
    closure: hepta_gateway::HeptaNameRepositoryClosureResponse,
    dependency: hepta_gateway::HeptaEngineDependencyClosureResponse,
}

#[derive(Debug, Serialize)]
struct WatchdogProductBoundaryState {
    status: &'static str,
    role: &'static str,
    channel_owner: &'static str,
    telegram_external_read: bool,
    telegram_external_send: bool,
    telegram_poll_loop_owner: bool,
    native_real_mutation: bool,
    production_readiness_classification: &'static str,
}

pub(super) fn watchdog_state_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    json_or_error(&watchdog_state(options, telegram_plugin))
}

fn watchdog_state(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> WatchdogStateResponse {
    let operator_report = operator_security_report(options, telegram_plugin);
    let route_report = &operator_report.control_ui_route_parity;
    let owner = operator_report.telegram_owner_handoff_status.clone();
    let production = operator_report.telegram_production_readiness_status.clone();
    let activation = operator_report.post_activation_plan.clone();
    let stores = operator_report.post_execution_stores.clone();
    let poll = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let architecture = architecture_state();
    let legacy_poll_safe = matches!(poll.status, "gated" | "disabled")
        && !poll.worker_spawned_by_status
        && !poll.external_network_read_by_status
        && !poll.external_send_by_status;
    let legacy_boundary_ready = operator_report.status == "attention"
        && operator_report.security_mode == "legacy_owner_coexistence_ready"
        && operator_report.legacy_owner_coexistence_ready
        && operator_report.attention_reason == "telegram_replacement_not_requested"
        && owner.active_owner == "legacy_openclaw"
        && owner.conflict_free
        && !owner.double_poller_risk
        && !owner.hepta_poll_loop_armed
        && legacy_poll_safe
        && !activation.activation_currently_enabled
        && !activation.real_mutation_performed
        && !activation.external_side_effects
        && stores.status == "ready"
        && stores.store_jsonl_valid
        && stores.store_capacity_ok;
    let manifest_sha256 = crate::route_manifest::route_manifest_digest().unwrap_or_default();
    let product_boundary = watchdog_product_boundary_state(
        legacy_boundary_ready,
        &owner,
        &poll,
        &operator_report.telegram_gate_summary,
        &activation,
    );
    let ready = product_boundary.status == "ready"
        && route_report.ready
        && manifest_sha256.len() == 64
        && architecture.all_ready;
    WatchdogStateResponse {
        schema_version: WATCHDOG_STATE_SCHEMA,
        product: "Hepta",
        runtime: "hepta",
        status: if ready { "ready" } else { "blocked" },
        side_effect_free: true,
        route: WatchdogRouteState {
            status: route_report.status,
            route_count: route_report.route_count,
            implemented_route_count: route_report.implemented_route_count,
            missing_route_count: route_report.missing_route_count,
            manifest_schema_version: crate::route_manifest::ROUTE_EFFECT_GATE_MANIFEST_SCHEMA,
            manifest_sha256,
        },
        operator: WatchdogOperatorState {
            status: operator_report.status,
            security_mode: operator_report.security_mode,
            legacy_owner_coexistence_ready: operator_report.legacy_owner_coexistence_ready,
            attention_reason: operator_report.attention_reason,
            loopback_bound: operator_report.loopback_bound,
        },
        owner,
        poll,
        production,
        native_post: WatchdogNativePostState { activation, stores },
        architecture,
        product_boundary,
    }
}

fn watchdog_product_boundary_state(
    legacy_boundary_ready: bool,
    owner: &NativeTelegramOwnerHandoffStatus,
    poll: &native_telegram::NativeTelegramPollLoopStatus,
    gates: &native_telegram::NativeTelegramGatewayGateSummary,
    activation: &NativePostActivationPlanResponse,
) -> WatchdogProductBoundaryState {
    let telegram_poll_loop_owner = owner.hepta_poll_loop_armed || poll.loop_invokes_drain_once;
    let telegram_external_read = owner.hepta_telegram_requested && gates.live_read_gate_enabled;
    let telegram_external_send = owner.hepta_telegram_requested
        && gates.delivery_approval_gate_enabled
        && gates.model_turn_gate_enabled
        && gates.send_gate_enabled;
    let native_real_mutation = activation.activation_currently_enabled
        || activation.real_mutation_performed
        || activation.external_side_effects
        || activation.gateway_mutation_performed
        || activation.task_published
        || activation.chat_mutated;
    let safe = legacy_boundary_ready
        && owner.active_owner == "legacy_openclaw"
        && !telegram_external_read
        && !telegram_external_send
        && !telegram_poll_loop_owner
        && !native_real_mutation;
    WatchdogProductBoundaryState {
        status: if safe { "ready" } else { "blocked" },
        role: hepta_contracts::OPENCLAW_GOVERNED_BACKEND_ROLE,
        channel_owner: owner.active_owner,
        telegram_external_read,
        telegram_external_send,
        telegram_poll_loop_owner,
        native_real_mutation,
        production_readiness_classification: if safe {
            "legacy_openclaw_gated"
        } else {
            "boundary_violation"
        },
    }
}

fn architecture_state() -> WatchdogArchitectureState {
    let adapter = hepta_gateway::hepta_codex_engine_adapter_boundary_report();
    let core = hepta_gateway::hepta_core_fusion_readiness_report();
    let closure = hepta_gateway::hepta_name_repository_closure_report();
    let dependency = hepta_gateway::hepta_engine_dependency_closure_report();
    let adapter_ready = adapter.status == "ready"
        && adapter.source_command == HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND
        && adapter.canonical_endpoint == HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT
        && adapter.canonical_source_command == HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND
        && adapter.transition_alias_endpoint == HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT
        && adapter.transition_alias_source_command
            == HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND
        && adapter.hepta_named_route_alias_ready
        && adapter.transition_alias_retained
        && adapter.boundary_ready
        && adapter.surfaces.len() >= 6
        && adapter.surfaces.iter().all(|surface| {
            surface.typed_request_response_envelope_ready
                && surface.typed_adapter_parity_gate_ready
                && !surface.live_mutation_allowed
        })
        && adapter.parity_evidence.len() >= 6
        && adapter.parity_evidence.iter().all(|evidence| {
            evidence.evidence_ready
                && evidence.compatibility_dispatch_checked
                && evidence.behavior_equivalence_checked
                && evidence.observable_behavior_preserved
                && evidence.shadow_replay_checked
                && evidence.shadow_replay_observable_match
                && evidence.shadow_replay_side_effect_free
                && evidence.live_mutation_blocked
                && evidence.forbidden_side_effects_blocked
        })
        && adapter.adapter_parity_complete
        && adapter.adapter_parity_promotion_ready
        && adapter.adapter_parity_completion_gate_ready
        && adapter.adapter_parity_completion_gate_allows_promotion
        && adapter.adapter_shadow_replay_required_surface_count == adapter.surfaces.len()
        && adapter.adapter_shadow_replay_covered_surface_count == adapter.surfaces.len()
        && adapter.adapter_shadow_replay_remaining_surface_count == 0
        && adapter.adapter_parity_promotion_blockers.is_empty()
        && adapter.full_fusion_complete;
    let core_ready = core.status == "ready"
        && core.phase == "phase_5_engine_dependency_closure"
        && core.phase_2_engine_adapter_boundary_ready
        && core.phase_3_binary_package_inversion_ready
        && core.binary_package_inversion_gate_ready
        && core.binary_package_inversion_blockers.is_empty()
        && core.active_binary_package == "hepta-cli"
        && core.active_binary_target == "hepta"
        && core.intended_binary_package == "hepta-cli"
        && core.intended_binary_target == "hepta"
        && core.full_fusion_complete
        && core.phase_4_name_repository_closure_gate_ready
        && core.phase_4_name_repository_closure_ready
        && core.phase_4_name_repository_closure_remaining_surface_count == 0
        && core.phase_4_name_repository_closure_blockers.is_empty()
        && core.phase_5_engine_dependency_closure_gate_ready
        && core.phase_5_engine_dependency_closure_remaining_dependency_count == 0
        && core.phase_5_engine_dependency_closure_blockers.is_empty();
    let closure_ready = closure.status == "ready"
        && closure.closure_gate_ready
        && closure.phase_4_name_repository_closure_ready
        && closure.full_fusion_complete
        && closure.transition_surface_count >= 6
        && closure.closed_transition_surface_count == closure.transition_surface_count
        && closure.remaining_transition_surface_count == 0
        && closure.blockers.is_empty()
        && closure
            .surfaces
            .iter()
            .all(|surface| !surface.blocks_full_fusion);
    let dependency_ready = dependency.status == "ready"
        && dependency.closure_gate_ready
        && dependency.full_fusion_complete
        && dependency.direct_dependency_count >= 10
        && dependency.adapter_retained_dependency_count == 0
        && dependency.closed_direct_dependency_count == dependency.direct_dependency_count
        && dependency.remaining_direct_dependency_count == 0
        && dependency.blockers.is_empty()
        && dependency.surfaces.iter().all(|surface| {
            surface.closure_state == "closed_active_hepta_service_binary_isolated"
                && !surface.direct_dependency_retained
                && !surface.compatibility_adapter_required
                && surface.typed_adapter_parity_ready
                && !surface.blocks_full_fusion
        });
    let forbidden_side_effects_clear = [
        adapter.forbidden_real_side_effects,
        core.forbidden_real_side_effects,
        closure.forbidden_real_side_effects,
        dependency.forbidden_real_side_effects,
    ]
    .iter()
    .all(|effects| {
        !effects.public_ga_claimed
            && !effects.public_release_published
            && !effects.native_post_real_mutation_performed
            && !effects.task_publish_real_mutation_performed
            && !effects.telegram_send_performed
            && !effects.gateway_mutation_performed
            && !effects.launchd_mutated
            && !effects.credential_read
            && !effects.model_invoked
            && !effects.external_network_read
    });
    let all_ready = adapter_ready
        && core_ready
        && closure_ready
        && dependency_ready
        && forbidden_side_effects_clear;
    WatchdogArchitectureState {
        status: if all_ready { "ready" } else { "blocked" },
        all_ready,
        adapter_status: adapter.status,
        adapter_alias_status: adapter.status,
        adapter_surface_count: adapter.surfaces.len(),
        adapter_parity_evidence_count: adapter.parity_evidence.len(),
        adapter_parity_complete: adapter.adapter_parity_complete,
        adapter_parity_promotion_ready: adapter.adapter_parity_promotion_ready,
        adapter_parity_completion_gate_ready: adapter.adapter_parity_completion_gate_ready,
        adapter_parity_completion_gate_status: adapter.adapter_parity_completion_gate_status,
        adapter_parity_completion_gate_allows_promotion: adapter
            .adapter_parity_completion_gate_allows_promotion,
        adapter_shadow_replay_required_surface_count: adapter
            .adapter_shadow_replay_required_surface_count,
        adapter_shadow_replay_covered_surface_count: adapter
            .adapter_shadow_replay_covered_surface_count,
        adapter_shadow_replay_remaining_surface_count: adapter
            .adapter_shadow_replay_remaining_surface_count,
        adapter_parity_promotion_blocker_count: adapter.adapter_parity_promotion_blockers.len(),
        core_status: core.status,
        core_phase: core.phase,
        phase_3_binary_package_inversion_ready: core.phase_3_binary_package_inversion_ready,
        binary_package_inversion_gate_status: core.binary_package_inversion_gate_status,
        binary_package_inversion_blocker_count: core.binary_package_inversion_blockers.len(),
        active_binary_package: core.active_binary_package,
        intended_binary_package: core.intended_binary_package,
        installed_service_binary: core.installed_service_binary,
        phase_4_name_repository_closure_gate_status: core
            .phase_4_name_repository_closure_gate_status,
        phase_4_name_repository_closure_ready: core.phase_4_name_repository_closure_ready,
        phase_4_name_repository_closure_remaining_surface_count: core
            .phase_4_name_repository_closure_remaining_surface_count,
        name_repository_closure_status: closure.status,
        name_repository_closure_gate_status: closure.closure_gate_status,
        name_repository_closure_remaining_surface_count: closure.remaining_transition_surface_count,
        name_repository_closure_blocker_count: closure.blockers.len(),
        phase_5_engine_dependency_closure_gate_status: core
            .phase_5_engine_dependency_closure_gate_status,
        phase_5_engine_dependency_closure_ready: core.phase_5_engine_dependency_closure_gate_ready,
        phase_5_engine_dependency_closure_remaining_dependency_count: core
            .phase_5_engine_dependency_closure_remaining_dependency_count,
        engine_dependency_closure_status: dependency.status,
        engine_dependency_closure_gate_status: dependency.closure_gate_status,
        engine_dependency_closure_remaining_dependency_count: dependency
            .remaining_direct_dependency_count,
        engine_dependency_closure_blocker_count: dependency.blockers.len(),
        full_fusion_complete: adapter.full_fusion_complete
            && core.full_fusion_complete
            && closure.full_fusion_complete
            && dependency.full_fusion_complete,
        forbidden_side_effects_clear,
        adapter,
        core,
        closure,
        dependency,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_is_compact_typed_and_keeps_external_effects_disabled() {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: false,
            telegram_plugin_poll_ms: 1_500,
        };
        let plugin = native_telegram::telegram_plugin_status(false, 1_500);
        let body = watchdog_state_json(&options, &plugin);
        assert!(body.len() < report_pagination::MAX_DEFAULT_REPORT_BYTES);
        let value: serde_json::Value = serde_json::from_str(&body).expect("watchdog JSON");
        assert_eq!(value["schema_version"], WATCHDOG_STATE_SCHEMA);
        assert_eq!(value["status"], "ready");
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["poll"]["status"], "disabled");
        assert_eq!(value["poll"]["external_network_read_by_status"], false);
        assert_eq!(value["poll"]["external_send_by_status"], false);
        assert_eq!(
            value["native_post"]["activation"]["real_mutation_performed"],
            false
        );
        assert_eq!(
            value["native_post"]["activation"]["external_side_effects"],
            false
        );
        assert_eq!(value["architecture"]["all_ready"], true);
        assert_eq!(value["route"]["missing_route_count"], 0);
        assert_eq!(value["product_boundary"]["telegram_external_read"], false);
        assert_eq!(value["product_boundary"]["telegram_external_send"], false);
        assert_eq!(value["product_boundary"]["telegram_poll_loop_owner"], false);
        assert_eq!(value["product_boundary"]["native_real_mutation"], false);
        assert_eq!(value["product_boundary"]["status"], "ready");
    }

    #[test]
    fn projection_surfaces_unsafe_owner_gates_and_mutation_instead_of_masking_them() {
        let owner = telegram_owner_handoff_status_from_inputs(NativeTelegramOwnerHandoffInputs {
            legacy_config_path: Some("/tmp/openclaw.json".to_string()),
            legacy_config_found: true,
            legacy_config_parse_ok: true,
            legacy_telegram_enabled: Some(true),
            legacy_token_fingerprint: Some("sha256:samebot00000000".to_string()),
            legacy_config_error: None,
            hepta_token_fingerprint: Some("sha256:samebot00000000".to_string()),
            hepta_telegram_requested: true,
            hepta_poll_loop_armed: true,
            hepta_poll_loop_gate_enabled: true,
            hepta_delivery_approval_gate_enabled: true,
        });
        let poll = hepta_gateway::build_telegram_poll_loop_status(
            native_telegram::NativeTelegramPollLoopStatusInput {
                requested: true,
                poll_ms: 1_500,
                poll_loop_gate_env: native_telegram::TELEGRAM_POLL_LOOP_ENV,
                poll_loop_gate_enabled: true,
                delivery_approval_gate_env: native_telegram::TELEGRAM_DELIVERY_APPROVED_ENV,
                delivery_approval_gate_enabled: true,
                live_read_gate_env: native_telegram::TELEGRAM_LIVE_READ_ENV,
                model_turn_gate_env: native_telegram::TELEGRAM_MODEL_TURN_GATE_ENV,
                send_gate_env: native_telegram::TELEGRAM_SEND_GATE_ENV,
            },
        );
        let gates = hepta_gateway::build_telegram_gateway_gate_summary(
            native_telegram::NativeTelegramGatewayGateSummaryInput {
                delivery_approval_gate_env: native_telegram::TELEGRAM_DELIVERY_APPROVED_ENV,
                delivery_approval_gate_enabled: true,
                live_read_gate_env: native_telegram::TELEGRAM_LIVE_READ_ENV,
                live_read_gate_enabled: true,
                model_turn_gate_env: native_telegram::TELEGRAM_MODEL_TURN_GATE_ENV,
                model_turn_gate_enabled: true,
                send_gate_env: native_telegram::TELEGRAM_SEND_GATE_ENV,
                send_gate_enabled: true,
            },
        );
        let mut activation = native_post_activation_plan_report();
        activation.activation_currently_enabled = true;
        activation.real_mutation_performed = true;
        activation.external_side_effects = true;
        let boundary = watchdog_product_boundary_state(false, &owner, &poll, &gates, &activation);

        assert_eq!(boundary.status, "blocked");
        assert_eq!(boundary.channel_owner, "conflict_risk");
        assert!(boundary.telegram_external_read);
        assert!(boundary.telegram_external_send);
        assert!(boundary.telegram_poll_loop_owner);
        assert!(boundary.native_real_mutation);
        assert_eq!(
            boundary.production_readiness_classification,
            "boundary_violation"
        );
    }
}
