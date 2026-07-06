use crate::ToolRegistryInvocationGuardRoute;
use crate::ToolRegistryRouterLookupShadowPlan;
use crate::ToolRegistryRouterLookupShadowRoute;
use crate::hepta_system_tool_registry_router_lookup_shadow_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolInvocationLedgerApprovalPreflightRoute {
    ApprovalLedgerPreflightRequired,
    BlockedByEnabledExecutionSwitch,
    BlockedByMissingLedgerBinding,
    BlockedByMissingApprovalBrokerBinding,
    BlockedByRouterLookupShadow,
    BlockedByRegistryGuard,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolInvocationLedgerApprovalPreflightInput {
    pub tool_invocation_ledger_binding_present: bool,
    pub approval_broker_preflight_binding_present: bool,
    pub ledger_write_switch_enabled: bool,
    pub approval_request_switch_enabled: bool,
}

impl Default for ToolInvocationLedgerApprovalPreflightInput {
    fn default() -> Self {
        Self {
            tool_invocation_ledger_binding_present: true,
            approval_broker_preflight_binding_present: true,
            ledger_write_switch_enabled: false,
            approval_request_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolInvocationLedgerApprovalPreflightEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_shadow_route: ToolRegistryRouterLookupShadowRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub preflight_route: ToolInvocationLedgerApprovalPreflightRoute,
    pub ledger_preflight_ready: bool,
    pub approval_preflight_required: bool,
    pub tool_invocation_ledger_binding_present: bool,
    pub approval_broker_preflight_binding_present: bool,
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
pub struct ToolInvocationLedgerApprovalPreflightPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_router_lookup_shadow_surface: &'static str,
    pub source_router_lookup_shadow_ready: bool,
    pub tool_invocation_ledger_binding_present: bool,
    pub approval_broker_preflight_binding_present: bool,
    pub ledger_write_switch_enabled: bool,
    pub approval_request_switch_enabled: bool,
    pub candidate_count: usize,
    pub ledger_approval_preflight_ready_count: usize,
    pub ledger_approval_preflight_blocked_count: usize,
    pub approval_ledger_preflight_required_count: usize,
    pub all_shadow_entries_bound_to_ledger_approval_preflight: bool,
    pub all_ledger_approval_entries_keep_approval_guard: bool,
    pub tool_invocation_ledger_approval_preflight_ready: bool,
    pub ledger_approval_preflight_allowed: bool,
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
    pub entries: Vec<ToolInvocationLedgerApprovalPreflightEntry>,
}

pub fn hepta_system_tool_invocation_ledger_approval_preflight_plan()
-> ToolInvocationLedgerApprovalPreflightPlan {
    let shadow = hepta_system_tool_registry_router_lookup_shadow_plan();
    tool_invocation_ledger_approval_preflight_plan(
        &shadow,
        &ToolInvocationLedgerApprovalPreflightInput::default(),
    )
}

pub fn tool_invocation_ledger_approval_preflight_plan(
    shadow: &ToolRegistryRouterLookupShadowPlan,
    input: &ToolInvocationLedgerApprovalPreflightInput,
) -> ToolInvocationLedgerApprovalPreflightPlan {
    let entries = shadow
        .entries
        .iter()
        .map(|entry| {
            let preflight_route = if input.ledger_write_switch_enabled
                || input.approval_request_switch_enabled
            {
                ToolInvocationLedgerApprovalPreflightRoute::BlockedByEnabledExecutionSwitch
            } else if !input.tool_invocation_ledger_binding_present {
                ToolInvocationLedgerApprovalPreflightRoute::BlockedByMissingLedgerBinding
            } else if !input.approval_broker_preflight_binding_present {
                ToolInvocationLedgerApprovalPreflightRoute::BlockedByMissingApprovalBrokerBinding
            } else if !entry.shadow_ready
                || entry.shadow_route
                    != ToolRegistryRouterLookupShadowRoute::DisabledApprovalLedgerLookupShadow
            {
                ToolInvocationLedgerApprovalPreflightRoute::BlockedByRouterLookupShadow
            } else if entry.registry_guard_route
                != ToolRegistryInvocationGuardRoute::RequireApprovalLedger
            {
                ToolInvocationLedgerApprovalPreflightRoute::BlockedByRegistryGuard
            } else {
                ToolInvocationLedgerApprovalPreflightRoute::ApprovalLedgerPreflightRequired
            };
            let ledger_preflight_ready = preflight_route
                == ToolInvocationLedgerApprovalPreflightRoute::ApprovalLedgerPreflightRequired
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !input.ledger_write_switch_enabled
                && !input.approval_request_switch_enabled;

            ToolInvocationLedgerApprovalPreflightEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                source_shadow_route: entry.shadow_route,
                registry_guard_route: entry.registry_guard_route,
                preflight_route,
                ledger_preflight_ready,
                approval_preflight_required: preflight_route
                    == ToolInvocationLedgerApprovalPreflightRoute::ApprovalLedgerPreflightRequired,
                tool_invocation_ledger_binding_present: input
                    .tool_invocation_ledger_binding_present,
                approval_broker_preflight_binding_present: input
                    .approval_broker_preflight_binding_present,
                router_registration_lookup_enabled: false,
                registry_lookup_executed: false,
                registry_source_of_truth_enabled: false,
                tool_registration_enabled: false,
                tool_invocation_enabled: false,
                ledger_write_enabled: input.ledger_write_switch_enabled,
                approval_request_enabled: input.approval_request_switch_enabled,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let ledger_approval_preflight_ready_count = entries
        .iter()
        .filter(|entry| entry.ledger_preflight_ready)
        .count();
    let approval_ledger_preflight_required_count = entries
        .iter()
        .filter(|entry| {
            entry.preflight_route
                == ToolInvocationLedgerApprovalPreflightRoute::ApprovalLedgerPreflightRequired
        })
        .count();
    let ledger_approval_preflight_blocked_count =
        entries.len() - ledger_approval_preflight_ready_count;
    let all_shadow_entries_bound_to_ledger_approval_preflight = input
        .tool_invocation_ledger_binding_present
        && input.approval_broker_preflight_binding_present
        && ledger_approval_preflight_ready_count == entries.len()
        && approval_ledger_preflight_required_count == entries.len();
    let all_ledger_approval_entries_keep_approval_guard = entries.iter().all(|entry| {
        if entry.preflight_route
            == ToolInvocationLedgerApprovalPreflightRoute::ApprovalLedgerPreflightRequired
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
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
    let tool_invocation_ledger_approval_preflight_ready = shadow.router_lookup_shadow_ready
        && !input.ledger_write_switch_enabled
        && !input.approval_request_switch_enabled
        && all_shadow_entries_bound_to_ledger_approval_preflight
        && all_ledger_approval_entries_keep_approval_guard;

    ToolInvocationLedgerApprovalPreflightPlan {
        runtime: "hepta",
        surface: "tool_invocation_ledger_approval_preflight",
        plugin_id: shadow.plugin_id,
        status: if tool_invocation_ledger_approval_preflight_ready {
            "ready"
        } else {
            "blocked"
        },
        source_router_lookup_shadow_surface: shadow.surface,
        source_router_lookup_shadow_ready: shadow.router_lookup_shadow_ready,
        tool_invocation_ledger_binding_present: input.tool_invocation_ledger_binding_present,
        approval_broker_preflight_binding_present: input.approval_broker_preflight_binding_present,
        ledger_write_switch_enabled: input.ledger_write_switch_enabled,
        approval_request_switch_enabled: input.approval_request_switch_enabled,
        candidate_count: entries.len(),
        ledger_approval_preflight_ready_count,
        ledger_approval_preflight_blocked_count,
        approval_ledger_preflight_required_count,
        all_shadow_entries_bound_to_ledger_approval_preflight,
        all_ledger_approval_entries_keep_approval_guard,
        tool_invocation_ledger_approval_preflight_ready,
        ledger_approval_preflight_allowed: tool_invocation_ledger_approval_preflight_ready
            && ledger_approval_preflight_ready_count == entries.len()
            && approval_ledger_preflight_required_count == entries.len(),
        router_registration_lookup_enabled: false,
        registry_lookup_executed: false,
        registry_source_of_truth_enabled: false,
        tool_registration_enabled: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_execution_adapter_preflight_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_invocation_ledger_approval_preflight_binds_shadow_to_ledger_approval_plan() {
        let plan = hepta_system_tool_invocation_ledger_approval_preflight_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_router_lookup_shadow_surface,
            "tool_registry_router_lookup_shadow"
        );
        assert!(plan.source_router_lookup_shadow_ready);
        assert!(plan.tool_invocation_ledger_binding_present);
        assert!(plan.approval_broker_preflight_binding_present);
        assert!(!plan.ledger_write_switch_enabled);
        assert!(!plan.approval_request_switch_enabled);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.ledger_approval_preflight_ready_count, 2);
        assert_eq!(plan.ledger_approval_preflight_blocked_count, 0);
        assert_eq!(plan.approval_ledger_preflight_required_count, 2);
        assert!(plan.all_shadow_entries_bound_to_ledger_approval_preflight);
        assert!(plan.all_ledger_approval_entries_keep_approval_guard);
        assert!(plan.tool_invocation_ledger_approval_preflight_ready);
        assert!(plan.ledger_approval_preflight_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.preflight_route
                == ToolInvocationLedgerApprovalPreflightRoute::ApprovalLedgerPreflightRequired
                && entry.ledger_preflight_ready
                && entry.approval_preflight_required
                && entry.registry_guard_route
                    == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
        }));
    }

    #[test]
    fn tool_invocation_ledger_approval_preflight_does_not_write_or_request() {
        let plan = hepta_system_tool_invocation_ledger_approval_preflight_plan();

        assert!(plan.tool_invocation_ledger_approval_preflight_ready);
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
            !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        }));
    }

    #[test]
    fn tool_invocation_ledger_approval_preflight_fails_closed_without_ledger_binding() {
        let shadow = hepta_system_tool_registry_router_lookup_shadow_plan();
        let input = ToolInvocationLedgerApprovalPreflightInput {
            tool_invocation_ledger_binding_present: false,
            approval_broker_preflight_binding_present: true,
            ledger_write_switch_enabled: false,
            approval_request_switch_enabled: false,
        };

        let plan = tool_invocation_ledger_approval_preflight_plan(&shadow, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.tool_invocation_ledger_binding_present);
        assert_eq!(plan.ledger_approval_preflight_ready_count, 0);
        assert_eq!(plan.ledger_approval_preflight_blocked_count, 2);
        assert!(!plan.tool_invocation_ledger_approval_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.preflight_route
                == ToolInvocationLedgerApprovalPreflightRoute::BlockedByMissingLedgerBinding
        }));
    }

    #[test]
    fn tool_invocation_ledger_approval_preflight_fails_closed_without_approval_broker_binding() {
        let shadow = hepta_system_tool_registry_router_lookup_shadow_plan();
        let input = ToolInvocationLedgerApprovalPreflightInput {
            tool_invocation_ledger_binding_present: true,
            approval_broker_preflight_binding_present: false,
            ledger_write_switch_enabled: false,
            approval_request_switch_enabled: false,
        };

        let plan = tool_invocation_ledger_approval_preflight_plan(&shadow, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.approval_broker_preflight_binding_present);
        assert_eq!(plan.ledger_approval_preflight_ready_count, 0);
        assert_eq!(plan.ledger_approval_preflight_blocked_count, 2);
        assert!(!plan.tool_invocation_ledger_approval_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.preflight_route
                == ToolInvocationLedgerApprovalPreflightRoute::BlockedByMissingApprovalBrokerBinding
        }));
    }

    #[test]
    fn tool_invocation_ledger_approval_preflight_fails_closed_when_write_or_request_switch_enabled()
    {
        let shadow = hepta_system_tool_registry_router_lookup_shadow_plan();
        let input = ToolInvocationLedgerApprovalPreflightInput {
            tool_invocation_ledger_binding_present: true,
            approval_broker_preflight_binding_present: true,
            ledger_write_switch_enabled: true,
            approval_request_switch_enabled: true,
        };

        let plan = tool_invocation_ledger_approval_preflight_plan(&shadow, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.ledger_write_switch_enabled);
        assert!(plan.approval_request_switch_enabled);
        assert_eq!(plan.ledger_approval_preflight_ready_count, 0);
        assert_eq!(plan.ledger_approval_preflight_blocked_count, 2);
        assert!(!plan.tool_invocation_ledger_approval_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.preflight_route
                == ToolInvocationLedgerApprovalPreflightRoute::BlockedByEnabledExecutionSwitch
        }));
    }

    #[test]
    fn tool_invocation_ledger_approval_preflight_fails_closed_without_router_shadow() {
        let mut shadow = hepta_system_tool_registry_router_lookup_shadow_plan();
        shadow.router_lookup_shadow_ready = false;
        shadow.shadow_ready_count = 0;
        shadow.shadow_blocked_count = 2;
        for entry in &mut shadow.entries {
            entry.shadow_ready = false;
            entry.shadow_route = ToolRegistryRouterLookupShadowRoute::BlockedByLookupPrecondition;
        }

        let plan = tool_invocation_ledger_approval_preflight_plan(
            &shadow,
            &ToolInvocationLedgerApprovalPreflightInput::default(),
        );

        assert_eq!(plan.status, "blocked");
        assert_eq!(plan.ledger_approval_preflight_ready_count, 0);
        assert_eq!(plan.ledger_approval_preflight_blocked_count, 2);
        assert!(!plan.tool_invocation_ledger_approval_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.preflight_route
                == ToolInvocationLedgerApprovalPreflightRoute::BlockedByRouterLookupShadow
        }));
    }
}
