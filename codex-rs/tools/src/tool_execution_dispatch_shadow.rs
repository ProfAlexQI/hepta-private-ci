use crate::ShadowLookupRoute;
use crate::ToolExecutionAdapterPreflightPlan;
use crate::ToolExecutionAdapterPreflightRoute;
use crate::ToolInvocationLedgerRehearsalRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::ToolRegistryShadowPipelinePlan;
use crate::hepta_system_tool_execution_adapter_preflight_plan;
use crate::hepta_system_tool_registry_shadow_pipeline_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionDispatchShadowRoute {
    DisabledExecutionDispatchShadow,
    BlockedByEnabledDispatchSwitch,
    BlockedByMissingShadowBinding,
    BlockedByExecutionAdapterPreflight,
    BlockedByRegistryGuard,
    BlockedByRegistryShadowPipeline,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionDispatchShadowInput {
    pub dispatch_shadow_binding_present: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
}

impl Default for ToolExecutionDispatchShadowInput {
    fn default() -> Self {
        Self {
            dispatch_shadow_binding_present: true,
            tool_invocation_execution_switch_enabled: false,
            adapter_dispatch_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionDispatchShadowEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_adapter_preflight_route: ToolExecutionAdapterPreflightRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub registry_shadow_lookup_route: Option<ShadowLookupRoute>,
    pub registry_shadow_ledger_rehearsal_route: Option<ToolInvocationLedgerRehearsalRoute>,
    pub dispatch_shadow_route: ToolExecutionDispatchShadowRoute,
    pub dispatch_shadow_ready: bool,
    pub execution_adapter_preflight_ready: bool,
    pub registry_shadow_pipeline_ready: bool,
    pub registry_shadow_receipt_rehearsed: bool,
    pub dispatch_shadow_binding_present: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
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
pub struct ToolExecutionDispatchShadowPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_execution_adapter_preflight_surface: &'static str,
    pub source_execution_adapter_preflight_ready: bool,
    pub source_registry_shadow_pipeline_surface: &'static str,
    pub source_registry_shadow_pipeline_ready: bool,
    pub dispatch_shadow_binding_present: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub candidate_count: usize,
    pub dispatch_shadow_ready_count: usize,
    pub dispatch_shadow_blocked_count: usize,
    pub disabled_execution_dispatch_shadow_count: usize,
    pub registry_shadow_pipeline_ready_count: usize,
    pub registry_shadow_pipeline_blocked_count: usize,
    pub registry_shadow_pipeline_receipt_count: usize,
    pub all_execution_adapter_preflight_entries_shadowed: bool,
    pub all_dispatch_entries_bound_to_registry_shadow_pipeline: bool,
    pub all_dispatch_shadow_entries_keep_approval_guard: bool,
    pub tool_execution_dispatch_shadow_ready: bool,
    pub execution_dispatch_shadow_allowed: bool,
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
    pub entries: Vec<ToolExecutionDispatchShadowEntry>,
}

pub fn hepta_system_tool_execution_dispatch_shadow_plan() -> ToolExecutionDispatchShadowPlan {
    let preflight = hepta_system_tool_execution_adapter_preflight_plan();
    let registry_pipeline = hepta_system_tool_registry_shadow_pipeline_plan();
    tool_execution_dispatch_shadow_plan_with_registry_shadow_pipeline(
        &preflight,
        &registry_pipeline,
        &ToolExecutionDispatchShadowInput::default(),
    )
}

pub fn tool_execution_dispatch_shadow_plan(
    preflight: &ToolExecutionAdapterPreflightPlan,
    input: &ToolExecutionDispatchShadowInput,
) -> ToolExecutionDispatchShadowPlan {
    let registry_pipeline = hepta_system_tool_registry_shadow_pipeline_plan();
    tool_execution_dispatch_shadow_plan_with_registry_shadow_pipeline(
        preflight,
        &registry_pipeline,
        input,
    )
}

pub fn tool_execution_dispatch_shadow_plan_with_registry_shadow_pipeline(
    preflight: &ToolExecutionAdapterPreflightPlan,
    registry_pipeline: &ToolRegistryShadowPipelinePlan,
    input: &ToolExecutionDispatchShadowInput,
) -> ToolExecutionDispatchShadowPlan {
    tool_execution_dispatch_shadow_plan_inner(preflight, registry_pipeline, input)
}

fn tool_execution_dispatch_shadow_plan_inner(
    preflight: &ToolExecutionAdapterPreflightPlan,
    registry_pipeline: &ToolRegistryShadowPipelinePlan,
    input: &ToolExecutionDispatchShadowInput,
) -> ToolExecutionDispatchShadowPlan {
    let entries = preflight
        .entries
        .iter()
        .map(|entry| {
            let registry_pipeline_entry = registry_pipeline
                .entries
                .iter()
                .find(|pipeline_entry| pipeline_entry.candidate_tool_id == entry.candidate_tool_id);
            let registry_shadow_lookup_route =
                registry_pipeline_entry.map(|entry| entry.shadow_lookup_route);
            let registry_shadow_ledger_rehearsal_route =
                registry_pipeline_entry.and_then(|entry| entry.ledger_rehearsal_route);
            let registry_shadow_receipt_rehearsed = registry_pipeline_entry.is_some_and(|entry| {
                entry.receipt_id.is_some() && entry.ledger_rehearsal_attempted
            });
            let registry_shadow_pipeline_ready = registry_pipeline_entry.is_some_and(|entry| {
                registry_pipeline.shadow_pipeline_ready
                    && entry.dispatch_preflight_ready
                    && entry.shadow_lookup_route == ShadowLookupRoute::RequiresApprovalLedger
                    && entry.ledger_rehearsal_route
                        == Some(
                            ToolInvocationLedgerRehearsalRoute::RehearsedApprovalRequiredReceipt,
                        )
                    && entry.receipt_id.is_some()
                    && entry.approval_required
                    && !entry.registry_registered_live
                    && !entry.registry_lookup_executed_live
                    && !entry.tool_invoked
                    && !entry.ledger_persisted
                    && !entry.receipt_persisted
                    && entry.side_effect_free
            });
            let dispatch_shadow_route = if input.tool_invocation_execution_switch_enabled
                || input.adapter_dispatch_switch_enabled
            {
                ToolExecutionDispatchShadowRoute::BlockedByEnabledDispatchSwitch
            } else if !input.dispatch_shadow_binding_present {
                ToolExecutionDispatchShadowRoute::BlockedByMissingShadowBinding
            } else if !entry.execution_adapter_preflight_ready
                || entry.adapter_preflight_route
                    != ToolExecutionAdapterPreflightRoute::DisabledExecutionAdapterPreflight
            {
                ToolExecutionDispatchShadowRoute::BlockedByExecutionAdapterPreflight
            } else if entry.registry_guard_route
                != ToolRegistryInvocationGuardRoute::RequireApprovalLedger
            {
                ToolExecutionDispatchShadowRoute::BlockedByRegistryGuard
            } else if !registry_shadow_pipeline_ready {
                ToolExecutionDispatchShadowRoute::BlockedByRegistryShadowPipeline
            } else {
                ToolExecutionDispatchShadowRoute::DisabledExecutionDispatchShadow
            };
            let dispatch_shadow_ready = dispatch_shadow_route
                == ToolExecutionDispatchShadowRoute::DisabledExecutionDispatchShadow
                && !entry.tool_invocation_execution_switch_enabled
                && !entry.adapter_dispatch_switch_enabled
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.result_receipt_write_enabled
                && !input.tool_invocation_execution_switch_enabled
                && !input.adapter_dispatch_switch_enabled
                && registry_shadow_pipeline_ready
                && registry_shadow_receipt_rehearsed;

            ToolExecutionDispatchShadowEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_adapter_preflight_route: entry.adapter_preflight_route,
                registry_guard_route: entry.registry_guard_route,
                registry_shadow_lookup_route,
                registry_shadow_ledger_rehearsal_route,
                dispatch_shadow_route,
                dispatch_shadow_ready,
                execution_adapter_preflight_ready: entry.execution_adapter_preflight_ready,
                registry_shadow_pipeline_ready,
                registry_shadow_receipt_rehearsed,
                dispatch_shadow_binding_present: input.dispatch_shadow_binding_present,
                tool_invocation_execution_switch_enabled: input
                    .tool_invocation_execution_switch_enabled,
                adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
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

    let dispatch_shadow_ready_count = entries
        .iter()
        .filter(|entry| entry.dispatch_shadow_ready)
        .count();
    let disabled_execution_dispatch_shadow_count = entries
        .iter()
        .filter(|entry| {
            entry.dispatch_shadow_route
                == ToolExecutionDispatchShadowRoute::DisabledExecutionDispatchShadow
        })
        .count();
    let registry_shadow_pipeline_ready_count = entries
        .iter()
        .filter(|entry| entry.registry_shadow_pipeline_ready)
        .count();
    let registry_shadow_pipeline_receipt_count = entries
        .iter()
        .filter(|entry| entry.registry_shadow_receipt_rehearsed)
        .count();
    let registry_shadow_pipeline_blocked_count =
        entries.len() - registry_shadow_pipeline_ready_count;
    let dispatch_shadow_blocked_count = entries.len() - dispatch_shadow_ready_count;
    let all_execution_adapter_preflight_entries_shadowed = input.dispatch_shadow_binding_present
        && dispatch_shadow_ready_count == entries.len()
        && disabled_execution_dispatch_shadow_count == entries.len();
    let all_dispatch_entries_bound_to_registry_shadow_pipeline = registry_pipeline
        .shadow_pipeline_ready
        && registry_shadow_pipeline_ready_count == entries.len()
        && registry_shadow_pipeline_receipt_count == entries.len();
    let all_dispatch_shadow_entries_keep_approval_guard = entries.iter().all(|entry| {
        if entry.dispatch_shadow_route
            == ToolExecutionDispatchShadowRoute::DisabledExecutionDispatchShadow
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.tool_invocation_execution_switch_enabled
                && !entry.adapter_dispatch_switch_enabled
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.result_receipt_write_enabled
                && entry.registry_shadow_pipeline_ready
                && entry.registry_shadow_receipt_rehearsed
        } else {
            true
        }
    });
    let tool_execution_dispatch_shadow_ready = preflight.tool_execution_adapter_preflight_ready
        && !input.tool_invocation_execution_switch_enabled
        && !input.adapter_dispatch_switch_enabled
        && registry_pipeline.shadow_pipeline_ready
        && all_execution_adapter_preflight_entries_shadowed
        && all_dispatch_entries_bound_to_registry_shadow_pipeline
        && all_dispatch_shadow_entries_keep_approval_guard;

    ToolExecutionDispatchShadowPlan {
        runtime: "hepta",
        surface: "tool_execution_dispatch_shadow",
        plugin_id: preflight.plugin_id,
        status: if tool_execution_dispatch_shadow_ready {
            "ready"
        } else {
            "blocked"
        },
        source_execution_adapter_preflight_surface: preflight.surface,
        source_execution_adapter_preflight_ready: preflight.tool_execution_adapter_preflight_ready,
        source_registry_shadow_pipeline_surface: registry_pipeline.surface,
        source_registry_shadow_pipeline_ready: registry_pipeline.shadow_pipeline_ready,
        dispatch_shadow_binding_present: input.dispatch_shadow_binding_present,
        tool_invocation_execution_switch_enabled: input.tool_invocation_execution_switch_enabled,
        adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
        candidate_count: entries.len(),
        dispatch_shadow_ready_count,
        dispatch_shadow_blocked_count,
        disabled_execution_dispatch_shadow_count,
        registry_shadow_pipeline_ready_count,
        registry_shadow_pipeline_blocked_count,
        registry_shadow_pipeline_receipt_count,
        all_execution_adapter_preflight_entries_shadowed,
        all_dispatch_entries_bound_to_registry_shadow_pipeline,
        all_dispatch_shadow_entries_keep_approval_guard,
        tool_execution_dispatch_shadow_ready,
        execution_dispatch_shadow_allowed: tool_execution_dispatch_shadow_ready
            && dispatch_shadow_ready_count == entries.len()
            && disabled_execution_dispatch_shadow_count == entries.len(),
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
        next_migration_step: "restore_tool_execution_operator_approval_packet_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_system_plugin_tool_contribution_inventory_preview_plan;
    use crate::hepta_system_tool_registry_read_only_dispatch_preflight_plan;
    use crate::tool_registry_shadow_pipeline_plan;

    #[test]
    fn tool_execution_dispatch_shadow_binds_disabled_adapter_routes() {
        let plan = hepta_system_tool_execution_dispatch_shadow_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_execution_adapter_preflight_surface,
            "tool_execution_adapter_preflight"
        );
        assert!(plan.source_execution_adapter_preflight_ready);
        assert_eq!(
            plan.source_registry_shadow_pipeline_surface,
            "tool_registry_shadow_pipeline"
        );
        assert!(plan.source_registry_shadow_pipeline_ready);
        assert!(plan.dispatch_shadow_binding_present);
        assert!(!plan.tool_invocation_execution_switch_enabled);
        assert!(!plan.adapter_dispatch_switch_enabled);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.dispatch_shadow_ready_count, 2);
        assert_eq!(plan.dispatch_shadow_blocked_count, 0);
        assert_eq!(plan.disabled_execution_dispatch_shadow_count, 2);
        assert_eq!(plan.registry_shadow_pipeline_ready_count, 2);
        assert_eq!(plan.registry_shadow_pipeline_blocked_count, 0);
        assert_eq!(plan.registry_shadow_pipeline_receipt_count, 2);
        assert!(plan.all_execution_adapter_preflight_entries_shadowed);
        assert!(plan.all_dispatch_entries_bound_to_registry_shadow_pipeline);
        assert!(plan.all_dispatch_shadow_entries_keep_approval_guard);
        assert!(plan.tool_execution_dispatch_shadow_ready);
        assert!(plan.execution_dispatch_shadow_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.dispatch_shadow_route
                == ToolExecutionDispatchShadowRoute::DisabledExecutionDispatchShadow
                && entry.dispatch_shadow_ready
                && entry.execution_adapter_preflight_ready
                && entry.registry_shadow_pipeline_ready
                && entry.registry_shadow_receipt_rehearsed
                && entry.registry_shadow_lookup_route
                    == Some(ShadowLookupRoute::RequiresApprovalLedger)
                && entry.registry_shadow_ledger_rehearsal_route
                    == Some(ToolInvocationLedgerRehearsalRoute::RehearsedApprovalRequiredReceipt)
        }));
    }

    #[test]
    fn tool_execution_dispatch_shadow_does_not_dispatch_or_invoke() {
        let plan = hepta_system_tool_execution_dispatch_shadow_plan();

        assert!(plan.tool_execution_dispatch_shadow_ready);
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
            !entry.tool_invocation_execution_switch_enabled
                && !entry.adapter_dispatch_switch_enabled
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.result_receipt_write_enabled
        }));
    }

    #[test]
    fn tool_execution_dispatch_shadow_fails_closed_when_registry_pipeline_blocks() {
        let mut contributions = hepta_system_plugin_tool_contribution_inventory_preview_plan();
        let duplicate = contributions.candidate_inventory_entries[0].clone();
        contributions.candidate_inventory_entries.push(duplicate);
        let dispatch = hepta_system_tool_registry_read_only_dispatch_preflight_plan();
        let registry_pipeline = tool_registry_shadow_pipeline_plan(&contributions, &dispatch);
        let preflight = hepta_system_tool_execution_adapter_preflight_plan();

        let plan = tool_execution_dispatch_shadow_plan_with_registry_shadow_pipeline(
            &preflight,
            &registry_pipeline,
            &ToolExecutionDispatchShadowInput::default(),
        );

        assert_eq!(registry_pipeline.status, "blocked");
        assert_eq!(plan.status, "blocked");
        assert!(!plan.source_registry_shadow_pipeline_ready);
        assert_eq!(plan.dispatch_shadow_ready_count, 0);
        assert_eq!(plan.dispatch_shadow_blocked_count, 2);
        assert_eq!(plan.disabled_execution_dispatch_shadow_count, 0);
        assert_eq!(plan.registry_shadow_pipeline_ready_count, 0);
        assert_eq!(plan.registry_shadow_pipeline_blocked_count, 2);
        assert_eq!(plan.registry_shadow_pipeline_receipt_count, 2);
        assert!(!plan.all_dispatch_entries_bound_to_registry_shadow_pipeline);
        assert!(!plan.tool_execution_dispatch_shadow_ready);
        assert!(!plan.execution_dispatch_shadow_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.dispatch_shadow_route
                == ToolExecutionDispatchShadowRoute::BlockedByRegistryShadowPipeline
                && !entry.dispatch_shadow_ready
                && !entry.registry_shadow_pipeline_ready
                && entry.registry_shadow_receipt_rehearsed
                && entry.registry_shadow_lookup_route
                    == Some(ShadowLookupRoute::RequiresApprovalLedger)
                && entry.registry_shadow_ledger_rehearsal_route
                    == Some(ToolInvocationLedgerRehearsalRoute::RehearsedApprovalRequiredReceipt)
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.result_receipt_write_enabled
        }));
    }

    #[test]
    fn tool_execution_dispatch_shadow_fails_closed_without_shadow_binding() {
        let preflight = hepta_system_tool_execution_adapter_preflight_plan();
        let input = ToolExecutionDispatchShadowInput {
            dispatch_shadow_binding_present: false,
            tool_invocation_execution_switch_enabled: false,
            adapter_dispatch_switch_enabled: false,
        };

        let plan = tool_execution_dispatch_shadow_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.dispatch_shadow_binding_present);
        assert_eq!(plan.dispatch_shadow_ready_count, 0);
        assert_eq!(plan.dispatch_shadow_blocked_count, 2);
        assert!(!plan.tool_execution_dispatch_shadow_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.dispatch_shadow_route
                == ToolExecutionDispatchShadowRoute::BlockedByMissingShadowBinding
        }));
    }

    #[test]
    fn tool_execution_dispatch_shadow_fails_closed_when_dispatch_switch_enabled() {
        let preflight = hepta_system_tool_execution_adapter_preflight_plan();
        let input = ToolExecutionDispatchShadowInput {
            dispatch_shadow_binding_present: true,
            tool_invocation_execution_switch_enabled: true,
            adapter_dispatch_switch_enabled: true,
        };

        let plan = tool_execution_dispatch_shadow_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.tool_invocation_execution_switch_enabled);
        assert!(plan.adapter_dispatch_switch_enabled);
        assert_eq!(plan.dispatch_shadow_ready_count, 0);
        assert_eq!(plan.dispatch_shadow_blocked_count, 2);
        assert!(!plan.tool_execution_dispatch_shadow_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.dispatch_shadow_route
                == ToolExecutionDispatchShadowRoute::BlockedByEnabledDispatchSwitch
        }));
    }

    #[test]
    fn tool_execution_dispatch_shadow_fails_closed_without_adapter_preflight() {
        let mut preflight = hepta_system_tool_execution_adapter_preflight_plan();
        preflight.tool_execution_adapter_preflight_ready = false;
        preflight.execution_adapter_preflight_ready_count = 0;
        preflight.execution_adapter_preflight_blocked_count = 2;
        for entry in &mut preflight.entries {
            entry.execution_adapter_preflight_ready = false;
            entry.adapter_preflight_route =
                ToolExecutionAdapterPreflightRoute::BlockedByReceiptProjection;
        }

        let plan = tool_execution_dispatch_shadow_plan(
            &preflight,
            &ToolExecutionDispatchShadowInput::default(),
        );

        assert_eq!(plan.status, "blocked");
        assert_eq!(plan.dispatch_shadow_ready_count, 0);
        assert_eq!(plan.dispatch_shadow_blocked_count, 2);
        assert!(!plan.tool_execution_dispatch_shadow_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.dispatch_shadow_route
                == ToolExecutionDispatchShadowRoute::BlockedByExecutionAdapterPreflight
        }));
    }
}
