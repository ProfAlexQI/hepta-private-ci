use crate::ToolExecutionOperatorApprovalReceiptProjectionPlan;
use crate::ToolExecutionOperatorApprovalReceiptProjectionRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_operator_approval_receipt_projection_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionOperatorApprovalDecisionPreflightRoute {
    OperatorApprovalDecisionPendingExplicitApproval,
    BlockedByReceiptProjection,
    BlockedByMissingDecisionPolicy,
    BlockedByMissingOperatorIdentityBinding,
    BlockedByPrematureDecisionMutation,
    BlockedByLiveCutoverSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionOperatorApprovalDecisionPreflightInput {
    pub operator_decision_policy_present: bool,
    pub operator_identity_binding_present: bool,
    pub operator_decision_record_written: bool,
    pub operator_decision_receipt_written: bool,
    pub operator_acceptance_present: bool,
    pub approval_request_sent: bool,
    pub canary_cutover_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
}

impl Default for ToolExecutionOperatorApprovalDecisionPreflightInput {
    fn default() -> Self {
        Self {
            operator_decision_policy_present: true,
            operator_identity_binding_present: true,
            operator_decision_record_written: false,
            operator_decision_receipt_written: false,
            operator_acceptance_present: false,
            approval_request_sent: false,
            canary_cutover_switch_enabled: false,
            live_cutover_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionOperatorApprovalDecisionPreflightEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_receipt_projection_route: ToolExecutionOperatorApprovalReceiptProjectionRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub operator_approval_decision_preflight_route:
        ToolExecutionOperatorApprovalDecisionPreflightRoute,
    pub operator_approval_decision_preflight_ready: bool,
    pub operator_decision_pending: bool,
    pub operator_decision_write_blocked: bool,
    pub operator_acceptance_blocked: bool,
    pub operator_decision_policy_present: bool,
    pub operator_identity_binding_present: bool,
    pub operator_decision_record_written: bool,
    pub operator_decision_receipt_written: bool,
    pub operator_acceptance_present: bool,
    pub approval_request_sent: bool,
    pub canary_cutover_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub execution_adapter_dispatch_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub result_receipt_write_enabled: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionOperatorApprovalDecisionPreflightPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_receipt_projection_surface: &'static str,
    pub source_receipt_projection_ready: bool,
    pub source_operator_decision_write_allowed: bool,
    pub source_live_cutover_allowed: bool,
    pub operator_decision_policy_present: bool,
    pub operator_identity_binding_present: bool,
    pub operator_decision_record_written: bool,
    pub operator_decision_receipt_written: bool,
    pub operator_acceptance_present: bool,
    pub approval_request_sent: bool,
    pub canary_cutover_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
    pub candidate_count: usize,
    pub operator_approval_decision_preflight_ready_count: usize,
    pub operator_approval_decision_preflight_blocked_count: usize,
    pub operator_decision_pending_count: usize,
    pub operator_decision_write_blocked_count: usize,
    pub operator_acceptance_blocked_count: usize,
    pub all_receipt_projections_bound_to_decision_preflight: bool,
    pub all_decision_preflight_entries_keep_approval_guard: bool,
    pub tool_execution_operator_approval_decision_preflight_ready: bool,
    pub tool_execution_operator_decision_acceptance_allowed: bool,
    pub tool_execution_canary_cutover_allowed: bool,
    pub tool_execution_live_cutover_allowed: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub execution_adapter_dispatched: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub operator_decision_record_written_flag: bool,
    pub operator_decision_receipt_written_flag: bool,
    pub result_receipt_written: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<ToolExecutionOperatorApprovalDecisionPreflightEntry>,
}

pub fn hepta_system_tool_execution_operator_approval_decision_preflight_plan()
-> ToolExecutionOperatorApprovalDecisionPreflightPlan {
    let projection = hepta_system_tool_execution_operator_approval_receipt_projection_plan();
    tool_execution_operator_approval_decision_preflight_plan(
        &projection,
        &ToolExecutionOperatorApprovalDecisionPreflightInput::default(),
    )
}

pub fn tool_execution_operator_approval_decision_preflight_plan(
    projection: &ToolExecutionOperatorApprovalReceiptProjectionPlan,
    input: &ToolExecutionOperatorApprovalDecisionPreflightInput,
) -> ToolExecutionOperatorApprovalDecisionPreflightPlan {
    let entries = projection
        .entries
        .iter()
        .map(|entry| {
            let route = if input.canary_cutover_switch_enabled
                || input.live_cutover_switch_enabled
            {
                ToolExecutionOperatorApprovalDecisionPreflightRoute::BlockedByLiveCutoverSwitch
            } else if input.operator_decision_record_written
                || input.operator_decision_receipt_written
                || input.operator_acceptance_present
                || input.approval_request_sent
            {
                ToolExecutionOperatorApprovalDecisionPreflightRoute::BlockedByPrematureDecisionMutation
            } else if !input.operator_decision_policy_present {
                ToolExecutionOperatorApprovalDecisionPreflightRoute::BlockedByMissingDecisionPolicy
            } else if !input.operator_identity_binding_present {
                ToolExecutionOperatorApprovalDecisionPreflightRoute::BlockedByMissingOperatorIdentityBinding
            } else if !entry.operator_approval_receipt_projection_ready
                || entry.operator_approval_receipt_projection_route
                    != ToolExecutionOperatorApprovalReceiptProjectionRoute::OperatorApprovalReceiptProjectionReady
            {
                ToolExecutionOperatorApprovalDecisionPreflightRoute::BlockedByReceiptProjection
            } else {
                ToolExecutionOperatorApprovalDecisionPreflightRoute::OperatorApprovalDecisionPendingExplicitApproval
            };
            let ready = route
                == ToolExecutionOperatorApprovalDecisionPreflightRoute::OperatorApprovalDecisionPendingExplicitApproval
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && entry.operator_decision_receipt_required
                && entry.operator_decision_readback_evidence_required;
            let operator_decision_pending = ready;
            let operator_decision_write_blocked = ready
                && !input.operator_decision_record_written
                && !input.operator_decision_receipt_written
                && !input.approval_request_sent;
            let operator_acceptance_blocked =
                ready && !input.operator_acceptance_present && !input.live_cutover_switch_enabled;

            ToolExecutionOperatorApprovalDecisionPreflightEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_receipt_projection_route: entry
                    .operator_approval_receipt_projection_route,
                registry_guard_route: entry.registry_guard_route,
                operator_approval_decision_preflight_route: route,
                operator_approval_decision_preflight_ready: ready,
                operator_decision_pending,
                operator_decision_write_blocked,
                operator_acceptance_blocked,
                operator_decision_policy_present: input.operator_decision_policy_present,
                operator_identity_binding_present: input.operator_identity_binding_present,
                operator_decision_record_written: input.operator_decision_record_written,
                operator_decision_receipt_written: input.operator_decision_receipt_written,
                operator_acceptance_present: input.operator_acceptance_present,
                approval_request_sent: input.approval_request_sent,
                canary_cutover_switch_enabled: input.canary_cutover_switch_enabled,
                live_cutover_switch_enabled: input.live_cutover_switch_enabled,
                router_registration_lookup_enabled: false,
                registry_lookup_executed: false,
                registry_source_of_truth_enabled: false,
                tool_registration_enabled: false,
                execution_adapter_dispatch_enabled: false,
                tool_invocation_enabled: false,
                ledger_write_enabled: false,
                approval_request_enabled: false,
                result_receipt_write_enabled: false,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let ready_count = entries
        .iter()
        .filter(|entry| entry.operator_approval_decision_preflight_ready)
        .count();
    let pending_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_pending)
        .count();
    let write_blocked_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_write_blocked)
        .count();
    let acceptance_blocked_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_blocked)
        .count();
    let all_receipt_projections_bound_to_decision_preflight = ready_count == entries.len()
        && pending_count == entries.len()
        && write_blocked_count == entries.len()
        && acceptance_blocked_count == entries.len();
    let all_decision_preflight_entries_keep_approval_guard = entries.iter().all(|entry| {
        if entry.operator_approval_decision_preflight_route
            == ToolExecutionOperatorApprovalDecisionPreflightRoute::OperatorApprovalDecisionPendingExplicitApproval
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.operator_decision_record_written
                && !entry.operator_decision_receipt_written
                && !entry.operator_acceptance_present
                && !entry.approval_request_sent
                && !entry.canary_cutover_switch_enabled
                && !entry.live_cutover_switch_enabled
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.execution_adapter_dispatch_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.result_receipt_write_enabled
        } else {
            true
        }
    });
    let tool_execution_operator_approval_decision_preflight_ready = projection
        .tool_execution_operator_approval_receipt_projection_ready
        && !projection.tool_execution_operator_decision_write_allowed
        && !projection.tool_execution_live_cutover_allowed
        && input.operator_decision_policy_present
        && input.operator_identity_binding_present
        && !input.operator_decision_record_written
        && !input.operator_decision_receipt_written
        && !input.operator_acceptance_present
        && !input.approval_request_sent
        && !input.canary_cutover_switch_enabled
        && !input.live_cutover_switch_enabled
        && all_receipt_projections_bound_to_decision_preflight
        && all_decision_preflight_entries_keep_approval_guard;

    ToolExecutionOperatorApprovalDecisionPreflightPlan {
        runtime: "hepta",
        surface: "tool_execution_operator_approval_decision_preflight",
        plugin_id: projection.plugin_id,
        status: if tool_execution_operator_approval_decision_preflight_ready {
            "ready"
        } else {
            "blocked"
        },
        source_receipt_projection_surface: projection.surface,
        source_receipt_projection_ready: projection
            .tool_execution_operator_approval_receipt_projection_ready,
        source_operator_decision_write_allowed: projection
            .tool_execution_operator_decision_write_allowed,
        source_live_cutover_allowed: projection.tool_execution_live_cutover_allowed,
        operator_decision_policy_present: input.operator_decision_policy_present,
        operator_identity_binding_present: input.operator_identity_binding_present,
        operator_decision_record_written: input.operator_decision_record_written,
        operator_decision_receipt_written: input.operator_decision_receipt_written,
        operator_acceptance_present: input.operator_acceptance_present,
        approval_request_sent: input.approval_request_sent,
        canary_cutover_switch_enabled: input.canary_cutover_switch_enabled,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        candidate_count: entries.len(),
        operator_approval_decision_preflight_ready_count: ready_count,
        operator_approval_decision_preflight_blocked_count: entries.len() - ready_count,
        operator_decision_pending_count: pending_count,
        operator_decision_write_blocked_count: write_blocked_count,
        operator_acceptance_blocked_count: acceptance_blocked_count,
        all_receipt_projections_bound_to_decision_preflight,
        all_decision_preflight_entries_keep_approval_guard,
        tool_execution_operator_approval_decision_preflight_ready,
        tool_execution_operator_decision_acceptance_allowed: false,
        tool_execution_canary_cutover_allowed: false,
        tool_execution_live_cutover_allowed: false,
        router_registration_lookup_enabled: false,
        registry_lookup_executed: false,
        registry_source_of_truth_enabled: false,
        tool_registration_enabled: false,
        execution_adapter_dispatched: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        operator_decision_record_written_flag: false,
        operator_decision_receipt_written_flag: false,
        result_receipt_written: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_execution_canary_cutover_plan_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_execution_operator_approval_decision_preflight_collects_pending_decisions() {
        let plan = hepta_system_tool_execution_operator_approval_decision_preflight_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_receipt_projection_surface,
            "tool_execution_operator_approval_receipt_projection"
        );
        assert!(plan.source_receipt_projection_ready);
        assert!(!plan.source_operator_decision_write_allowed);
        assert!(!plan.source_live_cutover_allowed);
        assert!(plan.operator_decision_policy_present);
        assert!(plan.operator_identity_binding_present);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.operator_approval_decision_preflight_ready_count, 2);
        assert_eq!(plan.operator_approval_decision_preflight_blocked_count, 0);
        assert_eq!(plan.operator_decision_pending_count, 2);
        assert_eq!(plan.operator_decision_write_blocked_count, 2);
        assert_eq!(plan.operator_acceptance_blocked_count, 2);
        assert!(plan.all_receipt_projections_bound_to_decision_preflight);
        assert!(plan.all_decision_preflight_entries_keep_approval_guard);
        assert!(plan.tool_execution_operator_approval_decision_preflight_ready);
        assert!(!plan.tool_execution_operator_decision_acceptance_allowed);
        assert!(!plan.tool_execution_canary_cutover_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);
    }

    #[test]
    fn tool_execution_operator_approval_decision_preflight_does_not_enable_cutover() {
        let plan = hepta_system_tool_execution_operator_approval_decision_preflight_plan();

        assert!(plan.tool_execution_operator_approval_decision_preflight_ready);
        assert!(!plan.tool_execution_operator_decision_acceptance_allowed);
        assert!(!plan.tool_execution_canary_cutover_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.operator_decision_record_written_flag);
        assert!(!plan.operator_decision_receipt_written_flag);
        assert!(!plan.result_receipt_written);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn tool_execution_operator_approval_decision_preflight_fails_closed_without_policy() {
        let projection = hepta_system_tool_execution_operator_approval_receipt_projection_plan();
        let input = ToolExecutionOperatorApprovalDecisionPreflightInput {
            operator_decision_policy_present: false,
            ..ToolExecutionOperatorApprovalDecisionPreflightInput::default()
        };

        let plan = tool_execution_operator_approval_decision_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.operator_decision_policy_present);
        assert_eq!(plan.operator_approval_decision_preflight_ready_count, 0);
        assert_eq!(plan.operator_approval_decision_preflight_blocked_count, 2);
        assert!(!plan.tool_execution_operator_approval_decision_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_decision_preflight_route
                == ToolExecutionOperatorApprovalDecisionPreflightRoute::BlockedByMissingDecisionPolicy
        }));
    }

    #[test]
    fn tool_execution_operator_approval_decision_preflight_fails_closed_on_premature_decision_mutation()
     {
        let projection = hepta_system_tool_execution_operator_approval_receipt_projection_plan();
        let input = ToolExecutionOperatorApprovalDecisionPreflightInput {
            operator_decision_record_written: true,
            operator_decision_receipt_written: true,
            operator_acceptance_present: true,
            approval_request_sent: true,
            ..ToolExecutionOperatorApprovalDecisionPreflightInput::default()
        };

        let plan = tool_execution_operator_approval_decision_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.operator_decision_record_written);
        assert!(plan.operator_decision_receipt_written);
        assert!(plan.operator_acceptance_present);
        assert!(plan.approval_request_sent);
        assert!(!plan.tool_execution_operator_approval_decision_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_decision_preflight_route
                == ToolExecutionOperatorApprovalDecisionPreflightRoute::BlockedByPrematureDecisionMutation
        }));
    }

    #[test]
    fn tool_execution_operator_approval_decision_preflight_fails_closed_when_cutover_switch_enabled()
     {
        let projection = hepta_system_tool_execution_operator_approval_receipt_projection_plan();
        let input = ToolExecutionOperatorApprovalDecisionPreflightInput {
            canary_cutover_switch_enabled: true,
            live_cutover_switch_enabled: true,
            ..ToolExecutionOperatorApprovalDecisionPreflightInput::default()
        };

        let plan = tool_execution_operator_approval_decision_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.canary_cutover_switch_enabled);
        assert!(plan.live_cutover_switch_enabled);
        assert!(!plan.tool_execution_operator_approval_decision_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_decision_preflight_route
                == ToolExecutionOperatorApprovalDecisionPreflightRoute::BlockedByLiveCutoverSwitch
        }));
    }
}
