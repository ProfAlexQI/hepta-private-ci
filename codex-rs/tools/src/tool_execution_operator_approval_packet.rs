use crate::ToolExecutionCutoverPreflightPlan;
use crate::ToolExecutionCutoverPreflightRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_cutover_preflight_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionOperatorApprovalPacketRoute {
    OperatorApprovalPacketReadyForReview,
    BlockedByCutoverPreflight,
    BlockedByMissingPacketTemplate,
    BlockedByMissingOperatorSessionBinding,
    BlockedByPrematureApprovalMutation,
    BlockedByLiveCutoverSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionOperatorApprovalPacketInput {
    pub operator_packet_template_present: bool,
    pub operator_session_binding_present: bool,
    pub approval_request_sent: bool,
    pub operator_approval_record_written: bool,
    pub operator_acceptance_present: bool,
    pub live_cutover_switch_enabled: bool,
}

impl Default for ToolExecutionOperatorApprovalPacketInput {
    fn default() -> Self {
        Self {
            operator_packet_template_present: true,
            operator_session_binding_present: true,
            approval_request_sent: false,
            operator_approval_record_written: false,
            operator_acceptance_present: false,
            live_cutover_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionOperatorApprovalPacketEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_cutover_preflight_route: ToolExecutionCutoverPreflightRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub operator_approval_packet_route: ToolExecutionOperatorApprovalPacketRoute,
    pub operator_approval_packet_ready: bool,
    pub operator_review_required: bool,
    pub approval_request_blocked: bool,
    pub operator_packet_template_present: bool,
    pub operator_session_binding_present: bool,
    pub approval_request_sent: bool,
    pub operator_approval_record_written: bool,
    pub operator_acceptance_present: bool,
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
pub struct ToolExecutionOperatorApprovalPacketPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_cutover_preflight_surface: &'static str,
    pub source_cutover_preflight_ready: bool,
    pub source_live_cutover_allowed: bool,
    pub operator_packet_template_present: bool,
    pub operator_session_binding_present: bool,
    pub approval_request_sent: bool,
    pub operator_approval_record_written: bool,
    pub operator_acceptance_present: bool,
    pub live_cutover_switch_enabled: bool,
    pub candidate_count: usize,
    pub operator_approval_packet_ready_count: usize,
    pub operator_approval_packet_blocked_count: usize,
    pub operator_review_required_count: usize,
    pub approval_request_blocked_count: usize,
    pub all_cutover_preflight_entries_bound_to_operator_packet: bool,
    pub all_operator_packets_keep_approval_guard: bool,
    pub tool_execution_operator_approval_packet_ready: bool,
    pub tool_execution_operator_approval_request_allowed: bool,
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
    pub entries: Vec<ToolExecutionOperatorApprovalPacketEntry>,
}

pub fn hepta_system_tool_execution_operator_approval_packet_plan()
-> ToolExecutionOperatorApprovalPacketPlan {
    let preflight = hepta_system_tool_execution_cutover_preflight_plan();
    tool_execution_operator_approval_packet_plan(
        &preflight,
        &ToolExecutionOperatorApprovalPacketInput::default(),
    )
}

pub fn tool_execution_operator_approval_packet_plan(
    preflight: &ToolExecutionCutoverPreflightPlan,
    input: &ToolExecutionOperatorApprovalPacketInput,
) -> ToolExecutionOperatorApprovalPacketPlan {
    let entries = preflight
        .entries
        .iter()
        .map(|entry| {
            let operator_approval_packet_route = if input.live_cutover_switch_enabled {
                ToolExecutionOperatorApprovalPacketRoute::BlockedByLiveCutoverSwitch
            } else if input.approval_request_sent
                || input.operator_approval_record_written
                || input.operator_acceptance_present
            {
                ToolExecutionOperatorApprovalPacketRoute::BlockedByPrematureApprovalMutation
            } else if !input.operator_packet_template_present {
                ToolExecutionOperatorApprovalPacketRoute::BlockedByMissingPacketTemplate
            } else if !input.operator_session_binding_present {
                ToolExecutionOperatorApprovalPacketRoute::BlockedByMissingOperatorSessionBinding
            } else if !entry.cutover_preflight_ready
                || entry.cutover_preflight_route
                    != ToolExecutionCutoverPreflightRoute::CutoverPreflightBlockedUntilExplicitApproval
            {
                ToolExecutionOperatorApprovalPacketRoute::BlockedByCutoverPreflight
            } else {
                ToolExecutionOperatorApprovalPacketRoute::OperatorApprovalPacketReadyForReview
            };
            let operator_approval_packet_ready = operator_approval_packet_route
                == ToolExecutionOperatorApprovalPacketRoute::OperatorApprovalPacketReadyForReview
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && entry.explicit_cutover_approval_required
                && entry.live_cutover_blocked;
            let operator_review_required = operator_approval_packet_ready;
            let approval_request_blocked = operator_approval_packet_ready
                && !input.approval_request_sent
                && !input.operator_approval_record_written
                && !input.operator_acceptance_present;

            ToolExecutionOperatorApprovalPacketEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_cutover_preflight_route: entry.cutover_preflight_route,
                registry_guard_route: entry.registry_guard_route,
                operator_approval_packet_route,
                operator_approval_packet_ready,
                operator_review_required,
                approval_request_blocked,
                operator_packet_template_present: input.operator_packet_template_present,
                operator_session_binding_present: input.operator_session_binding_present,
                approval_request_sent: input.approval_request_sent,
                operator_approval_record_written: input.operator_approval_record_written,
                operator_acceptance_present: input.operator_acceptance_present,
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

    let operator_approval_packet_ready_count = entries
        .iter()
        .filter(|entry| entry.operator_approval_packet_ready)
        .count();
    let operator_review_required_count = entries
        .iter()
        .filter(|entry| entry.operator_review_required)
        .count();
    let approval_request_blocked_count = entries
        .iter()
        .filter(|entry| entry.approval_request_blocked)
        .count();
    let operator_approval_packet_blocked_count =
        entries.len() - operator_approval_packet_ready_count;
    let all_cutover_preflight_entries_bound_to_operator_packet =
        operator_approval_packet_ready_count == entries.len()
            && operator_review_required_count == entries.len()
            && approval_request_blocked_count == entries.len();
    let all_operator_packets_keep_approval_guard = entries.iter().all(|entry| {
        if entry.operator_approval_packet_route
            == ToolExecutionOperatorApprovalPacketRoute::OperatorApprovalPacketReadyForReview
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.approval_request_sent
                && !entry.operator_approval_record_written
                && !entry.operator_acceptance_present
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
    let tool_execution_operator_approval_packet_ready = preflight
        .tool_execution_cutover_preflight_ready
        && !preflight.tool_execution_live_cutover_allowed
        && input.operator_packet_template_present
        && input.operator_session_binding_present
        && !input.approval_request_sent
        && !input.operator_approval_record_written
        && !input.operator_acceptance_present
        && !input.live_cutover_switch_enabled
        && all_cutover_preflight_entries_bound_to_operator_packet
        && all_operator_packets_keep_approval_guard;

    ToolExecutionOperatorApprovalPacketPlan {
        runtime: "hepta",
        surface: "tool_execution_operator_approval_packet",
        plugin_id: preflight.plugin_id,
        status: if tool_execution_operator_approval_packet_ready {
            "ready"
        } else {
            "blocked"
        },
        source_cutover_preflight_surface: preflight.surface,
        source_cutover_preflight_ready: preflight.tool_execution_cutover_preflight_ready,
        source_live_cutover_allowed: preflight.tool_execution_live_cutover_allowed,
        operator_packet_template_present: input.operator_packet_template_present,
        operator_session_binding_present: input.operator_session_binding_present,
        approval_request_sent: input.approval_request_sent,
        operator_approval_record_written: input.operator_approval_record_written,
        operator_acceptance_present: input.operator_acceptance_present,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        candidate_count: entries.len(),
        operator_approval_packet_ready_count,
        operator_approval_packet_blocked_count,
        operator_review_required_count,
        approval_request_blocked_count,
        all_cutover_preflight_entries_bound_to_operator_packet,
        all_operator_packets_keep_approval_guard,
        tool_execution_operator_approval_packet_ready,
        tool_execution_operator_approval_request_allowed: false,
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
        next_migration_step: "restore_tool_execution_operator_approval_decision_preflight_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_execution_operator_approval_packet_collects_review_items() {
        let plan = hepta_system_tool_execution_operator_approval_packet_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_cutover_preflight_surface,
            "tool_execution_cutover_preflight"
        );
        assert!(plan.source_cutover_preflight_ready);
        assert!(!plan.source_live_cutover_allowed);
        assert!(plan.operator_packet_template_present);
        assert!(plan.operator_session_binding_present);
        assert!(!plan.approval_request_sent);
        assert!(!plan.operator_approval_record_written);
        assert!(!plan.operator_acceptance_present);
        assert!(!plan.live_cutover_switch_enabled);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.operator_approval_packet_ready_count, 2);
        assert_eq!(plan.operator_approval_packet_blocked_count, 0);
        assert_eq!(plan.operator_review_required_count, 2);
        assert_eq!(plan.approval_request_blocked_count, 2);
        assert!(plan.all_cutover_preflight_entries_bound_to_operator_packet);
        assert!(plan.all_operator_packets_keep_approval_guard);
        assert!(plan.tool_execution_operator_approval_packet_ready);
        assert!(!plan.tool_execution_operator_approval_request_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_packet_route
                == ToolExecutionOperatorApprovalPacketRoute::OperatorApprovalPacketReadyForReview
                && entry.operator_approval_packet_ready
                && entry.operator_review_required
                && entry.approval_request_blocked
        }));
    }

    #[test]
    fn tool_execution_operator_approval_packet_does_not_mutate_approval_state() {
        let plan = hepta_system_tool_execution_operator_approval_packet_plan();

        assert!(plan.tool_execution_operator_approval_packet_ready);
        assert!(!plan.tool_execution_operator_approval_request_allowed);
        assert!(!plan.router_registration_lookup_enabled);
        assert!(!plan.registry_lookup_executed);
        assert!(!plan.registry_source_of_truth_enabled);
        assert!(!plan.tool_registration_enabled);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.result_receipt_written);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
        assert!(plan.entries.iter().all(|entry| {
            !entry.approval_request_sent
                && !entry.operator_approval_record_written
                && !entry.operator_acceptance_present
                && !entry.live_cutover_switch_enabled
                && !entry.execution_adapter_dispatch_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.result_receipt_write_enabled
        }));
    }

    #[test]
    fn tool_execution_operator_approval_packet_fails_closed_without_template() {
        let preflight = hepta_system_tool_execution_cutover_preflight_plan();
        let input = ToolExecutionOperatorApprovalPacketInput {
            operator_packet_template_present: false,
            operator_session_binding_present: true,
            approval_request_sent: false,
            operator_approval_record_written: false,
            operator_acceptance_present: false,
            live_cutover_switch_enabled: false,
        };

        let plan = tool_execution_operator_approval_packet_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.operator_packet_template_present);
        assert_eq!(plan.operator_approval_packet_ready_count, 0);
        assert_eq!(plan.operator_approval_packet_blocked_count, 2);
        assert!(!plan.tool_execution_operator_approval_packet_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_packet_route
                == ToolExecutionOperatorApprovalPacketRoute::BlockedByMissingPacketTemplate
        }));
    }

    #[test]
    fn tool_execution_operator_approval_packet_fails_closed_on_premature_approval_mutation() {
        let preflight = hepta_system_tool_execution_cutover_preflight_plan();
        let input = ToolExecutionOperatorApprovalPacketInput {
            operator_packet_template_present: true,
            operator_session_binding_present: true,
            approval_request_sent: true,
            operator_approval_record_written: true,
            operator_acceptance_present: true,
            live_cutover_switch_enabled: false,
        };

        let plan = tool_execution_operator_approval_packet_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.approval_request_sent);
        assert!(plan.operator_approval_record_written);
        assert!(plan.operator_acceptance_present);
        assert_eq!(plan.operator_approval_packet_ready_count, 0);
        assert_eq!(plan.operator_approval_packet_blocked_count, 2);
        assert!(!plan.tool_execution_operator_approval_packet_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_packet_route
                == ToolExecutionOperatorApprovalPacketRoute::BlockedByPrematureApprovalMutation
        }));
    }

    #[test]
    fn tool_execution_operator_approval_packet_fails_closed_when_live_cutover_switch_enabled() {
        let preflight = hepta_system_tool_execution_cutover_preflight_plan();
        let input = ToolExecutionOperatorApprovalPacketInput {
            operator_packet_template_present: true,
            operator_session_binding_present: true,
            approval_request_sent: false,
            operator_approval_record_written: false,
            operator_acceptance_present: false,
            live_cutover_switch_enabled: true,
        };

        let plan = tool_execution_operator_approval_packet_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.live_cutover_switch_enabled);
        assert_eq!(plan.operator_approval_packet_ready_count, 0);
        assert_eq!(plan.operator_approval_packet_blocked_count, 2);
        assert!(!plan.tool_execution_operator_approval_packet_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.operator_approval_packet_route
                == ToolExecutionOperatorApprovalPacketRoute::BlockedByLiveCutoverSwitch
        }));
    }
}
