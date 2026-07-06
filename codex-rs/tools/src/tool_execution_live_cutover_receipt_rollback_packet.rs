use crate::ToolExecutionLiveCutoverOperatorDecisionPreflightPlan;
use crate::ToolExecutionLiveCutoverOperatorDecisionPreflightRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_live_cutover_operator_decision_preflight_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionLiveCutoverReceiptRollbackPacketRoute {
    LiveCutoverReceiptRollbackPacketReady,
    PreflightOnlyNonSelectedCandidate,
    BlockedByOperatorDecisionPreflight,
    BlockedByMissingRollbackAnchor,
    BlockedByMissingRollbackReadback,
    BlockedByMissingResultReceiptSchema,
    BlockedByMissingKillSwitch,
    BlockedByPrematureLiveCutoverMutation,
    BlockedByExecutionSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverReceiptRollbackPacketInput {
    pub rollback_anchor_present: bool,
    pub rollback_readback_channel_present: bool,
    pub result_receipt_schema_present: bool,
    pub operator_summary_template_present: bool,
    pub kill_switch_present: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_decision_receipt_written: bool,
    pub operator_cutover_readback_evidence_written: bool,
    pub operator_cutover_acceptance_recorded: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub live_cutover_started: bool,
    pub result_receipt_written: bool,
    pub rollback_executed: bool,
    pub rollback_receipt_written: bool,
}

impl Default for ToolExecutionLiveCutoverReceiptRollbackPacketInput {
    fn default() -> Self {
        Self {
            rollback_anchor_present: true,
            rollback_readback_channel_present: true,
            result_receipt_schema_present: true,
            operator_summary_template_present: true,
            kill_switch_present: true,
            approval_request_sent: false,
            operator_cutover_decision_receipt_written: false,
            operator_cutover_readback_evidence_written: false,
            operator_cutover_acceptance_recorded: false,
            live_cutover_switch_enabled: false,
            adapter_dispatch_switch_enabled: false,
            tool_invocation_execution_switch_enabled: false,
            live_cutover_started: false,
            result_receipt_written: false,
            rollback_executed: false,
            rollback_receipt_written: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverReceiptRollbackPacketEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_live_cutover_operator_decision_preflight_route:
        ToolExecutionLiveCutoverOperatorDecisionPreflightRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub selected_for_status_canary: bool,
    pub preflight_only_non_selected_candidate: bool,
    pub live_cutover_receipt_rollback_packet_route:
        ToolExecutionLiveCutoverReceiptRollbackPacketRoute,
    pub live_cutover_receipt_rollback_packet_ready: bool,
    pub rollback_anchor_present: bool,
    pub rollback_readback_channel_present: bool,
    pub result_receipt_schema_present: bool,
    pub operator_summary_template_present: bool,
    pub kill_switch_present: bool,
    pub rollback_plan_required: bool,
    pub rollback_readback_required: bool,
    pub result_receipt_required: bool,
    pub rollback_receipt_required: bool,
    pub operator_summary_required: bool,
    pub live_cutover_start_blocked: bool,
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
    pub result_receipt_written: bool,
    pub rollback_executed: bool,
    pub rollback_receipt_written: bool,
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
pub struct ToolExecutionLiveCutoverReceiptRollbackPacketPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_live_cutover_operator_decision_preflight_surface: &'static str,
    pub source_live_cutover_operator_decision_preflight_ready: bool,
    pub source_live_cutover_operator_decision_acceptance_allowed: bool,
    pub source_live_cutover_allowed: bool,
    pub rollback_anchor_present: bool,
    pub rollback_readback_channel_present: bool,
    pub result_receipt_schema_present: bool,
    pub operator_summary_template_present: bool,
    pub kill_switch_present: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_decision_receipt_written: bool,
    pub operator_cutover_readback_evidence_written: bool,
    pub operator_cutover_acceptance_recorded: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub live_cutover_started: bool,
    pub result_receipt_written: bool,
    pub rollback_executed: bool,
    pub rollback_receipt_written: bool,
    pub candidate_count: usize,
    pub live_cutover_receipt_rollback_packet_ready_count: usize,
    pub live_cutover_receipt_rollback_packet_blocked_count: usize,
    pub rollback_anchor_present_count: usize,
    pub rollback_readback_required_count: usize,
    pub result_receipt_required_count: usize,
    pub rollback_receipt_required_count: usize,
    pub operator_summary_required_count: usize,
    pub live_cutover_start_blocked_count: usize,
    pub rollback_execution_blocked_count: usize,
    pub result_receipt_write_blocked_count: usize,
    pub selected_status_canary_count: usize,
    pub preflight_only_non_selected_count: usize,
    pub all_live_cutover_operator_decision_preflight_entries_bound_to_receipt_rollback_packet: bool,
    pub all_live_cutover_receipt_rollback_packets_keep_no_invocation_guard: bool,
    pub tool_execution_live_cutover_receipt_rollback_packet_ready: bool,
    pub tool_execution_live_cutover_start_allowed: bool,
    pub tool_execution_live_cutover_rollback_allowed: bool,
    pub tool_execution_live_cutover_result_receipt_write_allowed: bool,
    pub tool_execution_live_cutover_allowed: bool,
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
    pub entries: Vec<ToolExecutionLiveCutoverReceiptRollbackPacketEntry>,
}

pub fn hepta_system_tool_execution_live_cutover_receipt_rollback_packet_plan()
-> ToolExecutionLiveCutoverReceiptRollbackPacketPlan {
    let decision = hepta_system_tool_execution_live_cutover_operator_decision_preflight_plan();
    tool_execution_live_cutover_receipt_rollback_packet_plan(
        &decision,
        &ToolExecutionLiveCutoverReceiptRollbackPacketInput::default(),
    )
}

pub fn tool_execution_live_cutover_receipt_rollback_packet_plan(
    decision: &ToolExecutionLiveCutoverOperatorDecisionPreflightPlan,
    input: &ToolExecutionLiveCutoverReceiptRollbackPacketInput,
) -> ToolExecutionLiveCutoverReceiptRollbackPacketPlan {
    let entries = decision
        .entries
        .iter()
        .map(|entry| {
            let route = if input.live_cutover_switch_enabled
                || input.adapter_dispatch_switch_enabled
                || input.tool_invocation_execution_switch_enabled
            {
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::BlockedByExecutionSwitch
            } else if input.approval_request_sent
                || input.operator_cutover_decision_receipt_written
                || input.operator_cutover_readback_evidence_written
                || input.operator_cutover_acceptance_recorded
                || input.live_cutover_started
                || input.result_receipt_written
                || input.rollback_executed
                || input.rollback_receipt_written
            {
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::BlockedByPrematureLiveCutoverMutation
            } else if !input.rollback_anchor_present {
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::BlockedByMissingRollbackAnchor
            } else if !input.rollback_readback_channel_present {
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::BlockedByMissingRollbackReadback
            } else if !input.result_receipt_schema_present
                || !input.operator_summary_template_present
            {
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::BlockedByMissingResultReceiptSchema
            } else if !input.kill_switch_present {
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::BlockedByMissingKillSwitch
            } else if entry.preflight_only_non_selected_candidate
                && entry.live_cutover_operator_decision_preflight_route
                    == ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::PreflightOnlyNonSelectedCandidate
            {
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::PreflightOnlyNonSelectedCandidate
            } else if !entry.live_cutover_operator_decision_preflight_ready
                || entry.live_cutover_operator_decision_preflight_route
                    != ToolExecutionLiveCutoverOperatorDecisionPreflightRoute::LiveCutoverOperatorDecisionPendingExplicitApproval
            {
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::BlockedByOperatorDecisionPreflight
            } else {
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::LiveCutoverReceiptRollbackPacketReady
            };
            let ready = matches!(
                route,
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::LiveCutoverReceiptRollbackPacketReady
                    | ToolExecutionLiveCutoverReceiptRollbackPacketRoute::PreflightOnlyNonSelectedCandidate
            )
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && (entry.preflight_only_non_selected_candidate
                    || (entry.operator_cutover_decision_pending
                        && entry.operator_cutover_decision_write_blocked
                        && entry.operator_cutover_acceptance_blocked));
            let live_cutover_start_blocked = ready
                && entry.selected_for_status_canary
                && !input.operator_cutover_acceptance_recorded
                && !input.live_cutover_switch_enabled
                && !input.live_cutover_started;
            let rollback_execution_blocked =
                ready && entry.selected_for_status_canary && !input.rollback_executed;
            let result_receipt_write_blocked = ready
                && entry.selected_for_status_canary
                && !input.result_receipt_written
                && !input.rollback_receipt_written;

            ToolExecutionLiveCutoverReceiptRollbackPacketEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_live_cutover_operator_decision_preflight_route: entry
                    .live_cutover_operator_decision_preflight_route,
                registry_guard_route: entry.registry_guard_route,
                selected_for_status_canary: entry.selected_for_status_canary,
                preflight_only_non_selected_candidate: entry.preflight_only_non_selected_candidate,
                live_cutover_receipt_rollback_packet_route: route,
                live_cutover_receipt_rollback_packet_ready: ready,
                rollback_anchor_present: input.rollback_anchor_present,
                rollback_readback_channel_present: input.rollback_readback_channel_present,
                result_receipt_schema_present: input.result_receipt_schema_present,
                operator_summary_template_present: input.operator_summary_template_present,
                kill_switch_present: input.kill_switch_present,
                rollback_plan_required: entry.selected_for_status_canary,
                rollback_readback_required: entry.selected_for_status_canary,
                result_receipt_required: entry.selected_for_status_canary,
                rollback_receipt_required: entry.selected_for_status_canary,
                operator_summary_required: entry.selected_for_status_canary,
                live_cutover_start_blocked,
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
                result_receipt_written: input.result_receipt_written,
                rollback_executed: input.rollback_executed,
                rollback_receipt_written: input.rollback_receipt_written,
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
        .filter(|entry| entry.live_cutover_receipt_rollback_packet_ready)
        .count();
    let rollback_anchor_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_present)
        .count();
    let rollback_readback_count = entries
        .iter()
        .filter(|entry| {
            entry.rollback_plan_required
                && entry.rollback_readback_required
                && entry.rollback_readback_channel_present
        })
        .count();
    let result_receipt_count = entries
        .iter()
        .filter(|entry| entry.result_receipt_required && entry.result_receipt_schema_present)
        .count();
    let rollback_receipt_count = entries
        .iter()
        .filter(|entry| entry.rollback_receipt_required && entry.rollback_readback_channel_present)
        .count();
    let operator_summary_count = entries
        .iter()
        .filter(|entry| entry.operator_summary_required && entry.operator_summary_template_present)
        .count();
    let start_blocked_count = entries
        .iter()
        .filter(|entry| entry.live_cutover_start_blocked)
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
    let all_live_cutover_operator_decision_preflight_entries_bound_to_receipt_rollback_packet =
        ready_count == entries.len()
            && rollback_anchor_count == entries.len()
            && selected_status_canary_count == 1
            && preflight_only_non_selected_count + selected_status_canary_count == entries.len()
            && rollback_readback_count == selected_status_canary_count
            && result_receipt_count == selected_status_canary_count
            && rollback_receipt_count == selected_status_canary_count
            && operator_summary_count == selected_status_canary_count
            && start_blocked_count == selected_status_canary_count
            && rollback_blocked_count == selected_status_canary_count
            && receipt_write_blocked_count == selected_status_canary_count;
    let all_live_cutover_receipt_rollback_packets_keep_no_invocation_guard =
        entries.iter().all(|entry| {
            if matches!(
                entry.live_cutover_receipt_rollback_packet_route,
                ToolExecutionLiveCutoverReceiptRollbackPacketRoute::LiveCutoverReceiptRollbackPacketReady
                    | ToolExecutionLiveCutoverReceiptRollbackPacketRoute::PreflightOnlyNonSelectedCandidate
            ) {
                entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                    && !entry.approval_request_sent
                    && !entry.operator_cutover_decision_receipt_written
                    && !entry.operator_cutover_readback_evidence_written
                    && !entry.operator_cutover_acceptance_recorded
                    && !entry.live_cutover_switch_enabled
                    && !entry.adapter_dispatch_switch_enabled
                    && !entry.tool_invocation_execution_switch_enabled
                    && !entry.live_cutover_started
                    && !entry.result_receipt_written
                    && !entry.rollback_executed
                    && !entry.rollback_receipt_written
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
    let tool_execution_live_cutover_receipt_rollback_packet_ready = decision
        .tool_execution_live_cutover_operator_decision_preflight_ready
        && !decision.tool_execution_live_cutover_operator_decision_acceptance_allowed
        && !decision.tool_execution_live_cutover_allowed
        && input.rollback_anchor_present
        && input.rollback_readback_channel_present
        && input.result_receipt_schema_present
        && input.operator_summary_template_present
        && input.kill_switch_present
        && !input.approval_request_sent
        && !input.operator_cutover_decision_receipt_written
        && !input.operator_cutover_readback_evidence_written
        && !input.operator_cutover_acceptance_recorded
        && !input.live_cutover_switch_enabled
        && !input.adapter_dispatch_switch_enabled
        && !input.tool_invocation_execution_switch_enabled
        && !input.live_cutover_started
        && !input.result_receipt_written
        && !input.rollback_executed
        && !input.rollback_receipt_written
        && all_live_cutover_operator_decision_preflight_entries_bound_to_receipt_rollback_packet
        && all_live_cutover_receipt_rollback_packets_keep_no_invocation_guard;

    ToolExecutionLiveCutoverReceiptRollbackPacketPlan {
        runtime: "hepta",
        surface: "tool_execution_live_cutover_receipt_rollback_packet",
        plugin_id: decision.plugin_id,
        status: if tool_execution_live_cutover_receipt_rollback_packet_ready {
            "ready"
        } else {
            "blocked"
        },
        source_live_cutover_operator_decision_preflight_surface: decision.surface,
        source_live_cutover_operator_decision_preflight_ready: decision
            .tool_execution_live_cutover_operator_decision_preflight_ready,
        source_live_cutover_operator_decision_acceptance_allowed: decision
            .tool_execution_live_cutover_operator_decision_acceptance_allowed,
        source_live_cutover_allowed: decision.tool_execution_live_cutover_allowed,
        rollback_anchor_present: input.rollback_anchor_present,
        rollback_readback_channel_present: input.rollback_readback_channel_present,
        result_receipt_schema_present: input.result_receipt_schema_present,
        operator_summary_template_present: input.operator_summary_template_present,
        kill_switch_present: input.kill_switch_present,
        approval_request_sent: input.approval_request_sent,
        operator_cutover_decision_receipt_written: input.operator_cutover_decision_receipt_written,
        operator_cutover_readback_evidence_written: input
            .operator_cutover_readback_evidence_written,
        operator_cutover_acceptance_recorded: input.operator_cutover_acceptance_recorded,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
        tool_invocation_execution_switch_enabled: input.tool_invocation_execution_switch_enabled,
        live_cutover_started: input.live_cutover_started,
        result_receipt_written: input.result_receipt_written,
        rollback_executed: input.rollback_executed,
        rollback_receipt_written: input.rollback_receipt_written,
        candidate_count: entries.len(),
        live_cutover_receipt_rollback_packet_ready_count: ready_count,
        live_cutover_receipt_rollback_packet_blocked_count: entries.len() - ready_count,
        rollback_anchor_present_count: rollback_anchor_count,
        rollback_readback_required_count: rollback_readback_count,
        result_receipt_required_count: result_receipt_count,
        rollback_receipt_required_count: rollback_receipt_count,
        operator_summary_required_count: operator_summary_count,
        live_cutover_start_blocked_count: start_blocked_count,
        rollback_execution_blocked_count: rollback_blocked_count,
        result_receipt_write_blocked_count: receipt_write_blocked_count,
        selected_status_canary_count,
        preflight_only_non_selected_count,
        all_live_cutover_operator_decision_preflight_entries_bound_to_receipt_rollback_packet,
        all_live_cutover_receipt_rollback_packets_keep_no_invocation_guard,
        tool_execution_live_cutover_receipt_rollback_packet_ready,
        tool_execution_live_cutover_start_allowed: false,
        tool_execution_live_cutover_rollback_allowed: false,
        tool_execution_live_cutover_result_receipt_write_allowed: false,
        tool_execution_live_cutover_allowed: false,
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
        next_migration_step: "restore_tool_execution_live_cutover_final_gate_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_cutover_receipt_rollback_packet_collects_packet_requirements() {
        let plan = hepta_system_tool_execution_live_cutover_receipt_rollback_packet_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_live_cutover_operator_decision_preflight_surface,
            "tool_execution_live_cutover_operator_decision_preflight"
        );
        assert!(plan.source_live_cutover_operator_decision_preflight_ready);
        assert!(!plan.source_live_cutover_operator_decision_acceptance_allowed);
        assert!(!plan.source_live_cutover_allowed);
        assert!(plan.rollback_anchor_present);
        assert!(plan.rollback_readback_channel_present);
        assert!(plan.result_receipt_schema_present);
        assert!(plan.operator_summary_template_present);
        assert!(plan.kill_switch_present);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.live_cutover_receipt_rollback_packet_ready_count, 2);
        assert_eq!(plan.live_cutover_receipt_rollback_packet_blocked_count, 0);
        assert_eq!(plan.rollback_anchor_present_count, 2);
        assert_eq!(plan.rollback_readback_required_count, 1);
        assert_eq!(plan.result_receipt_required_count, 1);
        assert_eq!(plan.rollback_receipt_required_count, 1);
        assert_eq!(plan.operator_summary_required_count, 1);
        assert_eq!(plan.live_cutover_start_blocked_count, 1);
        assert_eq!(plan.rollback_execution_blocked_count, 1);
        assert_eq!(plan.result_receipt_write_blocked_count, 1);
        assert_eq!(plan.selected_status_canary_count, 1);
        assert_eq!(plan.preflight_only_non_selected_count, 1);
        assert!(
            plan.all_live_cutover_operator_decision_preflight_entries_bound_to_receipt_rollback_packet
        );
        assert!(plan.all_live_cutover_receipt_rollback_packets_keep_no_invocation_guard);
        assert!(plan.tool_execution_live_cutover_receipt_rollback_packet_ready);
        assert!(!plan.tool_execution_live_cutover_start_allowed);
        assert!(!plan.tool_execution_live_cutover_rollback_allowed);
        assert!(!plan.tool_execution_live_cutover_result_receipt_write_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);

        let selected = plan
            .entries
            .iter()
            .find(|entry| entry.selected_for_status_canary)
            .expect("selected status canary rollback packet entry");
        assert_eq!(
            selected.live_cutover_receipt_rollback_packet_route,
            ToolExecutionLiveCutoverReceiptRollbackPacketRoute::LiveCutoverReceiptRollbackPacketReady
        );
        assert!(selected.live_cutover_start_blocked);
        assert!(selected.rollback_execution_blocked);
        assert!(selected.result_receipt_write_blocked);

        let preflight_only = plan
            .entries
            .iter()
            .find(|entry| entry.preflight_only_non_selected_candidate)
            .expect("non-selected preflight-only rollback packet entry");
        assert_eq!(
            preflight_only.live_cutover_receipt_rollback_packet_route,
            ToolExecutionLiveCutoverReceiptRollbackPacketRoute::PreflightOnlyNonSelectedCandidate
        );
        assert!(!preflight_only.live_cutover_start_blocked);
        assert!(!preflight_only.rollback_execution_blocked);
        assert!(!preflight_only.result_receipt_write_blocked);
    }

    #[test]
    fn live_cutover_receipt_rollback_packet_does_not_write_or_start() {
        let plan = hepta_system_tool_execution_live_cutover_receipt_rollback_packet_plan();

        assert!(plan.tool_execution_live_cutover_receipt_rollback_packet_ready);
        assert!(!plan.tool_execution_live_cutover_start_allowed);
        assert!(!plan.tool_execution_live_cutover_rollback_allowed);
        assert!(!plan.tool_execution_live_cutover_result_receipt_write_allowed);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.result_receipt_written);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn live_cutover_receipt_rollback_packet_fails_closed_without_rollback_anchor() {
        let decision = hepta_system_tool_execution_live_cutover_operator_decision_preflight_plan();
        let input = ToolExecutionLiveCutoverReceiptRollbackPacketInput {
            rollback_anchor_present: false,
            ..ToolExecutionLiveCutoverReceiptRollbackPacketInput::default()
        };

        let plan = tool_execution_live_cutover_receipt_rollback_packet_plan(&decision, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.rollback_anchor_present);
        assert_eq!(plan.live_cutover_receipt_rollback_packet_ready_count, 0);
        assert_eq!(plan.live_cutover_receipt_rollback_packet_blocked_count, 2);
        assert!(!plan.tool_execution_live_cutover_receipt_rollback_packet_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_receipt_rollback_packet_route
                == ToolExecutionLiveCutoverReceiptRollbackPacketRoute::BlockedByMissingRollbackAnchor
        }));
    }

    #[test]
    fn live_cutover_receipt_rollback_packet_fails_closed_on_premature_live_mutation() {
        let decision = hepta_system_tool_execution_live_cutover_operator_decision_preflight_plan();
        let input = ToolExecutionLiveCutoverReceiptRollbackPacketInput {
            approval_request_sent: true,
            operator_cutover_decision_receipt_written: true,
            operator_cutover_readback_evidence_written: true,
            operator_cutover_acceptance_recorded: true,
            live_cutover_started: true,
            result_receipt_written: true,
            rollback_executed: true,
            rollback_receipt_written: true,
            ..ToolExecutionLiveCutoverReceiptRollbackPacketInput::default()
        };

        let plan = tool_execution_live_cutover_receipt_rollback_packet_plan(&decision, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.approval_request_sent);
        assert!(plan.live_cutover_started);
        assert!(plan.result_receipt_written);
        assert!(plan.rollback_executed);
        assert!(!plan.tool_execution_live_cutover_receipt_rollback_packet_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_receipt_rollback_packet_route
                == ToolExecutionLiveCutoverReceiptRollbackPacketRoute::BlockedByPrematureLiveCutoverMutation
        }));
    }

    #[test]
    fn live_cutover_receipt_rollback_packet_fails_closed_when_execution_switch_enabled() {
        let decision = hepta_system_tool_execution_live_cutover_operator_decision_preflight_plan();
        let input = ToolExecutionLiveCutoverReceiptRollbackPacketInput {
            live_cutover_switch_enabled: true,
            adapter_dispatch_switch_enabled: true,
            tool_invocation_execution_switch_enabled: true,
            ..ToolExecutionLiveCutoverReceiptRollbackPacketInput::default()
        };

        let plan = tool_execution_live_cutover_receipt_rollback_packet_plan(&decision, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.live_cutover_switch_enabled);
        assert!(plan.adapter_dispatch_switch_enabled);
        assert!(plan.tool_invocation_execution_switch_enabled);
        assert!(!plan.tool_execution_live_cutover_receipt_rollback_packet_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_receipt_rollback_packet_route
                == ToolExecutionLiveCutoverReceiptRollbackPacketRoute::BlockedByExecutionSwitch
        }));
    }
}
