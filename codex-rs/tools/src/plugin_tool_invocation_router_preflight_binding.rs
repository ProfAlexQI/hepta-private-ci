use crate::PluginToolManifestPreflightDecisionRoute;
use crate::PluginToolManifestSchemaCutoverPreflightPlan;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_plugin_tool_manifest_schema_cutover_preflight_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum PluginToolInvocationRouterPreflightDecisionRoute {
    ForwardRequireApprovalLedgerDryRun,
    BlockManifestPreconditions,
    BlockSourceRegistry,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PluginToolInvocationRouterPreflightBindingEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_registry_dry_run_ready: bool,
    pub source_manifest_schema_preflight_ready: bool,
    pub router_binding_present: bool,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub registration_preconditions_satisfied: bool,
    pub registration_cutover_allowed: bool,
    pub router_decision_route: PluginToolInvocationRouterPreflightDecisionRoute,
    pub router_blocked: bool,
    pub router_blocked_reason: Option<&'static str>,
    pub router_registration_lookup_enabled: bool,
    pub tool_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PluginToolInvocationRouterPreflightBindingPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_registry_dry_run_surface: &'static str,
    pub source_registry_dry_run_ready: bool,
    pub source_manifest_schema_preflight_surface: &'static str,
    pub source_manifest_schema_preflight_ready: bool,
    pub source_manifest_parser_fields_surface: &'static str,
    pub source_manifest_parser_fields_ready: bool,
    pub candidate_count: usize,
    pub router_bound_candidate_count: usize,
    pub router_unbound_candidate_count: usize,
    pub router_blocked_candidate_count: usize,
    pub router_blocked_by_source_registry_count: usize,
    pub router_blocked_by_manifest_precondition_count: usize,
    pub router_forward_require_approval_ledger_count: usize,
    pub registration_precondition_satisfied_count: usize,
    pub registration_cutover_allowed: bool,
    pub all_candidates_bound_to_router: bool,
    pub all_missing_manifest_preconditions_blocked: bool,
    pub all_forwarded_candidates_keep_approval_ledger: bool,
    pub invocation_router_preflight_binding_ready: bool,
    pub router_registration_lookup_enabled: bool,
    pub registration_execution_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<PluginToolInvocationRouterPreflightBindingEntry>,
}

pub fn hepta_system_plugin_tool_invocation_router_preflight_binding_plan()
-> PluginToolInvocationRouterPreflightBindingPlan {
    let preflight = hepta_system_plugin_tool_manifest_schema_cutover_preflight_plan();
    plugin_tool_invocation_router_preflight_binding_plan(&preflight)
}

