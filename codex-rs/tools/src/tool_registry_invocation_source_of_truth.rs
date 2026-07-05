use crate::PluginToolInvocationRouterPreflightBindingPlan;
use crate::PluginToolInvocationRouterPreflightDecisionRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_plugin_tool_invocation_router_preflight_binding_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolRegistryInvocationSourceRoute {
    ApprovalLedgerDryRunSourceOnly,
    BlockedByRouterPreflight,
    BlockedBySourceRegistry,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryInvocationSourceOfTruthEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub router_decision_route: PluginToolInvocationRouterPreflightDecisionRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub invocation_source_route: ToolRegistryInvocationSourceRoute,
    pub invocation_source_ready: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryInvocationSourceOfTruthPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_router_preflight_surface: &'static str,
    pub source_router_preflight_ready: bool,
    pub source_router_forward_count: usize,
    pub candidate_count: usize,
    pub invocation_source_ready_count: usize,
    pub invocation_source_blocked_count: usize,
    pub approval_ledger_dry_run_source_count: usize,
    pub all_forwarded_candidates_bound_to_invocation_source: bool,
    pub all_invocation_sources_keep_approval_ledger_guard: bool,
    pub invocation_source_of_truth_plan_ready: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<ToolRegistryInvocationSourceOfTruthEntry>,
}

pub fn hepta_system_tool_registry_invocation_source_of_truth_plan()
-> ToolRegistryInvocationSourceOfTruthPlan {
    let router = hepta_system_plugin_tool_invocation_router_preflight_binding_plan();
    tool_registry_invocation_source_of_truth_plan(&router)
}

