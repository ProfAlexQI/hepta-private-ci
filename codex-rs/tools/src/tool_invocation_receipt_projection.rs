use crate::ToolInvocationLedgerApprovalPreflightPlan;
use crate::ToolInvocationLedgerApprovalPreflightRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_invocation_ledger_approval_preflight_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolInvocationReceiptProjectionRoute {
    ResultReceiptProjectionRequired,
    BlockedByEnabledExecutionSwitch,
    BlockedByMissingReceiptBinding,
    BlockedByMissingReadbackEvidenceBinding,
    BlockedByLedgerApprovalPreflight,
    BlockedByRegistryGuard,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolInvocationReceiptProjectionInput {
    pub receipt_projection_binding_present: bool,
    pub readback_evidence_binding_present: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub result_receipt_write_switch_enabled: bool,
}

impl Default for ToolInvocationReceiptProjectionInput {
    fn default() -> Self {
        Self {
            receipt_projection_binding_present: true,
            readback_evidence_binding_present: true,
            tool_invocation_execution_switch_enabled: false,
            result_receipt_write_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolInvocationReceiptProjectionEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_preflight_route: ToolInvocationLedgerApprovalPreflightRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub receipt_projection_route: ToolInvocationReceiptProjectionRoute,
    pub receipt_projection_ready: bool,
    pub result_receipt_required: bool,
    pub readback_evidence_required: bool,
    pub receipt_projection_binding_present: bool,
    pub readback_evidence_binding_present: bool,
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
pub struct ToolInvocationReceiptProjectionPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_ledger_approval_preflight_surface: &'static str,
    pub source_ledger_approval_preflight_ready: bool,
    pub receipt_projection_binding_present: bool,
    pub readback_evidence_binding_present: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub result_receipt_write_switch_enabled: bool,
    pub candidate_count: usize,
    pub receipt_projection_ready_count: usize,
    pub receipt_projection_blocked_count: usize,
    pub result_receipt_projection_required_count: usize,
    pub readback_evidence_required_count: usize,
    pub all_ledger_approval_entries_bound_to_receipt_projection: bool,
    pub all_receipt_projection_entries_keep_approval_guard: bool,
    pub tool_invocation_receipt_projection_ready: bool,
    pub result_receipt_projection_allowed: bool,
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
    pub entries: Vec<ToolInvocationReceiptProjectionEntry>,
}

pub fn hepta_system_tool_invocation_receipt_projection_plan() -> ToolInvocationReceiptProjectionPlan
{
    let preflight = hepta_system_tool_invocation_ledger_approval_preflight_plan();
    tool_invocation_receipt_projection_plan(
        &preflight,
        &ToolInvocationReceiptProjectionInput::default(),
    )
}

pub fn tool_invocation_receipt_projection_plan(
    preflight: &ToolInvocationLedgerApprovalPreflightPlan,
    input: &ToolInvocationReceiptProjectionInput,
) -> ToolInvocationReceiptProjectionPlan {
    let entries = preflight
        .entries
        .iter()
        .map(|entry| {
            let receipt_projection_route = if input.tool_invocation_execution_switch_enabled
                || input.result_receipt_write_switch_enabled
            {
                ToolInvocationReceiptProjectionRoute::BlockedByEnabledExecutionSwitch
            } else if !input.receipt_projection_binding_present {
                ToolInvocationReceiptProjectionRoute::BlockedByMissingReceiptBinding
            } else if !input.readback_evidence_binding_present {
                ToolInvocationReceiptProjectionRoute::BlockedByMissingReadbackEvidenceBinding
            } else if !entry.ledger_preflight_ready
                || entry.preflight_route
                    != ToolInvocationLedgerApprovalPreflightRoute::ApprovalLedgerPreflightRequired
            {
                ToolInvocationReceiptProjectionRoute::BlockedByLedgerApprovalPreflight
            } else if entry.registry_guard_route
                != ToolRegistryInvocationGuardRoute::RequireApprovalLedger
            {
                ToolInvocationReceiptProjectionRoute::BlockedByRegistryGuard
            } else {
                ToolInvocationReceiptProjectionRoute::ResultReceiptProjectionRequired
            };
            let receipt_projection_ready = receipt_projection_route
                == ToolInvocationReceiptProjectionRoute::ResultReceiptProjectionRequired
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !input.tool_invocation_execution_switch_enabled
                && !input.result_receipt_write_switch_enabled;

            ToolInvocationReceiptProjectionEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                source_preflight_route: entry.preflight_route,
                registry_guard_route: entry.registry_guard_route,
                receipt_projection_route,
                receipt_projection_ready,
                result_receipt_required: receipt_projection_route
                    == ToolInvocationReceiptProjectionRoute::ResultReceiptProjectionRequired,
                readback_evidence_required: receipt_projection_route
                    == ToolInvocationReceiptProjectionRoute::ResultReceiptProjectionRequired,
                receipt_projection_binding_present: input.receipt_projection_binding_present,
                readback_evidence_binding_present: input.readback_evidence_binding_present,
                router_registration_lookup_enabled: false,
                registry_lookup_executed: false,
                registry_source_of_truth_enabled: false,
                tool_registration_enabled: false,
                tool_invocation_enabled: input.tool_invocation_execution_switch_enabled,
                ledger_write_enabled: false,
                approval_request_enabled: false,
                result_receipt_write_enabled: input.result_receipt_write_switch_enabled,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let receipt_projection_ready_count = entries
        .iter()
        .filter(|entry| entry.receipt_projection_ready)
        .count();
    let result_receipt_projection_required_count = entries
        .iter()
        .filter(|entry| {
            entry.receipt_projection_route
                == ToolInvocationReceiptProjectionRoute::ResultReceiptProjectionRequired
        })
        .count();
    let readback_evidence_required_count = entries
        .iter()
        .filter(|entry| entry.readback_evidence_required)
        .count();
    let receipt_projection_blocked_count = entries.len() - receipt_projection_ready_count;
    let all_ledger_approval_entries_bound_to_receipt_projection = input
        .receipt_projection_binding_present
        && input.readback_evidence_binding_present
        && receipt_projection_ready_count == entries.len()
        && result_receipt_projection_required_count == entries.len()
        && readback_evidence_required_count == entries.len();
    let all_receipt_projection_entries_keep_approval_guard = entries.iter().all(|entry| {
        if entry.receipt_projection_route
            == ToolInvocationReceiptProjectionRoute::ResultReceiptProjectionRequired
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
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
    let tool_invocation_receipt_projection_ready = preflight
        .tool_invocation_ledger_approval_preflight_ready
        && !input.tool_invocation_execution_switch_enabled
        && !input.result_receipt_write_switch_enabled
        && all_ledger_approval_entries_bound_to_receipt_projection
        && all_receipt_projection_entries_keep_approval_guard;

    ToolInvocationReceiptProjectionPlan {
        runtime: "hepta",
        surface: "tool_invocation_receipt_projection",
        plugin_id: preflight.plugin_id,
        status: if tool_invocation_receipt_projection_ready {
            "ready"
        } else {
            "blocked"
        },
        source_ledger_approval_preflight_surface: preflight.surface,
        source_ledger_approval_preflight_ready: preflight
            .tool_invocation_ledger_approval_preflight_ready,
        receipt_projection_binding_present: input.receipt_projection_binding_present,
        readback_evidence_binding_present: input.readback_evidence_binding_present,
        tool_invocation_execution_switch_enabled: input.tool_invocation_execution_switch_enabled,
        result_receipt_write_switch_enabled: input.result_receipt_write_switch_enabled,
        candidate_count: entries.len(),
        receipt_projection_ready_count,
        receipt_projection_blocked_count,
        result_receipt_projection_required_count,
        readback_evidence_required_count,
        all_ledger_approval_entries_bound_to_receipt_projection,
        all_receipt_projection_entries_keep_approval_guard,
        tool_invocation_receipt_projection_ready,
        result_receipt_projection_allowed: tool_invocation_receipt_projection_ready
            && receipt_projection_ready_count == entries.len()
            && result_receipt_projection_required_count == entries.len()
            && readback_evidence_required_count == entries.len(),
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
        next_migration_step: "restore_tool_execution_dispatch_shadow_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_invocation_receipt_projection_binds_ledger_preflight_to_receipt_plan() {
        let plan = hepta_system_tool_invocation_receipt_projection_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_ledger_approval_preflight_surface,
            "tool_invocation_ledger_approval_preflight"
        );
        assert!(plan.source_ledger_approval_preflight_ready);
        assert!(plan.receipt_projection_binding_present);
        assert!(plan.readback_evidence_binding_present);
        assert!(!plan.tool_invocation_execution_switch_enabled);
        assert!(!plan.result_receipt_write_switch_enabled);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.receipt_projection_ready_count, 2);
        assert_eq!(plan.receipt_projection_blocked_count, 0);
        assert_eq!(plan.result_receipt_projection_required_count, 2);
        assert_eq!(plan.readback_evidence_required_count, 2);
        assert!(plan.all_ledger_approval_entries_bound_to_receipt_projection);
        assert!(plan.all_receipt_projection_entries_keep_approval_guard);
        assert!(plan.tool_invocation_receipt_projection_ready);
        assert!(plan.result_receipt_projection_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.receipt_projection_route
                == ToolInvocationReceiptProjectionRoute::ResultReceiptProjectionRequired
                && entry.receipt_projection_ready
                && entry.result_receipt_required
                && entry.readback_evidence_required
        }));
    }

    #[test]
    fn tool_invocation_receipt_projection_does_not_execute_or_write_receipts() {
        let plan = hepta_system_tool_invocation_receipt_projection_plan();

        assert!(plan.tool_invocation_receipt_projection_ready);
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
            !entry.router_registration_lookup_enabled
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
    fn tool_invocation_receipt_projection_fails_closed_without_receipt_binding() {
        let preflight = hepta_system_tool_invocation_ledger_approval_preflight_plan();
        let input = ToolInvocationReceiptProjectionInput {
            receipt_projection_binding_present: false,
            readback_evidence_binding_present: true,
            tool_invocation_execution_switch_enabled: false,
            result_receipt_write_switch_enabled: false,
        };

        let plan = tool_invocation_receipt_projection_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.receipt_projection_binding_present);
        assert_eq!(plan.receipt_projection_ready_count, 0);
        assert_eq!(plan.receipt_projection_blocked_count, 2);
        assert!(!plan.tool_invocation_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.receipt_projection_route
                == ToolInvocationReceiptProjectionRoute::BlockedByMissingReceiptBinding
        }));
    }

    #[test]
    fn tool_invocation_receipt_projection_fails_closed_without_readback_binding() {
        let preflight = hepta_system_tool_invocation_ledger_approval_preflight_plan();
        let input = ToolInvocationReceiptProjectionInput {
            receipt_projection_binding_present: true,
            readback_evidence_binding_present: false,
            tool_invocation_execution_switch_enabled: false,
            result_receipt_write_switch_enabled: false,
        };

        let plan = tool_invocation_receipt_projection_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.readback_evidence_binding_present);
        assert_eq!(plan.receipt_projection_ready_count, 0);
        assert_eq!(plan.receipt_projection_blocked_count, 2);
        assert!(!plan.tool_invocation_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.receipt_projection_route
                == ToolInvocationReceiptProjectionRoute::BlockedByMissingReadbackEvidenceBinding
        }));
    }

    #[test]
    fn tool_invocation_receipt_projection_fails_closed_when_execution_or_write_switch_enabled() {
        let preflight = hepta_system_tool_invocation_ledger_approval_preflight_plan();
        let input = ToolInvocationReceiptProjectionInput {
            receipt_projection_binding_present: true,
            readback_evidence_binding_present: true,
            tool_invocation_execution_switch_enabled: true,
            result_receipt_write_switch_enabled: true,
        };

        let plan = tool_invocation_receipt_projection_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.tool_invocation_execution_switch_enabled);
        assert!(plan.result_receipt_write_switch_enabled);
        assert_eq!(plan.receipt_projection_ready_count, 0);
        assert_eq!(plan.receipt_projection_blocked_count, 2);
        assert!(!plan.tool_invocation_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.receipt_projection_route
                == ToolInvocationReceiptProjectionRoute::BlockedByEnabledExecutionSwitch
        }));
    }

    #[test]
    fn tool_invocation_receipt_projection_fails_closed_without_ledger_approval_preflight() {
        let mut preflight = hepta_system_tool_invocation_ledger_approval_preflight_plan();
        preflight.tool_invocation_ledger_approval_preflight_ready = false;
        preflight.ledger_approval_preflight_ready_count = 0;
        preflight.ledger_approval_preflight_blocked_count = 2;
        for entry in &mut preflight.entries {
            entry.ledger_preflight_ready = false;
            entry.preflight_route =
                ToolInvocationLedgerApprovalPreflightRoute::BlockedByRouterLookupShadow;
        }

        let plan = tool_invocation_receipt_projection_plan(
            &preflight,
            &ToolInvocationReceiptProjectionInput::default(),
        );

        assert_eq!(plan.status, "blocked");
        assert_eq!(plan.receipt_projection_ready_count, 0);
        assert_eq!(plan.receipt_projection_blocked_count, 2);
        assert!(!plan.tool_invocation_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.receipt_projection_route
                == ToolInvocationReceiptProjectionRoute::BlockedByLedgerApprovalPreflight
        }));
    }
}
