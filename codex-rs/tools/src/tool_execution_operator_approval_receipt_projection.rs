use crate::ToolExecutionOperatorApprovalPacketPlan;
use crate::ToolExecutionOperatorApprovalPacketRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_operator_approval_packet_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionOperatorApprovalReceiptProjectionRoute {
    OperatorApprovalReceiptProjectionReady,
    BlockedByOperatorApprovalPacket,
    BlockedByMissingReceiptProjection,
    BlockedByMissingReadbackEvidenceSlot,
    BlockedByPrematureDecisionMutation,
    BlockedByLiveCutoverSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionOperatorApprovalReceiptProjectionInput {
    pub operator_decision_receipt_projection_present: bool,
    pub operator_decision_readback_evidence_slot_present: bool,
    pub operator_decision_record_written: bool,
    pub operator_decision_receipt_written: bool,
    pub operator_acceptance_present: bool,
    pub approval_request_sent: bool,
    pub live_cutover_switch_enabled: bool,
}

impl Default for ToolExecutionOperatorApprovalReceiptProjectionInput {
    fn default() -> Self {
        Self {
            operator_decision_receipt_projection_present: true,
            operator_decision_readback_evidence_slot_present: true,
            operator_decision_record_written: false,
            operator_decision_receipt_written: false,
            operator_acceptance_present: false,
            approval_request_sent: false,
            live_cutover_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionOperatorApprovalReceiptProjectionEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_operator_approval_packet_route: ToolExecutionOperatorApprovalPacketRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub operator_approval_receipt_projection_route:
        ToolExecutionOperatorApprovalReceiptProjectionRoute,
    pub operator_approval_receipt_projection_ready: bool,
    pub operator_decision_receipt_required: bool,
    pub operator_decision_readback_evidence_required: bool,
    pub operator_decision_receipt_write_blocked: bool,
    pub operator_decision_receipt_projection_present: bool,
    pub operator_decision_readback_evidence_slot_present: bool,
    pub operator_decision_record_written: bool,
    pub operator_decision_receipt_written: bool,
    pub operator_acceptance_present: bool,
    pub approval_request_sent: bool,
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
pub struct ToolExecutionOperatorApprovalReceiptProjectionPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_operator_approval_packet_surface: &'static str,
    pub source_operator_approval_packet_ready: bool,
    pub source_approval_request_allowed: bool,
    pub source_live_cutover_allowed: bool,
    pub operator_decision_receipt_projection_present: bool,
    pub operator_decision_readback_evidence_slot_present: bool,
    pub operator_decision_record_written: bool,
    pub operator_decision_receipt_written: bool,
    pub operator_acceptance_present: bool,
    pub approval_request_sent: bool,
    pub live_cutover_switch_enabled: bool,
    pub candidate_count: usize,
    pub operator_approval_receipt_projection_ready_count: usize,
    pub operator_approval_receipt_projection_blocked_count: usize,
    pub operator_decision_receipt_required_count: usize,
    pub operator_decision_readback_evidence_required_count: usize,
    pub operator_decision_receipt_write_blocked_count: usize,
    pub all_operator_packets_bound_to_receipt_projection: bool,
    pub all_operator_receipt_projections_keep_approval_guard: bool,
    pub tool_execution_operator_approval_receipt_projection_ready: bool,
    pub tool_execution_operator_decision_write_allowed: bool,
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
    pub entries: Vec<ToolExecutionOperatorApprovalReceiptProjectionEntry>,
}

pub fn hepta_system_tool_execution_operator_approval_receipt_projection_plan()
-> ToolExecutionOperatorApprovalReceiptProjectionPlan {
    let packet = hepta_system_tool_execution_operator_approval_packet_plan();
    tool_execution_operator_approval_receipt_projection_plan(
        &packet,
        &ToolExecutionOperatorApprovalReceiptProjectionInput::default(),
    )
}

pub fn tool_execution_operator_approval_receipt_projection_plan(
    packet: &ToolExecutionOperatorApprovalPacketPlan,
    input: &ToolExecutionOperatorApprovalReceiptProjectionInput,
) -> ToolExecutionOperatorApprovalReceiptProjectionPlan {
    let entries = packet
        .entries
        .iter()
        .map(|entry| {
            let operator_approval_receipt_projection_route =
                if input.live_cutover_switch_enabled {
                    ToolExecutionOperatorApprovalReceiptProjectionRoute::BlockedByLiveCutoverSwitch
                } else if input.operator_decision_record_written
                    || input.operator_decision_receipt_written
                    || input.operator_acceptance_present
                    || input.approval_request_sent
                {
                    ToolExecutionOperatorApprovalReceiptProjectionRoute::BlockedByPrematureDecisionMutation
                } else if !input.operator_decision_receipt_projection_present {
                    ToolExecutionOperatorApprovalReceiptProjectionRoute::BlockedByMissingReceiptProjection
                } else if !input.operator_decision_readback_evidence_slot_present {
                    ToolExecutionOperatorApprovalReceiptProjectionRoute::BlockedByMissingReadbackEvidenceSlot
                } else if !entry.operator_approval_packet_ready
                    || entry.operator_approval_packet_route
                        != ToolExecutionOperatorApprovalPacketRoute::OperatorApprovalPacketReadyForReview
                {
                    ToolExecutionOperatorApprovalReceiptProjectionRoute::BlockedByOperatorApprovalPacket
                } else {
                    ToolExecutionOperatorApprovalReceiptProjectionRoute::OperatorApprovalReceiptProjectionReady
                };
            let operator_approval_receipt_projection_ready =
                operator_approval_receipt_projection_route
                    == ToolExecutionOperatorApprovalReceiptProjectionRoute::OperatorApprovalReceiptProjectionReady
                    && entry.registry_guard_route
                        == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                    && entry.operator_review_required
                    && entry.approval_request_blocked;
            let operator_decision_receipt_required =
                operator_approval_receipt_projection_ready;
            let operator_decision_readback_evidence_required =
                operator_approval_receipt_projection_ready;
            let operator_decision_receipt_write_blocked =
                operator_approval_receipt_projection_ready
                    && !input.operator_decision_record_written
                    && !input.operator_decision_receipt_written
                    && !input.operator_acceptance_present
                    && !input.approval_request_sent;

            ToolExecutionOperatorApprovalReceiptProjectionEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_operator_approval_packet_route: entry.operator_approval_packet_route,
                registry_guard_route: entry.registry_guard_route,
                operator_approval_receipt_projection_route,
                operator_approval_receipt_projection_ready,
                operator_decision_receipt_required,
                operator_decision_readback_evidence_required,
                operator_decision_receipt_write_blocked,
                operator_decision_receipt_projection_present: input
                    .operator_decision_receipt_projection_present,
                operator_decision_readback_evidence_slot_present: input
                    .operator_decision_readback_evidence_slot_present,
                operator_decision_record_written: input.operator_decision_record_written,
                operator_decision_receipt_written: input.operator_decision_receipt_written,
                operator_acceptance_present: input.operator_acceptance_present,
                approval_request_sent: input.approval_request_sent,
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

    let operator_approval_receipt_projection_ready_count = entries
        .iter()
        .filter(|entry| entry.operator_approval_receipt_projection_ready)
        .count();
    let operator_decision_receipt_required_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_receipt_required)
        .count();
    let operator_decision_readback_evidence_required_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_readback_evidence_required)
        .count();
    let operator_decision_receipt_write_blocked_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_receipt_write_blocked)
        .count();
    let operator_approval_receipt_projection_blocked_count =
        entries.len() - operator_approval_receipt_projection_ready_count;
    let all_operator_packets_bound_to_receipt_projection =
        operator_approval_receipt_projection_ready_count == entries.len()
            && operator_decision_receipt_required_count == entries.len()
            && operator_decision_readback_evidence_required_count == entries.len()
            && operator_decision_receipt_write_blocked_count == entries.len();
    let all_operator_receipt_projections_keep_approval_guard = entries.iter().all(|entry| {
        if entry.operator_approval_receipt_projection_route
            == ToolExecutionOperatorApprovalReceiptProjectionRoute::OperatorApprovalReceiptProjectionReady
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.operator_decision_record_written
                && !entry.operator_decision_receipt_written
                && !entry.operator_acceptance_present
                && !entry.approval_request_sent
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
    let tool_execution_operator_approval_receipt_projection_ready = packet
        .tool_execution_operator_approval_packet_ready
        && !packet.tool_execution_operator_approval_request_allowed
        && !packet.tool_execution_live_cutover_allowed
        && input.operator_decision_receipt_projection_present
        && input.operator_decision_readback_evidence_slot_present
        && !input.operator_decision_record_written
        && !input.operator_decision_receipt_written
        && !input.operator_acceptance_present
        && !input.approval_request_sent
        && !input.live_cutover_switch_enabled
        && all_operator_packets_bound_to_receipt_projection
        && all_operator_receipt_projections_keep_approval_guard;

    ToolExecutionOperatorApprovalReceiptProjectionPlan {
        runtime: "hepta",
        surface: "tool_execution_operator_approval_receipt_projection",
        plugin_id: packet.plugin_id,
        status: if tool_execution_operator_approval_receipt_projection_ready {
            "ready"
        } else {
            "blocked"
        },
        source_operator_approval_packet_surface: packet.surface,
        source_operator_approval_packet_ready: packet.tool_execution_operator_approval_packet_ready,
        source_approval_request_allowed: packet.tool_execution_operator_approval_request_allowed,
        source_live_cutover_allowed: packet.tool_execution_live_cutover_allowed,
        operator_decision_receipt_projection_present: input
            .operator_decision_receipt_projection_present,
        operator_decision_readback_evidence_slot_present: input
            .operator_decision_readback_evidence_slot_present,
        operator_decision_record_written: input.operator_decision_record_written,
        operator_decision_receipt_written: input.operator_decision_receipt_written,
        operator_acceptance_present: input.operator_acceptance_present,
        approval_request_sent: input.approval_request_sent,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        candidate_count: entries.len(),
        operator_approval_receipt_projection_ready_count,
        operator_approval_receipt_projection_blocked_count,
        operator_decision_receipt_required_count,
        operator_decision_readback_evidence_required_count,
        operator_decision_receipt_write_blocked_count,
        all_operator_packets_bound_to_receipt_projection,
        all_operator_receipt_projections_keep_approval_guard,
        tool_execution_operator_approval_receipt_projection_ready,
        tool_execution_operator_decision_write_allowed: false,
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
    fn tool_execution_operator_approval_receipt_projection_collects_receipt_slots() {
        let plan = hepta_system_tool_execution_operator_approval_receipt_projection_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_operator_approval_packet_surface,
            "tool_execution_operator_approval_packet"
        );
        assert!(plan.source_operator_approval_packet_ready);
        assert!(!plan.source_approval_request_allowed);
        assert!(!plan.source_live_cutover_allowed);
        assert!(plan.operator_decision_receipt_projection_present);
        assert!(plan.operator_decision_readback_evidence_slot_present);
        assert!(!plan.operator_decision_record_written);
        assert!(!plan.operator_decision_receipt_written);
        assert!(!plan.operator_acceptance_present);
        assert!(!plan.approval_request_sent);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.operator_approval_receipt_projection_ready_count, 2);
        assert_eq!(plan.operator_approval_receipt_projection_blocked_count, 0);
        assert_eq!(plan.operator_decision_receipt_required_count, 2);
        assert_eq!(plan.operator_decision_readback_evidence_required_count, 2);
        assert_eq!(plan.operator_decision_receipt_write_blocked_count, 2);
        assert!(plan.all_operator_packets_bound_to_receipt_projection);
        assert!(plan.all_operator_receipt_projections_keep_approval_guard);
        assert!(plan.tool_execution_operator_approval_receipt_projection_ready);
        assert!(!plan.tool_execution_operator_decision_write_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_receipt_projection_route
                == ToolExecutionOperatorApprovalReceiptProjectionRoute::OperatorApprovalReceiptProjectionReady
                && entry.operator_approval_receipt_projection_ready
                && entry.operator_decision_receipt_required
                && entry.operator_decision_readback_evidence_required
                && entry.operator_decision_receipt_write_blocked
        }));
    }

    #[test]
    fn tool_execution_operator_approval_receipt_projection_does_not_write_decisions() {
        let plan = hepta_system_tool_execution_operator_approval_receipt_projection_plan();

        assert!(plan.tool_execution_operator_approval_receipt_projection_ready);
        assert!(!plan.tool_execution_operator_decision_write_allowed);
        assert!(!plan.router_registration_lookup_enabled);
        assert!(!plan.registry_lookup_executed);
        assert!(!plan.registry_source_of_truth_enabled);
        assert!(!plan.tool_registration_enabled);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.operator_decision_record_written_flag);
        assert!(!plan.operator_decision_receipt_written_flag);
        assert!(!plan.result_receipt_written);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
        assert!(plan.entries.iter().all(|entry| {
            !entry.operator_decision_record_written
                && !entry.operator_decision_receipt_written
                && !entry.operator_acceptance_present
                && !entry.approval_request_sent
                && !entry.live_cutover_switch_enabled
                && !entry.execution_adapter_dispatch_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.result_receipt_write_enabled
        }));
    }

    #[test]
    fn tool_execution_operator_approval_receipt_projection_fails_closed_without_projection() {
        let packet = hepta_system_tool_execution_operator_approval_packet_plan();
        let input = ToolExecutionOperatorApprovalReceiptProjectionInput {
            operator_decision_receipt_projection_present: false,
            operator_decision_readback_evidence_slot_present: true,
            operator_decision_record_written: false,
            operator_decision_receipt_written: false,
            operator_acceptance_present: false,
            approval_request_sent: false,
            live_cutover_switch_enabled: false,
        };

        let plan = tool_execution_operator_approval_receipt_projection_plan(&packet, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.operator_decision_receipt_projection_present);
        assert_eq!(plan.operator_approval_receipt_projection_ready_count, 0);
        assert_eq!(plan.operator_approval_receipt_projection_blocked_count, 2);
        assert!(!plan.tool_execution_operator_approval_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_receipt_projection_route
                == ToolExecutionOperatorApprovalReceiptProjectionRoute::BlockedByMissingReceiptProjection
        }));
    }

    #[test]
    fn tool_execution_operator_approval_receipt_projection_fails_closed_on_premature_decision_mutation()
     {
        let packet = hepta_system_tool_execution_operator_approval_packet_plan();
        let input = ToolExecutionOperatorApprovalReceiptProjectionInput {
            operator_decision_receipt_projection_present: true,
            operator_decision_readback_evidence_slot_present: true,
            operator_decision_record_written: true,
            operator_decision_receipt_written: true,
            operator_acceptance_present: true,
            approval_request_sent: true,
            live_cutover_switch_enabled: false,
        };

        let plan = tool_execution_operator_approval_receipt_projection_plan(&packet, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.operator_decision_record_written);
        assert!(plan.operator_decision_receipt_written);
        assert!(plan.operator_acceptance_present);
        assert!(plan.approval_request_sent);
        assert_eq!(plan.operator_approval_receipt_projection_ready_count, 0);
        assert_eq!(plan.operator_approval_receipt_projection_blocked_count, 2);
        assert!(!plan.tool_execution_operator_approval_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_receipt_projection_route
                == ToolExecutionOperatorApprovalReceiptProjectionRoute::BlockedByPrematureDecisionMutation
        }));
    }

    #[test]
    fn tool_execution_operator_approval_receipt_projection_fails_closed_when_live_cutover_switch_enabled()
     {
        let packet = hepta_system_tool_execution_operator_approval_packet_plan();
        let input = ToolExecutionOperatorApprovalReceiptProjectionInput {
            operator_decision_receipt_projection_present: true,
            operator_decision_readback_evidence_slot_present: true,
            operator_decision_record_written: false,
            operator_decision_receipt_written: false,
            operator_acceptance_present: false,
            approval_request_sent: false,
            live_cutover_switch_enabled: true,
        };

        let plan = tool_execution_operator_approval_receipt_projection_plan(&packet, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.live_cutover_switch_enabled);
        assert_eq!(plan.operator_approval_receipt_projection_ready_count, 0);
        assert_eq!(plan.operator_approval_receipt_projection_blocked_count, 2);
        assert!(!plan.tool_execution_operator_approval_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_receipt_projection_route
                == ToolExecutionOperatorApprovalReceiptProjectionRoute::BlockedByLiveCutoverSwitch
        }));
    }
}
