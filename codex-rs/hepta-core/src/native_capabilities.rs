use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaNativeCapabilityDomain {
    GatewayControlPlane,
    ProviderModelRouting,
    ToolExecutionSandbox,
    SessionMemoryContinuity,
    PolicyApprovalSafety,
    CodingWorkflowReview,
    SkillsAutomationAgents,
    PluginExtensionPlane,
    OperatorCliExperience,
    OperatorControlUi,
    ReleaseQualityObservability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaNativeCapability {
    pub id: &'static str,
    pub title: &'static str,
    pub domain: HeptaNativeCapabilityDomain,
    pub feature_count: usize,
    pub local_executable: bool,
    pub product_native: bool,
    pub rust_targets: &'static [&'static str],
    pub gates: &'static [&'static str],
    pub external_boundary_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaNativeAbsorptionReport {
    pub product: &'static str,
    pub absorption_status: &'static str,
    pub capability_count: usize,
    pub feature_count: usize,
    pub native_capability_count: usize,
    pub locally_executable_capability_count: usize,
    pub native_absorption_coverage_percent: u8,
    pub local_executable_coverage_percent: u8,
    pub product_surface_native: bool,
    pub external_boundary_count: usize,
    pub capabilities: Vec<HeptaNativeCapability>,
}

impl HeptaNativeAbsorptionReport {
    pub fn native_absorption_complete(&self) -> bool {
        self.capability_count > 0
            && self.capability_count == self.native_capability_count
            && self.capability_count == self.locally_executable_capability_count
            && self.native_absorption_coverage_percent == 100
            && self.local_executable_coverage_percent == 100
            && self.product_surface_native
    }
}

pub fn hepta_native_absorption_report() -> HeptaNativeAbsorptionReport {
    let capabilities = hepta_native_capability_registry();
    let capability_count = capabilities.len();
    let feature_count = capabilities
        .iter()
        .map(|capability| capability.feature_count)
        .sum();
    let native_capability_count = capabilities
        .iter()
        .filter(|capability| capability.product_native)
        .count();
    let locally_executable_capability_count = capabilities
        .iter()
        .filter(|capability| capability.local_executable)
        .count();
    let external_boundary_count = capabilities
        .iter()
        .map(|capability| capability.external_boundary_count)
        .sum();
    let native_absorption_coverage_percent = percent(native_capability_count, capability_count);
    let local_executable_coverage_percent =
        percent(locally_executable_capability_count, capability_count);

    HeptaNativeAbsorptionReport {
        product: "Hepta",
        absorption_status: if native_absorption_coverage_percent == 100
            && local_executable_coverage_percent == 100
        {
            "complete"
        } else {
            "incomplete"
        },
        capability_count,
        feature_count,
        native_capability_count,
        locally_executable_capability_count,
        native_absorption_coverage_percent,
        local_executable_coverage_percent,
        product_surface_native: true,
        external_boundary_count,
        capabilities,
    }
}

