use crate::ToolExecutionDispatchShadowPlan;
use crate::ToolExecutionDispatchShadowRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_dispatch_shadow_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionCutoverPreflightRoute {
    CutoverPreflightBlockedUntilExplicitApproval,
    BlockedByEnabledExecutionSwitch,
    BlockedByMissingCutoverMatrixBinding,
    BlockedByExecutionDispatchShadow,
    BlockedByRegistryGuard,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionCutoverPreflightInput {
    pub cutover_matrix_binding_present: bool,
    pub explicit_cutover_approval_present: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
}

impl Default for ToolExecutionCutoverPreflightInput {
    fn default() -> Self {
        Self {
            cutover_matrix_binding_present: true,
            explicit_cutover_approval_present: false,
            tool_invocation_execution_switch_enabled: false,
            adapter_dispatch_switch_enabled: false,
            live_cutover_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionCutoverPreflightEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_dispatch_shadow_route: ToolExecutionDispatchShadowRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub cutover_preflight_route: ToolExecutionCutoverPreflightRoute,
    pub cutover_preflight_ready: bool,
    pub explicit_cutover_approval_required: bool,
    pub live_cutover_blocked: bool,
    pub cutover_matrix_binding_present: bool,
    pub explicit_cutover_approval_present: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub result_receipt_write_enabled: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionCutoverPreflightPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_execution_dispatch_shadow_surface: &'static str,
    pub source_execution_dispatch_shadow_ready: bool,
    pub cutover_matrix_binding_present: bool,
    pub explicit_cutover_approval_present: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
    pub candidate_count: usize,
    pub cutover_preflight_ready_count: usize,
    pub cutover_preflight_blocked_count: usize,
    pub explicit_cutover_approval_required_count: usize,
    pub live_cutover_blocked_count: usize,
    pub all_dispatch_shadow_entries_bound_to_cutover_preflight: bool,
    pub all_cutover_entries_keep_approval_guard: bool,
    pub tool_execution_cutover_preflight_ready: bool,
    pub tool_execution_live_cutover_allowed: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub result_receipt_written: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<ToolExecutionCutoverPreflightEntry>,
}

pub fn hepta_system_tool_execution_cutover_preflight_plan() -> ToolExecutionCutoverPreflightPlan {
    let shadow = hepta_system_tool_execution_dispatch_shadow_plan();
    tool_execution_cutover_preflight_plan(&shadow, &ToolExecutionCutoverPreflightInput::default())
}

pub fn tool_execution_cutover_preflight_plan(
    shadow: &ToolExecutionDispatchShadowPlan,
    input: &ToolExecutionCutoverPreflightInput,
) -> ToolExecutionCutoverPreflightPlan {
    let entries = shadow
        .entries
        .iter()
        .map(|entry| {
            let cutover_preflight_route = if input.tool_invocation_execution_switch_enabled
                || input.adapter_dispatch_switch_enabled
                || input.live_cutover_switch_enabled
            {
                ToolExecutionCutoverPreflightRoute::BlockedByEnabledExecutionSwitch
            } else if !input.cutover_matrix_binding_present {
                ToolExecutionCutoverPreflightRoute::BlockedByMissingCutoverMatrixBinding
            } else if !entry.dispatch_shadow_ready
                || entry.dispatch_shadow_route
                    != ToolExecutionDispatchShadowRoute::DisabledExecutionDispatchShadow
            {
                ToolExecutionCutoverPreflightRoute::BlockedByExecutionDispatchShadow
            } else if entry.registry_guard_route
                != ToolRegistryInvocationGuardRoute::RequireApprovalLedger
            {
                ToolExecutionCutoverPreflightRoute::BlockedByRegistryGuard
            } else {
                ToolExecutionCutoverPreflightRoute::CutoverPreflightBlockedUntilExplicitApproval
            };
            let explicit_cutover_approval_required = cutover_preflight_route
                == ToolExecutionCutoverPreflightRoute::CutoverPreflightBlockedUntilExplicitApproval
                && !input.explicit_cutover_approval_present;
            let live_cutover_blocked = cutover_preflight_route
                == ToolExecutionCutoverPreflightRoute::CutoverPreflightBlockedUntilExplicitApproval
                && explicit_cutover_approval_required
                && !input.live_cutover_switch_enabled;
            let cutover_preflight_ready = live_cutover_blocked
                && !entry.tool_invocation_execution_switch_enabled
                && !entry.adapter_dispatch_switch_enabled
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.result_receipt_write_enabled;

            ToolExecutionCutoverPreflightEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_dispatch_shadow_route: entry.dispatch_shadow_route,
                registry_guard_route: entry.registry_guard_route,
                cutover_preflight_route,
                cutover_preflight_ready,
                explicit_cutover_approval_required,
                live_cutover_blocked,
                cutover_matrix_binding_present: input.cutover_matrix_binding_present,
                explicit_cutover_approval_present: input.explicit_cutover_approval_present,
                tool_invocation_execution_switch_enabled: input
                    .tool_invocation_execution_switch_enabled,
                adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
                live_cutover_switch_enabled: input.live_cutover_switch_enabled,
                router_registration_lookup_enabled: false,
                registry_lookup_executed: false,
                registry_source_of_truth_enabled: false,
                tool_registration_enabled: false,
                tool_invocation_enabled: false,
                ledger_write_enabled: false,
                approval_request_enabled: false,
                result_receipt_write_enabled: false,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let cutover_preflight_ready_count = entries
        .iter()
        .filter(|entry| entry.cutover_preflight_ready)
        .count();
    let explicit_cutover_approval_required_count = entries
        .iter()
        .filter(|entry| entry.explicit_cutover_approval_required)
        .count();
    let live_cutover_blocked_count = entries
        .iter()
        .filter(|entry| entry.live_cutover_blocked)
        .count();
    let cutover_preflight_blocked_count = entries.len() - cutover_preflight_ready_count;
    let all_dispatch_shadow_entries_bound_to_cutover_preflight = input
        .cutover_matrix_binding_present
        && cutover_preflight_ready_count == entries.len()
        && explicit_cutover_approval_required_count == entries.len()
        && live_cutover_blocked_count == entries.len();
    let all_cutover_entries_keep_approval_guard = entries.iter().all(|entry| {
        if entry.cutover_preflight_route
            == ToolExecutionCutoverPreflightRoute::CutoverPreflightBlockedUntilExplicitApproval
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.explicit_cutover_approval_present
                && !entry.tool_invocation_execution_switch_enabled
                && !entry.adapter_dispatch_switch_enabled
                && !entry.live_cutover_switch_enabled
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.result_receipt_write_enabled
        } else {
            true
        }
    });
    let tool_execution_cutover_preflight_ready = shadow.tool_execution_dispatch_shadow_ready
        && !input.explicit_cutover_approval_present
        && !input.tool_invocation_execution_switch_enabled
        && !input.adapter_dispatch_switch_enabled
        && !input.live_cutover_switch_enabled
        && all_dispatch_shadow_entries_bound_to_cutover_preflight
        && all_cutover_entries_keep_approval_guard;

    ToolExecutionCutoverPreflightPlan {
        runtime: "hepta",
        surface: "tool_execution_cutover_preflight",
        plugin_id: shadow.plugin_id,
        status: if tool_execution_cutover_preflight_ready {
            "ready"
        } else {
            "blocked"
        },
        source_execution_dispatch_shadow_surface: shadow.surface,
        source_execution_dispatch_shadow_ready: shadow.tool_execution_dispatch_shadow_ready,
        cutover_matrix_binding_present: input.cutover_matrix_binding_present,
        explicit_cutover_approval_present: input.explicit_cutover_approval_present,
        tool_invocation_execution_switch_enabled: input.tool_invocation_execution_switch_enabled,
        adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        candidate_count: entries.len(),
        cutover_preflight_ready_count,
        cutover_preflight_blocked_count,
        explicit_cutover_approval_required_count,
        live_cutover_blocked_count,
        all_dispatch_shadow_entries_bound_to_cutover_preflight,
        all_cutover_entries_keep_approval_guard,
        tool_execution_cutover_preflight_ready,
        tool_execution_live_cutover_allowed: false,
        router_registration_lookup_enabled: false,
        registry_lookup_executed: false,
        registry_source_of_truth_enabled: false,
        tool_registration_enabled: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        result_receipt_written: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_execution_operator_approval_receipt_projection_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_execution_cutover_preflight_collects_cutover_blockers() {
        let plan = hepta_system_tool_execution_cutover_preflight_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_execution_dispatch_shadow_surface,
            "tool_execution_dispatch_shadow"
        );
        assert!(plan.source_execution_dispatch_shadow_ready);
        assert!(plan.cutover_matrix_binding_present);
        assert!(!plan.explicit_cutover_approval_present);
        assert!(!plan.tool_invocation_execution_switch_enabled);
        assert!(!plan.adapter_dispatch_switch_enabled);
        assert!(!plan.live_cutover_switch_enabled);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.cutover_preflight_ready_count, 2);
        assert_eq!(plan.cutover_preflight_blocked_count, 0);
        assert_eq!(plan.explicit_cutover_approval_required_count, 2);
        assert_eq!(plan.live_cutover_blocked_count, 2);
        assert!(plan.all_dispatch_shadow_entries_bound_to_cutover_preflight);
        assert!(plan.all_cutover_entries_keep_approval_guard);
        assert!(plan.tool_execution_cutover_preflight_ready);
        assert!(!plan.tool_execution_live_cutover_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.cutover_preflight_route
                == ToolExecutionCutoverPreflightRoute::CutoverPreflightBlockedUntilExplicitApproval
                && entry.cutover_preflight_ready
                && entry.explicit_cutover_approval_required
                && entry.live_cutover_blocked
        }));
    }

    #[test]
    fn tool_execution_cutover_preflight_does_not_enable_live_execution() {
        let plan = hepta_system_tool_execution_cutover_preflight_plan();

        assert!(plan.tool_execution_cutover_preflight_ready);
        assert!(!plan.tool_execution_live_cutover_allowed);
        assert!(!plan.router_registration_lookup_enabled);
        assert!(!plan.registry_lookup_executed);
        assert!(!plan.registry_source_of_truth_enabled);
        assert!(!plan.tool_registration_enabled);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.result_receipt_written);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
        assert!(plan.entries.iter().all(|entry| {
            !entry.explicit_cutover_approval_present
                && !entry.tool_invocation_execution_switch_enabled
                && !entry.adapter_dispatch_switch_enabled
                && !entry.live_cutover_switch_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.result_receipt_write_enabled
        }));
    }

    #[test]
    fn tool_execution_cutover_preflight_fails_closed_without_matrix_binding() {
        let shadow = hepta_system_tool_execution_dispatch_shadow_plan();
        let input = ToolExecutionCutoverPreflightInput {
            cutover_matrix_binding_present: false,
            explicit_cutover_approval_present: false,
            tool_invocation_execution_switch_enabled: false,
            adapter_dispatch_switch_enabled: false,
            live_cutover_switch_enabled: false,
        };

        let plan = tool_execution_cutover_preflight_plan(&shadow, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.cutover_matrix_binding_present);
        assert_eq!(plan.cutover_preflight_ready_count, 0);
        assert_eq!(plan.cutover_preflight_blocked_count, 2);
        assert!(!plan.tool_execution_cutover_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.cutover_preflight_route
                == ToolExecutionCutoverPreflightRoute::BlockedByMissingCutoverMatrixBinding
        }));
    }

    #[test]
    fn tool_execution_cutover_preflight_fails_closed_when_execution_switch_enabled() {
        let shadow = hepta_system_tool_execution_dispatch_shadow_plan();
        let input = ToolExecutionCutoverPreflightInput {
            cutover_matrix_binding_present: true,
            explicit_cutover_approval_present: false,
            tool_invocation_execution_switch_enabled: true,
            adapter_dispatch_switch_enabled: true,
            live_cutover_switch_enabled: true,
        };

        let plan = tool_execution_cutover_preflight_plan(&shadow, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.tool_invocation_execution_switch_enabled);
        assert!(plan.adapter_dispatch_switch_enabled);
        assert!(plan.live_cutover_switch_enabled);
        assert_eq!(plan.cutover_preflight_ready_count, 0);
        assert_eq!(plan.cutover_preflight_blocked_count, 2);
        assert!(!plan.tool_execution_cutover_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.cutover_preflight_route
                == ToolExecutionCutoverPreflightRoute::BlockedByEnabledExecutionSwitch
        }));
    }

    #[test]
    fn tool_execution_cutover_preflight_fails_closed_without_dispatch_shadow() {
        let mut shadow = hepta_system_tool_execution_dispatch_shadow_plan();
        shadow.tool_execution_dispatch_shadow_ready = false;
        shadow.dispatch_shadow_ready_count = 0;
        shadow.dispatch_shadow_blocked_count = 2;
        for entry in &mut shadow.entries {
            entry.dispatch_shadow_ready = false;
            entry.dispatch_shadow_route =
                ToolExecutionDispatchShadowRoute::BlockedByExecutionAdapterPreflight;
        }

        let plan = tool_execution_cutover_preflight_plan(
            &shadow,
            &ToolExecutionCutoverPreflightInput::default(),
        );

        assert_eq!(plan.status, "blocked");
        assert_eq!(plan.cutover_preflight_ready_count, 0);
        assert_eq!(plan.cutover_preflight_blocked_count, 2);
        assert!(!plan.tool_execution_cutover_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.cutover_preflight_route
                == ToolExecutionCutoverPreflightRoute::BlockedByExecutionDispatchShadow
        }));
    }
}
