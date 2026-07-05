use crate::ToolInvocationLedgerApprovalPreflightPlan;
use crate::ToolInvocationLedgerApprovalPreflightRoute;
use crate::ToolInvocationReceiptProjectionPlan;
use crate::ToolInvocationReceiptProjectionRoute;
use crate::ToolRegistryInvocationSourceOfTruthPlan;
use crate::ToolRegistryInvocationSourceRoute;
use crate::ToolRegistryRouterLookupShadowPlan;
use crate::ToolRegistryRouterLookupShadowRoute;
use crate::hepta_system_tool_invocation_ledger_approval_preflight_plan;
use crate::hepta_system_tool_invocation_receipt_projection_plan;
use crate::hepta_system_tool_registry_invocation_source_of_truth_plan;
use crate::hepta_system_tool_registry_router_lookup_shadow_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolRegistryReadOnlyDispatchPreflightRoute {
    ReadOnlyDispatchReceiptProjectionReady,
    BlockedByEnabledMutationSwitch,
    BlockedByMissingDispatchBinding,
    BlockedByCandidateMismatch,
    BlockedByInvocationSource,
    BlockedByLookupShadow,
    BlockedByLedgerApprovalPreflight,
    BlockedByReceiptProjection,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryReadOnlyDispatchPreflightInput {
    pub dispatch_preflight_binding_present: bool,
    pub registry_dispatch_switch_enabled: bool,
    pub tool_invocation_switch_enabled: bool,
    pub ledger_write_switch_enabled: bool,
    pub approval_request_switch_enabled: bool,
    pub result_receipt_write_switch_enabled: bool,
}

impl Default for ToolRegistryReadOnlyDispatchPreflightInput {
    fn default() -> Self {
        Self {
            dispatch_preflight_binding_present: true,
            registry_dispatch_switch_enabled: false,
            tool_invocation_switch_enabled: false,
            ledger_write_switch_enabled: false,
            approval_request_switch_enabled: false,
            result_receipt_write_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryReadOnlyDispatchPreflightEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_invocation_route: ToolRegistryInvocationSourceRoute,
    pub lookup_shadow_route: ToolRegistryRouterLookupShadowRoute,
    pub ledger_preflight_route: ToolInvocationLedgerApprovalPreflightRoute,
    pub receipt_projection_route: ToolInvocationReceiptProjectionRoute,
    pub dispatch_preflight_route: ToolRegistryReadOnlyDispatchPreflightRoute,
    pub dispatch_preflight_ready: bool,
    pub registry_lookup_preview_required: bool,
    pub ledger_preview_required: bool,
    pub approval_preflight_required: bool,
    pub receipt_projection_required: bool,
    pub dispatch_preflight_binding_present: bool,
    pub registry_dispatch_switch_enabled: bool,
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
pub struct ToolRegistryReadOnlyDispatchPreflightPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_invocation_surface: &'static str,
    pub source_invocation_ready: bool,
    pub source_lookup_shadow_surface: &'static str,
    pub source_lookup_shadow_ready: bool,
    pub source_ledger_approval_preflight_surface: &'static str,
    pub source_ledger_approval_preflight_ready: bool,
    pub source_receipt_projection_surface: &'static str,
    pub source_receipt_projection_ready: bool,
    pub dispatch_preflight_binding_present: bool,
    pub candidate_count: usize,
    pub dispatch_preflight_ready_count: usize,
    pub dispatch_preflight_blocked_count: usize,
    pub registry_lookup_preview_required_count: usize,
    pub ledger_preview_required_count: usize,
    pub approval_preflight_required_count: usize,
    pub receipt_projection_required_count: usize,
    pub all_entries_bound_to_read_only_dispatch_preflight: bool,
    pub all_dispatch_entries_keep_no_invocation_guard: bool,
    pub read_only_dispatch_preflight_ready: bool,
    pub read_only_dispatch_preflight_allowed: bool,
    pub registry_dispatch_switch_enabled: bool,
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
    pub entries: Vec<ToolRegistryReadOnlyDispatchPreflightEntry>,
}

pub fn hepta_system_tool_registry_read_only_dispatch_preflight_plan()
-> ToolRegistryReadOnlyDispatchPreflightPlan {
    let source = hepta_system_tool_registry_invocation_source_of_truth_plan();
    let lookup = hepta_system_tool_registry_router_lookup_shadow_plan();
    let ledger = hepta_system_tool_invocation_ledger_approval_preflight_plan();
    let receipt = hepta_system_tool_invocation_receipt_projection_plan();
    tool_registry_read_only_dispatch_preflight_plan(
        &source,
        &lookup,
        &ledger,
        &receipt,
        &ToolRegistryReadOnlyDispatchPreflightInput::default(),
    )
}

pub fn tool_registry_read_only_dispatch_preflight_plan(
    source: &ToolRegistryInvocationSourceOfTruthPlan,
    lookup: &ToolRegistryRouterLookupShadowPlan,
    ledger: &ToolInvocationLedgerApprovalPreflightPlan,
    receipt: &ToolInvocationReceiptProjectionPlan,
    input: &ToolRegistryReadOnlyDispatchPreflightInput,
) -> ToolRegistryReadOnlyDispatchPreflightPlan {
    let entries = receipt
        .entries
        .iter()
        .map(|receipt_entry| {
            let source_entry = source
                .entries
                .iter()
                .find(|entry| entry.candidate_tool_id == receipt_entry.candidate_tool_id);
            let lookup_entry = lookup
                .entries
                .iter()
                .find(|entry| entry.candidate_tool_id == receipt_entry.candidate_tool_id);
            let ledger_entry = ledger
                .entries
                .iter()
                .find(|entry| entry.candidate_tool_id == receipt_entry.candidate_tool_id);
            let candidates_match =
                source_entry.is_some() && lookup_entry.is_some() && ledger_entry.is_some();
            let source_route = source_entry.map_or(
                ToolRegistryInvocationSourceRoute::BlockedByRouterPreflight,
                |entry| entry.invocation_source_route,
            );
            let lookup_route = lookup_entry.map_or(
                ToolRegistryRouterLookupShadowRoute::BlockedByLookupPrecondition,
                |entry| entry.shadow_route,
            );
            let ledger_route = ledger_entry.map_or(
                ToolInvocationLedgerApprovalPreflightRoute::BlockedByRouterLookupShadow,
                |entry| entry.preflight_route,
            );
            let receipt_route = receipt_entry.receipt_projection_route;
            let mutation_switch_enabled = input.registry_dispatch_switch_enabled
                || input.tool_invocation_switch_enabled
                || input.ledger_write_switch_enabled
                || input.approval_request_switch_enabled
                || input.result_receipt_write_switch_enabled;
            let dispatch_preflight_route = if mutation_switch_enabled {
                ToolRegistryReadOnlyDispatchPreflightRoute::BlockedByEnabledMutationSwitch
            } else if !input.dispatch_preflight_binding_present {
                ToolRegistryReadOnlyDispatchPreflightRoute::BlockedByMissingDispatchBinding
            } else if !candidates_match {
                ToolRegistryReadOnlyDispatchPreflightRoute::BlockedByCandidateMismatch
            } else if !source_entry.is_some_and(|entry| entry.invocation_source_ready) {
                ToolRegistryReadOnlyDispatchPreflightRoute::BlockedByInvocationSource
            } else if !lookup_entry.is_some_and(|entry| entry.shadow_ready) {
                ToolRegistryReadOnlyDispatchPreflightRoute::BlockedByLookupShadow
            } else if !ledger_entry.is_some_and(|entry| entry.ledger_preflight_ready) {
                ToolRegistryReadOnlyDispatchPreflightRoute::BlockedByLedgerApprovalPreflight
            } else if !receipt_entry.receipt_projection_ready {
                ToolRegistryReadOnlyDispatchPreflightRoute::BlockedByReceiptProjection
            } else {
                ToolRegistryReadOnlyDispatchPreflightRoute::ReadOnlyDispatchReceiptProjectionReady
            };
            let dispatch_preflight_ready = dispatch_preflight_route
                == ToolRegistryReadOnlyDispatchPreflightRoute::ReadOnlyDispatchReceiptProjectionReady
                && source_route
                    == ToolRegistryInvocationSourceRoute::ApprovalLedgerDryRunSourceOnly
                && lookup_route
                    == ToolRegistryRouterLookupShadowRoute::DisabledApprovalLedgerLookupShadow
                && ledger_route
                    == ToolInvocationLedgerApprovalPreflightRoute::ApprovalLedgerPreflightRequired
                && receipt_route
                    == ToolInvocationReceiptProjectionRoute::ResultReceiptProjectionRequired
                && !mutation_switch_enabled;

            ToolRegistryReadOnlyDispatchPreflightEntry {
                plugin_id: receipt_entry.plugin_id,
                candidate_tool_id: receipt_entry.candidate_tool_id,
                contribution_kind: receipt_entry.contribution_kind,
                source_invocation_route: source_route,
                lookup_shadow_route: lookup_route,
                ledger_preflight_route: ledger_route,
                receipt_projection_route: receipt_route,
                dispatch_preflight_route,
                dispatch_preflight_ready,
                registry_lookup_preview_required: dispatch_preflight_ready,
                ledger_preview_required: dispatch_preflight_ready,
                approval_preflight_required: dispatch_preflight_ready,
                receipt_projection_required: dispatch_preflight_ready,
                dispatch_preflight_binding_present: input.dispatch_preflight_binding_present,
                registry_dispatch_switch_enabled: input.registry_dispatch_switch_enabled,
                router_registration_lookup_enabled: false,
                registry_lookup_executed: false,
                registry_source_of_truth_enabled: false,
                tool_registration_enabled: false,
                tool_invocation_enabled: input.tool_invocation_switch_enabled,
                ledger_write_enabled: input.ledger_write_switch_enabled,
                approval_request_enabled: input.approval_request_switch_enabled,
                result_receipt_write_enabled: input.result_receipt_write_switch_enabled,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let dispatch_preflight_ready_count = entries
        .iter()
        .filter(|entry| entry.dispatch_preflight_ready)
        .count();
    let registry_lookup_preview_required_count = entries
        .iter()
        .filter(|entry| entry.registry_lookup_preview_required)
        .count();
    let ledger_preview_required_count = entries
        .iter()
        .filter(|entry| entry.ledger_preview_required)
        .count();
    let approval_preflight_required_count = entries
        .iter()
        .filter(|entry| entry.approval_preflight_required)
        .count();
    let receipt_projection_required_count = entries
        .iter()
        .filter(|entry| entry.receipt_projection_required)
        .count();
    let dispatch_preflight_blocked_count = entries.len() - dispatch_preflight_ready_count;
    let all_entries_bound_to_read_only_dispatch_preflight = input
        .dispatch_preflight_binding_present
        && dispatch_preflight_ready_count == entries.len()
        && registry_lookup_preview_required_count == entries.len()
        && ledger_preview_required_count == entries.len()
        && approval_preflight_required_count == entries.len()
        && receipt_projection_required_count == entries.len();
    let all_dispatch_entries_keep_no_invocation_guard = entries.iter().all(|entry| {
        if entry.dispatch_preflight_route
            == ToolRegistryReadOnlyDispatchPreflightRoute::ReadOnlyDispatchReceiptProjectionReady
        {
            !entry.registry_dispatch_switch_enabled
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
    let read_only_dispatch_preflight_ready = source.invocation_source_of_truth_plan_ready
        && lookup.router_lookup_shadow_ready
        && ledger.tool_invocation_ledger_approval_preflight_ready
        && receipt.tool_invocation_receipt_projection_ready
        && all_entries_bound_to_read_only_dispatch_preflight
        && all_dispatch_entries_keep_no_invocation_guard;

    ToolRegistryReadOnlyDispatchPreflightPlan {
        runtime: "hepta",
        surface: "tool_registry_read_only_dispatch_preflight",
        plugin_id: receipt.plugin_id,
        status: if read_only_dispatch_preflight_ready {
            "ready"
        } else {
            "blocked"
        },
        source_invocation_surface: source.surface,
        source_invocation_ready: source.invocation_source_of_truth_plan_ready,
        source_lookup_shadow_surface: lookup.surface,
        source_lookup_shadow_ready: lookup.router_lookup_shadow_ready,
        source_ledger_approval_preflight_surface: ledger.surface,
        source_ledger_approval_preflight_ready: ledger
            .tool_invocation_ledger_approval_preflight_ready,
        source_receipt_projection_surface: receipt.surface,
        source_receipt_projection_ready: receipt.tool_invocation_receipt_projection_ready,
        dispatch_preflight_binding_present: input.dispatch_preflight_binding_present,
        candidate_count: entries.len(),
        dispatch_preflight_ready_count,
        dispatch_preflight_blocked_count,
        registry_lookup_preview_required_count,
        ledger_preview_required_count,
        approval_preflight_required_count,
        receipt_projection_required_count,
        all_entries_bound_to_read_only_dispatch_preflight,
        all_dispatch_entries_keep_no_invocation_guard,
        read_only_dispatch_preflight_ready,
        read_only_dispatch_preflight_allowed: read_only_dispatch_preflight_ready,
        registry_dispatch_switch_enabled: input.registry_dispatch_switch_enabled,
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
        next_migration_step: "phase3_rebuild_temporal_lite_event_log_adapter_behind_feature_gate",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_only_dispatch_preflight_binds_lookup_ledger_approval_and_receipt_projection() {
        let plan = hepta_system_tool_registry_read_only_dispatch_preflight_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_invocation_surface,
            "tool_registry_invocation_source_of_truth"
        );
        assert!(plan.source_invocation_ready);
        assert_eq!(
            plan.source_lookup_shadow_surface,
            "tool_registry_router_lookup_shadow"
        );
        assert!(plan.source_lookup_shadow_ready);
        assert_eq!(
            plan.source_ledger_approval_preflight_surface,
            "tool_invocation_ledger_approval_preflight"
        );
        assert!(plan.source_ledger_approval_preflight_ready);
        assert_eq!(
            plan.source_receipt_projection_surface,
            "tool_invocation_receipt_projection"
        );
        assert!(plan.source_receipt_projection_ready);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.dispatch_preflight_ready_count, 2);
        assert_eq!(plan.dispatch_preflight_blocked_count, 0);
        assert_eq!(plan.registry_lookup_preview_required_count, 2);
        assert_eq!(plan.ledger_preview_required_count, 2);
        assert_eq!(plan.approval_preflight_required_count, 2);
        assert_eq!(plan.receipt_projection_required_count, 2);
        assert!(plan.all_entries_bound_to_read_only_dispatch_preflight);
        assert!(plan.all_dispatch_entries_keep_no_invocation_guard);
        assert!(plan.read_only_dispatch_preflight_ready);
        assert!(plan.read_only_dispatch_preflight_allowed);
    }

    #[test]
    fn read_only_dispatch_preflight_does_not_register_invoke_write_or_request() {
        let plan = hepta_system_tool_registry_read_only_dispatch_preflight_plan();

        assert!(plan.read_only_dispatch_preflight_ready);
        assert!(!plan.registry_dispatch_switch_enabled);
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
            entry.dispatch_preflight_route
                == ToolRegistryReadOnlyDispatchPreflightRoute::ReadOnlyDispatchReceiptProjectionReady
                && entry.dispatch_preflight_ready
                && !entry.registry_dispatch_switch_enabled
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
    fn read_only_dispatch_preflight_fails_closed_without_receipt_projection() {
        let source = hepta_system_tool_registry_invocation_source_of_truth_plan();
        let lookup = hepta_system_tool_registry_router_lookup_shadow_plan();
        let ledger = hepta_system_tool_invocation_ledger_approval_preflight_plan();
        let mut receipt = hepta_system_tool_invocation_receipt_projection_plan();
        receipt.tool_invocation_receipt_projection_ready = false;
        for entry in &mut receipt.entries {
            entry.receipt_projection_ready = false;
        }

        let plan = tool_registry_read_only_dispatch_preflight_plan(
            &source,
            &lookup,
            &ledger,
            &receipt,
            &ToolRegistryReadOnlyDispatchPreflightInput::default(),
        );

        assert_eq!(plan.status, "blocked");
        assert!(!plan.source_receipt_projection_ready);
        assert_eq!(plan.dispatch_preflight_ready_count, 0);
        assert_eq!(plan.dispatch_preflight_blocked_count, 2);
        assert!(!plan.read_only_dispatch_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.dispatch_preflight_route
                == ToolRegistryReadOnlyDispatchPreflightRoute::BlockedByReceiptProjection
        }));
    }

    #[test]
    fn read_only_dispatch_preflight_fails_closed_when_any_mutation_switch_is_enabled() {
        let source = hepta_system_tool_registry_invocation_source_of_truth_plan();
        let lookup = hepta_system_tool_registry_router_lookup_shadow_plan();
        let ledger = hepta_system_tool_invocation_ledger_approval_preflight_plan();
        let receipt = hepta_system_tool_invocation_receipt_projection_plan();
        let input = ToolRegistryReadOnlyDispatchPreflightInput {
            dispatch_preflight_binding_present: true,
            registry_dispatch_switch_enabled: true,
            tool_invocation_switch_enabled: true,
            ledger_write_switch_enabled: true,
            approval_request_switch_enabled: true,
            result_receipt_write_switch_enabled: true,
        };

        let plan = tool_registry_read_only_dispatch_preflight_plan(
            &source, &lookup, &ledger, &receipt, &input,
        );

        assert_eq!(plan.status, "blocked");
        assert_eq!(plan.dispatch_preflight_ready_count, 0);
        assert_eq!(plan.dispatch_preflight_blocked_count, 2);
        assert!(!plan.read_only_dispatch_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.dispatch_preflight_route
                == ToolRegistryReadOnlyDispatchPreflightRoute::BlockedByEnabledMutationSwitch
                && entry.tool_invocation_enabled
                && entry.ledger_write_enabled
                && entry.approval_request_enabled
                && entry.result_receipt_write_enabled
        }));
    }
}
