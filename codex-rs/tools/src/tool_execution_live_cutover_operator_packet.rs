use crate::ToolExecutionLiveCutoverPreflightPlan;
use crate::ToolExecutionLiveCutoverPreflightRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_live_cutover_preflight_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionLiveCutoverOperatorPacketRoute {
    LiveCutoverOperatorPacketReadyForReview,
    PreflightOnlyNonSelectedCandidate,
    BlockedByLiveCutoverPreflight,
    BlockedByMissingOperatorPacketTemplate,
    BlockedByMissingOperatorSessionBinding,
    BlockedByPrematureOperatorPacketMutation,
    BlockedByLiveCutoverSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverOperatorPacketInput {
    pub operator_packet_template_present: bool,
    pub operator_session_binding_present: bool,
    pub operator_packet_rendered: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_approval_record_written: bool,
    pub operator_cutover_acceptance_present: bool,
    pub live_cutover_switch_enabled: bool,
}

impl Default for ToolExecutionLiveCutoverOperatorPacketInput {
    fn default() -> Self {
        Self {
            operator_packet_template_present: true,
            operator_session_binding_present: true,
            operator_packet_rendered: false,
            approval_request_sent: false,
            operator_cutover_approval_record_written: false,
            operator_cutover_acceptance_present: false,
            live_cutover_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverOperatorPacketEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_live_cutover_preflight_route: ToolExecutionLiveCutoverPreflightRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub selected_for_status_canary: bool,
    pub preflight_only_non_selected_candidate: bool,
    pub live_cutover_operator_packet_route: ToolExecutionLiveCutoverOperatorPacketRoute,
    pub live_cutover_operator_packet_ready: bool,
    pub operator_review_required: bool,
    pub remaining_blocker_readback_required: bool,
    pub approval_request_blocked: bool,
    pub explicit_live_cutover_approval_required: bool,
    pub explicit_live_cutover_approval_present: bool,
    pub operator_packet_template_present: bool,
    pub operator_session_binding_present: bool,
    pub operator_packet_rendered: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_approval_record_written: bool,
    pub operator_cutover_acceptance_present: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub live_cutover_started: bool,
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
pub struct ToolExecutionLiveCutoverOperatorPacketPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_live_cutover_preflight_surface: &'static str,
    pub source_live_cutover_preflight_ready: bool,
    pub source_live_cutover_allowed: bool,
    pub operator_packet_template_present: bool,
    pub operator_session_binding_present: bool,
    pub operator_packet_rendered: bool,
    pub approval_request_sent: bool,
    pub operator_cutover_approval_record_written: bool,
    pub operator_cutover_acceptance_present: bool,
    pub live_cutover_switch_enabled: bool,
    pub candidate_count: usize,
    pub live_cutover_operator_packet_ready_count: usize,
    pub live_cutover_operator_packet_blocked_count: usize,
    pub operator_review_required_count: usize,
    pub remaining_blocker_readback_required_count: usize,
    pub approval_request_blocked_count: usize,
    pub selected_status_canary_count: usize,
    pub preflight_only_non_selected_count: usize,
    pub all_live_cutover_preflight_entries_bound_to_operator_packet: bool,
    pub all_live_cutover_operator_packets_keep_no_invocation_guard: bool,
    pub tool_execution_live_cutover_operator_packet_ready: bool,
    pub tool_execution_live_cutover_approval_request_allowed: bool,
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
    pub entries: Vec<ToolExecutionLiveCutoverOperatorPacketEntry>,
}

pub fn hepta_system_tool_execution_live_cutover_operator_packet_plan()
-> ToolExecutionLiveCutoverOperatorPacketPlan {
    let preflight = hepta_system_tool_execution_live_cutover_preflight_plan();
    tool_execution_live_cutover_operator_packet_plan(
        &preflight,
        &ToolExecutionLiveCutoverOperatorPacketInput::default(),
    )
}

pub fn tool_execution_live_cutover_operator_packet_plan(
    preflight: &ToolExecutionLiveCutoverPreflightPlan,
    input: &ToolExecutionLiveCutoverOperatorPacketInput,
) -> ToolExecutionLiveCutoverOperatorPacketPlan {
    let entries = preflight
        .entries
        .iter()
        .map(|entry| {
            let route = if input.live_cutover_switch_enabled {
                ToolExecutionLiveCutoverOperatorPacketRoute::BlockedByLiveCutoverSwitch
            } else if input.operator_packet_rendered
                || input.approval_request_sent
                || input.operator_cutover_approval_record_written
                || input.operator_cutover_acceptance_present
            {
                ToolExecutionLiveCutoverOperatorPacketRoute::BlockedByPrematureOperatorPacketMutation
            } else if !input.operator_packet_template_present {
                ToolExecutionLiveCutoverOperatorPacketRoute::BlockedByMissingOperatorPacketTemplate
            } else if !input.operator_session_binding_present {
                ToolExecutionLiveCutoverOperatorPacketRoute::BlockedByMissingOperatorSessionBinding
            } else if entry.preflight_only_non_selected_candidate
                && entry.live_cutover_preflight_route
                    == ToolExecutionLiveCutoverPreflightRoute::PreflightOnlyNonSelectedCandidate
            {
                ToolExecutionLiveCutoverOperatorPacketRoute::PreflightOnlyNonSelectedCandidate
            } else if !entry.live_cutover_preflight_ready
                || entry.live_cutover_preflight_route
                    != ToolExecutionLiveCutoverPreflightRoute::LiveCutoverPreflightReadyPendingApproval
            {
                ToolExecutionLiveCutoverOperatorPacketRoute::BlockedByLiveCutoverPreflight
            } else {
                ToolExecutionLiveCutoverOperatorPacketRoute::LiveCutoverOperatorPacketReadyForReview
            };
            let ready = matches!(
                route,
                ToolExecutionLiveCutoverOperatorPacketRoute::LiveCutoverOperatorPacketReadyForReview
                    | ToolExecutionLiveCutoverOperatorPacketRoute::PreflightOnlyNonSelectedCandidate
            )
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && (entry.preflight_only_non_selected_candidate
                    || (entry.live_cutover_blocked
                        && entry.explicit_live_cutover_approval_required
                        && !entry.explicit_live_cutover_approval_present));
            let operator_review_required = ready && entry.selected_for_status_canary;
            let remaining_blocker_readback_required = ready && entry.selected_for_status_canary;
            let approval_request_blocked = ready
                && entry.selected_for_status_canary
                && !input.operator_packet_rendered
                && !input.approval_request_sent
                && !input.operator_cutover_approval_record_written;

            ToolExecutionLiveCutoverOperatorPacketEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_live_cutover_preflight_route: entry.live_cutover_preflight_route,
                registry_guard_route: entry.registry_guard_route,
                selected_for_status_canary: entry.selected_for_status_canary,
                preflight_only_non_selected_candidate: entry.preflight_only_non_selected_candidate,
                live_cutover_operator_packet_route: route,
                live_cutover_operator_packet_ready: ready,
                operator_review_required,
                remaining_blocker_readback_required,
                approval_request_blocked,
                explicit_live_cutover_approval_required: entry
                    .explicit_live_cutover_approval_required,
                explicit_live_cutover_approval_present: entry
                    .explicit_live_cutover_approval_present,
                operator_packet_template_present: input.operator_packet_template_present,
                operator_session_binding_present: input.operator_session_binding_present,
                operator_packet_rendered: input.operator_packet_rendered,
                approval_request_sent: input.approval_request_sent,
                operator_cutover_approval_record_written: input
                    .operator_cutover_approval_record_written,
                operator_cutover_acceptance_present: input.operator_cutover_acceptance_present,
                live_cutover_switch_enabled: input.live_cutover_switch_enabled,
                adapter_dispatch_switch_enabled: false,
                tool_invocation_execution_switch_enabled: false,
                live_cutover_started: false,
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
        .filter(|entry| entry.live_cutover_operator_packet_ready)
        .count();
    let review_count = entries
        .iter()
        .filter(|entry| entry.operator_review_required)
        .count();
    let readback_count = entries
        .iter()
        .filter(|entry| entry.remaining_blocker_readback_required)
        .count();
    let request_blocked_count = entries
        .iter()
        .filter(|entry| entry.approval_request_blocked)
        .count();
    let selected_status_canary_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary)
        .count();
    let preflight_only_non_selected_count = entries
        .iter()
        .filter(|entry| entry.preflight_only_non_selected_candidate)
        .count();
    let all_live_cutover_preflight_entries_bound_to_operator_packet = ready_count == entries.len()
        && selected_status_canary_count == 1
        && preflight_only_non_selected_count + selected_status_canary_count == entries.len()
        && review_count == selected_status_canary_count
        && readback_count == selected_status_canary_count
        && request_blocked_count == selected_status_canary_count;
    let all_live_cutover_operator_packets_keep_no_invocation_guard = entries.iter().all(|entry| {
        if matches!(
            entry.live_cutover_operator_packet_route,
            ToolExecutionLiveCutoverOperatorPacketRoute::LiveCutoverOperatorPacketReadyForReview
                | ToolExecutionLiveCutoverOperatorPacketRoute::PreflightOnlyNonSelectedCandidate
        ) {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.explicit_live_cutover_approval_present
                && !entry.operator_packet_rendered
                && !entry.approval_request_sent
                && !entry.operator_cutover_approval_record_written
                && !entry.operator_cutover_acceptance_present
                && !entry.live_cutover_switch_enabled
                && !entry.adapter_dispatch_switch_enabled
                && !entry.tool_invocation_execution_switch_enabled
                && !entry.live_cutover_started
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
    let tool_execution_live_cutover_operator_packet_ready = preflight
        .tool_execution_live_cutover_preflight_ready
        && !preflight.tool_execution_live_cutover_allowed
        && input.operator_packet_template_present
        && input.operator_session_binding_present
        && !input.operator_packet_rendered
        && !input.approval_request_sent
        && !input.operator_cutover_approval_record_written
        && !input.operator_cutover_acceptance_present
        && !input.live_cutover_switch_enabled
        && all_live_cutover_preflight_entries_bound_to_operator_packet
        && all_live_cutover_operator_packets_keep_no_invocation_guard;

    ToolExecutionLiveCutoverOperatorPacketPlan {
        runtime: "hepta",
        surface: "tool_execution_live_cutover_operator_packet",
        plugin_id: preflight.plugin_id,
        status: if tool_execution_live_cutover_operator_packet_ready {
            "ready"
        } else {
            "blocked"
        },
        source_live_cutover_preflight_surface: preflight.surface,
        source_live_cutover_preflight_ready: preflight.tool_execution_live_cutover_preflight_ready,
        source_live_cutover_allowed: preflight.tool_execution_live_cutover_allowed,
        operator_packet_template_present: input.operator_packet_template_present,
        operator_session_binding_present: input.operator_session_binding_present,
        operator_packet_rendered: input.operator_packet_rendered,
        approval_request_sent: input.approval_request_sent,
        operator_cutover_approval_record_written: input.operator_cutover_approval_record_written,
        operator_cutover_acceptance_present: input.operator_cutover_acceptance_present,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        candidate_count: entries.len(),
        live_cutover_operator_packet_ready_count: ready_count,
        live_cutover_operator_packet_blocked_count: entries.len() - ready_count,
        operator_review_required_count: review_count,
        remaining_blocker_readback_required_count: readback_count,
        approval_request_blocked_count: request_blocked_count,
        selected_status_canary_count,
        preflight_only_non_selected_count,
        all_live_cutover_preflight_entries_bound_to_operator_packet,
        all_live_cutover_operator_packets_keep_no_invocation_guard,
        tool_execution_live_cutover_operator_packet_ready,
        tool_execution_live_cutover_approval_request_allowed: false,
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
        next_migration_step: "restore_tool_execution_live_cutover_operator_receipt_projection_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_cutover_operator_packet_collects_review_packets() {
        let plan = hepta_system_tool_execution_live_cutover_operator_packet_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_live_cutover_preflight_surface,
            "tool_execution_live_cutover_preflight"
        );
        assert!(plan.source_live_cutover_preflight_ready);
        assert!(!plan.source_live_cutover_allowed);
        assert!(plan.operator_packet_template_present);
        assert!(plan.operator_session_binding_present);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.live_cutover_operator_packet_ready_count, 2);
        assert_eq!(plan.live_cutover_operator_packet_blocked_count, 0);
        assert_eq!(plan.operator_review_required_count, 1);
        assert_eq!(plan.remaining_blocker_readback_required_count, 1);
        assert_eq!(plan.approval_request_blocked_count, 1);
        assert_eq!(plan.selected_status_canary_count, 1);
        assert_eq!(plan.preflight_only_non_selected_count, 1);
        assert!(plan.all_live_cutover_preflight_entries_bound_to_operator_packet);
        assert!(plan.all_live_cutover_operator_packets_keep_no_invocation_guard);
        assert!(plan.tool_execution_live_cutover_operator_packet_ready);
        assert!(!plan.tool_execution_live_cutover_approval_request_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);

        let selected = plan
            .entries
            .iter()
            .find(|entry| entry.selected_for_status_canary)
            .expect("selected status canary operator packet entry");
        assert_eq!(
            selected.live_cutover_operator_packet_route,
            ToolExecutionLiveCutoverOperatorPacketRoute::LiveCutoverOperatorPacketReadyForReview
        );
        assert!(selected.approval_request_blocked);

        let preflight_only = plan
            .entries
            .iter()
            .find(|entry| entry.preflight_only_non_selected_candidate)
            .expect("non-selected preflight-only operator packet entry");
        assert_eq!(
            preflight_only.live_cutover_operator_packet_route,
            ToolExecutionLiveCutoverOperatorPacketRoute::PreflightOnlyNonSelectedCandidate
        );
        assert!(!preflight_only.approval_request_blocked);
    }

    #[test]
    fn live_cutover_operator_packet_does_not_request_or_execute() {
        let plan = hepta_system_tool_execution_live_cutover_operator_packet_plan();

        assert!(plan.tool_execution_live_cutover_operator_packet_ready);
        assert!(!plan.operator_packet_rendered);
        assert!(!plan.approval_request_sent);
        assert!(!plan.operator_cutover_approval_record_written);
        assert!(!plan.operator_cutover_acceptance_present);
        assert!(!plan.tool_execution_live_cutover_approval_request_allowed);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.result_receipt_written);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn live_cutover_operator_packet_fails_closed_without_template() {
        let preflight = hepta_system_tool_execution_live_cutover_preflight_plan();
        let input = ToolExecutionLiveCutoverOperatorPacketInput {
            operator_packet_template_present: false,
            ..ToolExecutionLiveCutoverOperatorPacketInput::default()
        };

        let plan = tool_execution_live_cutover_operator_packet_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.operator_packet_template_present);
        assert_eq!(plan.live_cutover_operator_packet_ready_count, 0);
        assert_eq!(plan.live_cutover_operator_packet_blocked_count, 2);
        assert!(!plan.tool_execution_live_cutover_operator_packet_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_operator_packet_route
                == ToolExecutionLiveCutoverOperatorPacketRoute::BlockedByMissingOperatorPacketTemplate
        }));
    }

    #[test]
    fn live_cutover_operator_packet_fails_closed_on_premature_packet_mutation() {
        let preflight = hepta_system_tool_execution_live_cutover_preflight_plan();
        let input = ToolExecutionLiveCutoverOperatorPacketInput {
            operator_packet_rendered: true,
            approval_request_sent: true,
            operator_cutover_approval_record_written: true,
            operator_cutover_acceptance_present: true,
            ..ToolExecutionLiveCutoverOperatorPacketInput::default()
        };

        let plan = tool_execution_live_cutover_operator_packet_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.operator_packet_rendered);
        assert!(plan.approval_request_sent);
        assert!(plan.operator_cutover_approval_record_written);
        assert!(plan.operator_cutover_acceptance_present);
        assert!(!plan.tool_execution_live_cutover_operator_packet_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_operator_packet_route
                == ToolExecutionLiveCutoverOperatorPacketRoute::BlockedByPrematureOperatorPacketMutation
        }));
    }

    #[test]
    fn live_cutover_operator_packet_fails_closed_when_live_switch_enabled() {
        let preflight = hepta_system_tool_execution_live_cutover_preflight_plan();
        let input = ToolExecutionLiveCutoverOperatorPacketInput {
            live_cutover_switch_enabled: true,
            ..ToolExecutionLiveCutoverOperatorPacketInput::default()
        };

        let plan = tool_execution_live_cutover_operator_packet_plan(&preflight, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.live_cutover_switch_enabled);
        assert!(!plan.tool_execution_live_cutover_operator_packet_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_operator_packet_route
                == ToolExecutionLiveCutoverOperatorPacketRoute::BlockedByLiveCutoverSwitch
        }));
    }
}