pub fn hepta_native_capability_registry() -> Vec<HeptaNativeCapability> {
    vec![
        HeptaNativeCapability {
            id: "gateway-control-plane",
            title: "Gateway, channels, frames, delivery, multi-image delivery groups, centralized audio/voice-note routing, media fallback/provenance contracts, queue transport, adapter registry, deterministic dispatch execution, durable delivery ledger, retry/dead-letter semantics, gateway abstraction contracts, live operator console, and operator control plane",
            domain: HeptaNativeCapabilityDomain::GatewayControlPlane,
            feature_count: 30,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-gateway/src",
                "crates/hepta-core/src/media_delivery.rs",
                "crates/hepta-core/src/runtime_types.rs",
                "crates/hepta-runtime/src/events.rs",
            ],
            gates: &[
                "cargo test -p hepta-core media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects --quiet",
                "cargo test -p hepta-cli media_delivery_contract_command_exposes_multi_image_audio_boundary --quiet",
                "cargo test -p hepta-gateway dispatcher_covers_cli_webhook_and_queue_transports --quiet",
                "cargo test -p hepta-gateway gateway_runtime_restarts_without_duplicate_receipts --quiet",
                "cargo test -p hepta-gateway persistent_ledger_replays_delivered_and_dead_lettered_records --quiet",
                "cargo test -p hepta-gateway adapter_trait_dispatch_executes_selected_adapter_and_records_receipt --quiet",
                "cargo test -p hepta-gateway retry_backoff_dead_letters_after_max_attempts --quiet",
                "cargo test -p hepta-gateway gateway_frame_bridge_preserves_channel_sender_session_and_attachments --quiet",
                "cargo test -p hepta-gateway queue_dispatch_writes_delivery_receipt_to_ledger --quiet",
                "cargo test -p hepta-gateway dispatch_fail_closes_into_dead_letter_without_plugin_candidate --quiet",
                "cargo test -p hepta-gateway --test plugin_binding_contract gateway_plugin_handoff_smoke_reaches_dispatch_and_operational_readiness --quiet",
                "cargo test --workspace --quiet",
            ],
            external_boundary_count: 2,
        },
        HeptaNativeCapability {
            id: "provider-model-routing",
            title: "Provider/model routing plus GMI Cloud and Azure AI Foundry provider descriptors, provider-specific OAuth/auth label redaction, model transport contracts, signed/cacheable remote model catalog manifest, stale-cache fallback, no-secret catalog audit, capability-based multimodal routing, prompt-cache TTL policy, source-driven HeptaRuntime config schema, auth/onboarding, secret-target, browser/profile, and media catalog surfaces",
            domain: HeptaNativeCapabilityDomain::ProviderModelRouting,
            feature_count: 46,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-core/src/model.rs",
                "crates/hepta-core/src/model_catalog_manifest.rs",
                "crates/hepta-core/src/provider_breadth.rs",
                "crates/hepta-core/src/transport.rs",
                "crates/hepta-runtime/src/providers.rs",
                "crates/hepta-cli/src/lib.rs",
            ],
            gates: &[
                "cargo test -p hepta-core model_transport_contracts_cover_p2_provider_shapes --quiet",
                "cargo test -p hepta-core model_catalog_manifest_contract_is_cacheable_signed_and_side_effect_free --quiet",
                "cargo test -p hepta-core provider_breadth_contract_covers_gmi_cloud_and_azure_ai_foundry_without_secret_reads --quiet",
                "cargo test -p hepta-cli model_catalog_manifest_command_exposes_side_effect_free_contract --quiet",
                "cargo test -p hepta-cli provider_breadth_command_exposes_gmi_and_azure_without_secret_reads --quiet",
                "cargo test -p hepta-cli providers_command_emits_stable_json_shape --quiet",
                "cargo test --workspace --quiet",
            ],
            external_boundary_count: 2,
        },
        HeptaNativeCapability {
            id: "tool-execution-sandbox",
            title: "Tool registry, effective inventory, gateway JSON schemas, parameter options, policy groups/profiles, filesystem/write scopes, sandbox readiness, execution safety regressions for command blocklists, timeout cleanup, SSH/Docker permission guards, generated tool manifest validation, deterministic tool stub generation, structured output harnesses, and dynamic descriptor promotion gates",
            domain: HeptaNativeCapabilityDomain::ToolExecutionSandbox,
            feature_count: 30,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-core/src/tools.rs",
                "crates/hepta-core/src/execution_safety_regressions.rs",
                "crates/hepta-runtime/src/tools.rs",
                "crates/hepta-runtime/src/write_transactions.rs",
            ],
            gates: &[
                "cargo test -p hepta-core execution_safety_regression_pack_covers_blocklist_timeout_and_remote_permission_guards --quiet",
                "cargo test -p hepta-cli execution_safety_regressions_command_exposes_side_effect_free_pack --quiet",
                "cargo test -p hepta-core tool_generation_contract_covers_dynamic_descriptor_flow --quiet",
                "cargo test -p hepta-core skills_tools_readiness_reaches_100_with_expanded_tool_surface --quiet",
                "cargo test -p hepta-runtime generated_skill_and_tool_helpers_are_invokable --quiet",
                "cargo test -p hepta-plugins execution_policy_records_retry_and_fallback_telemetry --quiet",
                "cargo test --workspace --quiet",
            ],
            external_boundary_count: 3,
        },
        HeptaNativeCapability {
            id: "session-memory-continuity",
            title: "Sessions, transcript history, recall, memory provider plane, fork/merge/diff, and continuity state",
            domain: HeptaNativeCapabilityDomain::SessionMemoryContinuity,
            feature_count: 10,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-core/src/memory.rs",
                "crates/hepta-memory/src",
                "crates/hepta-runtime/src/sessions.rs",
                "crates/hepta-runtime/src/transcript.rs",
            ],
            gates: &[
                "cargo test -p hepta-core memory_provider_plane --quiet",
                "cargo test -p hepta-cli memory_providers_command_exposes_native_provider_plane --quiet",
                "cargo test -p hepta-memory --quiet",
                "cargo test -p hepta-cli recall_command_json_uses_turn_frame_contract --quiet",
            ],
            external_boundary_count: 1,
        },
        HeptaNativeCapability {
            id: "policy-approval-safety",
            title: "Policy rules, approvals, capability gates, risk tiers, and write-safety rollback",
            domain: HeptaNativeCapabilityDomain::PolicyApprovalSafety,
            feature_count: 8,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-core/src/policy.rs",
                "crates/hepta-runtime/src/policy.rs",
                "crates/hepta-runtime/src/write_transactions.rs",
            ],
            gates: &[
                "cargo test -p hepta-cli policy_commands_emit_stable_surfaces --quiet",
                "cargo test --workspace --quiet",
            ],
            external_boundary_count: 1,
        },
        HeptaNativeCapability {
            id: "coding-workflow-review",
            title: "Coding task workflow, diff/review ergonomics, command discipline, and structured reports",
            domain: HeptaNativeCapabilityDomain::CodingWorkflowReview,
            feature_count: 8,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-cli/src/lib.rs",
                "crates/hepta-runtime/src",
                "docs/release",
            ],
            gates: &[
                "cargo test -p hepta-cli command_registry_includes_native_capabilities_with_stable_usage_and_description --quiet",
                "cargo test --workspace --quiet",
            ],
            external_boundary_count: 2,
        },
        HeptaNativeCapability {
            id: "skills-automation-agents",
            title: "Skill cataloging, skill lifecycle contracts, report-first skill curator lane, bounded self-improvement review fork, transcript-to-SKILL.md workshop, generated skill safety scan, quarantine/apply plans, snapshot refresh/audit ledger/reset rollback, native routines surface, webhook security gates, durable task lanes, steer/pause/resume/interrupt subagent controls, nested spawn-depth policy, remote worker backend contract and safety regression pack, subagent observatory/file leases, multi-round autonomous coding loop, autonomous worker safety envelope, output exfiltration redaction, scheduler hooks, first-class top-level multi-agent runtime pool, per-agent inbox/event loops, Tokio JoinSet concurrent scheduler, typed cross-agent message bus with delivery state/retry metadata, selectable barrier/reducer joins with consensus status, evidence-aware readiness ratings, top-level agent pause/resume/stop/steer/drain controls, snapshot-backed multi-agent recovery gate, agent-level resource leases, local model/tool loop execution backend, injected failure/timeout recovery gate, safe join state, and intelligence self-optimization supervisor linking skills/tools readiness, learned replay scoring, calibration feedback, worker patch promotion, evidence replay, multi-agent execution, and boundary-safe promotion handoff",
            domain: HeptaNativeCapabilityDomain::SkillsAutomationAgents,
            feature_count: 86,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-core/src/routines.rs",
                "crates/hepta-runtime/src/doctor",
                "crates/hepta-runtime/src/worker_tasks.rs",
                "crates/hepta-runtime/src/multi_agent.rs",
                "crates/hepta-cli/src/commands.rs",
                "crates/hepta-core/src/skill_curator.rs",
                "crates/hepta-core/src/self_improvement_review.rs",
                "crates/hepta-core/src/local_config_import.rs",
                "scripts/hepta-local-import.sh",
            ],
            gates: &[
                "cargo test -p hepta-core skill_workshop_contract_covers_generation_scan_apply_refresh --quiet",
                "cargo test -p hepta-core generated_skill_draft_is_safe_and_canonical --quiet",
                "cargo test -p hepta-core skill_curator_lane_report_is_report_first_and_review_gated --quiet",
                "cargo test -p hepta-core bounded_self_improvement_review_fork_is_class_first_and_side_effect_free --quiet",
                "cargo test -p hepta-core skills_tools_readiness_reaches_100_with_expanded_tool_surface --quiet",
                "cargo test -p hepta-cli gateway_transport_and_skill_lifecycle_commands_expose_contracts --quiet",
                "cargo test -p hepta-core routine_surface_covers_p1_automation_contract_without_reference_shadowing --quiet",
                "cargo test -p hepta-runtime autonomous_worker_host_command_enforces_sandbox_timeout_and_output_limits --quiet",
                "cargo test -p hepta-runtime cancelled_worker_task_does_not_execute_commands_and_is_supervisor_visible --quiet",
                "cargo test -p hepta-cli routine_surface_command_exposes_native_contract --quiet",
                "cargo test -p hepta-runtime worker_task_lifecycle_is_queryable_and_snapshot_backed --quiet",
                "cargo test -p hepta-runtime top_level_multi_agent_runtime_executes_true_concurrent_joinset --quiet",
                "cargo test -p hepta-runtime empty_agent_pool_no_longer_overclaims_100_percent_readiness --quiet",
                "cargo test -p hepta-runtime multi_agent_runtime_snapshot_recovers_inbox_controls_and_evidence --quiet",
                "cargo test -p hepta-runtime multi_agent_runtime_recovers_injected_failure_with_resource_leases --quiet",
                "cargo test -p hepta-cli multi_agent_runtime_command_reports_all_100_ratings --quiet",
                "cargo test -p hepta-intelligence self_optimization_supervisor --quiet",
                "cargo test -p hepta-cli intelligence_self_optimization --quiet",
                "cargo test -p hepta-cli worker_task_commands_cover_spawn_run_status_and_workers --quiet",
                "HEPTA_REQUIRE_LOCAL_IMPORT=1 ./scripts/hepta-v0.1-smoke.sh",
                "cargo test --workspace --quiet",
            ],
            external_boundary_count: 2,
        },
        HeptaNativeCapability {
            id: "plugin-extension-plane",
            title: "Plugin bindings, hook matrix, package lifecycle contracts for discover/install/enable/doctor/dispatch/health/disable/rollback, bundled manifests, setup surfaces, channel catalog/message schemas, debug proxy coverage, extension coverage, and scaffolding",
            domain: HeptaNativeCapabilityDomain::PluginExtensionPlane,
            feature_count: 24,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-core/src/plugin.rs",
                "crates/hepta-core/src/plugin_packaging.rs",
                "crates/hepta-plugins/src",
                "crates/hepta-gateway/src",
                "crates/hepta-runtime/src/doctor/integrity.rs",
            ],
            gates: &[
                "cargo test -p hepta-core plugin_hook_matrix_covers_p1_hooks_without_reference_shadowing --quiet",
                "cargo test -p hepta-core plugin_packaging_lifecycle_covers_install_enable_doctor_dispatch_health_disable_rollback --quiet",
                "cargo test -p hepta-cli plugin_hooks_command_exposes_native_hook_matrix --quiet",
                "cargo test -p hepta-cli plugin_packaging_lifecycle_command_exposes_dry_run_contract --quiet",
                "cargo test -p hepta-plugins --quiet",
                "cargo test -p hepta-gateway --quiet",
            ],
            external_boundary_count: 2,
        },
        HeptaNativeCapability {
            id: "operator-cli-experience",
            title: "Slash/chat/subcli command registries, JSON/text surfaces, semantic routers, doctor, and local operator workflows",
            domain: HeptaNativeCapabilityDomain::OperatorCliExperience,
            feature_count: 8,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-cli/src/commands.rs",
                "crates/hepta-cli/src/lib.rs",
                "crates/hepta-runtime/src/doctor/report_bundle.rs",
            ],
            gates: &[
                "cargo test -p hepta-cli render_help_includes_intuition_and_preserves_local_command_order --quiet",
                "cargo test --workspace --quiet",
            ],
            external_boundary_count: 1,
        },
        HeptaNativeCapability {
            id: "operator-control-ui",
            title: "Live operator Control UI, developer console, safe dashboard/TUI polish contracts, model dashboard tab, lazy cold start, auto-resume/reload state, guarded resume-picker delete, xterm-style read-only command stream, browser model-switch dry-run, read-only command runner, transcript search, session/task drilldowns, real local agent chat, real local worker task publisher, transcript inspector, diff/evidence/replay review, command palette, event timeline, local dashboard screens, command bindings, loopback bind guard, browser security headers, local operator security/RBAC guard report, confirmed local mutation boundary, dry-run planning boundary, and preview server",
            domain: HeptaNativeCapabilityDomain::OperatorControlUi,
            feature_count: 43,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "apps/hepta-control-ui",
                "crates/hepta-core/src/control_ui.rs",
                "crates/hepta-core/src/operator_dashboard_polish.rs",
                "crates/hepta-cli/src/lib.rs",
                "apps/hepta/src/main.rs",
                "scripts/hepta-control-ui-smoke.sh",
            ],
            gates: &[
                "cargo test -p hepta-core operator_dashboard_polish_contract_covers_model_tab_lazy_init_resume_and_command_stream_without_side_effects --quiet",
                "cargo test -p hepta-cli operator_dashboard_polish_command_exposes_safe_dashboard_contract --quiet",
                "cargo test -p hepta-core operator_security_report_reaches_local_100_without_external_claims --quiet",
                "cargo test -p hepta-core control_ui_report_is_complete_and_asset_backed --quiet",
                "cargo test -p hepta-cli control_ui_command_reports_complete_static_frontend --quiet",
                "./scripts/hepta-control-ui-smoke.sh",
            ],
            external_boundary_count: 0,
        },
        HeptaNativeCapability {
            id: "hepta-latest-delta-absorption",
            title: "Hepta latest delta absorption: live persistent Task-board v0, gateway/session operator security hardening, plugin provider ABI with plugin-local ctx.llm and transform hooks, MCP SSE/media robustness, post-write delta lint, cron no-agent watchdogs, i18n catalog, profile/session handoff, optional search/skill refresh, platform adapter watchlist, and reverted /goal checklist guardrail",
            domain: HeptaNativeCapabilityDomain::ReleaseQualityObservability,
            feature_count: 104,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-core/src/hepta_latest_absorption.rs",
                "crates/hepta-core/src/hepta_live_sequence.rs",
                "crates/hepta-core/src/hepta_runtime_surfaces.rs",
                "crates/hepta-core/src/plugin_provider_abi.rs",
                "crates/hepta-runtime/src/task_board.rs",
                "crates/hepta-cli/src/lib.rs",
                "crates/hepta-cli/src/commands.rs",
                "crates/hepta-cli/src/hepta_p1_absorption_ops.rs",
                "crates/hepta-cli/src/runtime_event_ops.rs",
                "docs/quality/HEPTA_LATEST_DELTA_AUDIT_2026-05-12.md",
            ],
            gates: &[
                "cargo test -p hepta-core hepta_latest_absorption_covers_full_priority_queue_without_side_effects --quiet",
                "cargo test -p hepta-core hepta_latest_absorption_tracks_goal_checklist_revert_guardrail --quiet",
                "cargo test -p hepta-cli hepta_latest_absorption_command_exposes_full_priority_queue_without_side_effects --quiet",
                "cargo test -p hepta-cli full_absorption_report_covers_all_requested_planes --quiet",
                "cargo test -p hepta-cli runtime_event_sample_covers_completion_boundaries_without_side_effects --quiet",
                "cargo test -p hepta-core hepta_live_sequence_sample_run_covers_all_eight_packs --quiet",
                "cargo test -p hepta-core hepta_runtime_surfaces --quiet",
                "cargo test -p hepta-core plugin_provider_abi --quiet",
                "cargo test -p hepta-runtime task_board --quiet",
                "cargo test -p hepta-cli hepta_live_sequence_command_runs_all_eight_packs_without_external_side_effects --quiet",
            ],
            external_boundary_count: 0,
        },
        HeptaNativeCapability {
            id: "release-quality-observability",
            title: "Doctor, event log with stable query metadata, external readiness, native production surface with build git provenance, fresh structured watchdog status, intelligence eval, agent competitive advantage ledger, external agent benchmark harness, smoke/soak/preflight, and release observability",
            domain: HeptaNativeCapabilityDomain::ReleaseQualityObservability,
            feature_count: 22,
            local_executable: true,
            product_native: true,
            rust_targets: &[
                "crates/hepta-runtime/src/doctor",
                "crates/hepta-core/src/external_production.rs",
                "crates/hepta-runtime/src/events.rs",
                "crates/hepta-core/build.rs",
                "crates/hepta-core/src/production_surface.rs",
                "crates/hepta-core/src/agent_competition.rs",
                "crates/hepta-core/src/external_agent_benchmark.rs",
                "scripts/hepta-external-production-gates.sh",
                "scripts/hepta-installed-live-watchdog-recurring.sh",
                "scripts/hepta-v0.1-smoke.sh",
                "scripts/hepta-v0.1-soak.sh",
                "scripts/hepta-v0.1-preflight.sh",
            ],
            gates: &[
                "cargo test -p hepta-core external_production_registry_tracks_seventeen_boundaries --quiet",
                "cargo test -p hepta-core production_surface_report_is_hepta_native --quiet",
                "cargo test -p hepta-core agent_competitive_advantage_report_is_local_100_without_public_overclaim --quiet",
                "cargo test -p hepta-core external_agent_benchmark_harness_is_ready_without_public_overclaim --quiet",
                "./scripts/hepta-external-production-gates.sh --plan",
                "HEPTA_SOAK_ITERATIONS=3 ./scripts/hepta-v0.1-soak.sh",
                "HEPTA_REQUIRE_LOCAL_IMPORT=1 HEPTA_SOAK_ITERATIONS=1 ./scripts/hepta-v0.1-preflight.sh",
            ],
            external_boundary_count: 1,
        },
    ]
}

fn percent(numerator: usize, denominator: usize) -> u8 {
    if denominator == 0 {
        return 0;
    }
    ((numerator * 100) / denominator) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_absorption_report_is_product_complete() {
        let report = hepta_native_absorption_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.feature_count, 419);
        assert_eq!(report.capability_count, 12);
        assert_eq!(report.native_absorption_coverage_percent, 100);
        assert_eq!(report.local_executable_coverage_percent, 100);
        assert_eq!(report.external_boundary_count, 17);
        assert!(report.product_surface_native);
        assert!(report.native_absorption_complete());
    }
}