pub fn tool_registry_invocation_source_of_truth_plan(
    router: &PluginToolInvocationRouterPreflightBindingPlan,
) -> ToolRegistryInvocationSourceOfTruthPlan {
    let entries = router
        .entries
        .iter()
        .map(|entry| {
            let invocation_source_route = match entry.router_decision_route {
                PluginToolInvocationRouterPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun => {
                    ToolRegistryInvocationSourceRoute::ApprovalLedgerDryRunSourceOnly
                }
                PluginToolInvocationRouterPreflightDecisionRoute::BlockManifestPreconditions => {
                    ToolRegistryInvocationSourceRoute::BlockedByRouterPreflight
                }
                PluginToolInvocationRouterPreflightDecisionRoute::BlockSourceRegistry => {
                    ToolRegistryInvocationSourceRoute::BlockedBySourceRegistry
                }
            };
            let invocation_source_ready = invocation_source_route
                == ToolRegistryInvocationSourceRoute::ApprovalLedgerDryRunSourceOnly
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.router_registration_lookup_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled;

            ToolRegistryInvocationSourceOfTruthEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                router_decision_route: entry.router_decision_route,
                registry_guard_route: entry.registry_guard_route,
                invocation_source_route,
                invocation_source_ready,
                router_registration_lookup_enabled: false,
                registry_source_of_truth_enabled: false,
                tool_registration_enabled: false,
                tool_invocation_enabled: false,
                ledger_write_enabled: false,
                approval_request_enabled: false,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let invocation_source_ready_count = entries
        .iter()
        .filter(|entry| entry.invocation_source_ready)
        .count();
    let approval_ledger_dry_run_source_count = entries
        .iter()
        .filter(|entry| {
            entry.invocation_source_route
                == ToolRegistryInvocationSourceRoute::ApprovalLedgerDryRunSourceOnly
        })
        .count();
    let invocation_source_blocked_count = entries.len() - invocation_source_ready_count;
    let all_forwarded_candidates_bound_to_invocation_source = approval_ledger_dry_run_source_count
        == router.router_forward_require_approval_ledger_count
        && invocation_source_ready_count == router.router_forward_require_approval_ledger_count;
    let all_invocation_sources_keep_approval_ledger_guard = entries.iter().all(|entry| {
        if entry.invocation_source_route
            == ToolRegistryInvocationSourceRoute::ApprovalLedgerDryRunSourceOnly
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.router_registration_lookup_enabled
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        } else {
            true
        }
    });
    let invocation_source_of_truth_plan_ready = router.invocation_router_preflight_binding_ready
        && all_forwarded_candidates_bound_to_invocation_source
        && all_invocation_sources_keep_approval_ledger_guard;

    ToolRegistryInvocationSourceOfTruthPlan {
        runtime: "hepta",
        surface: "tool_registry_invocation_source_of_truth",
        plugin_id: router.plugin_id,
        status: if invocation_source_of_truth_plan_ready {
            "ready"
        } else {
            "blocked"
        },
        source_router_preflight_surface: router.surface,
        source_router_preflight_ready: router.invocation_router_preflight_binding_ready,
        source_router_forward_count: router.router_forward_require_approval_ledger_count,
        candidate_count: entries.len(),
        invocation_source_ready_count,
        invocation_source_blocked_count,
        approval_ledger_dry_run_source_count,
        all_forwarded_candidates_bound_to_invocation_source,
        all_invocation_sources_keep_approval_ledger_guard,
        invocation_source_of_truth_plan_ready,
        router_registration_lookup_enabled: false,
        registry_source_of_truth_enabled: false,
        tool_registration_enabled: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_registry_registration_lookup_cutover_preflight_without_execution",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PluginToolInvocationRouterPreflightBindingPlan;

    #[test]
    fn tool_registry_invocation_source_of_truth_binds_forwarded_router_candidates() {
        let router = hepta_system_plugin_tool_invocation_router_preflight_binding_plan();
        let plan = tool_registry_invocation_source_of_truth_plan(&router);

        assert_eq!(plan.status, "ready");
        assert_eq!(plan.source_router_forward_count, 2);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.invocation_source_ready_count, 2);
        assert_eq!(plan.invocation_source_blocked_count, 0);
        assert_eq!(plan.approval_ledger_dry_run_source_count, 2);
        assert!(plan.all_forwarded_candidates_bound_to_invocation_source);
        assert!(plan.invocation_source_of_truth_plan_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.invocation_source_route
                == ToolRegistryInvocationSourceRoute::ApprovalLedgerDryRunSourceOnly
                && entry.invocation_source_ready
                && entry.registry_guard_route
                    == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
        }));
    }

    #[test]
    fn tool_registry_invocation_source_of_truth_does_not_enable_execution() {
        let plan = hepta_system_tool_registry_invocation_source_of_truth_plan();

        assert!(plan.invocation_source_of_truth_plan_ready);
        assert!(!plan.router_registration_lookup_enabled);
        assert!(!plan.registry_source_of_truth_enabled);
        assert!(!plan.tool_registration_enabled);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
        assert!(plan.entries.iter().all(|entry| {
            !entry.router_registration_lookup_enabled
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        }));
    }

    #[test]
    fn tool_registry_invocation_source_of_truth_blocks_when_router_has_no_forwarded_candidates() {
        let mut router = hepta_system_plugin_tool_invocation_router_preflight_binding_plan();
        router.router_forward_require_approval_ledger_count = 0;
        for entry in &mut router.entries {
            entry.router_decision_route =
                PluginToolInvocationRouterPreflightDecisionRoute::BlockManifestPreconditions;
            entry.router_blocked = true;
            entry.router_blocked_reason = Some("manifest_schema_or_policy_preconditions_missing");
        }

        let plan = tool_registry_invocation_source_of_truth_plan(&router);

        assert_eq!(plan.status, "ready");
        assert_eq!(plan.invocation_source_ready_count, 0);
        assert_eq!(plan.invocation_source_blocked_count, 2);
        assert_eq!(plan.approval_ledger_dry_run_source_count, 0);
        assert!(plan.all_forwarded_candidates_bound_to_invocation_source);
    }

    #[test]
    fn tool_registry_invocation_source_of_truth_respects_router_readiness() {
        let router = PluginToolInvocationRouterPreflightBindingPlan {
            invocation_router_preflight_binding_ready: false,
            ..hepta_system_plugin_tool_invocation_router_preflight_binding_plan()
        };

        let plan = tool_registry_invocation_source_of_truth_plan(&router);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.source_router_preflight_ready);
        assert!(!plan.invocation_source_of_truth_plan_ready);
    }
}
