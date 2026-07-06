use crate::ToolRegistryInvocationGuardRoute;
use crate::ToolRegistryRegistrationLookupCutoverPreflightPlan;
use crate::ToolRegistryRegistrationLookupCutoverRoute;
use crate::hepta_system_tool_registry_registration_lookup_cutover_preflight_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolRegistryRouterLookupShadowRoute {
    DisabledApprovalLedgerLookupShadow,
    BlockedByEnabledLookupSwitch,
    BlockedByMissingShadowBinding,
    BlockedByLookupPrecondition,
    BlockedByLookupGuard,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryRouterLookupShadowInput {
    pub registration_lookup_cutover_switch_enabled: bool,
    pub router_shadow_binding_present: bool,
}

impl Default for ToolRegistryRouterLookupShadowInput {
    fn default() -> Self {
        Self {
            registration_lookup_cutover_switch_enabled: false,
            router_shadow_binding_present: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryRouterLookupShadowEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub lookup_cutover_route: ToolRegistryRegistrationLookupCutoverRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub shadow_route: ToolRegistryRouterLookupShadowRoute,
    pub shadow_ready: bool,
    pub registration_lookup_cutover_switch_enabled: bool,
    pub router_shadow_binding_present: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryRouterLookupShadowPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_registration_lookup_preflight_surface: &'static str,
    pub source_registration_lookup_preflight_ready: bool,
    pub registration_lookup_cutover_switch_enabled: bool,
    pub router_shadow_binding_present: bool,
    pub candidate_count: usize,
    pub shadow_ready_count: usize,
    pub shadow_blocked_count: usize,
    pub disabled_lookup_shadow_count: usize,
    pub all_lookup_preflight_entries_shadowed: bool,
    pub all_shadow_entries_keep_approval_ledger_guard: bool,
    pub router_lookup_shadow_ready: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<ToolRegistryRouterLookupShadowEntry>,
}

pub fn hepta_system_tool_registry_router_lookup_shadow_plan() -> ToolRegistryRouterLookupShadowPlan
{
    let preflight = hepta_system_tool_registry_registration_lookup_cutover_preflight_plan();
    tool_registry_router_lookup_shadow_plan(
        &preflight,
        &ToolRegistryRouterLookupShadowInput::default(),
    )
}

pub fn tool_registry_router_lookup_shadow_plan(
    preflight: &ToolRegistryRegistrationLookupCutoverPreflightPlan,
    input: &ToolRegistryRouterLookupShadowInput,
) -> ToolRegistryRouterLookupShadowPlan {
    let entries = preflight
        .entries
        .iter()
        .map(|entry| {
            let shadow_route = if input.registration_lookup_cutover_switch_enabled {
                ToolRegistryRouterLookupShadowRoute::BlockedByEnabledLookupSwitch
            } else if !input.router_shadow_binding_present {
                ToolRegistryRouterLookupShadowRoute::BlockedByMissingShadowBinding
            } else if !entry.lookup_precondition_satisfied
                || entry.lookup_cutover_route
                    != ToolRegistryRegistrationLookupCutoverRoute::ApprovalLedgerLookupDryRun
            {
                ToolRegistryRouterLookupShadowRoute::BlockedByLookupPrecondition
            } else if entry.registry_guard_route
                != ToolRegistryInvocationGuardRoute::RequireApprovalLedger
            {
                ToolRegistryRouterLookupShadowRoute::BlockedByLookupGuard
            } else {
                ToolRegistryRouterLookupShadowRoute::DisabledApprovalLedgerLookupShadow
            };
            let shadow_ready = shadow_route
                == ToolRegistryRouterLookupShadowRoute::DisabledApprovalLedgerLookupShadow
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled;

            ToolRegistryRouterLookupShadowEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                lookup_cutover_route: entry.lookup_cutover_route,
                registry_guard_route: entry.registry_guard_route,
                shadow_route,
                shadow_ready,
                registration_lookup_cutover_switch_enabled: input
                    .registration_lookup_cutover_switch_enabled,
                router_shadow_binding_present: input.router_shadow_binding_present,
                router_registration_lookup_enabled: false,
                registry_lookup_executed: false,
                registry_source_of_truth_enabled: false,
                tool_registration_enabled: false,
                tool_invocation_enabled: false,
                ledger_write_enabled: false,
                approval_request_enabled: false,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let shadow_ready_count = entries.iter().filter(|entry| entry.shadow_ready).count();
    let disabled_lookup_shadow_count = entries
        .iter()
        .filter(|entry| {
            entry.shadow_route
                == ToolRegistryRouterLookupShadowRoute::DisabledApprovalLedgerLookupShadow
        })
        .count();
    let shadow_blocked_count = entries.len() - shadow_ready_count;
    let all_lookup_preflight_entries_shadowed = shadow_ready_count == entries.len()
        && disabled_lookup_shadow_count == entries.len()
        && entries
            .iter()
            .all(|entry| entry.router_shadow_binding_present);
    let all_shadow_entries_keep_approval_ledger_guard = entries.iter().all(|entry| {
        if entry.shadow_route
            == ToolRegistryRouterLookupShadowRoute::DisabledApprovalLedgerLookupShadow
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.registration_lookup_cutover_switch_enabled
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        } else {
            true
        }
    });
    let router_lookup_shadow_ready = preflight.registration_lookup_cutover_preflight_ready
        && !input.registration_lookup_cutover_switch_enabled
        && input.router_shadow_binding_present
        && all_lookup_preflight_entries_shadowed
        && all_shadow_entries_keep_approval_ledger_guard;

    ToolRegistryRouterLookupShadowPlan {
        runtime: "hepta",
        surface: "tool_registry_router_lookup_shadow",
        plugin_id: preflight.plugin_id,
        status: if router_lookup_shadow_ready {
            "ready"
        } else {
            "blocked"
        },
        source_registration_lookup_preflight_surface: preflight.surface,
        source_registration_lookup_preflight_ready: preflight
            .registration_lookup_cutover_preflight_ready,
        registration_lookup_cutover_switch_enabled: input
            .registration_lookup_cutover_switch_enabled,
        router_shadow_binding_present: input.router_shadow_binding_present,
        candidate_count: entries.len(),
        shadow_ready_count,
        shadow_blocked_count,
        disabled_lookup_shadow_count,
        all_lookup_preflight_entries_shadowed,
        all_shadow_entries_keep_approval_ledger_guard,
        router_lookup_shadow_ready,
        router_registration_lookup_enabled: false,
        registry_lookup_executed: false,
        registry_source_of_truth_enabled: false,
        tool_registration_enabled: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_invocation_receipt_projection_without_execution",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_registry_router_lookup_shadow_binds_disabled_lookup_switch() {
        let plan = hepta_system_tool_registry_router_lookup_shadow_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_registration_lookup_preflight_surface,
            "tool_registry_registration_lookup_cutover_preflight"
        );
        assert!(plan.source_registration_lookup_preflight_ready);
        assert!(!plan.registration_lookup_cutover_switch_enabled);
        assert!(plan.router_shadow_binding_present);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.shadow_ready_count, 2);
        assert_eq!(plan.shadow_blocked_count, 0);
        assert_eq!(plan.disabled_lookup_shadow_count, 2);
        assert!(plan.all_lookup_preflight_entries_shadowed);
        assert!(plan.all_shadow_entries_keep_approval_ledger_guard);
        assert!(plan.router_lookup_shadow_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.shadow_route
                == ToolRegistryRouterLookupShadowRoute::DisabledApprovalLedgerLookupShadow
                && entry.shadow_ready
                && entry.registry_guard_route
                    == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
        }));
    }

    #[test]
    fn tool_registry_router_lookup_shadow_does_not_execute_lookup() {
        let plan = hepta_system_tool_registry_router_lookup_shadow_plan();

        assert!(plan.router_lookup_shadow_ready);
        assert!(!plan.router_registration_lookup_enabled);
        assert!(!plan.registry_lookup_executed);
        assert!(!plan.registry_source_of_truth_enabled);
        assert!(!plan.tool_registration_enabled);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
        assert!(plan.entries.iter().all(|entry| {
            !entry.registration_lookup_cutover_switch_enabled
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        }));
    }

    #[test]
    fn tool_registry_router_lookup_shadow_fails_closed_when_switch_enabled() {
        let preflight = hepta_system_tool_registry_registration_lookup_cutover_preflight_plan();
        let input = ToolRegistryRouterLookupShadowInput {
            registration_lookup_cutover_switch_enabled: true,
            router_shadow_binding_present: true,
        };

        let plan = tool_registry_router_lookup_shadow_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.registration_lookup_cutover_switch_enabled);
        assert_eq!(plan.shadow_ready_count, 0);
        assert_eq!(plan.shadow_blocked_count, 2);
        assert!(!plan.router_lookup_shadow_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.shadow_route == ToolRegistryRouterLookupShadowRoute::BlockedByEnabledLookupSwitch
        }));
    }

    #[test]
    fn tool_registry_router_lookup_shadow_fails_closed_when_binding_missing() {
        let preflight = hepta_system_tool_registry_registration_lookup_cutover_preflight_plan();
        let input = ToolRegistryRouterLookupShadowInput {
            registration_lookup_cutover_switch_enabled: false,
            router_shadow_binding_present: false,
        };

        let plan = tool_registry_router_lookup_shadow_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.router_shadow_binding_present);
        assert_eq!(plan.shadow_ready_count, 0);
        assert_eq!(plan.shadow_blocked_count, 2);
        assert!(!plan.router_lookup_shadow_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.shadow_route == ToolRegistryRouterLookupShadowRoute::BlockedByMissingShadowBinding
        }));
    }

    #[test]
    fn tool_registry_router_lookup_shadow_fails_closed_without_lookup_precondition() {
        let mut preflight = hepta_system_tool_registry_registration_lookup_cutover_preflight_plan();
        preflight.lookup_precondition_satisfied_count = 0;
        preflight.registration_lookup_cutover_preflight_ready = false;
        preflight.registration_lookup_cutover_allowed = false;
        for entry in &mut preflight.entries {
            entry.lookup_precondition_satisfied = false;
            entry.lookup_cutover_route =
                ToolRegistryRegistrationLookupCutoverRoute::BlockedByInvocationSource;
        }

        let plan = tool_registry_router_lookup_shadow_plan(
            &preflight,
            &ToolRegistryRouterLookupShadowInput::default(),
        );

        assert_eq!(plan.status, "blocked");
        assert_eq!(plan.shadow_ready_count, 0);
        assert_eq!(plan.shadow_blocked_count, 2);
        assert!(!plan.router_lookup_shadow_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.shadow_route == ToolRegistryRouterLookupShadowRoute::BlockedByLookupPrecondition
        }));
    }
}