pub fn plugin_tool_invocation_router_preflight_binding_plan(
    preflight: &PluginToolManifestSchemaCutoverPreflightPlan,
) -> PluginToolInvocationRouterPreflightBindingPlan {
    let entries = preflight
        .entries
        .iter()
        .map(|entry| {
            let router_decision_route = match entry.decision_route {
                PluginToolManifestPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun => {
                    PluginToolInvocationRouterPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun
                }
                PluginToolManifestPreflightDecisionRoute::BlockManifestPreconditions => {
                    PluginToolInvocationRouterPreflightDecisionRoute::BlockManifestPreconditions
                }
                PluginToolManifestPreflightDecisionRoute::BlockSourceRegistry => {
                    PluginToolInvocationRouterPreflightDecisionRoute::BlockSourceRegistry
                }
            };
            let router_blocked = router_decision_route
                != PluginToolInvocationRouterPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun;
            let router_blocked_reason = match router_decision_route {
                PluginToolInvocationRouterPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun => None,
                PluginToolInvocationRouterPreflightDecisionRoute::BlockManifestPreconditions => {
                    Some("manifest_schema_or_policy_preconditions_missing")
                }
                PluginToolInvocationRouterPreflightDecisionRoute::BlockSourceRegistry => {
                    Some("source_registry_dry_run_not_ready")
                }
            };

            PluginToolInvocationRouterPreflightBindingEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                source_registry_dry_run_ready: entry.source_registry_dry_run_ready,
                source_manifest_schema_preflight_ready: preflight
                    .manifest_schema_cutover_preflight_ready,
                router_binding_present: true,
                registry_guard_route: entry.registry_guard_route,
                registration_preconditions_satisfied: entry.registration_preconditions_satisfied,
                registration_cutover_allowed: preflight.registration_cutover_allowed,
                router_decision_route,
                router_blocked,
                router_blocked_reason,
                router_registration_lookup_enabled: false,
                tool_registration_enabled: false,
                tool_invocation_enabled: false,
                ledger_write_enabled: false,
                approval_request_enabled: false,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let router_blocked_by_source_registry_count = entries
        .iter()
        .filter(|entry| {
            entry.router_decision_route
                == PluginToolInvocationRouterPreflightDecisionRoute::BlockSourceRegistry
        })
        .count();
    let router_blocked_by_manifest_precondition_count = entries
        .iter()
        .filter(|entry| {
            entry.router_decision_route
                == PluginToolInvocationRouterPreflightDecisionRoute::BlockManifestPreconditions
        })
        .count();
    let router_forward_require_approval_ledger_count = entries
        .iter()
        .filter(|entry| {
            entry.router_decision_route
                == PluginToolInvocationRouterPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun
        })
        .count();
    let router_blocked_candidate_count =
        router_blocked_by_source_registry_count + router_blocked_by_manifest_precondition_count;
    let all_candidates_bound_to_router = entries.iter().all(|entry| entry.router_binding_present);
    let all_missing_manifest_preconditions_blocked = entries.iter().all(|entry| {
        if entry.registration_preconditions_satisfied {
            true
        } else {
            entry.router_decision_route
                == PluginToolInvocationRouterPreflightDecisionRoute::BlockManifestPreconditions
                && entry.router_blocked
        }
    });
    let all_forwarded_candidates_keep_approval_ledger = entries.iter().all(|entry| {
        if entry.router_decision_route
            == PluginToolInvocationRouterPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.router_registration_lookup_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        } else {
            true
        }
    });
    let invocation_router_preflight_binding_ready = preflight
        .manifest_schema_cutover_preflight_ready
        && all_candidates_bound_to_router
        && all_missing_manifest_preconditions_blocked
        && all_forwarded_candidates_keep_approval_ledger;

    PluginToolInvocationRouterPreflightBindingPlan {
        runtime: "hepta",
        surface: "plugin_tool_invocation_router_preflight_binding",
        plugin_id: preflight.plugin_id,
        status: if invocation_router_preflight_binding_ready {
            "ready"
        } else {
            "blocked"
        },
        source_registry_dry_run_surface: preflight.source_registry_dry_run_surface,
        source_registry_dry_run_ready: preflight.source_registry_dry_run_ready,
        source_manifest_schema_preflight_surface: preflight.surface,
        source_manifest_schema_preflight_ready: preflight.manifest_schema_cutover_preflight_ready,
        source_manifest_parser_fields_surface: preflight.source_manifest_parser_fields_surface,
        source_manifest_parser_fields_ready: preflight.source_manifest_parser_fields_ready,
        candidate_count: entries.len(),
        router_bound_candidate_count: entries.len(),
        router_unbound_candidate_count: 0,
        router_blocked_candidate_count,
        router_blocked_by_source_registry_count,
        router_blocked_by_manifest_precondition_count,
        router_forward_require_approval_ledger_count,
        registration_precondition_satisfied_count: preflight
            .registration_precondition_satisfied_count,
        registration_cutover_allowed: preflight.registration_cutover_allowed,
        all_candidates_bound_to_router,
        all_missing_manifest_preconditions_blocked,
        all_forwarded_candidates_keep_approval_ledger,
        invocation_router_preflight_binding_ready,
        router_registration_lookup_enabled: false,
        registration_execution_enabled: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_registry_invocation_source_of_truth_without_execution",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PluginToolManifestPreflightInput;
    use crate::hepta_system_plugin_tool_contribution_inventory_preview_plan;
    use crate::hepta_system_plugin_tool_manifest_schema_cutover_preflight_plan;
    use crate::hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan;
    use crate::hepta_system_plugin_tool_replacement_fixture_preflight_input;
    use crate::plugin_tool_manifest_schema_cutover_preflight_plan;
    use crate::plugin_tool_registry_source_of_truth_dry_run_plan;

    fn complete_manifest_preflight() -> PluginToolManifestSchemaCutoverPreflightPlan {
        let registry_plan = hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan();
        let input = hepta_system_plugin_tool_replacement_fixture_preflight_input();
        plugin_tool_manifest_schema_cutover_preflight_plan(&registry_plan, &input)
    }

    #[test]
    fn plugin_tool_invocation_router_preflight_binding_blocks_missing_manifest_preconditions() {
        let registry_plan = hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan();
        let preflight = plugin_tool_manifest_schema_cutover_preflight_plan(
            &registry_plan,
            &PluginToolManifestPreflightInput::default(),
        );
        let plan = plugin_tool_invocation_router_preflight_binding_plan(&preflight);

        assert_eq!(plan.status, "ready");
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.router_bound_candidate_count, 2);
        assert_eq!(plan.router_unbound_candidate_count, 0);
        assert_eq!(plan.router_blocked_candidate_count, 2);
        assert_eq!(plan.router_blocked_by_manifest_precondition_count, 2);
        assert_eq!(plan.router_forward_require_approval_ledger_count, 0);
        assert_eq!(plan.registration_precondition_satisfied_count, 0);
        assert!(!plan.registration_cutover_allowed);
        assert!(plan.invocation_router_preflight_binding_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.router_decision_route
                == PluginToolInvocationRouterPreflightDecisionRoute::BlockManifestPreconditions
                && entry.router_blocked
                && entry.router_blocked_reason
                    == Some("manifest_schema_or_policy_preconditions_missing")
                && !entry.router_registration_lookup_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        }));
    }

    #[test]
    fn plugin_tool_invocation_router_preflight_binding_forwards_complete_preconditions_as_dry_run()
    {
        let preflight = hepta_system_plugin_tool_manifest_schema_cutover_preflight_plan();
        let plan = plugin_tool_invocation_router_preflight_binding_plan(&preflight);

        assert_eq!(plan.status, "ready");
        assert_eq!(plan.router_blocked_candidate_count, 0);
        assert_eq!(plan.router_forward_require_approval_ledger_count, 2);
        assert_eq!(plan.registration_precondition_satisfied_count, 2);
        assert!(plan.registration_cutover_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.router_decision_route
                == PluginToolInvocationRouterPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun
                && !entry.router_blocked
                && entry.router_blocked_reason.is_none()
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.router_registration_lookup_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        }));
    }

    #[test]
    fn plugin_tool_invocation_router_preflight_binding_does_not_register_or_invoke() {
        let plan = complete_manifest_preflight();
        let router = plugin_tool_invocation_router_preflight_binding_plan(&plan);

        assert!(router.registration_cutover_allowed);
        assert!(!router.router_registration_lookup_enabled);
        assert!(!router.registration_execution_enabled);
        assert!(!router.tool_invocation_enabled);
        assert!(!router.ledger_written);
        assert!(!router.approval_requested);
        assert!(!router.live_mutation_ready);
        assert!(router.side_effect_free);
    }

    #[test]
    fn plugin_tool_invocation_router_preflight_binding_blocks_source_registry_failures() {
        let mut preview = hepta_system_plugin_tool_contribution_inventory_preview_plan();
        preview.entries.push(preview.entries[0].clone());
        preview
            .candidate_inventory_entries
            .push(preview.candidate_inventory_entries[0].clone());
        let registry_plan = plugin_tool_registry_source_of_truth_dry_run_plan(&preview);
        let preflight = plugin_tool_manifest_schema_cutover_preflight_plan(
            &registry_plan,
            &PluginToolManifestPreflightInput::default(),
        );

        let router = plugin_tool_invocation_router_preflight_binding_plan(&preflight);

        assert_eq!(router.status, "blocked");
        assert!(!router.source_registry_dry_run_ready);
        assert!(router.entries.iter().all(|entry| {
            entry.router_decision_route
                == PluginToolInvocationRouterPreflightDecisionRoute::BlockSourceRegistry
                && entry.router_blocked
                && entry.router_blocked_reason == Some("source_registry_dry_run_not_ready")
        }));
    }
}
