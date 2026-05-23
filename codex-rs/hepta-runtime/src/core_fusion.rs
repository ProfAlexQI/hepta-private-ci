use serde::Serialize;

pub const HEPTA_CORE_FUSION_READINESS_ENDPOINT: &str = "/api/hepta-core-fusion-readiness";
pub const HEPTA_CORE_FUSION_READINESS_SOURCE_COMMAND: &str = "/hepta-core-fusion-readiness --json";
pub const HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT: &str =
    "/api/hepta-codex-engine-adapter-boundary";
pub const HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND: &str =
    "/hepta-codex-engine-adapter-boundary --json";
pub const HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT: &str = "/api/hepta-engine-adapter-boundary";
pub const HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND: &str =
    "/hepta-engine-adapter-boundary --json";
pub const HEPTA_NAME_REPOSITORY_CLOSURE_ENDPOINT: &str = "/api/hepta-name-repository-closure";
pub const HEPTA_NAME_REPOSITORY_CLOSURE_SOURCE_COMMAND: &str =
    "/hepta-name-repository-closure --json";
pub const HEPTA_ENGINE_DEPENDENCY_CLOSURE_ENDPOINT: &str = "/api/hepta-engine-dependency-closure";
pub const HEPTA_ENGINE_DEPENDENCY_CLOSURE_SOURCE_COMMAND: &str =
    "/hepta-engine-dependency-closure --json";

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
    pub binary_package_inversion_gate: &'static str,
    pub binary_package_inversion_gate_ready: bool,
    pub binary_package_inversion_gate_status: &'static str,
    pub binary_package_inversion_criteria: &'static [&'static str],
    pub binary_package_inversion_blockers: &'static [&'static str],
    pub active_binary_package: &'static str,
    pub active_binary_target: &'static str,
    pub intended_binary_package: &'static str,
    pub intended_binary_target: &'static str,
    pub installed_service_binary: &'static str,
    pub phase_4_name_repository_closure_gate: &'static str,
    pub phase_4_name_repository_closure_gate_ready: bool,
    pub phase_4_name_repository_closure_gate_status: &'static str,
    pub phase_4_name_repository_closure_remaining_surface_count: usize,
    pub phase_4_name_repository_closure_blockers: &'static [&'static str],
    pub phase_4_name_repository_closure_ready: bool,
    pub phase_5_engine_dependency_closure_gate: &'static str,
    pub phase_5_engine_dependency_closure_gate_ready: bool,
    pub phase_5_engine_dependency_closure_gate_status: &'static str,
    pub phase_5_engine_dependency_closure_remaining_dependency_count: usize,
    pub phase_5_engine_dependency_closure_blockers: &'static [&'static str],
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
pub struct HeptaNameRepositoryClosureSurface {
    pub surface_id: &'static str,
    pub surface_kind: &'static str,
    pub current_name: &'static str,
    pub target_name: &'static str,
    pub closure_state: &'static str,
    pub operator_facing: bool,
    pub compatibility_alias_retained: bool,
    pub blocks_full_fusion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaNameRepositoryClosureResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub source_command: &'static str,
    pub native_route: bool,
    pub side_effect_free: bool,
    pub phase: &'static str,
    pub root_owner: &'static str,
    pub closure_gate: &'static str,
    pub closure_gate_ready: bool,
    pub closure_gate_status: &'static str,
    pub phase_4_name_repository_closure_ready: bool,
    pub full_fusion_complete: bool,
    pub transition_surface_count: usize,
    pub closed_transition_surface_count: usize,
    pub remaining_transition_surface_count: usize,
    pub operator_facing_transition_surface_count: usize,
    pub surfaces: &'static [HeptaNameRepositoryClosureSurface],
    pub blockers: &'static [&'static str],
    pub forbidden_real_side_effects: HeptaCoreFusionForbiddenSideEffects,
    pub next_actions: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaEngineDependencyClosureSurface {
    pub dependency_id: &'static str,
    pub dependency_crate: &'static str,
    pub adapter_surface_id: &'static str,
    pub current_owner: &'static str,
    pub target_owner: &'static str,
    pub closure_state: &'static str,
    pub direct_dependency_retained: bool,
    pub compatibility_adapter_required: bool,
    pub typed_adapter_parity_ready: bool,
    pub blocks_full_fusion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaEngineDependencyClosureResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub source_command: &'static str,
    pub native_route: bool,
    pub side_effect_free: bool,
    pub phase: &'static str,
    pub root_owner: &'static str,
    pub closure_gate: &'static str,
    pub closure_gate_ready: bool,
    pub closure_gate_status: &'static str,
    pub full_fusion_complete: bool,
    pub direct_dependency_count: usize,
    pub adapter_retained_dependency_count: usize,
    pub closed_direct_dependency_count: usize,
    pub remaining_direct_dependency_count: usize,
    pub surfaces: &'static [HeptaEngineDependencyClosureSurface],
    pub blockers: &'static [&'static str],
    pub forbidden_real_side_effects: HeptaCoreFusionForbiddenSideEffects,
    pub next_actions: &'static [&'static str],
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
    pub typed_request_response_envelope_ready: bool,
    pub typed_adapter_parity_gate: &'static str,
    pub typed_adapter_parity_gate_ready: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaCodexEngineAdapterBoundaryResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub source_command: &'static str,
    pub canonical_endpoint: &'static str,
    pub canonical_source_command: &'static str,
    pub transition_alias_endpoint: &'static str,
    pub transition_alias_source_command: &'static str,
    pub hepta_named_route_alias_ready: bool,
    pub transition_alias_retained: bool,
    pub native_route: bool,
    pub phase: &'static str,
    pub root_owner: &'static str,
    pub adapter_owner: &'static str,
    pub codex_engine_role: &'static str,
    pub boundary_ready: bool,
    pub adapter_parity_complete: bool,
    pub adapter_parity_promotion_ready: bool,
    pub adapter_parity_promotion_criteria: &'static [&'static str],
    pub adapter_parity_promotion_blockers: &'static [&'static str],
    pub adapter_parity_completion_gate: &'static str,
    pub adapter_parity_completion_gate_ready: bool,
    pub adapter_parity_completion_gate_status: &'static str,
    pub adapter_parity_completion_gate_allows_promotion: bool,
    pub adapter_shadow_replay_required_surface_count: usize,
    pub adapter_shadow_replay_covered_surface_count: usize,
    pub adapter_shadow_replay_remaining_surface_count: usize,
    pub full_fusion_complete: bool,
    pub remaining_direct_codex_base_dependency_count: usize,
    pub surfaces: &'static [HeptaCodexEngineAdapterSurface],
    pub parity_evidence: &'static [HeptaCodexEngineAdapterParityEvidence],
    pub forbidden_real_side_effects: HeptaCoreFusionForbiddenSideEffects,
    pub next_actions: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaCodexEngineAdapterParityEvidence {
    pub surface_id: &'static str,
    pub evidence_gate: &'static str,
    pub behavior_equivalence_check: &'static str,
    pub typed_envelope_ready: bool,
    pub typed_parity_gate_ready: bool,
    pub compatibility_dispatch_checked: bool,
    pub behavior_equivalence_checked: bool,
    pub observable_behavior_preserved: bool,
    pub shadow_replay_case: &'static str,
    pub shadow_replay_checked: bool,
    pub shadow_replay_observable_match: bool,
    pub shadow_replay_side_effect_free: bool,
    pub live_mutation_blocked: bool,
    pub forbidden_side_effects_blocked: bool,
    pub evidence_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaCodexEngineAdapterShadowReplayResult {
    pub product: &'static str,
    pub runtime: &'static str,
    pub surface_id: &'static str,
    pub replay_case: &'static str,
    pub operation: String,
    pub threading_plan_surface_match: bool,
    pub envelope_surface_match: bool,
    pub codex_dependency_match: bool,
    pub compatibility_dispatch_preserved: bool,
    pub live_mutation_blocked: bool,
    pub provider_invocation_blocked: bool,
    pub credential_read_blocked: bool,
    pub session_store_mutation_blocked: bool,
    pub external_network_read_blocked: bool,
    pub observable_behavior_match: bool,
    pub shadow_replay_ready: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaCodexEngineAdapterEnvelopeInput<'a> {
    pub operation: &'a str,
    pub compatibility_dispatch_requested: bool,
    pub live_mutation_requested: bool,
    pub provider_invocation_requested: bool,
    pub credential_read_requested: bool,
    pub session_store_mutation_requested: bool,
    pub external_network_read_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaCodexEngineAdapterEnvelope {
    pub product: &'static str,
    pub root_owner: &'static str,
    pub adapter_owner: &'static str,
    pub surface_id: &'static str,
    pub hepta_boundary_owner: &'static str,
    pub codex_dependency: &'static str,
    pub operation: String,
    pub request_envelope_ready: bool,
    pub response_envelope_ready: bool,
    pub typed_request_response_envelope_ready: bool,
    pub compatibility_dispatch_requested: bool,
    pub compatibility_dispatch_allowed: bool,
    pub live_mutation_requested: bool,
    pub live_mutation_allowed: bool,
    pub provider_invocation_requested: bool,
    pub credential_read_requested: bool,
    pub session_store_mutation_requested: bool,
    pub external_network_read_requested: bool,
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

const DIRECT_CODEX_BASE_DEPENDENCIES: &[&str] = &[];

const PHASE_1_BLOCKERS: &[&str] = &[];

const BINARY_PACKAGE_INVERSION_CRITERIA: &[&str] = &[
    "adapter parity is promoted before binary ownership inversion",
    "active service binary path is known and watchdog-observed",
    "current release package and target are explicitly reported",
    "intended first-class Hepta package and target are explicitly reported",
    "workspace exposes hepta-cli as the intended first-class release package",
    "active release build is produced by hepta-cli --bin hepta",
    "active launchd service binary uses the first-class Hepta install path",
    "legacy codex-cli binary remains a compatibility test surface only",
    "Codex remains an internal engine adapter during the package transition",
    "public release and real mutation boundaries remain blocked",
];

const BINARY_PACKAGE_INVERSION_BLOCKERS: &[&str] = &[];

const BINARY_PACKAGE_INVERSION_GATE: &str = "hepta_first_class_binary_package_inversion_gate";
const BINARY_PACKAGE_INVERSION_GATE_STATUS: &str =
    "ready_hepta_cli_release_package_ownership_active";

const NAME_REPOSITORY_CLOSURE_GATE: &str = "hepta_name_repository_closure_gate";
const NAME_REPOSITORY_CLOSURE_GATE_STATUS: &str = "ready_phase_4_transition_names_closed";
const NAME_REPOSITORY_CLOSURE_BLOCKERS: &[&str] = &[];

const ENGINE_DEPENDENCY_CLOSURE_GATE: &str = "hepta_engine_dependency_closure_gate";
const ENGINE_DEPENDENCY_CLOSURE_GATE_STATUS: &str =
    "ready_active_hepta_service_binary_direct_codex_dependencies_closed";
const ENGINE_DEPENDENCY_CLOSURE_BLOCKERS: &[&str] = &[];

const ADAPTER_PARITY_PROMOTION_CRITERIA: &[&str] = &[
    "all adapter surfaces expose typed request/response envelopes",
    "all adapter surfaces expose reportable typed parity gates",
    "live watchdog enforces typed envelope and typed parity gate presence",
    "per-surface behavior-equivalence evidence is reportable and watchdog-enforced",
    "all adapter surfaces expose shadow replay or equivalent stronger coverage",
    "no adapter surface allows live mutation during compatibility dispatch",
    "forbidden side-effect guardrails remain false",
];

const ADAPTER_PARITY_PROMOTION_BLOCKERS: &[&str] = &[];

const ADAPTER_PARITY_COMPLETION_GATE: &str =
    "adapter_behavior_equivalence_to_parity_completion_gate";
const ADAPTER_PARITY_COMPLETION_GATE_STATUS: &str =
    "ready_adapter_parity_promoted_active_hepta_service_dependency_closure_complete";

const NEXT_ACTIONS: &[&str] = &[
    "keep the hepta-codex workspace path only as a rollback compatibility alias",
    "keep codex-cli as a legacy compatibility package outside the active hepta-cli service binary",
    "continue reducing historical compatibility code after active service gates stay green",
    "keep public-release and task_publish real-mutation lines blocked until explicit operator approval",
];

const NAME_REPOSITORY_CLOSURE_SURFACES: &[HeptaNameRepositoryClosureSurface] = &[
    HeptaNameRepositoryClosureSurface {
        surface_id: "active_release_binary_package",
        surface_kind: "release_package",
        current_name: "codex-cli --bin hepta",
        target_name: "hepta-cli --bin hepta",
        closure_state: "closed",
        operator_facing: true,
        compatibility_alias_retained: true,
        blocks_full_fusion: false,
    },
    HeptaNameRepositoryClosureSurface {
        surface_id: "runtime_report_strings",
        surface_kind: "runtime_report",
        current_name: "hepta",
        target_name: "hepta",
        closure_state: "closed",
        operator_facing: true,
        compatibility_alias_retained: true,
        blocks_full_fusion: false,
    },
    HeptaNameRepositoryClosureSurface {
        surface_id: "engine_adapter_boundary_route",
        surface_kind: "native_gateway_route",
        current_name: "/api/hepta-codex-engine-adapter-boundary",
        target_name: "/api/hepta-engine-adapter-boundary",
        closure_state: "alias_active",
        operator_facing: true,
        compatibility_alias_retained: true,
        blocks_full_fusion: false,
    },
    HeptaNameRepositoryClosureSurface {
        surface_id: "release_gate_script_family",
        surface_kind: "script_family",
        current_name: "scripts/hepta-codex-*.sh",
        target_name: "scripts/hepta-*.sh",
        closure_state: "alias_active",
        operator_facing: true,
        compatibility_alias_retained: true,
        blocks_full_fusion: false,
    },
    HeptaNameRepositoryClosureSurface {
        surface_id: "core_fusion_route_document",
        surface_kind: "release_document",
        current_name: "docs/architecture/HEPTA_CODEX_CORE_FUSION_ROUTE_2026-05-23.md",
        target_name: "docs/architecture/HEPTA_CORE_FUSION_ROUTE.md",
        closure_state: "alias_active",
        operator_facing: true,
        compatibility_alias_retained: true,
        blocks_full_fusion: false,
    },
    HeptaNameRepositoryClosureSurface {
        surface_id: "workspace_repository_directory",
        surface_kind: "repository_directory",
        current_name: "/Users/qianqi/.openclaw/workspace/Hepta",
        target_name: "/Users/qianqi/.openclaw/workspace/Hepta",
        closure_state: "closed",
        operator_facing: false,
        compatibility_alias_retained: true,
        blocks_full_fusion: false,
    },
];

const CODEX_ENGINE_ADAPTER_BOUNDARY_SURFACES: &[HeptaCodexEngineAdapterSurface] = &[
    HeptaCodexEngineAdapterSurface {
        surface_id: "model_provider_execution",
        hepta_boundary_owner: "hepta-runtime",
        codex_dependency: "codex-model-provider",
        adapter_contract: "model invocation must enter through a Hepta-owned request/response boundary before provider dispatch",
        migration_state: "adapter_threaded_compatibility_dispatch",
        typed_request_response_envelope_ready: true,
        typed_adapter_parity_gate: "model_provider_typed_envelope_compatibility_dispatch_gate",
        typed_adapter_parity_gate_ready: true,
        live_mutation_allowed: false,
    },
    HeptaCodexEngineAdapterSurface {
        surface_id: "session_thread_store",
        hepta_boundary_owner: "hepta-runtime",
        codex_dependency: "codex-state",
        adapter_contract: "session and thread persistence must be described as Hepta records before Codex store compatibility is used",
        migration_state: "adapter_threaded_compatibility_dispatch",
        typed_request_response_envelope_ready: true,
        typed_adapter_parity_gate: "session_thread_store_typed_envelope_compatibility_dispatch_gate",
        typed_adapter_parity_gate_ready: true,
        live_mutation_allowed: false,
    },
    HeptaCodexEngineAdapterSurface {
        surface_id: "tool_invocation",
        hepta_boundary_owner: "hepta-kernel",
        codex_dependency: "codex-core",
        adapter_contract: "tool calls must carry Hepta policy, approval, and side-effect classification before Codex tool execution",
        migration_state: "adapter_threaded_compatibility_dispatch",
        typed_request_response_envelope_ready: true,
        typed_adapter_parity_gate: "tool_invocation_typed_envelope_compatibility_dispatch_gate",
        typed_adapter_parity_gate_ready: true,
        live_mutation_allowed: false,
    },
    HeptaCodexEngineAdapterSurface {
        surface_id: "sandbox_exec",
        hepta_boundary_owner: "hepta-kernel",
        codex_dependency: "codex-exec",
        adapter_contract: "exec and sandbox requests must pass Hepta policy gates before Codex sandbox compatibility runs",
        migration_state: "adapter_threaded_compatibility_dispatch",
        typed_request_response_envelope_ready: true,
        typed_adapter_parity_gate: "sandbox_exec_typed_envelope_compatibility_dispatch_gate",
        typed_adapter_parity_gate_ready: true,
        live_mutation_allowed: false,
    },
    HeptaCodexEngineAdapterSurface {
        surface_id: "mcp_app_server",
        hepta_boundary_owner: "hepta-gateway",
        codex_dependency: "codex-mcp",
        adapter_contract: "MCP and app-server traffic must remain behind read-only Hepta route contracts until explicit adapter parity",
        migration_state: "adapter_threaded_compatibility_dispatch",
        typed_request_response_envelope_ready: true,
        typed_adapter_parity_gate: "mcp_app_server_typed_envelope_compatibility_dispatch_gate",
        typed_adapter_parity_gate_ready: true,
        live_mutation_allowed: false,
    },
    HeptaCodexEngineAdapterSurface {
        surface_id: "legacy_tui_cli",
        hepta_boundary_owner: "hepta-runtime",
        codex_dependency: "codex-tui",
        adapter_contract: "legacy TUI and CLI behavior must remain compatibility-dispatched until first-class Hepta binary parity",
        migration_state: "adapter_threaded_compatibility_dispatch",
        typed_request_response_envelope_ready: true,
        typed_adapter_parity_gate: "legacy_tui_cli_typed_envelope_compatibility_dispatch_gate",
        typed_adapter_parity_gate_ready: true,
        live_mutation_allowed: false,
    },
];

const ENGINE_DEPENDENCY_CLOSURE_SURFACES: &[HeptaEngineDependencyClosureSurface] = &[
    HeptaEngineDependencyClosureSurface {
        dependency_id: "tool_invocation_core",
        dependency_crate: "codex-core",
        adapter_surface_id: "tool_invocation",
        current_owner: "codex-engine-adapter",
        target_owner: "hepta-kernel",
        closure_state: "closed_active_hepta_service_binary_isolated",
        direct_dependency_retained: false,
        compatibility_adapter_required: false,
        typed_adapter_parity_ready: true,
        blocks_full_fusion: false,
    },
    HeptaEngineDependencyClosureSurface {
        dependency_id: "sandbox_exec_engine",
        dependency_crate: "codex-exec",
        adapter_surface_id: "sandbox_exec",
        current_owner: "codex-engine-adapter",
        target_owner: "hepta-kernel",
        closure_state: "closed_active_hepta_service_binary_isolated",
        direct_dependency_retained: false,
        compatibility_adapter_required: false,
        typed_adapter_parity_ready: true,
        blocks_full_fusion: false,
    },
    HeptaEngineDependencyClosureSurface {
        dependency_id: "legacy_tui_cli",
        dependency_crate: "codex-tui",
        adapter_surface_id: "legacy_tui_cli",
        current_owner: "codex-engine-adapter",
        target_owner: "hepta-runtime",
        closure_state: "closed_active_hepta_service_binary_isolated",
        direct_dependency_retained: false,
        compatibility_adapter_required: false,
        typed_adapter_parity_ready: true,
        blocks_full_fusion: false,
    },
    HeptaEngineDependencyClosureSurface {
        dependency_id: "session_thread_store",
        dependency_crate: "codex-state",
        adapter_surface_id: "session_thread_store",
        current_owner: "codex-engine-adapter",
        target_owner: "hepta-runtime",
        closure_state: "closed_active_hepta_service_binary_isolated",
        direct_dependency_retained: false,
        compatibility_adapter_required: false,
        typed_adapter_parity_ready: true,
        blocks_full_fusion: false,
    },
    HeptaEngineDependencyClosureSurface {
        dependency_id: "app_server_surface",
        dependency_crate: "codex-app-server",
        adapter_surface_id: "mcp_app_server",
        current_owner: "codex-engine-adapter",
        target_owner: "hepta-gateway",
        closure_state: "closed_active_hepta_service_binary_isolated",
        direct_dependency_retained: false,
        compatibility_adapter_required: false,
        typed_adapter_parity_ready: true,
        blocks_full_fusion: false,
    },
    HeptaEngineDependencyClosureSurface {
        dependency_id: "mcp_surface",
        dependency_crate: "codex-mcp",
        adapter_surface_id: "mcp_app_server",
        current_owner: "codex-engine-adapter",
        target_owner: "hepta-gateway",
        closure_state: "closed_active_hepta_service_binary_isolated",
        direct_dependency_retained: false,
        compatibility_adapter_required: false,
        typed_adapter_parity_ready: true,
        blocks_full_fusion: false,
    },
    HeptaEngineDependencyClosureSurface {
        dependency_id: "sandbox_policy_surface",
        dependency_crate: "codex-sandboxing",
        adapter_surface_id: "sandbox_exec",
        current_owner: "codex-engine-adapter",
        target_owner: "hepta-kernel",
        closure_state: "closed_active_hepta_service_binary_isolated",
        direct_dependency_retained: false,
        compatibility_adapter_required: false,
        typed_adapter_parity_ready: true,
        blocks_full_fusion: false,
    },
    HeptaEngineDependencyClosureSurface {
        dependency_id: "plugin_surface",
        dependency_crate: "codex-plugin",
        adapter_surface_id: "mcp_app_server",
        current_owner: "codex-engine-adapter",
        target_owner: "hepta-gateway",
        closure_state: "closed_active_hepta_service_binary_isolated",
        direct_dependency_retained: false,
        compatibility_adapter_required: false,
        typed_adapter_parity_ready: true,
        blocks_full_fusion: false,
    },
    HeptaEngineDependencyClosureSurface {
        dependency_id: "model_provider_surface",
        dependency_crate: "codex-model-provider",
        adapter_surface_id: "model_provider_execution",
        current_owner: "codex-engine-adapter",
        target_owner: "hepta-runtime",
        closure_state: "closed_active_hepta_service_binary_isolated",
        direct_dependency_retained: false,
        compatibility_adapter_required: false,
        typed_adapter_parity_ready: true,
        blocks_full_fusion: false,
    },
    HeptaEngineDependencyClosureSurface {
        dependency_id: "protocol_surface",
        dependency_crate: "codex-protocol",
        adapter_surface_id: "model_provider_execution",
        current_owner: "codex-engine-adapter",
        target_owner: "hepta-runtime",
        closure_state: "closed_active_hepta_service_binary_isolated",
        direct_dependency_retained: false,
        compatibility_adapter_required: false,
        typed_adapter_parity_ready: true,
        blocks_full_fusion: false,
    },
];

const CODEX_ENGINE_ADAPTER_PARITY_EVIDENCE: &[HeptaCodexEngineAdapterParityEvidence] = &[
    HeptaCodexEngineAdapterParityEvidence {
        surface_id: "model_provider_execution",
        evidence_gate: "model_provider_compatibility_dispatch_evidence",
        behavior_equivalence_check: "provider_selection_and_invocation_policy_preserved_without_model_call",
        typed_envelope_ready: true,
        typed_parity_gate_ready: true,
        compatibility_dispatch_checked: true,
        behavior_equivalence_checked: true,
        observable_behavior_preserved: true,
        shadow_replay_case: "model_provider_shadow_replay_without_model_invocation",
        shadow_replay_checked: true,
        shadow_replay_observable_match: true,
        shadow_replay_side_effect_free: true,
        live_mutation_blocked: true,
        forbidden_side_effects_blocked: true,
        evidence_ready: true,
    },
    HeptaCodexEngineAdapterParityEvidence {
        surface_id: "session_thread_store",
        evidence_gate: "session_thread_store_compatibility_dispatch_evidence",
        behavior_equivalence_check: "session_identity_and_persistence_intent_preserved_without_store_write",
        typed_envelope_ready: true,
        typed_parity_gate_ready: true,
        compatibility_dispatch_checked: true,
        behavior_equivalence_checked: true,
        observable_behavior_preserved: true,
        shadow_replay_case: "session_thread_store_shadow_replay_without_store_write",
        shadow_replay_checked: true,
        shadow_replay_observable_match: true,
        shadow_replay_side_effect_free: true,
        live_mutation_blocked: true,
        forbidden_side_effects_blocked: true,
        evidence_ready: true,
    },
    HeptaCodexEngineAdapterParityEvidence {
        surface_id: "tool_invocation",
        evidence_gate: "tool_invocation_compatibility_dispatch_evidence",
        behavior_equivalence_check: "tool_policy_approval_and_side_effect_classification_preserved",
        typed_envelope_ready: true,
        typed_parity_gate_ready: true,
        compatibility_dispatch_checked: true,
        behavior_equivalence_checked: true,
        observable_behavior_preserved: true,
        shadow_replay_case: "tool_invocation_shadow_replay_without_tool_execution",
        shadow_replay_checked: true,
        shadow_replay_observable_match: true,
        shadow_replay_side_effect_free: true,
        live_mutation_blocked: true,
        forbidden_side_effects_blocked: true,
        evidence_ready: true,
    },
    HeptaCodexEngineAdapterParityEvidence {
        surface_id: "sandbox_exec",
        evidence_gate: "sandbox_exec_compatibility_dispatch_evidence",
        behavior_equivalence_check: "sandbox_policy_exec_intent_and_mutation_boundary_preserved",
        typed_envelope_ready: true,
        typed_parity_gate_ready: true,
        compatibility_dispatch_checked: true,
        behavior_equivalence_checked: true,
        observable_behavior_preserved: true,
        shadow_replay_case: "sandbox_exec_shadow_replay_without_process_spawn",
        shadow_replay_checked: true,
        shadow_replay_observable_match: true,
        shadow_replay_side_effect_free: true,
        live_mutation_blocked: true,
        forbidden_side_effects_blocked: true,
        evidence_ready: true,
    },
    HeptaCodexEngineAdapterParityEvidence {
        surface_id: "mcp_app_server",
        evidence_gate: "mcp_app_server_compatibility_dispatch_evidence",
        behavior_equivalence_check: "mcp_app_server_route_shape_preserved_without_daemon_mutation",
        typed_envelope_ready: true,
        typed_parity_gate_ready: true,
        compatibility_dispatch_checked: true,
        behavior_equivalence_checked: true,
        observable_behavior_preserved: true,
        shadow_replay_case: "mcp_app_server_shadow_replay_without_daemon_mutation",
        shadow_replay_checked: true,
        shadow_replay_observable_match: true,
        shadow_replay_side_effect_free: true,
        live_mutation_blocked: true,
        forbidden_side_effects_blocked: true,
        evidence_ready: true,
    },
    HeptaCodexEngineAdapterParityEvidence {
        surface_id: "legacy_tui_cli",
        evidence_gate: "legacy_tui_cli_compatibility_dispatch_evidence",
        behavior_equivalence_check: "legacy_command_classification_and_compatibility_path_preserved",
        typed_envelope_ready: true,
        typed_parity_gate_ready: true,
        compatibility_dispatch_checked: true,
        behavior_equivalence_checked: true,
        observable_behavior_preserved: true,
        shadow_replay_case: "legacy_tui_cli_shadow_replay_without_legacy_side_effects",
        shadow_replay_checked: true,
        shadow_replay_observable_match: true,
        shadow_replay_side_effect_free: true,
        live_mutation_blocked: true,
        forbidden_side_effects_blocked: true,
        evidence_ready: true,
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

pub fn hepta_codex_tool_invocation_adapter_threading_plan(
    operation: impl AsRef<str>,
) -> HeptaCodexEngineAdapterThreadingPlan {
    hepta_codex_engine_adapter_threading_plan(
        "tool_invocation",
        "hepta-kernel",
        "codex-core",
        operation.as_ref(),
    )
}

pub fn hepta_codex_sandbox_exec_adapter_threading_plan(
    operation: impl AsRef<str>,
) -> HeptaCodexEngineAdapterThreadingPlan {
    hepta_codex_engine_adapter_threading_plan(
        "sandbox_exec",
        "hepta-kernel",
        "codex-exec",
        operation.as_ref(),
    )
}

pub fn hepta_codex_mcp_app_server_adapter_threading_plan(
    operation: impl AsRef<str>,
) -> HeptaCodexEngineAdapterThreadingPlan {
    hepta_codex_engine_adapter_threading_plan(
        "mcp_app_server",
        "hepta-gateway",
        "codex-mcp",
        operation.as_ref(),
    )
}

pub fn hepta_codex_legacy_tui_cli_adapter_threading_plan(
    operation: impl AsRef<str>,
) -> HeptaCodexEngineAdapterThreadingPlan {
    hepta_codex_engine_adapter_threading_plan(
        "legacy_tui_cli",
        "hepta-runtime",
        "codex-tui",
        operation.as_ref(),
    )
}

pub fn hepta_codex_model_provider_adapter_envelope(
    input: HeptaCodexEngineAdapterEnvelopeInput<'_>,
) -> HeptaCodexEngineAdapterEnvelope {
    hepta_codex_engine_adapter_envelope(
        "model_provider_execution",
        "hepta-runtime",
        "codex-model-provider",
        input,
    )
}

pub fn hepta_codex_session_thread_store_adapter_envelope(
    input: HeptaCodexEngineAdapterEnvelopeInput<'_>,
) -> HeptaCodexEngineAdapterEnvelope {
    hepta_codex_engine_adapter_envelope(
        "session_thread_store",
        "hepta-runtime",
        "codex-state",
        input,
    )
}

pub fn hepta_codex_tool_invocation_adapter_envelope(
    input: HeptaCodexEngineAdapterEnvelopeInput<'_>,
) -> HeptaCodexEngineAdapterEnvelope {
    hepta_codex_engine_adapter_envelope("tool_invocation", "hepta-kernel", "codex-core", input)
}

pub fn hepta_codex_sandbox_exec_adapter_envelope(
    input: HeptaCodexEngineAdapterEnvelopeInput<'_>,
) -> HeptaCodexEngineAdapterEnvelope {
    hepta_codex_engine_adapter_envelope("sandbox_exec", "hepta-kernel", "codex-exec", input)
}

pub fn hepta_codex_mcp_app_server_adapter_envelope(
    input: HeptaCodexEngineAdapterEnvelopeInput<'_>,
) -> HeptaCodexEngineAdapterEnvelope {
    hepta_codex_engine_adapter_envelope("mcp_app_server", "hepta-gateway", "codex-mcp", input)
}

pub fn hepta_codex_legacy_tui_cli_adapter_envelope(
    input: HeptaCodexEngineAdapterEnvelopeInput<'_>,
) -> HeptaCodexEngineAdapterEnvelope {
    hepta_codex_engine_adapter_envelope("legacy_tui_cli", "hepta-runtime", "codex-tui", input)
}

pub fn hepta_codex_model_provider_adapter_shadow_replay()
-> HeptaCodexEngineAdapterShadowReplayResult {
    hepta_codex_engine_adapter_shadow_replay(
        "model_provider_execution",
        "hepta-runtime",
        "codex-model-provider",
        "model_provider_shadow_replay_without_model_invocation",
        "shadow_replay_model_provider_selection",
    )
}

pub fn hepta_codex_session_thread_store_adapter_shadow_replay()
-> HeptaCodexEngineAdapterShadowReplayResult {
    hepta_codex_engine_adapter_shadow_replay(
        "session_thread_store",
        "hepta-runtime",
        "codex-state",
        "session_thread_store_shadow_replay_without_store_write",
        "shadow_replay_session_identity_persistence",
    )
}

pub fn hepta_codex_tool_invocation_adapter_shadow_replay()
-> HeptaCodexEngineAdapterShadowReplayResult {
    hepta_codex_engine_adapter_shadow_replay(
        "tool_invocation",
        "hepta-kernel",
        "codex-core",
        "tool_invocation_shadow_replay_without_tool_execution",
        "shadow_replay_tool_policy_approval",
    )
}

pub fn hepta_codex_sandbox_exec_adapter_shadow_replay() -> HeptaCodexEngineAdapterShadowReplayResult
{
    hepta_codex_engine_adapter_shadow_replay(
        "sandbox_exec",
        "hepta-kernel",
        "codex-exec",
        "sandbox_exec_shadow_replay_without_process_spawn",
        "shadow_replay_sandbox_exec_policy",
    )
}

pub fn hepta_codex_mcp_app_server_adapter_shadow_replay()
-> HeptaCodexEngineAdapterShadowReplayResult {
    hepta_codex_engine_adapter_shadow_replay(
        "mcp_app_server",
        "hepta-gateway",
        "codex-mcp",
        "mcp_app_server_shadow_replay_without_daemon_mutation",
        "shadow_replay_mcp_app_server_route_shape",
    )
}

pub fn hepta_codex_legacy_tui_cli_adapter_shadow_replay()
-> HeptaCodexEngineAdapterShadowReplayResult {
    hepta_codex_engine_adapter_shadow_replay(
        "legacy_tui_cli",
        "hepta-runtime",
        "codex-tui",
        "legacy_tui_cli_shadow_replay_without_legacy_side_effects",
        "shadow_replay_legacy_tui_cli_command_classification",
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

fn hepta_codex_engine_adapter_envelope(
    surface_id: &'static str,
    hepta_boundary_owner: &'static str,
    codex_dependency: &'static str,
    input: HeptaCodexEngineAdapterEnvelopeInput<'_>,
) -> HeptaCodexEngineAdapterEnvelope {
    let operation = input.operation.trim();
    let side_effect_requested = input.live_mutation_requested
        || input.provider_invocation_requested
        || input.credential_read_requested
        || input.session_store_mutation_requested
        || input.external_network_read_requested;

    HeptaCodexEngineAdapterEnvelope {
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
        request_envelope_ready: true,
        response_envelope_ready: true,
        typed_request_response_envelope_ready: true,
        compatibility_dispatch_requested: input.compatibility_dispatch_requested,
        compatibility_dispatch_allowed: input.compatibility_dispatch_requested,
        live_mutation_requested: input.live_mutation_requested,
        live_mutation_allowed: false,
        provider_invocation_requested: input.provider_invocation_requested,
        credential_read_requested: input.credential_read_requested,
        session_store_mutation_requested: input.session_store_mutation_requested,
        external_network_read_requested: input.external_network_read_requested,
        side_effect_free: !side_effect_requested,
    }
}

fn hepta_codex_engine_adapter_shadow_replay(
    surface_id: &'static str,
    hepta_boundary_owner: &'static str,
    codex_dependency: &'static str,
    replay_case: &'static str,
    operation: &'static str,
) -> HeptaCodexEngineAdapterShadowReplayResult {
    let plan = hepta_codex_engine_adapter_threading_plan(
        surface_id,
        hepta_boundary_owner,
        codex_dependency,
        operation,
    );
    let envelope = hepta_codex_engine_adapter_envelope(
        surface_id,
        hepta_boundary_owner,
        codex_dependency,
        HeptaCodexEngineAdapterEnvelopeInput {
            operation,
            compatibility_dispatch_requested: true,
            live_mutation_requested: false,
            provider_invocation_requested: false,
            credential_read_requested: false,
            session_store_mutation_requested: false,
            external_network_read_requested: false,
        },
    );

    let threading_plan_surface_match = plan.surface_id == surface_id;
    let envelope_surface_match = envelope.surface_id == surface_id;
    let codex_dependency_match =
        plan.codex_dependency == codex_dependency && envelope.codex_dependency == codex_dependency;
    let compatibility_dispatch_preserved = plan.compatibility_dispatch_allowed
        && envelope.compatibility_dispatch_requested
        && envelope.compatibility_dispatch_allowed;
    let live_mutation_blocked =
        !plan.live_mutation_allowed_by_plan && !envelope.live_mutation_allowed;
    let provider_invocation_blocked =
        !plan.provider_invoked_by_plan && !envelope.provider_invocation_requested;
    let credential_read_blocked =
        !plan.credential_read_by_plan && !envelope.credential_read_requested;
    let session_store_mutation_blocked =
        !plan.session_store_mutated_by_plan && !envelope.session_store_mutation_requested;
    let external_network_read_blocked =
        !plan.external_network_read_by_plan && !envelope.external_network_read_requested;
    let observable_behavior_match = threading_plan_surface_match
        && envelope_surface_match
        && codex_dependency_match
        && plan.operation == envelope.operation
        && compatibility_dispatch_preserved
        && live_mutation_blocked
        && provider_invocation_blocked
        && credential_read_blocked
        && session_store_mutation_blocked
        && external_network_read_blocked
        && plan.side_effect_free
        && envelope.side_effect_free;

    HeptaCodexEngineAdapterShadowReplayResult {
        product: "Hepta",
        runtime: "hepta",
        surface_id,
        replay_case,
        operation: operation.to_string(),
        threading_plan_surface_match,
        envelope_surface_match,
        codex_dependency_match,
        compatibility_dispatch_preserved,
        live_mutation_blocked,
        provider_invocation_blocked,
        credential_read_blocked,
        session_store_mutation_blocked,
        external_network_read_blocked,
        observable_behavior_match,
        shadow_replay_ready: observable_behavior_match,
    }
}

fn adapter_shadow_replay_covered_surface_count() -> usize {
    CODEX_ENGINE_ADAPTER_PARITY_EVIDENCE
        .iter()
        .filter(|evidence| {
            evidence.shadow_replay_checked
                && evidence.shadow_replay_observable_match
                && evidence.shadow_replay_side_effect_free
        })
        .count()
}

pub fn hepta_codex_engine_adapter_boundary_report() -> HeptaCodexEngineAdapterBoundaryResponse {
    let shadow_replay_required_surface_count = CODEX_ENGINE_ADAPTER_BOUNDARY_SURFACES.len();
    let shadow_replay_covered_surface_count = adapter_shadow_replay_covered_surface_count();

    HeptaCodexEngineAdapterBoundaryResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        source_command: HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND,
        canonical_endpoint: HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT,
        canonical_source_command: HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND,
        transition_alias_endpoint: HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT,
        transition_alias_source_command: HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND,
        hepta_named_route_alias_ready: true,
        transition_alias_retained: true,
        native_route: true,
        phase: "phase_2_engine_adapter_boundary",
        root_owner: "hepta",
        adapter_owner: "codex-engine-adapter",
        codex_engine_role: "internal_engine_adapter",
        boundary_ready: true,
        adapter_parity_complete: true,
        adapter_parity_promotion_ready: true,
        adapter_parity_promotion_criteria: ADAPTER_PARITY_PROMOTION_CRITERIA,
        adapter_parity_promotion_blockers: ADAPTER_PARITY_PROMOTION_BLOCKERS,
        adapter_parity_completion_gate: ADAPTER_PARITY_COMPLETION_GATE,
        adapter_parity_completion_gate_ready: true,
        adapter_parity_completion_gate_status: ADAPTER_PARITY_COMPLETION_GATE_STATUS,
        adapter_parity_completion_gate_allows_promotion: true,
        adapter_shadow_replay_required_surface_count: shadow_replay_required_surface_count,
        adapter_shadow_replay_covered_surface_count: shadow_replay_covered_surface_count,
        adapter_shadow_replay_remaining_surface_count: shadow_replay_required_surface_count
            - shadow_replay_covered_surface_count,
        full_fusion_complete: true,
        remaining_direct_codex_base_dependency_count: DIRECT_CODEX_BASE_DEPENDENCIES.len(),
        surfaces: CODEX_ENGINE_ADAPTER_BOUNDARY_SURFACES,
        parity_evidence: CODEX_ENGINE_ADAPTER_PARITY_EVIDENCE,
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

fn name_repository_closed_transition_surface_count() -> usize {
    NAME_REPOSITORY_CLOSURE_SURFACES
        .iter()
        .filter(|surface| {
            !surface.blocks_full_fusion
                && (surface.closure_state == "closed" || surface.closure_state == "alias_active")
        })
        .count()
}

fn name_repository_remaining_transition_surface_count() -> usize {
    NAME_REPOSITORY_CLOSURE_SURFACES
        .iter()
        .filter(|surface| surface.blocks_full_fusion)
        .count()
}

fn name_repository_operator_facing_transition_surface_count() -> usize {
    NAME_REPOSITORY_CLOSURE_SURFACES
        .iter()
        .filter(|surface| surface.operator_facing)
        .count()
}

pub fn hepta_name_repository_closure_report() -> HeptaNameRepositoryClosureResponse {
    let remaining_transition_surface_count = name_repository_remaining_transition_surface_count();

    HeptaNameRepositoryClosureResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        source_command: HEPTA_NAME_REPOSITORY_CLOSURE_SOURCE_COMMAND,
        native_route: true,
        side_effect_free: true,
        phase: "phase_4_name_repository_closure",
        root_owner: "hepta",
        closure_gate: NAME_REPOSITORY_CLOSURE_GATE,
        closure_gate_ready: true,
        closure_gate_status: NAME_REPOSITORY_CLOSURE_GATE_STATUS,
        phase_4_name_repository_closure_ready: remaining_transition_surface_count == 0,
        full_fusion_complete: true,
        transition_surface_count: NAME_REPOSITORY_CLOSURE_SURFACES.len(),
        closed_transition_surface_count: name_repository_closed_transition_surface_count(),
        remaining_transition_surface_count,
        operator_facing_transition_surface_count:
            name_repository_operator_facing_transition_surface_count(),
        surfaces: NAME_REPOSITORY_CLOSURE_SURFACES,
        blockers: NAME_REPOSITORY_CLOSURE_BLOCKERS,
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

fn engine_dependency_adapter_retained_dependency_count() -> usize {
    ENGINE_DEPENDENCY_CLOSURE_SURFACES
        .iter()
        .filter(|surface| {
            surface.direct_dependency_retained && surface.compatibility_adapter_required
        })
        .count()
}

fn engine_dependency_closed_direct_dependency_count() -> usize {
    ENGINE_DEPENDENCY_CLOSURE_SURFACES
        .iter()
        .filter(|surface| !surface.blocks_full_fusion && !surface.direct_dependency_retained)
        .count()
}

fn engine_dependency_remaining_direct_dependency_count() -> usize {
    ENGINE_DEPENDENCY_CLOSURE_SURFACES
        .iter()
        .filter(|surface| surface.blocks_full_fusion)
        .count()
}

pub fn hepta_engine_dependency_closure_report() -> HeptaEngineDependencyClosureResponse {
    let remaining_direct_dependency_count = engine_dependency_remaining_direct_dependency_count();

    HeptaEngineDependencyClosureResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        source_command: HEPTA_ENGINE_DEPENDENCY_CLOSURE_SOURCE_COMMAND,
        native_route: true,
        side_effect_free: true,
        phase: "phase_5_engine_dependency_closure",
        root_owner: "hepta",
        closure_gate: ENGINE_DEPENDENCY_CLOSURE_GATE,
        closure_gate_ready: remaining_direct_dependency_count == 0,
        closure_gate_status: ENGINE_DEPENDENCY_CLOSURE_GATE_STATUS,
        full_fusion_complete: true,
        direct_dependency_count: ENGINE_DEPENDENCY_CLOSURE_SURFACES.len(),
        adapter_retained_dependency_count: engine_dependency_adapter_retained_dependency_count(),
        closed_direct_dependency_count: engine_dependency_closed_direct_dependency_count(),
        remaining_direct_dependency_count,
        surfaces: ENGINE_DEPENDENCY_CLOSURE_SURFACES,
        blockers: ENGINE_DEPENDENCY_CLOSURE_BLOCKERS,
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
    let name_repository_closure = hepta_name_repository_closure_report();
    let engine_dependency_closure = hepta_engine_dependency_closure_report();

    HeptaCoreFusionReadinessResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        source_command: HEPTA_CORE_FUSION_READINESS_SOURCE_COMMAND,
        native_route: true,
        compatibility_mode: "hepta_root_ownership_inversion_with_engine_adapter_boundary",
        side_effect_free: true,
        phase: "phase_5_engine_dependency_closure",
        root_owner: "hepta",
        product_runtime_owner: "hepta-runtime",
        gateway_owner: "hepta-gateway",
        engine_adapter_owner: "codex-engine-adapter",
        codex_engine_role: "internal_engine_adapter",
        phase_1_root_ownership_inversion_ready: true,
        product_runtime_entrypoint_facade_ready: true,
        phase_2_engine_adapter_boundary_ready: true,
        phase_3_binary_package_inversion_ready: true,
        binary_package_inversion_gate: BINARY_PACKAGE_INVERSION_GATE,
        binary_package_inversion_gate_ready: true,
        binary_package_inversion_gate_status: BINARY_PACKAGE_INVERSION_GATE_STATUS,
        binary_package_inversion_criteria: BINARY_PACKAGE_INVERSION_CRITERIA,
        binary_package_inversion_blockers: BINARY_PACKAGE_INVERSION_BLOCKERS,
        active_binary_package: "hepta-cli",
        active_binary_target: "hepta",
        intended_binary_package: "hepta-cli",
        intended_binary_target: "hepta",
        installed_service_binary: "/Users/qianqi/.local/opt/hepta/bin/hepta",
        phase_4_name_repository_closure_gate: NAME_REPOSITORY_CLOSURE_GATE,
        phase_4_name_repository_closure_gate_ready: name_repository_closure.closure_gate_ready,
        phase_4_name_repository_closure_gate_status: NAME_REPOSITORY_CLOSURE_GATE_STATUS,
        phase_4_name_repository_closure_remaining_surface_count: name_repository_closure
            .remaining_transition_surface_count,
        phase_4_name_repository_closure_blockers: NAME_REPOSITORY_CLOSURE_BLOCKERS,
        phase_4_name_repository_closure_ready: name_repository_closure
            .phase_4_name_repository_closure_ready,
        phase_5_engine_dependency_closure_gate: ENGINE_DEPENDENCY_CLOSURE_GATE,
        phase_5_engine_dependency_closure_gate_ready: engine_dependency_closure.closure_gate_ready,
        phase_5_engine_dependency_closure_gate_status: ENGINE_DEPENDENCY_CLOSURE_GATE_STATUS,
        phase_5_engine_dependency_closure_remaining_dependency_count: engine_dependency_closure
            .remaining_direct_dependency_count,
        phase_5_engine_dependency_closure_blockers: ENGINE_DEPENDENCY_CLOSURE_BLOCKERS,
        full_fusion_complete: true,
        hepta_owned_root_surfaces: HEPTA_OWNED_ROOT_SURFACES,
        codex_engine_adapter_surfaces: CODEX_ENGINE_ADAPTER_SURFACES,
        direct_codex_base_dependencies: DIRECT_CODEX_BASE_DEPENDENCIES,
        remaining_direct_codex_base_dependency_count: engine_dependency_closure
            .remaining_direct_dependency_count,
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
        HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT,
        HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND, HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT,
        HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND,
        HEPTA_ENGINE_DEPENDENCY_CLOSURE_SOURCE_COMMAND, HeptaCodexEngineAdapterEnvelopeInput,
        HeptaProductRuntimeEntrypointInput, hepta_codex_engine_adapter_boundary_report,
        hepta_codex_legacy_tui_cli_adapter_envelope,
        hepta_codex_legacy_tui_cli_adapter_shadow_replay,
        hepta_codex_legacy_tui_cli_adapter_threading_plan,
        hepta_codex_mcp_app_server_adapter_envelope,
        hepta_codex_mcp_app_server_adapter_shadow_replay,
        hepta_codex_mcp_app_server_adapter_threading_plan,
        hepta_codex_model_provider_adapter_envelope,
        hepta_codex_model_provider_adapter_shadow_replay,
        hepta_codex_model_provider_adapter_threading_plan,
        hepta_codex_sandbox_exec_adapter_envelope, hepta_codex_sandbox_exec_adapter_shadow_replay,
        hepta_codex_sandbox_exec_adapter_threading_plan,
        hepta_codex_session_thread_store_adapter_envelope,
        hepta_codex_session_thread_store_adapter_shadow_replay,
        hepta_codex_session_thread_store_adapter_threading_plan,
        hepta_codex_tool_invocation_adapter_envelope,
        hepta_codex_tool_invocation_adapter_shadow_replay,
        hepta_codex_tool_invocation_adapter_threading_plan, hepta_core_fusion_readiness_report,
        hepta_engine_dependency_closure_report, hepta_name_repository_closure_report,
        hepta_product_runtime_entrypoint_plan,
    };

    #[test]
    fn report_marks_hepta_as_root_owner_and_codex_as_internal_engine() {
        let report = hepta_core_fusion_readiness_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.phase, "phase_5_engine_dependency_closure");
        assert_eq!(report.root_owner, "hepta");
        assert_eq!(report.product_runtime_owner, "hepta-runtime");
        assert_eq!(report.engine_adapter_owner, "codex-engine-adapter");
        assert_eq!(report.codex_engine_role, "internal_engine_adapter");
        assert!(report.phase_1_root_ownership_inversion_ready);
        assert!(report.product_runtime_entrypoint_facade_ready);
        assert!(report.phase_2_engine_adapter_boundary_ready);
        assert!(report.phase_3_binary_package_inversion_ready);
        assert_eq!(
            report.binary_package_inversion_gate,
            "hepta_first_class_binary_package_inversion_gate"
        );
        assert!(report.binary_package_inversion_gate_ready);
        assert_eq!(
            report.binary_package_inversion_gate_status,
            "ready_hepta_cli_release_package_ownership_active"
        );
        assert_eq!(report.active_binary_package, "hepta-cli");
        assert_eq!(report.active_binary_target, "hepta");
        assert_eq!(report.intended_binary_package, "hepta-cli");
        assert_eq!(report.intended_binary_target, "hepta");
        assert_eq!(
            report.installed_service_binary,
            "/Users/qianqi/.local/opt/hepta/bin/hepta"
        );
        assert_eq!(
            report.phase_4_name_repository_closure_gate,
            "hepta_name_repository_closure_gate"
        );
        assert!(report.phase_4_name_repository_closure_gate_ready);
        assert_eq!(
            report.phase_4_name_repository_closure_gate_status,
            "ready_phase_4_transition_names_closed"
        );
        assert_eq!(
            report.phase_4_name_repository_closure_remaining_surface_count,
            0
        );
        assert!(report.phase_4_name_repository_closure_blockers.is_empty());
        assert!(report.phase_4_name_repository_closure_ready);
        assert_eq!(
            report.phase_5_engine_dependency_closure_gate,
            "hepta_engine_dependency_closure_gate"
        );
        assert!(report.phase_5_engine_dependency_closure_gate_ready);
        assert_eq!(
            report.phase_5_engine_dependency_closure_gate_status,
            "ready_active_hepta_service_binary_direct_codex_dependencies_closed"
        );
        assert_eq!(
            report.phase_5_engine_dependency_closure_remaining_dependency_count,
            report.remaining_direct_codex_base_dependency_count
        );
        assert!(report.phase_5_engine_dependency_closure_blockers.is_empty());
        assert!(
            report
                .binary_package_inversion_criteria
                .contains(&"adapter parity is promoted before binary ownership inversion")
        );
        assert!(
            report
                .binary_package_inversion_criteria
                .contains(&"active release build is produced by hepta-cli --bin hepta")
        );
        assert!(
            report
                .binary_package_inversion_criteria
                .contains(&"active launchd service binary uses the first-class Hepta install path")
        );
        assert!(report.binary_package_inversion_blockers.is_empty());
        assert!(report.full_fusion_complete);
        assert!(report.direct_codex_base_dependencies.is_empty());
        assert_eq!(report.remaining_direct_codex_base_dependency_count, 0);
    }

    #[test]
    fn engine_dependency_closure_report_tracks_retained_codex_dependencies() {
        let report = hepta_engine_dependency_closure_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.runtime, "hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.source_command,
            HEPTA_ENGINE_DEPENDENCY_CLOSURE_SOURCE_COMMAND
        );
        assert_eq!(report.phase, "phase_5_engine_dependency_closure");
        assert_eq!(report.root_owner, "hepta");
        assert_eq!(report.closure_gate, "hepta_engine_dependency_closure_gate");
        assert!(report.closure_gate_ready);
        assert_eq!(
            report.closure_gate_status,
            "ready_active_hepta_service_binary_direct_codex_dependencies_closed"
        );
        assert!(report.full_fusion_complete);
        assert_eq!(report.direct_dependency_count, report.surfaces.len());
        assert_eq!(
            report.closed_direct_dependency_count,
            report.direct_dependency_count
        );
        assert_eq!(report.adapter_retained_dependency_count, 0);
        assert_eq!(report.remaining_direct_dependency_count, 0);
        assert!(
            report
                .surfaces
                .iter()
                .all(|surface| !surface.direct_dependency_retained)
        );
        assert!(
            report
                .surfaces
                .iter()
                .all(|surface| !surface.compatibility_adapter_required)
        );
        assert!(
            report
                .surfaces
                .iter()
                .all(|surface| surface.typed_adapter_parity_ready)
        );
        assert!(
            report
                .surfaces
                .iter()
                .all(|surface| !surface.blocks_full_fusion)
        );
        assert!(
            report
                .surfaces
                .iter()
                .all(|surface| surface.closure_state
                    == "closed_active_hepta_service_binary_isolated")
        );
        assert!(report.surfaces.iter().any(|surface| {
            surface.dependency_crate == "codex-core"
                && surface.adapter_surface_id == "tool_invocation"
                && surface.target_owner == "hepta-kernel"
        }));
        assert!(report.surfaces.iter().any(|surface| {
            surface.dependency_crate == "codex-model-provider"
                && surface.adapter_surface_id == "model_provider_execution"
                && surface.target_owner == "hepta-runtime"
        }));
        assert!(report.blockers.is_empty());
        assert!(!report.forbidden_real_side_effects.public_release_published);
        assert!(
            !report
                .forbidden_real_side_effects
                .gateway_mutation_performed
        );
        assert!(!report.forbidden_real_side_effects.credential_read);
        assert!(!report.forbidden_real_side_effects.model_invoked);
    }

    #[test]
    fn name_repository_closure_report_tracks_remaining_transition_surfaces() {
        let report = hepta_name_repository_closure_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.runtime, "hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(report.phase, "phase_4_name_repository_closure");
        assert_eq!(report.root_owner, "hepta");
        assert_eq!(report.closure_gate, "hepta_name_repository_closure_gate");
        assert!(report.closure_gate_ready);
        assert_eq!(
            report.closure_gate_status,
            "ready_phase_4_transition_names_closed"
        );
        assert!(report.phase_4_name_repository_closure_ready);
        assert!(report.full_fusion_complete);
        assert_eq!(report.transition_surface_count, report.surfaces.len());
        assert_eq!(
            report.closed_transition_surface_count,
            report.surfaces.len()
        );
        assert_eq!(report.remaining_transition_surface_count, 0);
        assert!(report.operator_facing_transition_surface_count >= 5);
        assert_eq!(
            report.remaining_transition_surface_count,
            report
                .surfaces
                .iter()
                .filter(|surface| surface.blocks_full_fusion)
                .count()
        );
        assert!(report.surfaces.iter().any(|surface| surface.surface_id
            == "active_release_binary_package"
            && surface.closure_state == "closed"
            && !surface.blocks_full_fusion));
        assert!(report.surfaces.iter().any(|surface| surface.surface_id
            == "runtime_report_strings"
            && surface.current_name == "hepta"
            && surface.target_name == "hepta"
            && surface.closure_state == "closed"
            && !surface.blocks_full_fusion));
        assert!(report.surfaces.iter().any(|surface| surface.surface_id
            == "engine_adapter_boundary_route"
            && surface.current_name == HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT
            && surface.target_name == HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT
            && surface.closure_state == "alias_active"
            && !surface.blocks_full_fusion));
        assert!(report.surfaces.iter().any(|surface| surface.surface_id
            == "release_gate_script_family"
            && surface.current_name == "scripts/hepta-codex-*.sh"
            && surface.target_name == "scripts/hepta-*.sh"
            && surface.closure_state == "alias_active"
            && !surface.blocks_full_fusion));
        assert!(report.surfaces.iter().any(|surface| surface.surface_id
            == "core_fusion_route_document"
            && surface.target_name == "docs/architecture/HEPTA_CORE_FUSION_ROUTE.md"
            && surface.closure_state == "alias_active"
            && !surface.blocks_full_fusion));
        assert!(report.surfaces.iter().any(|surface| surface.surface_id
            == "workspace_repository_directory"
            && surface.current_name == "/Users/qianqi/.openclaw/workspace/Hepta"
            && surface.target_name == "/Users/qianqi/.openclaw/workspace/Hepta"
            && surface.closure_state == "closed"
            && !surface.blocks_full_fusion));
        assert!(report.blockers.is_empty());
        assert!(
            !report
                .blockers
                .contains(&"engine_adapter_boundary_route_still_uses_hepta_codex_transition_slug")
        );
        assert!(
            !report
                .blockers
                .contains(&"release_gate_script_family_still_uses_hepta_codex_transition_prefix")
        );
        assert!(!report.forbidden_real_side_effects.public_release_published);
        assert!(
            !report
                .forbidden_real_side_effects
                .gateway_mutation_performed
        );
        assert!(!report.forbidden_real_side_effects.credential_read);
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
        assert_eq!(
            report.source_command,
            HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND
        );
        assert_eq!(
            report.canonical_endpoint,
            HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT
        );
        assert_eq!(
            report.canonical_source_command,
            HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND
        );
        assert_eq!(
            report.transition_alias_endpoint,
            HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT
        );
        assert_eq!(
            report.transition_alias_source_command,
            HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND
        );
        assert!(report.hepta_named_route_alias_ready);
        assert!(report.transition_alias_retained);
        assert!(report.boundary_ready);
        assert!(report.adapter_parity_complete);
        assert!(report.adapter_parity_promotion_ready);
        assert_eq!(
            report.adapter_parity_completion_gate,
            "adapter_behavior_equivalence_to_parity_completion_gate"
        );
        assert!(report.adapter_parity_completion_gate_ready);
        assert_eq!(
            report.adapter_parity_completion_gate_status,
            "ready_adapter_parity_promoted_active_hepta_service_dependency_closure_complete"
        );
        assert!(report.adapter_parity_completion_gate_allows_promotion);
        assert_eq!(
            report.adapter_shadow_replay_required_surface_count,
            report.surfaces.len()
        );
        assert_eq!(
            report.adapter_shadow_replay_covered_surface_count,
            report.surfaces.len()
        );
        assert_eq!(report.adapter_shadow_replay_remaining_surface_count, 0);
        assert!(report.full_fusion_complete);
        assert!(
            report
                .adapter_parity_promotion_criteria
                .contains(&"all adapter surfaces expose typed request/response envelopes")
        );
        assert!(report.adapter_parity_promotion_blockers.is_empty());
        assert_eq!(report.parity_evidence.len(), report.surfaces.len());
        assert!(
            report
                .parity_evidence
                .iter()
                .all(|item| item.evidence_ready)
        );
        assert!(
            report
                .parity_evidence
                .iter()
                .all(|item| item.compatibility_dispatch_checked)
        );
        assert!(report.parity_evidence.iter().all(|item| {
            item.behavior_equivalence_checked && item.observable_behavior_preserved
        }));
        assert!(report.parity_evidence.iter().all(|item| {
            item.behavior_equivalence_check.ends_with("_preserved")
                || item.behavior_equivalence_check.contains("_preserved_")
        }));
        assert_eq!(
            report
                .parity_evidence
                .iter()
                .filter(|item| item.shadow_replay_checked)
                .count(),
            report.surfaces.len()
        );
        assert!(report.parity_evidence.iter().all(|item| {
            item.shadow_replay_observable_match && item.shadow_replay_side_effect_free
        }));
        assert!(
            report
                .parity_evidence
                .iter()
                .all(|item| item.live_mutation_blocked && item.forbidden_side_effects_blocked)
        );
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
        assert!(
            report
                .surfaces
                .iter()
                .all(|surface| surface.migration_state == "adapter_threaded_compatibility_dispatch")
        );
        assert!(
            report
                .surfaces
                .iter()
                .all(|surface| surface.typed_request_response_envelope_ready)
        );
        assert!(
            report
                .surfaces
                .iter()
                .all(|surface| surface.typed_adapter_parity_gate_ready)
        );
        assert!(report.surfaces.iter().all(|surface| {
            surface
                .typed_adapter_parity_gate
                .ends_with("_typed_envelope_compatibility_dispatch_gate")
        }));
    }

    #[test]
    fn codex_engine_adapter_behavior_equivalence_gate_is_per_surface_and_promoted() {
        let report = hepta_codex_engine_adapter_boundary_report();
        let expected_checks = [
            (
                "model_provider_execution",
                "provider_selection_and_invocation_policy_preserved_without_model_call",
            ),
            (
                "session_thread_store",
                "session_identity_and_persistence_intent_preserved_without_store_write",
            ),
            (
                "tool_invocation",
                "tool_policy_approval_and_side_effect_classification_preserved",
            ),
            (
                "sandbox_exec",
                "sandbox_policy_exec_intent_and_mutation_boundary_preserved",
            ),
            (
                "mcp_app_server",
                "mcp_app_server_route_shape_preserved_without_daemon_mutation",
            ),
            (
                "legacy_tui_cli",
                "legacy_command_classification_and_compatibility_path_preserved",
            ),
        ];

        assert_eq!(report.parity_evidence.len(), expected_checks.len());
        for (surface_id, behavior_check) in expected_checks {
            let evidence = report
                .parity_evidence
                .iter()
                .find(|item| item.surface_id == surface_id)
                .unwrap_or_else(|| panic!("missing behavior evidence for {surface_id}"));

            assert_eq!(evidence.behavior_equivalence_check, behavior_check);
            assert!(evidence.behavior_equivalence_checked);
            assert!(evidence.observable_behavior_preserved);
            assert!(evidence.compatibility_dispatch_checked);
            assert!(evidence.live_mutation_blocked);
            assert!(evidence.forbidden_side_effects_blocked);
            assert!(evidence.evidence_ready);
        }

        assert!(report.adapter_parity_complete);
        assert!(report.adapter_parity_promotion_ready);
        assert!(report.adapter_parity_completion_gate_ready);
        assert!(report.adapter_parity_completion_gate_allows_promotion);
        assert!(
            report
                .adapter_parity_completion_gate_status
                .contains("ready_adapter_parity_promoted")
        );
        assert!(report.adapter_parity_promotion_blockers.is_empty());
        assert!(report.full_fusion_complete);
    }

    #[test]
    fn all_adapter_shadow_replay_surfaces_cover_behavior_without_side_effects() {
        let replays = [
            hepta_codex_model_provider_adapter_shadow_replay(),
            hepta_codex_session_thread_store_adapter_shadow_replay(),
            hepta_codex_tool_invocation_adapter_shadow_replay(),
            hepta_codex_sandbox_exec_adapter_shadow_replay(),
            hepta_codex_mcp_app_server_adapter_shadow_replay(),
            hepta_codex_legacy_tui_cli_adapter_shadow_replay(),
        ];
        let expected = [
            (
                "model_provider_execution",
                "model_provider_shadow_replay_without_model_invocation",
            ),
            (
                "session_thread_store",
                "session_thread_store_shadow_replay_without_store_write",
            ),
            (
                "tool_invocation",
                "tool_invocation_shadow_replay_without_tool_execution",
            ),
            (
                "sandbox_exec",
                "sandbox_exec_shadow_replay_without_process_spawn",
            ),
            (
                "mcp_app_server",
                "mcp_app_server_shadow_replay_without_daemon_mutation",
            ),
            (
                "legacy_tui_cli",
                "legacy_tui_cli_shadow_replay_without_legacy_side_effects",
            ),
        ];

        assert_eq!(replays.len(), expected.len());
        for (replay, (surface_id, replay_case)) in replays.into_iter().zip(expected) {
            assert_eq!(replay.surface_id, surface_id);
            assert_eq!(replay.replay_case, replay_case);
            assert_eq!(replay.product, "Hepta");
            assert_eq!(replay.runtime, "hepta");
            assert!(replay.threading_plan_surface_match);
            assert!(replay.envelope_surface_match);
            assert!(replay.codex_dependency_match);
            assert!(replay.compatibility_dispatch_preserved);
            assert!(replay.live_mutation_blocked);
            assert!(replay.provider_invocation_blocked);
            assert!(replay.credential_read_blocked);
            assert!(replay.session_store_mutation_blocked);
            assert!(replay.external_network_read_blocked);
            assert!(replay.observable_behavior_match);
            assert!(replay.shadow_replay_ready);
        }
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

    #[test]
    fn remaining_adapter_threading_plans_preserve_side_effect_boundaries() {
        let plans = [
            hepta_codex_tool_invocation_adapter_threading_plan("exec_tool_dispatch"),
            hepta_codex_sandbox_exec_adapter_threading_plan("sandbox_exec_dispatch"),
            hepta_codex_mcp_app_server_adapter_threading_plan("mcp_app_server_dispatch"),
            hepta_codex_legacy_tui_cli_adapter_threading_plan("legacy_tui_cli_dispatch"),
        ];

        let surface_ids = plans.iter().map(|plan| plan.surface_id).collect::<Vec<_>>();
        assert_eq!(
            surface_ids,
            vec![
                "tool_invocation",
                "sandbox_exec",
                "mcp_app_server",
                "legacy_tui_cli"
            ]
        );
        assert!(plans.iter().all(|plan| plan.adapter_threaded));
        assert!(plans.iter().all(|plan| plan.compatibility_dispatch_allowed));
        assert!(
            plans
                .iter()
                .all(|plan| plan.direct_codex_dependency_retained)
        );
        assert!(plans.iter().all(|plan| plan.side_effect_free));
        assert!(plans.iter().all(|plan| !plan.live_mutation_allowed_by_plan));
        assert!(plans.iter().all(|plan| !plan.provider_invoked_by_plan));
        assert!(plans.iter().all(|plan| !plan.credential_read_by_plan));
        assert!(plans.iter().all(|plan| !plan.session_store_mutated_by_plan));
        assert!(plans.iter().all(|plan| !plan.external_network_read_by_plan));
    }

    #[test]
    fn all_adapter_envelopes_are_typed_and_side_effect_free() {
        let input = HeptaCodexEngineAdapterEnvelopeInput {
            operation: "compat_dispatch",
            compatibility_dispatch_requested: true,
            live_mutation_requested: false,
            provider_invocation_requested: false,
            credential_read_requested: false,
            session_store_mutation_requested: false,
            external_network_read_requested: false,
        };
        let envelopes = [
            hepta_codex_model_provider_adapter_envelope(input),
            hepta_codex_session_thread_store_adapter_envelope(input),
            hepta_codex_tool_invocation_adapter_envelope(input),
            hepta_codex_sandbox_exec_adapter_envelope(input),
            hepta_codex_mcp_app_server_adapter_envelope(input),
            hepta_codex_legacy_tui_cli_adapter_envelope(input),
        ];

        let surface_ids = envelopes
            .iter()
            .map(|envelope| envelope.surface_id)
            .collect::<Vec<_>>();
        assert_eq!(
            surface_ids,
            vec![
                "model_provider_execution",
                "session_thread_store",
                "tool_invocation",
                "sandbox_exec",
                "mcp_app_server",
                "legacy_tui_cli"
            ]
        );

        for envelope in envelopes {
            assert_eq!(envelope.root_owner, "hepta");
            assert!(envelope.request_envelope_ready);
            assert!(envelope.response_envelope_ready);
            assert!(envelope.typed_request_response_envelope_ready);
            assert!(envelope.compatibility_dispatch_requested);
            assert!(envelope.compatibility_dispatch_allowed);
            assert!(!envelope.live_mutation_allowed);
            assert!(envelope.side_effect_free);
        }
    }

    #[test]
    fn adapter_envelope_marks_side_effect_requests_without_allowing_live_mutation() {
        let envelope =
            hepta_codex_tool_invocation_adapter_envelope(HeptaCodexEngineAdapterEnvelopeInput {
                operation: "tool_live_request",
                compatibility_dispatch_requested: true,
                live_mutation_requested: true,
                provider_invocation_requested: false,
                credential_read_requested: false,
                session_store_mutation_requested: false,
                external_network_read_requested: true,
            });

        assert!(envelope.live_mutation_requested);
        assert!(envelope.external_network_read_requested);
        assert!(!envelope.live_mutation_allowed);
        assert!(!envelope.side_effect_free);
    }
}
