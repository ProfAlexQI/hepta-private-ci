use serde::Serialize;

pub const HEPTA_CORE_FUSION_READINESS_ENDPOINT: &str = "/api/hepta-core-fusion-readiness";
pub const HEPTA_CORE_FUSION_READINESS_SOURCE_COMMAND: &str = "/hepta-core-fusion-readiness --json";

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
    "route the hepta binary entrypoint through a Hepta-owned product-runtime facade before Codex compatibility dispatch",
    "introduce explicit CodexEngineAdapter boundaries for model/session/tool/sandbox/thread-store surfaces",
    "promote the installed binary from codex-cli --bin hepta toward first-class Hepta binary ownership after adapter parity",
    "keep public-release and task_publish real-mutation lines blocked until explicit operator approval",
];

pub fn hepta_core_fusion_readiness_report() -> HeptaCoreFusionReadinessResponse {
    HeptaCoreFusionReadinessResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: "ready",
        source_command: HEPTA_CORE_FUSION_READINESS_SOURCE_COMMAND,
        native_route: true,
        compatibility_mode: "hepta_root_ownership_inversion_phase1",
        side_effect_free: true,
        phase: "phase_1_root_ownership_inversion",
        root_owner: "hepta",
        product_runtime_owner: "hepta-runtime",
        gateway_owner: "hepta-gateway",
        engine_adapter_owner: "codex-engine-adapter",
        codex_engine_role: "internal_engine_adapter",
        phase_1_root_ownership_inversion_ready: true,
        phase_2_engine_adapter_boundary_ready: false,
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

#[cfg(test)]
mod tests {
    use super::hepta_core_fusion_readiness_report;

    #[test]
    fn report_marks_hepta_as_root_owner_and_codex_as_internal_engine() {
        let report = hepta_core_fusion_readiness_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.root_owner, "hepta");
        assert_eq!(report.product_runtime_owner, "hepta-runtime");
        assert_eq!(report.engine_adapter_owner, "codex-engine-adapter");
        assert_eq!(report.codex_engine_role, "internal_engine_adapter");
        assert!(report.phase_1_root_ownership_inversion_ready);
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
}
