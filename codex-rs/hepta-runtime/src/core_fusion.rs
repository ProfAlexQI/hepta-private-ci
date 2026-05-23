use serde::Serialize;

pub const HEPTA_CORE_FUSION_READINESS_ENDPOINT: &str = "/api/hepta-core-fusion-readiness";
pub const HEPTA_CORE_FUSION_READINESS_SOURCE_COMMAND: &str = "/hepta-core-fusion-readiness --json";
pub const HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT: &str =
    "/api/hepta-codex-engine-adapter-boundary";
pub const HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND: &str =
    "/hepta-codex-engine-adapter-boundary --json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaCoreFusionReadinessResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub source_command: &'static str,
    pub native_route: bool,
    pub compatibility_mode: &'static str,
    pub side_effect_free: bool,
    pub phase: &'static str,
    pub root_owner: &'static str,
    pub product_runtime_owner: &'static str,
    pub gateway_owner: &'static str,
    pub engine_adapter_owner: &'static str,
    pub codex_engine_role: &'static str,
    pub phase_1_root_ownership_inversion_ready: bool,
    pub product_runtime_entrypoint_facade_ready: bool,
    pub phase_2_engine_adapter_boundary_ready: bool,
    pub phase_3_binary_package_inversion_ready: bool,
    pub phase_4_name_repository_closure_ready: bool,
    pub full_fusion_complete: bool,
    pub hepta_owned_root_surfaces: &'static [&'static str],
    pub codex_engine_adapter_surfaces: &'static [&'static str],
    pub direct_codex_base_dependencies: &'static [&'static str],
    pub remaining_direct_codex_base_dependency_count: usize,
    pub forbidden_real_side_effects: HeptaCoreFusionForbiddenSideEffects,
    pub blockers: &'static [&'static str],
    pub next_actions: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaCoreFusionForbiddenSideEffects {
    pub public_ga_claimed: bool,
    pub public_release_published: bool,
    pub native_post_real_mutation_performed: bool,
    pub task_publish_real_mutation_performed: bool,
    pub telegram_send_performed: bool,
    pub gateway_mutation_performed: bool,
    pub launchd_mutated: bool,
    pub credential_read: bool,
    pub model_invoked: bool,
    pub external_network_read: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaProductRuntimeEntrypointInput<'a> {
    pub native_gateway_requested: bool,
    pub first_cli_arg: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaProductRuntimeEntrypointPlan {
    pub product: &'static str,
    pub root_owner: &'static str,
    pub facade_owner: &'static str,
    pub dispatch_target: &'static str,
    pub native_gateway_dispatch: bool,
    pub codex_compatibility_dispatch_required: bool,
    pub cli_parse_required: bool,
    pub codex_engine_role: &'static str,
    pub first_cli_arg_kind: &'static str,
    pub side_effect_free: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaCodexEngineAdapterSurface {
    pub surface_id: &'static str,
    pub hepta_boundary_owner: &'static str,
    pub codex_dependency: &'static str,
    pub adapter_contract: &'static str,
    pub migration_state: &'static str,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaCodexEngineAdapterBoundaryResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub source_command: &'static str,
    pub native_route: bool,
    pub phase: &'static str,
    pub root_owner: &'static str,
    pub adapter_owner: &'static str,
    pub codex_engine_role: &'static str,
    pub boundary_ready: bool,
    pub adapter_parity_complete: bool,
    pub full_fusion_complete: bool,
    pub remaining_direct_codex_base_dependency_count: usize,
    pub surfaces: &'static [HeptaCodexEngineAdapterSurface],
    pub forbidden_real_side_effects: HeptaCoreFusionForbiddenSideEffects,
    pub next_actions: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaCodexEngineAdapterThreadingPlan {
    pub product: &'static str,
    pub root_owner: &'static str,
    pub adapter_owner: &'static str,
    pub surface_id: &'static str,
    pub hepta_boundary_owner: &'static str,
    pub codex_dependency: &'static str,
    pub operation: String,
    pub adapter_threaded: bool,
    pub compatibility_dispatch_allowed: bool,
    pub direct_codex_dependency_retained: bool,
    pub live_mutation_allowed_by_plan: bool,
    pub provider_invoked_by_plan: bool,
    pub credential_read_by_plan: bool,
    pub session_store_mutated_by_plan: bool,
    pub external_network_read_by_plan: bool,
    pub side_effect_free: bool,
}

const HEPTA_OWNED_ROOT_SURFACES: &[&str] = &[
    "hepta-core",
    "hepta-kernel",
    "hepta-runtime",
    "hepta-gateway",
    "native gateway read-only reports",
    "Telegram policy/readiness planning",
    "native-post dry-run evidence planning",
];

const CODEX_ENGINE_ADAPTER_SURFACES: &[&str] = &[
    "model provider execution",
    "session and thread store compatibility",
    "tool invocation compatibility",
    "sandbox and exec compatibility",
    "MCP and app-server compatibility",
    "TUI and legacy command compatibility",
];

const DIRECT_CODEX_BASE_DEPENDENCIES: &[&str] = &[
    "codex-core",
    "codex-exec",
    "codex-tui",
    "codex-state",
    "codex-app-server",
    "codex-mcp",
    "codex-sandboxing",
    "codex-plugin",
    "codex-model-provider",
    "codex-protocol",
];

const PHASE_1_BLOCKERS: &[&str] = &[];

const NEXT_ACTIONS: &[&str] = &[
    "extend CodexEngineAdapter contracts from model/session compatibility dispatch into tool, sandbox, MCP, app-server, and legacy TUI surfaces",
    "promote the installed binary from codex-cli --bin hepta toward first-class Hepta binary ownership after adapter parity",
    "keep public-release and task_publish real-mutation lines blocked until explicit operator approval",
];

const CODEX_ENGINE_ADAPTER_BOUNDARY_SURFACES: &[HeptaCodexEngineAdapterSurface] = &[
    HeptaCodexEngineAdapterSurface {
        surface_id: "model_provider_execution",
        hepta_boundary_owner: "hepta-runtime",
        codex_dependency: "codex-model-provider",
        adapter_contract: "model invocation must enter through a Hepta-owned request/response boundary before provider dispatch",
        migration_state: "adapter_threaded_compatibility_dispatch",
        live_mutation_allowed: false,
    },
    HeptaCodexEngineAdapterSurface {
        surface_id: "session_thread_store",
        hepta_boundary_owner: "hepta-runtime",
        codex_dependency: "codex-state",
        adapter_contract: "session and thread persistence must be described as Hepta records before Codex store compatibility is used",
        migration_state: "adapter_threaded_compatibility_dispatch",
        live_mutation_allowed: false,
    },
    HeptaCodexEngineAdapterSurface {
        surface_id: "tool_invocation",
        hepta_boundary_owner: "hepta-kernel",
        codex_dependency: "codex-core",
        adapter_contract: "tool calls must carry Hepta policy, approval, and side-effect classification before Codex tool execution",
        migration_state: "boundary_defined_adapter_pending",
        live_mutation_allowed: false,
    },
    HeptaCodexEngineAdapterSurface {
        surface_id: "sandbox_exec",
        hepta_boundary_owner: "hepta-kernel",
        codex_dependency: "codex-exec",
        adapter_contract: "exec and sandbox requests must pass Hepta policy gates before Codex sandbox compatibility runs",
        migration_state: "boundary_defined_adapter_pending",
        live_mutation_allowed: false,
    },
    HeptaCodexEngineAdapterSurface {
        surface_id: "mcp_app_server",
        hepta_boundary_owner: "hepta-gateway",
        codex_dependency: "codex-mcp",
        adapter_contract: "MCP and app-server traffic must remain behind read-only Hepta route contracts until explicit adapter parity",
        migration_state: "boundary_defined_adapter_pending",
        live_mutation_allowed: false,
    },
    HeptaCodexEngineAdapterSurface {
        surface_id: "legacy_tui_cli",
        hepta_boundary_owner: "hepta-runtime",
        codex_dependency: "codex-tui",
        adapter_contract: "legacy TUI and CLI behavior must remain compatibility-dispatched until first-class Hepta binary parity",
        migration_state: "boundary_defined_adapter_pending",
        live_mutation_allowed: false,
    },
];

pub fn hepta_product_runtime_entrypoint_plan(
    input: HeptaProductRuntimeEntrypointInput<'_>,
) -> HeptaProductRuntimeEntrypointPlan {
    let first_cli_arg_kind = input
        .first_cli_arg
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(classify_first_cli_arg)
        .unwrap_or("interactive_default");
    let native_gateway_dispatch = input.native_gateway_requested;

    HeptaProductRuntimeEntrypointPlan {
        product: "Hepta",
        root_owner: "hepta",
        facade_owner: "hepta-runtime",
        dispatch_target: if native_gateway_dispatch {
            "hepta_native_gateway"
        } else {
            "codex_compatibility_cli_dispatch"
        },
        native_gateway_dispatch,
        codex_compatibility_dispatch_required: !native_gateway_dispatch,
        cli_parse_required: !native_gateway_dispatch,
        codex_engine_role: "internal_engine_adapter",
        first_cli_arg_kind,
        side_effect_free: true,
    }
}

pub fn hepta_codex_model_provider_adapter_threading_plan(
    operation: impl AsRef<str>,
) -> HeptaCodexEngineAdapterThreadingPlan {
    hepta_codex_engine_adapter_threading_plan(
        "model_provider_execution",
        "hepta-runtime",
        "codex-model-provider",
        operation.as_ref(),
    )
}

pub fn hepta_codex_session_thread_store_adapter_threading_plan(
    operation: impl AsRef<str>,
) -> HeptaCodexEngineAdapterThreadingPlan {
    hepta_codex_engine_adapter_threading_plan(
        "session_thread_store",
        "hepta-runtime",
        "codex-state",
        operation.as_ref(),
    )
}

fn hepta_codex_engine_adapter_threading_plan(
    surface_id: &'static str,
    hepta_boundary_owner: &'static str,
    codex_dependency: &'static str,
    operation: &str,
) -> HeptaCodexEngineAdapterThreadingPlan {
    let operation = operation.trim();

    HeptaCodexEngineAdapterThreadingPlan {
        product: "Hepta",
        root_owner: "hepta",
        adapter_owner: "codex-engine-adapter",
        surface_id,
        hepta_boundary_owner,
        codex_dependency,
        operation: if operation.is_empty() {
            "codex_compatibility_dispatch".to_string()
        } else {
            operation.to_string()
        },
        adapter_threaded: true,
        compatibility_dispatch_allowed: true,
        direct_codex_dependency_retained: true,
        live_mutation_allowed_by_plan: false,
        provider_invoked_by_plan: false,
        credential_read_by_plan: false,
        session_store_mutated_by_plan: false,
        external_network_read_by_plan: false,
        side_effect_free: true,
    }
}

pub fn hepta_codex_engine_adapter_boundary_report() -> HeptaCodexEngineAdapterBoundaryResponse {
    HeptaCodexEngineAdapterBoundaryResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: "ready",
        source_command: HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND,
        native_route: true,
        phase: "phase_2_engine_adapter_boundary",
        root_owner: "hepta",
        adapter_owner: "codex-engine-adapter",
        codex_engine_role: "internal_engine_adapter",
        boundary_ready: true,
        adapter_parity_complete: false,
        full_fusion_complete: false,
        remaining_direct_codex_base_dependency_count: DIRECT_CODEX_BASE_DEPENDENCIES.len(),
        surfaces: CODEX_ENGINE_ADAPTER_BOUNDARY_SURFACES,
        forbidden_real_side_effects: HeptaCoreFusionForbiddenSideEffects {
            public_ga_claimed: false,
            public_release_published: false,
            native_post_real_mutation_performed: false,
            task_publish_real_mutation_performed: false,
            telegram_send_performed: false,
            gateway_mutation_performed: false,
            launchd_mutated: false,
            credential_read: false,
            model_invoked: false,
            external_network_read: false,
        },
        next_actions: NEXT_ACTIONS,
    }
}

pub fn hepta_core_fusion_readiness_report() -> HeptaCoreFusionReadinessResponse {
    HeptaCoreFusionReadinessResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: "ready",
        source_command: HEPTA_CORE_FUSION_READINESS_SOURCE_COMMAND,
        native_route: true,
        compatibility_mode: "hepta_root_ownership_inversion_with_engine_adapter_boundary",
        side_effect_free: true,
        phase: "phase_2_engine_adapter_boundary",
        root_owner: "hepta",
        product_runtime_owner: "hepta-runtime",
        gateway_owner: "hepta-gateway",
        engine_adapter_owner: "codex-engine-adapter",
        codex_engine_role: "internal_engine_adapter",
        phase_1_root_ownership_inversion_ready: true,
        product_runtime_entrypoint_facade_ready: true,
        phase_2_engine_adapter_boundary_ready: true,
        phase_3_binary_package_inversion_ready: false,
        phase_4_name_repository_closure_ready: false,
        full_fusion_complete: false,
        hepta_owned_root_surfaces: HEPTA_OWNED_ROOT_SURFACES,
        codex_engine_adapter_surfaces: CODEX_ENGINE_ADAPTER_SURFACES,
        direct_codex_base_dependencies: DIRECT_CODEX_BASE_DEPENDENCIES,
        remaining_direct_codex_base_dependency_count: DIRECT_CODEX_BASE_DEPENDENCIES.len(),
        forbidden_real_side_effects: HeptaCoreFusionForbiddenSideEffects {
            public_ga_claimed: false,
            public_release_published: false,
            native_post_real_mutation_performed: false,
            task_publish_real_mutation_performed: false,
            telegram_send_performed: false,
            gateway_mutation_performed: false,
            launchd_mutated: false,
            credential_read: false,
            model_invoked: false,
            external_network_read: false,
        },
        blockers: PHASE_1_BLOCKERS,
        next_actions: NEXT_ACTIONS,
    }
}

fn classify_first_cli_arg(arg: &str) -> &'static str {
    match arg {
        "exec" | "e" | "review" | "resume" | "fork" | "debug" => "runtime_command",
        "mcp" | "plugin" | "mcp-server" | "app-server" | "remote-control" | "cloud"
        | "cloud-tasks" => "service_or_integration_command",
        "login" | "logout" | "doctor" | "completion" | "update" | "sandbox" | "features" => {
            "operator_command"
        }
        value if value.starts_with('-') => "interactive_option",
        _ => "prompt_or_compatibility_command",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HeptaProductRuntimeEntrypointInput, hepta_codex_engine_adapter_boundary_report,
        hepta_codex_model_provider_adapter_threading_plan,
        hepta_codex_session_thread_store_adapter_threading_plan,
        hepta_core_fusion_readiness_report, hepta_product_runtime_entrypoint_plan,
    };

    #[test]
    fn report_marks_hepta_as_root_owner_and_codex_as_internal_engine() {
        let report = hepta_core_fusion_readiness_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.root_owner, "hepta");
        assert_eq!(report.product_runtime_owner, "hepta-runtime");
        assert_eq!(report.engine_adapter_owner, "codex-engine-adapter");
        assert_eq!(report.codex_engine_role, "internal_engine_adapter");
        assert!(report.phase_1_root_ownership_inversion_ready);
        assert!(report.product_runtime_entrypoint_facade_ready);
        assert!(report.phase_2_engine_adapter_boundary_ready);
        assert!(!report.full_fusion_complete);
        assert!(
            report
                .direct_codex_base_dependencies
                .contains(&"codex-core")
        );
        assert!(report.remaining_direct_codex_base_dependency_count > 0);
    }

    #[test]
    fn report_does_not_claim_public_release_or_real_mutation() {
        let report = hepta_core_fusion_readiness_report();
        let side_effects = report.forbidden_real_side_effects;

        assert!(report.side_effect_free);
        assert!(!side_effects.public_ga_claimed);
        assert!(!side_effects.public_release_published);
        assert!(!side_effects.native_post_real_mutation_performed);
        assert!(!side_effects.task_publish_real_mutation_performed);
        assert!(!side_effects.gateway_mutation_performed);
        assert!(!side_effects.credential_read);
        assert!(!side_effects.model_invoked);
        assert!(!side_effects.external_network_read);
    }

    #[test]
    fn entrypoint_facade_selects_native_gateway_before_cli_parse_when_requested() {
        let plan = hepta_product_runtime_entrypoint_plan(HeptaProductRuntimeEntrypointInput {
            native_gateway_requested: true,
            first_cli_arg: Some("exec"),
        });

        assert_eq!(plan.root_owner, "hepta");
        assert_eq!(plan.facade_owner, "hepta-runtime");
        assert_eq!(plan.dispatch_target, "hepta_native_gateway");
        assert!(plan.native_gateway_dispatch);
        assert!(!plan.codex_compatibility_dispatch_required);
        assert!(!plan.cli_parse_required);
        assert_eq!(plan.codex_engine_role, "internal_engine_adapter");
        assert!(plan.side_effect_free);
    }

    #[test]
    fn entrypoint_facade_keeps_codex_compatibility_dispatch_explicit() {
        let plan = hepta_product_runtime_entrypoint_plan(HeptaProductRuntimeEntrypointInput {
            native_gateway_requested: false,
            first_cli_arg: Some("review"),
        });

        assert_eq!(plan.dispatch_target, "codex_compatibility_cli_dispatch");
        assert!(!plan.native_gateway_dispatch);
        assert!(plan.codex_compatibility_dispatch_required);
        assert!(plan.cli_parse_required);
        assert_eq!(plan.first_cli_arg_kind, "runtime_command");
        assert_eq!(plan.codex_engine_role, "internal_engine_adapter");
    }

    #[test]
    fn codex_engine_adapter_boundary_lists_all_remaining_runtime_surfaces() {
        let report = hepta_codex_engine_adapter_boundary_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.root_owner, "hepta");
        assert_eq!(report.adapter_owner, "codex-engine-adapter");
        assert!(report.boundary_ready);
        assert!(!report.adapter_parity_complete);
        assert!(!report.full_fusion_complete);
        assert_eq!(
            report.remaining_direct_codex_base_dependency_count,
            super::DIRECT_CODEX_BASE_DEPENDENCIES.len()
        );

        let surface_ids = report
            .surfaces
            .iter()
            .map(|surface| surface.surface_id)
            .collect::<Vec<_>>();
        assert!(surface_ids.contains(&"model_provider_execution"));
        assert!(surface_ids.contains(&"session_thread_store"));
        assert!(surface_ids.contains(&"tool_invocation"));
        assert!(surface_ids.contains(&"sandbox_exec"));
        assert!(surface_ids.contains(&"mcp_app_server"));
        assert!(surface_ids.contains(&"legacy_tui_cli"));
        assert!(
            report
                .surfaces
                .iter()
                .all(|surface| !surface.live_mutation_allowed)
        );
        assert!(report.surfaces.iter().any(|surface| {
            surface.surface_id == "model_provider_execution"
                && surface.migration_state == "adapter_threaded_compatibility_dispatch"
        }));
        assert!(report.surfaces.iter().any(|surface| {
            surface.surface_id == "session_thread_store"
                && surface.migration_state == "adapter_threaded_compatibility_dispatch"
        }));
    }

    #[test]
    fn codex_engine_adapter_boundary_is_side_effect_free() {
        let side_effects = hepta_codex_engine_adapter_boundary_report().forbidden_real_side_effects;

        assert!(!side_effects.public_ga_claimed);
        assert!(!side_effects.public_release_published);
        assert!(!side_effects.native_post_real_mutation_performed);
        assert!(!side_effects.task_publish_real_mutation_performed);
        assert!(!side_effects.telegram_send_performed);
        assert!(!side_effects.gateway_mutation_performed);
        assert!(!side_effects.credential_read);
        assert!(!side_effects.model_invoked);
        assert!(!side_effects.external_network_read);
    }

    #[test]
    fn model_provider_adapter_threading_plan_preserves_compatibility_dispatch() {
        let plan =
            hepta_codex_model_provider_adapter_threading_plan("interactive_tui_model_provider");

        assert_eq!(plan.root_owner, "hepta");
        assert_eq!(plan.surface_id, "model_provider_execution");
        assert_eq!(plan.codex_dependency, "codex-model-provider");
        assert!(plan.adapter_threaded);
        assert!(plan.compatibility_dispatch_allowed);
        assert!(plan.direct_codex_dependency_retained);
        assert!(!plan.live_mutation_allowed_by_plan);
        assert!(!plan.provider_invoked_by_plan);
        assert!(!plan.credential_read_by_plan);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn session_thread_store_adapter_threading_plan_preserves_store_semantics() {
        let plan =
            hepta_codex_session_thread_store_adapter_threading_plan("interactive_tui_state_store");

        assert_eq!(plan.surface_id, "session_thread_store");
        assert_eq!(plan.codex_dependency, "codex-state");
        assert!(plan.adapter_threaded);
        assert!(plan.compatibility_dispatch_allowed);
        assert!(plan.direct_codex_dependency_retained);
        assert!(!plan.session_store_mutated_by_plan);
        assert!(!plan.external_network_read_by_plan);
        assert!(plan.side_effect_free);
    }
}
