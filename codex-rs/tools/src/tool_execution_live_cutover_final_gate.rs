use crate::ToolExecutionLiveCutoverReceiptRollbackPacketPlan;
use crate::ToolExecutionLiveCutoverReceiptRollbackPacketRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_live_cutover_receipt_rollback_packet_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionLiveCutoverFinalGateRoute {
    LiveCutoverFinalGateReadyBlocked,
    PreflightOnlyNonSelectedCandidate,
    BlockedByReceiptRollbackPacket,
    BlockedByMissingFinalGatePolicy,
    BlockedByMissingFinalOperatorReadback,
    BlockedByPrematureApprovalOrMutation,
    BlockedByExecutionSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverFinalGateInput {
    pub final_gate_policy_present: bool,
    pub final_cutover_ticket_present: bool,
    pub final_operator_readback_required: bool,
    pub explicit_live_cutover_approval_present: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_decision_receipt_written: bool,
    pub operator_cutover_readback_evidence_written: bool,
    pub operator_cutover_acceptance_recorded: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub live_cutover_started: bool,
    pub rollback_executed: bool,
    pub rollback_receipt_written: bool,
    pub result_receipt_written: bool,
}

impl Default for ToolExecutionLiveCutoverFinalGateInput {
    fn default() -> Self {
        Self {
            final_gate_policy_present: true,
            final_cutover_ticket_present: true,
            final_operator_readback_required: true,
            explicit_live_cutover_approval_present: false,
            approval_request_sent: false,
            operator_cutover_decision_receipt_written: false,
            operator_cutover_readback_evidence_written: false,
            operator_cutover_acceptance_recorded: false,
            live_cutover_switch_enabled: false,
            adapter_dispatch_switch_enabled: false,
            tool_invocation_execution_switch_enabled: false,
            live_cutover_started: false,
            rollback_executed: false,
            rollback_receipt_written: false,
            result_receipt_written: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverFinalGateEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_live_cutover_receipt_rollback_packet_route:
        ToolExecutionLiveCutoverReceiptRollbackPacketRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub selected_for_status_canary: bool,
    pub preflight_only_non_selected_candidate: bool,
    pub live_cutover_final_gate_route: ToolExecutionLiveCutoverFinalGateRoute,
    pub live_cutover_final_gate_ready: bool,
    pub final_gate_policy_present: bool,
    pub final_cutover_ticket_present: bool,
    pub final_operator_readback_required: bool,
    pub explicit_live_cutover_approval_required: bool,
    pub explicit_live_cutover_approval_present: bool,
    pub live_cutover_blocked: bool,
    pub approval_request_blocked: bool,
    pub operator_acceptance_blocked: bool,
    pub execution_switch_blocked: bool,
    pub adapter_dispatch_blocked: bool,
    pub tool_invocation_blocked: bool,
    pub ledger_write_blocked: bool,
    pub rollback_execution_blocked: bool,
    pub result_receipt_write_blocked: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_decision_receipt_written: bool,
    pub operator_cutover_readback_evidence_written: bool,
    pub operator_cutover_acceptance_recorded: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub live_cutover_started: bool,
    pub rollback_executed: bool,
    pub rollback_receipt_written: bool,
    pub result_receipt_written: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub execution_adapter_dispatch_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverFinalGatePlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_live_cutover_receipt_rollback_packet_surface: &'static str,
    pub source_live_cutover_receipt_rollback_packet_ready: bool,
    pub source_live_cutover_start_allowed: bool,
    pub source_live_cutover_rollback_allowed: bool,
    pub source_live_cutover_result_receipt_write_allowed: bool,
    pub source_live_cutover_allowed: bool,
    pub final_gate_policy_present: bool,
    pub final_cutover_ticket_present: bool,
    pub final_operator_readback_required: bool,
    pub explicit_live_cutover_approval_present: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_decision_receipt_written: bool,
    pub operator_cutover_readback_evidence_written: bool,
    pub operator_cutover_acceptance_recorded: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub live_cutover_started: bool,
    pub rollback_executed: bool,
    pub rollback_receipt_written: bool,
    pub result_receipt_written: bool,
    pub candidate_count: usize,
    pub live_cutover_final_gate_ready_count: usize,
    pub live_cutover_final_gate_blocked_count: usize,
    pub explicit_live_cutover_approval_required_count: usize,
    pub explicit_live_cutover_approval_missing_count: usize,
    pub final_operator_readback_required_count: usize,
    pub live_cutover_blocked_count: usize,
    pub approval_request_blocked_count: usize,
    pub operator_acceptance_blocked_count: usize,
    pub execution_switch_blocked_count: usize,
    pub rollback_execution_blocked_count: usize,
    pub result_receipt_write_blocked_count: usize,
    pub selected_status_canary_count: usize,
    pub preflight_only_non_selected_count: usize,
    pub all_receipt_rollback_packets_bound_to_final_gate: bool,
    pub all_live_cutover_final_gate_entries_keep_no_invocation_guard: bool,
    pub tool_execution_live_cutover_final_gate_ready: bool,
    pub tool_execution_live_cutover_allowed: bool,
    pub tool_execution_public_ga_allowed: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub execution_adapter_dispatched: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<ToolExecutionLiveCutoverFinalGateEntry>,
}

pub fn hepta_system_tool_execution_live_cutover_final_gate_plan()
-> ToolExecutionLiveCutoverFinalGatePlan {
    let packet = hepta_system_tool_execution_live_cutover_receipt_rollback_packet_plan();
    tool_execution_live_cutover_final_gate_plan(
        &packet,
        &ToolExecutionLiveCutoverFinalGateInput::default(),
    )
}

pub fn tool_execution_live_cutover_final_gate_plan(
    packet: &ToolExecutionLiveCutoverReceiptRollbackPacketPlan,
    input: &ToolExecutionLiveCutoverFinalGateInput,
) -> ToolExecutionLiveCutoverFinalGatePlan {
    let entries = packet
        .entries
        .iter()
        .map(|entry| {
            let route = if input.live_cutover_switch_enabled
                || input.adapter_dispatch_switch_enabled
                || input.tool_invocation_execution_switch_enabled
            {
                ToolExecutionLiveCutoverFinalGateRoute::BlockedByExecutionSwitch
            } else if input.explicit_live_cutover_approval_present
                || input.approval_request_sent
                || input.operator_cutover_decision_receipt_written
                || input.operator_cutover_readback_evidence_written
                || input.operator_cutover_acceptance_recorded
                || input.live_cutover_started
                || input.rollback_executed
                || input.rollback_receipt_written
                || input.result_receipt_written
            {
                ToolExecutionLiveCutoverFinalGateRoute::BlockedByPrematureApprovalOrMutation
            } else if !input.final_gate_policy_present || !input.final_cutover_ticket_present {
                ToolExecutionLiveCutoverFinalGateRoute::BlockedByMissingFinalGatePolicy
            } else if !input.final_operator_readback_required {
                ToolExecutionLiveCutoverFinalGateRoute::BlockedByMissingFinalOperatorReadback
            } else if entry.preflight_only_non_selected_candidate
                && entry.live_cutover_receipt_rollback_packet_route
                    == ToolExecutionLiveCutoverReceiptRollbackPacketRoute::PreflightOnlyNonSelectedCandidate
            {
                ToolExecutionLiveCutoverFinalGateRoute::PreflightOnlyNonSelectedCandidate
            } else if !entry.live_cutover_receipt_rollback_packet_ready
                || entry.live_cutover_receipt_rollback_packet_route
                    != ToolExecutionLiveCutoverReceiptRollbackPacketRoute::LiveCutoverReceiptRollbackPacketReady
            {
                ToolExecutionLiveCutoverFinalGateRoute::BlockedByReceiptRollbackPacket
            } else {
                ToolExecutionLiveCutoverFinalGateRoute::LiveCutoverFinalGateReadyBlocked
            };
            let ready = matches!(
                route,
                ToolExecutionLiveCutoverFinalGateRoute::LiveCutoverFinalGateReadyBlocked
                    | ToolExecutionLiveCutoverFinalGateRoute::PreflightOnlyNonSelectedCandidate
            )
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && (entry.preflight_only_non_selected_candidate
                    || (entry.live_cutover_start_blocked
                        && entry.rollback_execution_blocked
                        && entry.result_receipt_write_blocked));
            let live_cutover_blocked = ready
                && entry.selected_for_status_canary
                && !input.explicit_live_cutover_approval_present
                && !input.live_cutover_switch_enabled
                && !input.live_cutover_started;
            let approval_request_blocked =
                ready && entry.selected_for_status_canary && !input.approval_request_sent;
            let operator_acceptance_blocked = ready
                && entry.selected_for_status_canary
                && !input.operator_cutover_acceptance_recorded;
            let execution_switch_blocked = ready
                && entry.selected_for_status_canary
                && !input.live_cutover_switch_enabled
                && !input.adapter_dispatch_switch_enabled
                && !input.tool_invocation_execution_switch_enabled;
            let rollback_execution_blocked =
                ready && entry.selected_for_status_canary && !input.rollback_executed;
            let result_receipt_write_blocked = ready
                && entry.selected_for_status_canary
                && !input.result_receipt_written
                && !input.rollback_receipt_written;

            ToolExecutionLiveCutoverFinalGateEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_live_cutover_receipt_rollback_packet_route: entry
                    .live_cutover_receipt_rollback_packet_route,
                registry_guard_route: entry.registry_guard_route,
                selected_for_status_canary: entry.selected_for_status_canary,
                preflight_only_non_selected_candidate: entry.preflight_only_non_selected_candidate,
                live_cutover_final_gate_route: route,
                live_cutover_final_gate_ready: ready,
                final_gate_policy_present: input.final_gate_policy_present,
                final_cutover_ticket_present: input.final_cutover_ticket_present,
                final_operator_readback_required: input.final_operator_readback_required
                    && entry.selected_for_status_canary,
                explicit_live_cutover_approval_required: entry.selected_for_status_canary,
                explicit_live_cutover_approval_present: input
                    .explicit_live_cutover_approval_present,
                live_cutover_blocked,
                approval_request_blocked,
                operator_acceptance_blocked,
                execution_switch_blocked,
                adapter_dispatch_blocked: execution_switch_blocked,
                tool_invocation_blocked: execution_switch_blocked,
                ledger_write_blocked: ready && entry.selected_for_status_canary,
                rollback_execution_blocked,
                result_receipt_write_blocked,
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
                live_cutover_started: input.live_cutover_started,
                rollback_executed: input.rollback_executed,
                rollback_receipt_written: input.rollback_receipt_written,
                result_receipt_written: input.result_receipt_written,
                router_registration_lookup_enabled: false,
                registry_lookup_executed: false,
                registry_source_of_truth_enabled: false,
                tool_registration_enabled: false,
                execution_adapter_dispatch_enabled: false,
                tool_invocation_enabled: false,
                ledger_write_enabled: false,
                approval_request_enabled: false,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let ready_count = entries
        .iter()
        .filter(|entry| entry.live_cutover_final_gate_ready)
        .count();
    let approval_required_count = entries
        .iter()
        .filter(|entry| entry.explicit_live_cutover_approval_required)
        .count();
    let approval_missing_count = entries
        .iter()
        .filter(|entry| {
            entry.explicit_live_cutover_approval_required
                && !entry.explicit_live_cutover_approval_present
        })
        .count();
    let readback_count = entries
        .iter()
        .filter(|entry| entry.final_operator_readback_required)
        .count();
    let live_blocked_count = entries
        .iter()
        .filter(|entry| entry.live_cutover_blocked)
        .count();
    let approval_request_blocked_count = entries
        .iter()
        .filter(|entry| entry.approval_request_blocked)
        .count();
    let acceptance_blocked_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_blocked)
        .count();
    let execution_switch_blocked_count = entries
        .iter()
        .filter(|entry| entry.execution_switch_blocked)
        .count();
    let rollback_blocked_count = entries
        .iter()
        .filter(|entry| entry.rollback_execution_blocked)
        .count();
    let receipt_write_blocked_count = entries
        .iter()
        .filter(|entry| entry.result_receipt_write_blocked)
        .count();
    let selected_status_canary_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary)
        .count();
    let preflight_only_non_selected_count = entries
        .iter()
        .filter(|entry| entry.preflight_only_non_selected_candidate)
        .count();
    let all_receipt_rollback_packets_bound_to_final_gate = ready_count == entries.len()
        && selected_status_canary_count == 1
        && preflight_only_non_selected_count + selected_status_canary_count == entries.len()
        && approval_required_count == selected_status_canary_count
        && approval_missing_count == selected_status_canary_count
        && readback_count == selected_status_canary_count
        && live_blocked_count == selected_status_canary_count
        && approval_request_blocked_count == selected_status_canary_count
        && acceptance_blocked_count == selected_status_canary_count
        && execution_switch_blocked_count == selected_status_canary_count
        && rollback_blocked_count == selected_status_canary_count
        && receipt_write_blocked_count == selected_status_canary_count;
    let all_live_cutover_final_gate_entries_keep_no_invocation_guard =
        entries.iter().all(|entry| {
            if matches!(
                entry.live_cutover_final_gate_route,
                ToolExecutionLiveCutoverFinalGateRoute::LiveCutoverFinalGateReadyBlocked
                    | ToolExecutionLiveCutoverFinalGateRoute::PreflightOnlyNonSelectedCandidate
            ) {
                entry.registry_guard_route
                    == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                    && !entry.explicit_live_cutover_approval_present
                    && !entry.approval_request_sent
                    && !entry.operator_cutover_decision_receipt_written
                    && !entry.operator_cutover_readback_evidence_written
                    && !entry.operator_cutover_acceptance_recorded
                    && !entry.live_cutover_switch_enabled
                    && !entry.adapter_dispatch_switch_enabled
                    && !entry.tool_invocation_execution_switch_enabled
                    && !entry.live_cutover_started
                    && !entry.rollback_executed
                    && !entry.rollback_receipt_written
                    && !entry.result_receipt_written
                    && !entry.router_registration_lookup_enabled
                    && !entry.registry_lookup_executed
                    && !entry.registry_source_of_truth_enabled
                    && !entry.tool_registration_enabled
                    && !entry.execution_adapter_dispatch_enabled
                    && !entry.tool_invocation_enabled
                    && !entry.ledger_write_enabled
                    && !entry.approval_request_enabled
            } else {
                true
            }
        });
    let tool_execution_live_cutover_final_gate_ready = packet
        .tool_execution_live_cutover_receipt_rollback_packet_ready
        && !packet.tool_execution_live_cutover_start_allowed
        && !packet.tool_execution_live_cutover_rollback_allowed
        && !packet.tool_execution_live_cutover_result_receipt_write_allowed
        && !packet.tool_execution_live_cutover_allowed
        && input.final_gate_policy_present
        && input.final_cutover_ticket_present
        && input.final_operator_readback_required
        && !input.explicit_live_cutover_approval_present
        && !input.approval_request_sent
        && !input.operator_cutover_decision_receipt_written
        && !input.operator_cutover_readback_evidence_written
        && !input.operator_cutover_acceptance_recorded
        && !input.live_cutover_switch_enabled
        && !input.adapter_dispatch_switch_enabled
        && !input.tool_invocation_execution_switch_enabled
        && !input.live_cutover_started
        && !input.rollback_executed
        && !input.rollback_receipt_written
        && !input.result_receipt_written
        && all_receipt_rollback_packets_bound_to_final_gate
        && all_live_cutover_final_gate_entries_keep_no_invocation_guard;

    ToolExecutionLiveCutoverFinalGatePlan {
        runtime: "hepta",
        surface: "tool_execution_live_cutover_final_gate",
        plugin_id: packet.plugin_id,
        status: if tool_execution_live_cutover_final_gate_ready {
            "ready"
        } else {
            "blocked"
        },
        source_live_cutover_receipt_rollback_packet_surface: packet.surface,
        source_live_cutover_receipt_rollback_packet_ready: packet
            .tool_execution_live_cutover_receipt_rollback_packet_ready,
        source_live_cutover_start_allowed: packet.tool_execution_live_cutover_start_allowed,
        source_live_cutover_rollback_allowed: packet.tool_execution_live_cutover_rollback_allowed,
        source_live_cutover_result_receipt_write_allowed: packet
            .tool_execution_live_cutover_result_receipt_write_allowed,
        source_live_cutover_allowed: packet.tool_execution_live_cutover_allowed,
        final_gate_policy_present: input.final_gate_policy_present,
        final_cutover_ticket_present: input.final_cutover_ticket_present,
        final_operator_readback_required: input.final_operator_readback_required,
        explicit_live_cutover_approval_present: input.explicit_live_cutover_approval_present,
        approval_request_sent: input.approval_request_sent,
        operator_cutover_decision_receipt_written: input.operator_cutover_decision_receipt_written,
        operator_cutover_readback_evidence_written: input
            .operator_cutover_readback_evidence_written,
        operator_cutover_acceptance_recorded: input.operator_cutover_acceptance_recorded,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
        tool_invocation_execution_switch_enabled: input.tool_invocation_execution_switch_enabled,
        live_cutover_started: input.live_cutover_started,
        rollback_executed: input.rollback_executed,
        rollback_receipt_written: input.rollback_receipt_written,
        result_receipt_written: input.result_receipt_written,
        candidate_count: entries.len(),
        live_cutover_final_gate_ready_count: ready_count,
        live_cutover_final_gate_blocked_count: entries.len() - ready_count,
        explicit_live_cutover_approval_required_count: approval_required_count,
        explicit_live_cutover_approval_missing_count: approval_missing_count,
        final_operator_readback_required_count: readback_count,
        live_cutover_blocked_count: live_blocked_count,
        approval_request_blocked_count,
        operator_acceptance_blocked_count: acceptance_blocked_count,
        execution_switch_blocked_count,
        rollback_execution_blocked_count: rollback_blocked_count,
        result_receipt_write_blocked_count: receipt_write_blocked_count,
        selected_status_canary_count,
        preflight_only_non_selected_count,
        all_receipt_rollback_packets_bound_to_final_gate,
        all_live_cutover_final_gate_entries_keep_no_invocation_guard,
        tool_execution_live_cutover_final_gate_ready,
        tool_execution_live_cutover_allowed: false,
        tool_execution_public_ga_allowed: false,
        router_registration_lookup_enabled: false,
        registry_lookup_executed: false,
        registry_source_of_truth_enabled: false,
        tool_registration_enabled: false,
        execution_adapter_dispatched: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "manual_operator_live_cutover_approval_required",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_cutover_final_gate_collects_final_blockers() {
        let plan = hepta_system_tool_execution_live_cutover_final_gate_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_live_cutover_receipt_rollback_packet_surface,
            "tool_execution_live_cutover_receipt_rollback_packet"
        );
        assert!(plan.source_live_cutover_receipt_rollback_packet_ready);
        assert!(!plan.source_live_cutover_start_allowed);
        assert!(!plan.source_live_cutover_rollback_allowed);
        assert!(!plan.source_live_cutover_result_receipt_write_allowed);
        assert!(!plan.source_live_cutover_allowed);
        assert!(plan.final_gate_policy_present);
        assert!(plan.final_cutover_ticket_present);
        assert!(plan.final_operator_readback_required);
        assert!(!plan.explicit_live_cutover_approval_present);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.live_cutover_final_gate_ready_count, 2);
        assert_eq!(plan.live_cutover_final_gate_blocked_count, 0);
        assert_eq!(plan.explicit_live_cutover_approval_required_count, 1);
        assert_eq!(plan.explicit_live_cutover_approval_missing_count, 1);
        assert_eq!(plan.final_operator_readback_required_count, 1);
        assert_eq!(plan.live_cutover_blocked_count, 1);
        assert_eq!(plan.approval_request_blocked_count, 1);
        assert_eq!(plan.operator_acceptance_blocked_count, 1);
        assert_eq!(plan.execution_switch_blocked_count, 1);
        assert_eq!(plan.rollback_execution_blocked_count, 1);
        assert_eq!(plan.result_receipt_write_blocked_count, 1);
        assert_eq!(plan.selected_status_canary_count, 1);
        assert_eq!(plan.preflight_only_non_selected_count, 1);
        assert!(plan.all_receipt_rollback_packets_bound_to_final_gate);
        assert!(plan.all_live_cutover_final_gate_entries_keep_no_invocation_guard);
        assert!(plan.tool_execution_live_cutover_final_gate_ready);
        assert!(!plan.tool_execution_live_cutover_allowed);
        assert!(!plan.tool_execution_public_ga_allowed);

        let selected = plan
            .entries
            .iter()
            .find(|entry| entry.selected_for_status_canary)
            .expect("selected status canary final gate entry");
        assert_eq!(
            selected.live_cutover_final_gate_route,
            ToolExecutionLiveCutoverFinalGateRoute::LiveCutoverFinalGateReadyBlocked
        );
        assert!(selected.live_cutover_blocked);
        assert!(selected.approval_request_blocked);

        let preflight_only = plan
            .entries
            .iter()
            .find(|entry| entry.preflight_only_non_selected_candidate)
            .expect("non-selected preflight-only final gate entry");
        assert_eq!(
            preflight_only.live_cutover_final_gate_route,
            ToolExecutionLiveCutoverFinalGateRoute::PreflightOnlyNonSelectedCandidate
        );
        assert!(!preflight_only.live_cutover_blocked);
        assert!(!preflight_only.approval_request_blocked);
    }

    #[test]
    fn live_cutover_final_gate_does_not_enable_live_mutation() {
        let plan = hepta_system_tool_execution_live_cutover_final_gate_plan();

        assert!(plan.tool_execution_live_cutover_final_gate_ready);
        assert!(!plan.tool_execution_live_cutover_allowed);
        assert!(!plan.tool_execution_public_ga_allowed);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn live_cutover_final_gate_fails_closed_without_policy() {
        let packet = hepta_system_tool_execution_live_cutover_receipt_rollback_packet_plan();
        let input = ToolExecutionLiveCutoverFinalGateInput {
            final_gate_policy_present: false,
            final_cutover_ticket_present: false,
            ..ToolExecutionLiveCutoverFinalGateInput::default()
        };

        let plan = tool_execution_live_cutover_final_gate_plan(&packet, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.final_gate_policy_present);
        assert!(!plan.final_cutover_ticket_present);
        assert_eq!(plan.live_cutover_final_gate_ready_count, 0);
        assert_eq!(plan.live_cutover_final_gate_blocked_count, 2);
        assert!(!plan.tool_execution_live_cutover_final_gate_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_final_gate_route
                == ToolExecutionLiveCutoverFinalGateRoute::BlockedByMissingFinalGatePolicy
        }));
    }

    #[test]
    fn live_cutover_final_gate_fails_closed_on_premature_approval_or_mutation() {
        let packet = hepta_system_tool_execution_live_cutover_receipt_rollback_packet_plan();
        let input = ToolExecutionLiveCutoverFinalGateInput {
            explicit_live_cutover_approval_present: true,
            approval_request_sent: true,
            operator_cutover_decision_receipt_written: true,
            operator_cutover_readback_evidence_written: true,
            operator_cutover_acceptance_recorded: true,
            live_cutover_started: true,
            rollback_executed: true,
            rollback_receipt_written: true,
            result_receipt_written: true,
            ..ToolExecutionLiveCutoverFinalGateInput::default()
        };

        let plan = tool_execution_live_cutover_final_gate_plan(&packet, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.explicit_live_cutover_approval_present);
        assert!(plan.approval_request_sent);
        assert!(plan.live_cutover_started);
        assert!(!plan.tool_execution_live_cutover_final_gate_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_final_gate_route
                == ToolExecutionLiveCutoverFinalGateRoute::BlockedByPrematureApprovalOrMutation
        }));
    }

    #[test]
    fn live_cutover_final_gate_fails_closed_when_execution_switch_enabled() {
        let packet = hepta_system_tool_execution_live_cutover_receipt_rollback_packet_plan();
        let input = ToolExecutionLiveCutoverFinalGateInput {
            live_cutover_switch_enabled: true,
            adapter_dispatch_switch_enabled: true,
            tool_invocation_execution_switch_enabled: true,
            ..ToolExecutionLiveCutoverFinalGateInput::default()
        };

        let plan = tool_execution_live_cutover_final_gate_plan(&packet, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.live_cutover_switch_enabled);
        assert!(plan.adapter_dispatch_switch_enabled);
        assert!(plan.tool_invocation_execution_switch_enabled);
        assert!(!plan.tool_execution_live_cutover_final_gate_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_final_gate_route
                == ToolExecutionLiveCutoverFinalGateRoute::BlockedByExecutionSwitch
        }));
    }
}
