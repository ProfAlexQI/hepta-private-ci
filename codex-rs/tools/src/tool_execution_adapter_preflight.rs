use crate::ToolInvocationReceiptProjectionPlan;
use crate::ToolInvocationReceiptProjectionRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_invocation_receipt_projection_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionAdapterPreflightRoute {
    DisabledExecutionAdapterPreflight,
    BlockedByEnabledExecutionSwitch,
    BlockedByMissingAdapterBinding,
    BlockedByReceiptProjection,
    BlockedByRegistryGuard,
    BlockedByUnknownAdapterKind,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionAdapterPreflightInput {
    pub execution_adapter_binding_present: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
}

impl Default for ToolExecutionAdapterPreflightInput {
    fn default() -> Self {
        Self {
            execution_adapter_binding_present: true,
            tool_invocation_execution_switch_enabled: false,
            adapter_dispatch_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionAdapterPreflightEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_receipt_projection_route: ToolInvocationReceiptProjectionRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub adapter_preflight_route: ToolExecutionAdapterPreflightRoute,
    pub execution_adapter_preflight_ready: bool,
    pub receipt_projection_ready: bool,
    pub result_receipt_required: bool,
    pub readback_evidence_required: bool,
    pub execution_adapter_binding_present: bool,
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
pub struct ToolExecutionAdapterPreflightPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_receipt_projection_surface: &'static str,
    pub source_receipt_projection_ready: bool,
    pub execution_adapter_binding_present: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub candidate_count: usize,
    pub execution_adapter_preflight_ready_count: usize,
    pub execution_adapter_preflight_blocked_count: usize,
    pub disabled_execution_adapter_preflight_count: usize,
    pub mcp_tool_call_adapter_preflight_count: usize,
    pub app_connector_invocation_adapter_preflight_count: usize,
    pub all_receipt_projection_entries_bound_to_execution_adapter_preflight: bool,
    pub all_execution_adapter_entries_keep_approval_guard: bool,
    pub tool_execution_adapter_preflight_ready: bool,
    pub execution_adapter_preflight_allowed: bool,
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
    pub entries: Vec<ToolExecutionAdapterPreflightEntry>,
}

pub fn hepta_system_tool_execution_adapter_preflight_plan() -> ToolExecutionAdapterPreflightPlan {
    let projection = hepta_system_tool_invocation_receipt_projection_plan();
    tool_execution_adapter_preflight_plan(
        &projection,
        &ToolExecutionAdapterPreflightInput::default(),
    )
}

pub fn tool_execution_adapter_preflight_plan(
    projection: &ToolInvocationReceiptProjectionPlan,
    input: &ToolExecutionAdapterPreflightInput,
) -> ToolExecutionAdapterPreflightPlan {
    let entries = projection
        .entries
        .iter()
        .map(|entry| {
            let execution_adapter_kind = execution_adapter_kind_for(entry.contribution_kind);
            let adapter_preflight_route = if input.tool_invocation_execution_switch_enabled
                || input.adapter_dispatch_switch_enabled
            {
                ToolExecutionAdapterPreflightRoute::BlockedByEnabledExecutionSwitch
            } else if !input.execution_adapter_binding_present {
                ToolExecutionAdapterPreflightRoute::BlockedByMissingAdapterBinding
            } else if !entry.receipt_projection_ready
                || entry.receipt_projection_route
                    != ToolInvocationReceiptProjectionRoute::ResultReceiptProjectionRequired
            {
                ToolExecutionAdapterPreflightRoute::BlockedByReceiptProjection
            } else if entry.registry_guard_route
                != ToolRegistryInvocationGuardRoute::RequireApprovalLedger
            {
                ToolExecutionAdapterPreflightRoute::BlockedByRegistryGuard
            } else if execution_adapter_kind == "unknown_execution_adapter" {
                ToolExecutionAdapterPreflightRoute::BlockedByUnknownAdapterKind
            } else {
                ToolExecutionAdapterPreflightRoute::DisabledExecutionAdapterPreflight
            };
            let execution_adapter_preflight_ready = adapter_preflight_route
                == ToolExecutionAdapterPreflightRoute::DisabledExecutionAdapterPreflight
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.result_receipt_write_enabled
                && !input.tool_invocation_execution_switch_enabled
                && !input.adapter_dispatch_switch_enabled;

            ToolExecutionAdapterPreflightEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind,
                source_receipt_projection_route: entry.receipt_projection_route,
                registry_guard_route: entry.registry_guard_route,
                adapter_preflight_route,
                execution_adapter_preflight_ready,
                receipt_projection_ready: entry.receipt_projection_ready,
                result_receipt_required: entry.result_receipt_required,
                readback_evidence_required: entry.readback_evidence_required,
                execution_adapter_binding_present: input.execution_adapter_binding_present,
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

    let execution_adapter_preflight_ready_count = entries
        .iter()
        .filter(|entry| entry.execution_adapter_preflight_ready)
        .count();
    let disabled_execution_adapter_preflight_count = entries
        .iter()
        .filter(|entry| {
            entry.adapter_preflight_route
                == ToolExecutionAdapterPreflightRoute::DisabledExecutionAdapterPreflight
        })
        .count();
    let mcp_tool_call_adapter_preflight_count = entries
        .iter()
        .filter(|entry| entry.execution_adapter_kind == "mcp_tool_call_adapter")
        .count();
    let app_connector_invocation_adapter_preflight_count = entries
        .iter()
        .filter(|entry| entry.execution_adapter_kind == "app_connector_invocation_adapter")
        .count();
    let execution_adapter_preflight_blocked_count =
        entries.len() - execution_adapter_preflight_ready_count;
    let all_receipt_projection_entries_bound_to_execution_adapter_preflight = input
        .execution_adapter_binding_present
        && execution_adapter_preflight_ready_count == entries.len()
        && disabled_execution_adapter_preflight_count == entries.len()
        && mcp_tool_call_adapter_preflight_count == 1
        && app_connector_invocation_adapter_preflight_count == 1;
    let all_execution_adapter_entries_keep_approval_guard = entries.iter().all(|entry| {
        if entry.adapter_preflight_route
            == ToolExecutionAdapterPreflightRoute::DisabledExecutionAdapterPreflight
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
        } else {
            true
        }
    });
    let tool_execution_adapter_preflight_ready = projection
        .tool_invocation_receipt_projection_ready
        && !input.tool_invocation_execution_switch_enabled
        && !input.adapter_dispatch_switch_enabled
        && all_receipt_projection_entries_bound_to_execution_adapter_preflight
        && all_execution_adapter_entries_keep_approval_guard;

    ToolExecutionAdapterPreflightPlan {
        runtime: "hepta",
        surface: "tool_execution_adapter_preflight",
        plugin_id: projection.plugin_id,
        status: if tool_execution_adapter_preflight_ready {
            "ready"
        } else {
            "blocked"
        },
        source_receipt_projection_surface: projection.surface,
        source_receipt_projection_ready: projection.tool_invocation_receipt_projection_ready,
        execution_adapter_binding_present: input.execution_adapter_binding_present,
        tool_invocation_execution_switch_enabled: input.tool_invocation_execution_switch_enabled,
        adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
        candidate_count: entries.len(),
        execution_adapter_preflight_ready_count,
        execution_adapter_preflight_blocked_count,
        disabled_execution_adapter_preflight_count,
        mcp_tool_call_adapter_preflight_count,
        app_connector_invocation_adapter_preflight_count,
        all_receipt_projection_entries_bound_to_execution_adapter_preflight,
        all_execution_adapter_entries_keep_approval_guard,
        tool_execution_adapter_preflight_ready,
        execution_adapter_preflight_allowed: tool_execution_adapter_preflight_ready
            && execution_adapter_preflight_ready_count == entries.len()
            && disabled_execution_adapter_preflight_count == entries.len(),
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
        next_migration_step: "restore_tool_execution_cutover_preflight_without_invocation",
        entries,
    }
}

fn execution_adapter_kind_for(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "mcp_tool_call_adapter",
        "app_connector" => "app_connector_invocation_adapter",
        _ => "unknown_execution_adapter",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_execution_adapter_preflight_binds_receipts_to_disabled_adapters() {
        let plan = hepta_system_tool_execution_adapter_preflight_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_receipt_projection_surface,
            "tool_invocation_receipt_projection"
        );
        assert!(plan.source_receipt_projection_ready);
        assert!(plan.execution_adapter_binding_present);
        assert!(!plan.tool_invocation_execution_switch_enabled);
        assert!(!plan.adapter_dispatch_switch_enabled);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.execution_adapter_preflight_ready_count, 2);
        assert_eq!(plan.execution_adapter_preflight_blocked_count, 0);
        assert_eq!(plan.disabled_execution_adapter_preflight_count, 2);
        assert_eq!(plan.mcp_tool_call_adapter_preflight_count, 1);
        assert_eq!(plan.app_connector_invocation_adapter_preflight_count, 1);
        assert!(plan.all_receipt_projection_entries_bound_to_execution_adapter_preflight);
        assert!(plan.all_execution_adapter_entries_keep_approval_guard);
        assert!(plan.tool_execution_adapter_preflight_ready);
        assert!(plan.execution_adapter_preflight_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.adapter_preflight_route
                == ToolExecutionAdapterPreflightRoute::DisabledExecutionAdapterPreflight
                && entry.execution_adapter_preflight_ready
                && entry.receipt_projection_ready
                && entry.result_receipt_required
                && entry.readback_evidence_required
        }));
    }

    #[test]
    fn tool_execution_adapter_preflight_does_not_invoke_or_write_receipts() {
        let plan = hepta_system_tool_execution_adapter_preflight_plan();

        assert!(plan.tool_execution_adapter_preflight_ready);
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
    fn tool_execution_adapter_preflight_fails_closed_without_adapter_binding() {
        let projection = hepta_system_tool_invocation_receipt_projection_plan();
        let input = ToolExecutionAdapterPreflightInput {
            execution_adapter_binding_present: false,
            tool_invocation_execution_switch_enabled: false,
            adapter_dispatch_switch_enabled: false,
        };

        let plan = tool_execution_adapter_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.execution_adapter_binding_present);
        assert_eq!(plan.execution_adapter_preflight_ready_count, 0);
        assert_eq!(plan.execution_adapter_preflight_blocked_count, 2);
        assert!(!plan.tool_execution_adapter_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.adapter_preflight_route
                == ToolExecutionAdapterPreflightRoute::BlockedByMissingAdapterBinding
        }));
    }

    #[test]
    fn tool_execution_adapter_preflight_fails_closed_when_execution_switch_enabled() {
        let projection = hepta_system_tool_invocation_receipt_projection_plan();
        let input = ToolExecutionAdapterPreflightInput {
            execution_adapter_binding_present: true,
            tool_invocation_execution_switch_enabled: true,
            adapter_dispatch_switch_enabled: true,
        };

        let plan = tool_execution_adapter_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.tool_invocation_execution_switch_enabled);
        assert!(plan.adapter_dispatch_switch_enabled);
        assert_eq!(plan.execution_adapter_preflight_ready_count, 0);
        assert_eq!(plan.execution_adapter_preflight_blocked_count, 2);
        assert!(!plan.tool_execution_adapter_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.adapter_preflight_route
                == ToolExecutionAdapterPreflightRoute::BlockedByEnabledExecutionSwitch
        }));
    }

    #[test]
    fn tool_execution_adapter_preflight_fails_closed_without_receipt_projection() {
        let mut projection = hepta_system_tool_invocation_receipt_projection_plan();
        projection.tool_invocation_receipt_projection_ready = false;
        projection.receipt_projection_ready_count = 0;
        projection.receipt_projection_blocked_count = 2;
        for entry in &mut projection.entries {
            entry.receipt_projection_ready = false;
            entry.receipt_projection_route =
                ToolInvocationReceiptProjectionRoute::BlockedByLedgerApprovalPreflight;
        }

        let plan = tool_execution_adapter_preflight_plan(
            &projection,
            &ToolExecutionAdapterPreflightInput::default(),
        );

        assert_eq!(plan.status, "blocked");
        assert_eq!(plan.execution_adapter_preflight_ready_count, 0);
        assert_eq!(plan.execution_adapter_preflight_blocked_count, 2);
        assert!(!plan.tool_execution_adapter_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.adapter_preflight_route
                == ToolExecutionAdapterPreflightRoute::BlockedByReceiptProjection
        }));
    }

    #[test]
    fn tool_execution_adapter_preflight_fails_closed_for_unknown_adapter_kind() {
        let mut projection = hepta_system_tool_invocation_receipt_projection_plan();
        projection.entries[0].contribution_kind = "unknown_tool_bridge";

        let plan = tool_execution_adapter_preflight_plan(
            &projection,
            &ToolExecutionAdapterPreflightInput::default(),
        );

        assert_eq!(plan.status, "blocked");
        assert_eq!(plan.execution_adapter_preflight_ready_count, 1);
        assert_eq!(plan.execution_adapter_preflight_blocked_count, 1);
        assert!(!plan.tool_execution_adapter_preflight_ready);
        assert!(plan.entries.iter().any(|entry| {
            entry.adapter_preflight_route
                == ToolExecutionAdapterPreflightRoute::BlockedByUnknownAdapterKind
        }));
    }
}
