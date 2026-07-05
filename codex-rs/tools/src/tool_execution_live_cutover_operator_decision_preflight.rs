use crate::ToolExecutionLiveCutoverOperatorReceiptProjectionPlan;
use crate::ToolExecutionLiveCutoverOperatorReceiptProjectionRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_live_cutover_operator_receipt_projection_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionLiveCutoverOperatorDecisionPreflightRoute {
    LiveCutoverOperatorDecisionPendingExplicitApproval,
    PreflightOnlyNonSelectedCandidate,
    BlockedByOperatorReceiptProjection,
    BlockedByMissingDecisionPolicy,
    BlockedByMissingOperatorIdentityBinding,
    BlockedByPrematureDecisionMutation,
    BlockedByLiveCutoverSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverOperatorDecisionPreflightInput {
    pub operator_cutover_decision_policy_present: bool,
    pub operator_identity_binding_present: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_decision_receipt_written: bool,
    pub operator_cutover_readback_evidence_written: bool,
    pub operator_cutover_acceptance_recorded: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
}

impl Default for ToolExecutionLiveCutoverOperatorDecisionPreflightInput {
    fn default() -> Self {
        Self {
            operator_cutover_decision_policy_present: true,
            operator_identity_binding_present: true,
            approval_request_sent: false,
            operator_cutover_decision_receipt_written: false,
            operator_cutover_readback_evidence_written: false,
            operator_cutover_acceptance_recorded: false,
            live_cutover_switch_enabled: false,
            adapter_dispatch_switch_enabled: false,
            tool_invocation_execution_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverOperatorDecisionPreflightEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_live_cutover_operator_receipt_projection_route:
        ToolExecutionLiveCutoverOperatorReceiptProjectionRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub selected_for_status_canary: bool,
    pub preflight_only_non_selected_candidate: bool,
    pub live_cutover_operator_decision_preflight_route:
        ToolExecutionLiveCutoverOperatorDecisionPreflightRoute,
    pub live_cutover_operator_decision_preflight_ready: bool,
    pub operator_cutover_decision_pending: bool,
    pub operator_cutover_decision_write_blocked: bool,
    pub operator_cutover_acceptance_blocked: bool,
    pub operator_cutover_decision_policy_present: bool,
    pub operator_identity_binding_present: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_decision_receipt_written: bool,
    pub operator_cutover_readback_evidence_written: bool,
    pub operator_cutover_acceptance_recorded: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
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
pub struct ToolExecutionLiveCutoverOperatorDecisionPreflightPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_live_cutover_operator_receipt_projection_surface: &'static str,
    pub source_live_cutover_operator_receipt_projection_ready: bool,
    pub source_live_cutover_operator_decision_write_allowed: bool,
    pub source_live_cutover_allowed: bool,
    pub operator_cutover_decision_policy_present: bool,
    pub operator_identity_binding_present: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_decision_receipt_written: bool,
    pub operator_cutover_readback_evidence_written: bool,
    pub operator_cutover_acceptance_recorded: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub candidate_count: usize,
    pub live_cutover_operator_decision_preflight_ready_count: usize,
    pub live_cutover_operator_decision_preflight_blocked_count: usize,
    pub operator_cutover_decision_pending_count: usize,
    pub operator_cutover_decision_write_blocked_count: usize,
    pub operator_cutover_acceptance_blocked_count: usize,
    pub selected_status_canary_count: usize,
    pub preflight_only_non_selected_count: usize,
    pub all_live_cutover_operator_receipt_projections_bound_to_decision_preflight: bool,
    pub all_live_cutover_operator_decision_preflight_entries_keep_no_invocation_guard: bool,
    pub tool_execution_live_cutover_operator_decision_preflight_ready: bool,
    pub tool_execution_live_cutover_operator_decision_acceptance_allowed: bool,
    pub tool_execution_live_cutover_allowed: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub execution_adapter_dispatched: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub result_receipt_written: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<ToolExecutionLiveCutoverOperatorDecisionPreflightEntry>,
}

pub fn hepta_system_tool_execution_live_cutover_operator_decision_preflight_plan()
-> ToolExecutionLiveCutoverOperatorDecisionPreflightPlan {
    let projection = hepta_system_tool_execution_live_cutover_operator_receipt_projection_plan();
    tool_execution_live_cutover_operator_decision_preflight_plan(
        &projection,
        &ToolExecutionLiveCutoverOperatorDecisionPreflightInput::default(),
    )
}

pub fn tool_execution_live_cutover_operator_decision_preflight_plan(
    projection: &ToolExecutionLiveCutoverOperatorReceiptProjectionPlan,
    input: &ToolExecutionLiveCutoverOperatorDecisionPreflightInput,
) -> ToolExecutionLiveCutoverOperatorDecisionPreflightPlan {
    let entries = projection
        .entries
        .iter()
        .map(|entry| {
            let route = if input.live_cutover_switch_enabled
                || input.adapter_dispatch_switch_enabled
                || input.tool_invocation_execution_switch_enabled
            {
                ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::BlockedByLiveCutoverSwitch
            } else if input.approval_request_sent
                || input.operator_cutover_decision_receipt_written
                || input.operator_cutover_readback_evidence_written
                || input.operator_cutover_acceptance_recorded
            {
                ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::BlockedByPrematureDecisionMutation
            } else if !input.operator_cutover_decision_policy_present {
                ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::BlockedByMissingDecisionPolicy
            } else if !input.operator_identity_binding_present {
                ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::BlockedByMissingOperatorIdentityBinding
            } else if entry.preflight_only_non_selected_candidate
                && entry.live_cutover_operator_receipt_projection_route
                    == ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::PreflightOnlyNonSelectedCandidate
            {
                ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::PreflightOnlyNonSelectedCandidate
            } else if !entry.live_cutover_operator_receipt_projection_ready
                || entry.live_cutover_operator_receipt_projection_route
                    != ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::LiveCutoverOperatorReceiptProjectionReady
            {
                ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::BlockedByOperatorReceiptProjection
            } else {
                ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::LiveCutoverOperatorDecisionPendingExplicitApproval
            };
            let ready = matches!(
                route,
                ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::LiveCutoverOperatorDecisionPendingExplicitApproval
                    | ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::PreflightOnlyNonSelectedCandidate
            )
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && (entry.preflight_only_non_selected_candidate
                    || (entry.operator_cutover_decision_receipt_required
                        && entry.operator_cutover_decision_readback_evidence_required
                        && entry.operator_cutover_decision_receipt_write_blocked
                        && entry.remaining_blocker_readback_required));
            let operator_cutover_decision_pending = ready && entry.selected_for_status_canary;
            let operator_cutover_decision_write_blocked = ready
                && entry.selected_for_status_canary
                && !input.approval_request_sent
                && !input.operator_cutover_decision_receipt_written
                && !input.operator_cutover_readback_evidence_written;
            let operator_cutover_acceptance_blocked = ready
                && entry.selected_for_status_canary
                && !input.operator_cutover_acceptance_recorded
                && !input.live_cutover_switch_enabled;

            ToolExecutionLiveCutoverOperatorDecisionPreflightEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_live_cutover_operator_receipt_projection_route: entry
                    .live_cutover_operator_receipt_projection_route,
                registry_guard_route: entry.registry_guard_route,
                selected_for_status_canary: entry.selected_for_status_canary,
                preflight_only_non_selected_candidate: entry.preflight_only_non_selected_candidate,
                live_cutover_operator_decision_preflight_route: route,
                live_cutover_operator_decision_preflight_ready: ready,
                operator_cutover_decision_pending,
                operator_cutover_decision_write_blocked,
                operator_cutover_acceptance_blocked,
                operator_cutover_decision_policy_present: input
                    .operator_cutover_decision_policy_present,
                operator_identity_binding_present: input.operator_identity_binding_present,
                approval_request_sent: input.approval_request_sent,
                operator_cutover_decision_receipt_written: input
                    .operator_cutover_decision_receipt_written,
                operator_cutover_readback_evidence_written: input
                    .operator_cutover_readback_evidence_written,
                operator_cutover_acceptance_recorded: input
                    .operator_cutover_acceptance_recorded,
                live_cutover_switch_enabled: input.live_cutover_switch_enabled,
                adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
                tool_invocation_execution_switch_enabled: input
                    .tool_invocation_execution_switch_enabled,
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
        .filter(|entry| entry.live_cutover_operator_decision_preflight_ready)
        .count();
    let pending_count = entries
        .iter()
        .filter(|entry| entry.operator_cutover_decision_pending)
        .count();
    let write_blocked_count = entries
        .iter()
        .filter(|entry| entry.operator_cutover_decision_write_blocked)
        .count();
    let acceptance_blocked_count = entries
        .iter()
        .filter(|entry| entry.operator_cutover_acceptance_blocked)
        .count();
    let selected_status_canary_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary)
        .count();
    let preflight_only_non_selected_count = entries
        .iter()
        .filter(|entry| entry.preflight_only_non_selected_candidate)
        .count();
    let all_live_cutover_operator_receipt_projections_bound_to_decision_preflight = ready_count
        == entries.len()
        && selected_status_canary_count == 1
        && preflight_only_non_selected_count + selected_status_canary_count == entries.len()
        && pending_count == selected_status_canary_count
        && write_blocked_count == selected_status_canary_count
        && acceptance_blocked_count == selected_status_canary_count;
    let all_live_cutover_operator_decision_preflight_entries_keep_no_invocation_guard =
        entries.iter().all(|entry| {
            if matches!(
                entry.live_cutover_operator_decision_preflight_route,
                ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::LiveCutoverOperatorDecisionPendingExplicitApproval
                    | ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::PreflightOnlyNonSelectedCandidate
            ) {
                entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                    && !entry.approval_request_sent
                    && !entry.operator_cutover_decision_receipt_written
                    && !entry.operator_cutover_readback_evidence_written
                    && !entry.operator_cutover_acceptance_recorded
                    && !entry.live_cutover_switch_enabled
                    && !entry.adapter_dispatch_switch_enabled
                    && !entry.tool_invocation_execution_switch_enabled
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
    let tool_execution_live_cutover_operator_decision_preflight_ready = projection
        .tool_execution_live_cutover_operator_receipt_projection_ready
        && !projection.tool_execution_live_cutover_operator_decision_write_allowed
        && !projection.tool_execution_live_cutover_allowed
        && input.operator_cutover_decision_policy_present
        && input.operator_identity_binding_present
        && !input.approval_request_sent
        && !input.operator_cutover_decision_receipt_written
        && !input.operator_cutover_readback_evidence_written
        && !input.operator_cutover_acceptance_recorded
        && !input.live_cutover_switch_enabled
        && !input.adapter_dispatch_switch_enabled
        && !input.tool_invocation_execution_switch_enabled
        && all_live_cutover_operator_receipt_projections_bound_to_decision_preflight
        && all_live_cutover_operator_decision_preflight_entries_keep_no_invocation_guard;

    ToolExecutionLiveCutoverOperatorDecisionPreflightPlan {
        runtime: "hepta",
        surface: "tool_execution_live_cutover_operator_decision_preflight",
        plugin_id: projection.plugin_id,
        status: if tool_execution_live_cutover_operator_decision_preflight_ready {
            "ready"
        } else {
            "blocked"
        },
        source_live_cutover_operator_receipt_projection_surface: projection.surface,
        source_live_cutover_operator_receipt_projection_ready: projection
            .tool_execution_live_cutover_operator_receipt_projection_ready,
        source_live_cutover_operator_decision_write_allowed: projection
            .tool_execution_live_cutover_operator_decision_write_allowed,
        source_live_cutover_allowed: projection.tool_execution_live_cutover_allowed,
        operator_cutover_decision_policy_present: input.operator_cutover_decision_policy_present,
        operator_identity_binding_present: input.operator_identity_binding_present,
        approval_request_sent: input.approval_request_sent,
        operator_cutover_decision_receipt_written: input.operator_cutover_decision_receipt_written,
        operator_cutover_readback_evidence_written: input
            .operator_cutover_readback_evidence_written,
        operator_cutover_acceptance_recorded: input.operator_cutover_acceptance_recorded,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
        tool_invocation_execution_switch_enabled: input.tool_invocation_execution_switch_enabled,
        candidate_count: entries.len(),
        live_cutover_operator_decision_preflight_ready_count: ready_count,
        live_cutover_operator_decision_preflight_blocked_count: entries.len() - ready_count,
        operator_cutover_decision_pending_count: pending_count,
        operator_cutover_decision_write_blocked_count: write_blocked_count,
        operator_cutover_acceptance_blocked_count: acceptance_blocked_count,
        selected_status_canary_count,
        preflight_only_non_selected_count,
        all_live_cutover_operator_receipt_projections_bound_to_decision_preflight,
        all_live_cutover_operator_decision_preflight_entries_keep_no_invocation_guard,
        tool_execution_live_cutover_operator_decision_preflight_ready,
        tool_execution_live_cutover_operator_decision_acceptance_allowed: false,
        tool_execution_live_cutover_allowed: false,
        router_registration_lookup_enabled: false,
        registry_lookup_executed: false,
        registry_source_of_truth_enabled: false,
        tool_registration_enabled: false,
        execution_adapter_dispatched: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        result_receipt_written: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_execution_live_cutover_receipt_rollback_packet_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_cutover_operator_decision_preflight_collects_pending_decisions() {
        let plan = hepta_system_tool_execution_live_cutover_operator_decision_preflight_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_live_cutover_operator_receipt_projection_surface,
            "tool_execution_live_cutover_operator_receipt_projection"
        );
        assert!(plan.source_live_cutover_operator_receipt_projection_ready);
        assert!(!plan.source_live_cutover_operator_decision_write_allowed);
        assert!(!plan.source_live_cutover_allowed);
        assert!(plan.operator_cutover_decision_policy_present);
        assert!(plan.operator_identity_binding_present);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.live_cutover_operator_decision_preflight_ready_count, 2);
        assert_eq!(
            plan.live_cutover_operator_decision_preflight_blocked_count,
            0
        );
        assert_eq!(plan.operator_cutover_decision_pending_count, 1);
        assert_eq!(plan.operator_cutover_decision_write_blocked_count, 1);
        assert_eq!(plan.operator_cutover_acceptance_blocked_count, 1);
        assert_eq!(plan.selected_status_canary_count, 1);
        assert_eq!(plan.preflight_only_non_selected_count, 1);
        assert!(plan.all_live_cutover_operator_receipt_projections_bound_to_decision_preflight);
        assert!(plan.all_live_cutover_operator_decision_preflight_entries_keep_no_invocation_guard);
        assert!(plan.tool_execution_live_cutover_operator_decision_preflight_ready);
        assert!(!plan.tool_execution_live_cutover_operator_decision_acceptance_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);

        let selected = plan
            .entries
            .iter()
            .find(|entry| entry.selected_for_status_canary)
            .expect("selected status canary operator decision entry");
        assert_eq!(
            selected.live_cutover_operator_decision_preflight_route,
            ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::LiveCutoverOperatorDecisionPendingExplicitApproval
        );
        assert!(selected.operator_cutover_decision_pending);

        let preflight_only = plan
            .entries
            .iter()
            .find(|entry| entry.preflight_only_non_selected_candidate)
            .expect("non-selected preflight-only operator decision entry");
        assert_eq!(
            preflight_only.live_cutover_operator_decision_preflight_route,
            ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::PreflightOnlyNonSelectedCandidate
        );
        assert!(!preflight_only.operator_cutover_decision_pending);
    }

    #[test]
    fn live_cutover_operator_decision_preflight_does_not_enable_live_cutover() {
        let plan = hepta_system_tool_execution_live_cutover_operator_decision_preflight_plan();

        assert!(plan.tool_execution_live_cutover_operator_decision_preflight_ready);
        assert!(!plan.tool_execution_live_cutover_operator_decision_acceptance_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.result_receipt_written);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn live_cutover_operator_decision_preflight_fails_closed_without_policy() {
        let projection =
            hepta_system_tool_execution_live_cutover_operator_receipt_projection_plan();
        let input = ToolExecutionLiveCutoverOperatorDecisionPreflightInput {
            operator_cutover_decision_policy_present: false,
            ..ToolExecutionLiveCutoverOperatorDecisionPreflightInput::default()
        };

        let plan =
            tool_execution_live_cutover_operator_decision_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.operator_cutover_decision_policy_present);
        assert_eq!(plan.live_cutover_operator_decision_preflight_ready_count, 0);
        assert_eq!(
            plan.live_cutover_operator_decision_preflight_blocked_count,
            2
        );
        assert!(!plan.tool_execution_live_cutover_operator_decision_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_operator_decision_preflight_route
                == ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::BlockedByMissingDecisionPolicy
        }));
    }

    #[test]
    fn live_cutover_operator_decision_preflight_fails_closed_on_premature_decision_mutation() {
        let projection =
            hepta_system_tool_execution_live_cutover_operator_receipt_projection_plan();
        let input = ToolExecutionLiveCutoverOperatorDecisionPreflightInput {
            approval_request_sent: true,
            operator_cutover_decision_receipt_written: true,
            operator_cutover_readback_evidence_written: true,
            operator_cutover_acceptance_recorded: true,
            ..ToolExecutionLiveCutoverOperatorDecisionPreflightInput::default()
        };

        let plan =
            tool_execution_live_cutover_operator_decision_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.approval_request_sent);
        assert!(plan.operator_cutover_decision_receipt_written);
        assert!(plan.operator_cutover_readback_evidence_written);
        assert!(plan.operator_cutover_acceptance_recorded);
        assert!(!plan.tool_execution_live_cutover_operator_decision_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_operator_decision_preflight_route
                == ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::BlockedByPrematureDecisionMutation
        }));
    }

    #[test]
    fn live_cutover_operator_decision_preflight_fails_closed_when_live_switch_enabled() {
        let projection =
            hepta_system_tool_execution_live_cutover_operator_receipt_projection_plan();
        let input = ToolExecutionLiveCutoverOperatorDecisionPreflightInput {
            live_cutover_switch_enabled: true,
            adapter_dispatch_switch_enabled: true,
            tool_invocation_execution_switch_enabled: true,
            ..ToolExecutionLiveCutoverOperatorDecisionPreflightInput::default()
        };

        let plan =
            tool_execution_live_cutover_operator_decision_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.live_cutover_switch_enabled);
        assert!(plan.adapter_dispatch_switch_enabled);
        assert!(plan.tool_invocation_execution_switch_enabled);
        assert!(!plan.tool_execution_live_cutover_operator_decision_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_operator_decision_preflight_route
                == ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::BlockedByLiveCutoverSwitch
        }));
    }
}
