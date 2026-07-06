use crate::ToolExecutionLiveCutoverOperatorPacketPlan;
use crate::ToolExecutionLiveCutoverOperatorPacketRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_live_cutover_operator_packet_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionLiveCutoverOperatorReceiptProjectionRoute {
    LiveCutoverOperatorReceiptProjectionReady,
    PreflightOnlyNonSelectedCandidate,
    BlockedByOperatorPacket,
    BlockedByMissingReceiptPolicy,
    BlockedByMissingOperatorReadback,
    BlockedByPrematureReceiptMutation,
    BlockedByLiveCutoverSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverOperatorReceiptProjectionInput {
    pub operator_cutover_receipt_policy_present: bool,
    pub operator_cutover_readback_channel_present: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_decision_receipt_written: bool,
    pub operator_cutover_readback_evidence_written: bool,
    pub operator_cutover_acceptance_recorded: bool,
    pub live_cutover_switch_enabled: bool,
}

impl Default for ToolExecutionLiveCutoverOperatorReceiptProjectionInput {
    fn default() -> Self {
        Self {
            operator_cutover_receipt_policy_present: true,
            operator_cutover_readback_channel_present: true,
            approval_request_sent: false,
            operator_cutover_decision_receipt_written: false,
            operator_cutover_readback_evidence_written: false,
            operator_cutover_acceptance_recorded: false,
            live_cutover_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverOperatorReceiptProjectionEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_live_cutover_operator_packet_route: ToolExecutionLiveCutoverOperatorPacketRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub selected_for_status_canary: bool,
    pub preflight_only_non_selected_candidate: bool,
    pub live_cutover_operator_receipt_projection_route:
        ToolExecutionLiveCutoverOperatorReceiptProjectionRoute,
    pub live_cutover_operator_receipt_projection_ready: bool,
    pub operator_cutover_receipt_policy_present: bool,
    pub operator_cutover_readback_channel_present: bool,
    pub operator_cutover_decision_receipt_required: bool,
    pub operator_cutover_decision_readback_evidence_required: bool,
    pub operator_cutover_decision_receipt_write_blocked: bool,
    pub remaining_blocker_readback_required: bool,
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
pub struct ToolExecutionLiveCutoverOperatorReceiptProjectionPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_live_cutover_operator_packet_surface: &'static str,
    pub source_live_cutover_operator_packet_ready: bool,
    pub source_live_cutover_approval_request_allowed: bool,
    pub source_live_cutover_allowed: bool,
    pub operator_cutover_receipt_policy_present: bool,
    pub operator_cutover_readback_channel_present: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_decision_receipt_written: bool,
    pub operator_cutover_readback_evidence_written: bool,
    pub operator_cutover_acceptance_recorded: bool,
    pub live_cutover_switch_enabled: bool,
    pub candidate_count: usize,
    pub live_cutover_operator_receipt_projection_ready_count: usize,
    pub live_cutover_operator_receipt_projection_blocked_count: usize,
    pub operator_cutover_decision_receipt_required_count: usize,
    pub operator_cutover_decision_readback_evidence_required_count: usize,
    pub operator_cutover_decision_receipt_write_blocked_count: usize,
    pub remaining_blocker_readback_required_count: usize,
    pub selected_status_canary_count: usize,
    pub preflight_only_non_selected_count: usize,
    pub all_live_cutover_operator_packets_bound_to_receipt_projection: bool,
    pub all_live_cutover_operator_receipts_keep_no_invocation_guard: bool,
    pub tool_execution_live_cutover_operator_receipt_projection_ready: bool,
    pub tool_execution_live_cutover_operator_decision_write_allowed: bool,
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
    pub entries: Vec<ToolExecutionLiveCutoverOperatorReceiptProjectionEntry>,
}

pub fn hepta_system_tool_execution_live_cutover_operator_receipt_projection_plan()
-> ToolExecutionLiveCutoverOperatorReceiptProjectionPlan {
    let packet = hepta_system_tool_execution_live_cutover_operator_packet_plan();
    tool_execution_live_cutover_operator_receipt_projection_plan(
        &packet,
        &ToolExecutionLiveCutoverOperatorReceiptProjectionInput::default(),
    )
}

pub fn tool_execution_live_cutover_operator_receipt_projection_plan(
    packet: &ToolExecutionLiveCutoverOperatorPacketPlan,
    input: &ToolExecutionLiveCutoverOperatorReceiptProjectionInput,
) -> ToolExecutionLiveCutoverOperatorReceiptProjectionPlan {
    let entries = packet
        .entries
        .iter()
        .map(|entry| {
            let route = if input.live_cutover_switch_enabled {
                ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::BlockedByLiveCutoverSwitch
            } else if input.approval_request_sent
                || input.operator_cutover_decision_receipt_written
                || input.operator_cutover_readback_evidence_written
                || input.operator_cutover_acceptance_recorded
            {
                ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::BlockedByPrematureReceiptMutation
            } else if !input.operator_cutover_receipt_policy_present {
                ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::BlockedByMissingReceiptPolicy
            } else if !input.operator_cutover_readback_channel_present {
                ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::BlockedByMissingOperatorReadback
            } else if entry.preflight_only_non_selected_candidate
                && entry.live_cutover_operator_packet_route
                    == ToolExecutionLiveCutoverOperatorPacketRoute::PreflightOnlyNonSelectedCandidate
            {
                ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::PreflightOnlyNonSelectedCandidate
            } else if !entry.live_cutover_operator_packet_ready
                || entry.live_cutover_operator_packet_route
                    != ToolExecutionLiveCutoverOperatorPacketRoute::LiveCutoverOperatorPacketReadyForReview
            {
                ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::BlockedByOperatorPacket
            } else {
                ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::LiveCutoverOperatorReceiptProjectionReady
            };
            let ready = matches!(
                route,
                ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::LiveCutoverOperatorReceiptProjectionReady
                    | ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::PreflightOnlyNonSelectedCandidate
            )
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && (entry.preflight_only_non_selected_candidate
                    || (entry.operator_review_required
                        && entry.remaining_blocker_readback_required
                        && entry.approval_request_blocked));
            let operator_cutover_decision_receipt_write_blocked = ready
                && entry.selected_for_status_canary
                && !input.approval_request_sent
                && !input.operator_cutover_decision_receipt_written
                && !input.operator_cutover_readback_evidence_written;

            ToolExecutionLiveCutoverOperatorReceiptProjectionEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_live_cutover_operator_packet_route: entry
                    .live_cutover_operator_packet_route,
                registry_guard_route: entry.registry_guard_route,
                selected_for_status_canary: entry.selected_for_status_canary,
                preflight_only_non_selected_candidate: entry.preflight_only_non_selected_candidate,
                live_cutover_operator_receipt_projection_route: route,
                live_cutover_operator_receipt_projection_ready: ready,
                operator_cutover_receipt_policy_present: input
                    .operator_cutover_receipt_policy_present,
                operator_cutover_readback_channel_present: input
                    .operator_cutover_readback_channel_present,
                operator_cutover_decision_receipt_required: entry.selected_for_status_canary,
                operator_cutover_decision_readback_evidence_required: entry
                    .selected_for_status_canary,
                operator_cutover_decision_receipt_write_blocked,
                remaining_blocker_readback_required: entry.remaining_blocker_readback_required,
                approval_request_sent: input.approval_request_sent,
                operator_cutover_decision_receipt_written: input
                    .operator_cutover_decision_receipt_written,
                operator_cutover_readback_evidence_written: input
                    .operator_cutover_readback_evidence_written,
                operator_cutover_acceptance_recorded: input.operator_cutover_acceptance_recorded,
                live_cutover_switch_enabled: input.live_cutover_switch_enabled,
                adapter_dispatch_switch_enabled: false,
                tool_invocation_execution_switch_enabled: false,
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
        .filter(|entry| entry.live_cutover_operator_receipt_projection_ready)
        .count();
    let receipt_required_count = entries
        .iter()
        .filter(|entry| entry.operator_cutover_decision_receipt_required)
        .count();
    let readback_required_count = entries
        .iter()
        .filter(|entry| entry.operator_cutover_decision_readback_evidence_required)
        .count();
    let write_blocked_count = entries
        .iter()
        .filter(|entry| entry.operator_cutover_decision_receipt_write_blocked)
        .count();
    let blocker_readback_count = entries
        .iter()
        .filter(|entry| entry.remaining_blocker_readback_required)
        .count();
    let selected_status_canary_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary)
        .count();
    let preflight_only_non_selected_count = entries
        .iter()
        .filter(|entry| entry.preflight_only_non_selected_candidate)
        .count();
    let all_live_cutover_operator_packets_bound_to_receipt_projection = ready_count
        == entries.len()
        && selected_status_canary_count == 1
        && preflight_only_non_selected_count + selected_status_canary_count == entries.len()
        && receipt_required_count == selected_status_canary_count
        && readback_required_count == selected_status_canary_count
        && write_blocked_count == selected_status_canary_count
        && blocker_readback_count == selected_status_canary_count;
    let all_live_cutover_operator_receipts_keep_no_invocation_guard = entries.iter().all(
        |entry| {
            if matches!(
                entry.live_cutover_operator_receipt_projection_route,
                ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::LiveCutoverOperatorReceiptProjectionReady
                    | ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::PreflightOnlyNonSelectedCandidate
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
        },
    );
    let tool_execution_live_cutover_operator_receipt_projection_ready = packet
        .tool_execution_live_cutover_operator_packet_ready
        && !packet.tool_execution_live_cutover_approval_request_allowed
        && !packet.tool_execution_live_cutover_allowed
        && input.operator_cutover_receipt_policy_present
        && input.operator_cutover_readback_channel_present
        && !input.approval_request_sent
        && !input.operator_cutover_decision_receipt_written
        && !input.operator_cutover_readback_evidence_written
        && !input.operator_cutover_acceptance_recorded
        && !input.live_cutover_switch_enabled
        && all_live_cutover_operator_packets_bound_to_receipt_projection
        && all_live_cutover_operator_receipts_keep_no_invocation_guard;

    ToolExecutionLiveCutoverOperatorReceiptProjectionPlan {
        runtime: "hepta",
        surface: "tool_execution_live_cutover_operator_receipt_projection",
        plugin_id: packet.plugin_id,
        status: if tool_execution_live_cutover_operator_receipt_projection_ready {
            "ready"
        } else {
            "blocked"
        },
        source_live_cutover_operator_packet_surface: packet.surface,
        source_live_cutover_operator_packet_ready: packet
            .tool_execution_live_cutover_operator_packet_ready,
        source_live_cutover_approval_request_allowed: packet
            .tool_execution_live_cutover_approval_request_allowed,
        source_live_cutover_allowed: packet.tool_execution_live_cutover_allowed,
        operator_cutover_receipt_policy_present: input.operator_cutover_receipt_policy_present,
        operator_cutover_readback_channel_present: input.operator_cutover_readback_channel_present,
        approval_request_sent: input.approval_request_sent,
        operator_cutover_decision_receipt_written: input.operator_cutover_decision_receipt_written,
        operator_cutover_readback_evidence_written: input
            .operator_cutover_readback_evidence_written,
        operator_cutover_acceptance_recorded: input.operator_cutover_acceptance_recorded,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        candidate_count: entries.len(),
        live_cutover_operator_receipt_projection_ready_count: ready_count,
        live_cutover_operator_receipt_projection_blocked_count: entries.len() - ready_count,
        operator_cutover_decision_receipt_required_count: receipt_required_count,
        operator_cutover_decision_readback_evidence_required_count: readback_required_count,
        operator_cutover_decision_receipt_write_blocked_count: write_blocked_count,
        remaining_blocker_readback_required_count: blocker_readback_count,
        selected_status_canary_count,
        preflight_only_non_selected_count,
        all_live_cutover_operator_packets_bound_to_receipt_projection,
        all_live_cutover_operator_receipts_keep_no_invocation_guard,
        tool_execution_live_cutover_operator_receipt_projection_ready,
        tool_execution_live_cutover_operator_decision_write_allowed: false,
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
        next_migration_step: "restore_tool_execution_live_cutover_operator_decision_preflight_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_cutover_operator_receipt_projection_collects_receipt_slots() {
        let plan = hepta_system_tool_execution_live_cutover_operator_receipt_projection_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_live_cutover_operator_packet_surface,
            "tool_execution_live_cutover_operator_packet"
        );
        assert!(plan.source_live_cutover_operator_packet_ready);
        assert!(!plan.source_live_cutover_approval_request_allowed);
        assert!(!plan.source_live_cutover_allowed);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.live_cutover_operator_receipt_projection_ready_count, 2);
        assert_eq!(
            plan.live_cutover_operator_receipt_projection_blocked_count,
            0
        );
        assert_eq!(plan.operator_cutover_decision_receipt_required_count, 1);
        assert_eq!(
            plan.operator_cutover_decision_readback_evidence_required_count,
            1
        );
        assert_eq!(
            plan.operator_cutover_decision_receipt_write_blocked_count,
            1
        );
        assert_eq!(plan.remaining_blocker_readback_required_count, 1);
        assert_eq!(plan.selected_status_canary_count, 1);
        assert_eq!(plan.preflight_only_non_selected_count, 1);
        assert!(plan.all_live_cutover_operator_packets_bound_to_receipt_projection);
        assert!(plan.all_live_cutover_operator_receipts_keep_no_invocation_guard);
        assert!(plan.tool_execution_live_cutover_operator_receipt_projection_ready);
        assert!(!plan.tool_execution_live_cutover_operator_decision_write_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);

        let selected = plan
            .entries
            .iter()
            .find(|entry| entry.selected_for_status_canary)
            .expect("selected status canary operator receipt entry");
        assert_eq!(
            selected.live_cutover_operator_receipt_projection_route,
            ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::LiveCutoverOperatorReceiptProjectionReady
        );
        assert!(selected.operator_cutover_decision_receipt_write_blocked);

        let preflight_only = plan
            .entries
            .iter()
            .find(|entry| entry.preflight_only_non_selected_candidate)
            .expect("non-selected preflight-only operator receipt entry");
        assert_eq!(
            preflight_only.live_cutover_operator_receipt_projection_route,
            ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::PreflightOnlyNonSelectedCandidate
        );
        assert!(!preflight_only.operator_cutover_decision_receipt_write_blocked);
    }

    #[test]
    fn live_cutover_operator_receipt_projection_does_not_write_receipts() {
        let plan = hepta_system_tool_execution_live_cutover_operator_receipt_projection_plan();

        assert!(plan.tool_execution_live_cutover_operator_receipt_projection_ready);
        assert!(!plan.approval_request_sent);
        assert!(!plan.operator_cutover_decision_receipt_written);
        assert!(!plan.operator_cutover_readback_evidence_written);
        assert!(!plan.operator_cutover_acceptance_recorded);
        assert!(!plan.tool_execution_live_cutover_operator_decision_write_allowed);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.result_receipt_written);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn live_cutover_operator_receipt_projection_fails_closed_without_policy() {
        let packet = hepta_system_tool_execution_live_cutover_operator_packet_plan();
        let input = ToolExecutionLiveCutoverOperatorReceiptProjectionInput {
            operator_cutover_receipt_policy_present: false,
            ..ToolExecutionLiveCutoverOperatorReceiptProjectionInput::default()
        };

        let plan = tool_execution_live_cutover_operator_receipt_projection_plan(&packet, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.operator_cutover_receipt_policy_present);
        assert_eq!(plan.live_cutover_operator_receipt_projection_ready_count, 0);
        assert_eq!(
            plan.live_cutover_operator_receipt_projection_blocked_count,
            2
        );
        assert!(!plan.tool_execution_live_cutover_operator_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_operator_receipt_projection_route
                == ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::BlockedByMissingReceiptPolicy
        }));
    }

    #[test]
    fn live_cutover_operator_receipt_projection_fails_closed_on_premature_receipt_mutation() {
        let packet = hepta_system_tool_execution_live_cutover_operator_packet_plan();
        let input = ToolExecutionLiveCutoverOperatorReceiptProjectionInput {
            approval_request_sent: true,
            operator_cutover_decision_receipt_written: true,
            operator_cutover_readback_evidence_written: true,
            operator_cutover_acceptance_recorded: true,
            ..ToolExecutionLiveCutoverOperatorReceiptProjectionInput::default()
        };

        let plan = tool_execution_live_cutover_operator_receipt_projection_plan(&packet, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.approval_request_sent);
        assert!(plan.operator_cutover_decision_receipt_written);
        assert!(plan.operator_cutover_readback_evidence_written);
        assert!(plan.operator_cutover_acceptance_recorded);
        assert!(!plan.tool_execution_live_cutover_operator_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_operator_receipt_projection_route
                == ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::BlockedByPrematureReceiptMutation
        }));
    }

    #[test]
    fn live_cutover_operator_receipt_projection_fails_closed_when_live_switch_enabled() {
        let packet = hepta_system_tool_execution_live_cutover_operator_packet_plan();
        let input = ToolExecutionLiveCutoverOperatorReceiptProjectionInput {
            live_cutover_switch_enabled: true,
            ..ToolExecutionLiveCutoverOperatorReceiptProjectionInput::default()
        };

        let plan = tool_execution_live_cutover_operator_receipt_projection_plan(&packet, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.live_cutover_switch_enabled);
        assert!(!plan.tool_execution_live_cutover_operator_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_operator_receipt_projection_route
                == ToolExecutionLiveCutoverOperatorReceiptProjectionRoute::BlockedByLiveCutoverSwitch
        }));
    }
}
