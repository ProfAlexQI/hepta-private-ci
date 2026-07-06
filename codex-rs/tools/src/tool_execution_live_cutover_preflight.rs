use crate::ToolExecutionCanaryResultAcceptancePreflightPlan;
use crate::ToolExecutionCanaryResultAcceptancePreflightRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_canary_result_acceptance_preflight_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionLiveCutoverPreflightRoute {
    LiveCutoverPreflightReadyPendingApproval,
    PreflightOnlyNonSelectedCandidate,
    BlockedByCanaryResultAcceptancePreflight,
    BlockedByMissingOperatorIdentityBinding,
    BlockedByMissingRollbackAnchor,
    BlockedByMissingKillSwitch,
    BlockedByMissingObservabilityReadback,
    BlockedByPrematureLiveCutoverMutation,
    BlockedByExecutionSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverPreflightInput {
    pub operator_identity_binding_present: bool,
    pub explicit_live_cutover_approval_present: bool,
    pub rollback_anchor_present: bool,
    pub kill_switch_present: bool,
    pub observability_readback_required: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub live_cutover_started: bool,
    pub live_cutover_acceptance_record_written: bool,
    pub result_receipt_written: bool,
}

impl Default for ToolExecutionLiveCutoverPreflightInput {
    fn default() -> Self {
        Self {
            operator_identity_binding_present: true,
            explicit_live_cutover_approval_present: false,
            rollback_anchor_present: true,
            kill_switch_present: true,
            observability_readback_required: true,
            live_cutover_switch_enabled: false,
            adapter_dispatch_switch_enabled: false,
            tool_invocation_execution_switch_enabled: false,
            live_cutover_started: false,
            live_cutover_acceptance_record_written: false,
            result_receipt_written: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionLiveCutoverPreflightEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_acceptance_preflight_route: ToolExecutionCanaryResultAcceptancePreflightRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub selected_for_status_canary: bool,
    pub preflight_only_non_selected_candidate: bool,
    pub live_cutover_preflight_route: ToolExecutionLiveCutoverPreflightRoute,
    pub live_cutover_preflight_ready: bool,
    pub explicit_live_cutover_approval_required: bool,
    pub explicit_live_cutover_approval_present: bool,
    pub live_cutover_blocked: bool,
    pub operator_identity_binding_present: bool,
    pub rollback_anchor_present: bool,
    pub kill_switch_present: bool,
    pub observability_readback_required: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub live_cutover_started: bool,
    pub live_cutover_acceptance_record_written: bool,
    pub result_receipt_written: bool,
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
pub struct ToolExecutionLiveCutoverPreflightPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_acceptance_preflight_surface: &'static str,
    pub source_acceptance_preflight_ready: bool,
    pub source_canary_result_acceptance_allowed: bool,
    pub source_live_cutover_allowed: bool,
    pub operator_identity_binding_present: bool,
    pub explicit_live_cutover_approval_present: bool,
    pub rollback_anchor_present: bool,
    pub kill_switch_present: bool,
    pub observability_readback_required: bool,
    pub live_cutover_switch_enabled: bool,
    pub adapter_dispatch_switch_enabled: bool,
    pub tool_invocation_execution_switch_enabled: bool,
    pub live_cutover_started: bool,
    pub live_cutover_acceptance_record_written: bool,
    pub result_receipt_written: bool,
    pub candidate_count: usize,
    pub live_cutover_preflight_ready_count: usize,
    pub live_cutover_preflight_blocked_count: usize,
    pub explicit_live_cutover_approval_required_count: usize,
    pub explicit_live_cutover_approval_missing_count: usize,
    pub rollback_anchor_present_count: usize,
    pub kill_switch_present_count: usize,
    pub observability_readback_required_count: usize,
    pub selected_status_canary_count: usize,
    pub preflight_only_non_selected_count: usize,
    pub all_acceptance_preflight_entries_bound_to_live_cutover_preflight: bool,
    pub all_live_cutover_entries_keep_no_invocation_guard: bool,
    pub tool_execution_live_cutover_preflight_ready: bool,
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
    pub entries: Vec<ToolExecutionLiveCutoverPreflightEntry>,
}

pub fn hepta_system_tool_execution_live_cutover_preflight_plan()
-> ToolExecutionLiveCutoverPreflightPlan {
    let acceptance = hepta_system_tool_execution_canary_result_acceptance_preflight_plan();
    tool_execution_live_cutover_preflight_plan(
        &acceptance,
        &ToolExecutionLiveCutoverPreflightInput::default(),
    )
}

pub fn tool_execution_live_cutover_preflight_plan(
    acceptance: &ToolExecutionCanaryResultAcceptancePreflightPlan,
    input: &ToolExecutionLiveCutoverPreflightInput,
) -> ToolExecutionLiveCutoverPreflightPlan {
    let entries = acceptance
        .entries
        .iter()
        .map(|entry| {
            let route = if input.live_cutover_switch_enabled
                || input.adapter_dispatch_switch_enabled
                || input.tool_invocation_execution_switch_enabled
            {
                ToolExecutionLiveCutoverPreflightRoute::BlockedByExecutionSwitch
            } else if input.live_cutover_started
                || input.live_cutover_acceptance_record_written
                || input.result_receipt_written
            {
                ToolExecutionLiveCutoverPreflightRoute::BlockedByPrematureLiveCutoverMutation
            } else if !input.operator_identity_binding_present {
                ToolExecutionLiveCutoverPreflightRoute::BlockedByMissingOperatorIdentityBinding
            } else if !input.rollback_anchor_present {
                ToolExecutionLiveCutoverPreflightRoute::BlockedByMissingRollbackAnchor
            } else if !input.kill_switch_present {
                ToolExecutionLiveCutoverPreflightRoute::BlockedByMissingKillSwitch
            } else if !input.observability_readback_required {
                ToolExecutionLiveCutoverPreflightRoute::BlockedByMissingObservabilityReadback
            } else if entry.preflight_only_non_selected_candidate
                && entry.canary_result_acceptance_preflight_route
                    == ToolExecutionCanaryResultAcceptancePreflightRoute::PreflightOnlyNonSelectedCandidate
            {
                ToolExecutionLiveCutoverPreflightRoute::PreflightOnlyNonSelectedCandidate
            } else if !entry.canary_result_acceptance_preflight_ready
                || entry.canary_result_acceptance_preflight_route
                    != ToolExecutionCanaryResultAcceptancePreflightRoute::CanaryResultAcceptancePendingEvidence
            {
                ToolExecutionLiveCutoverPreflightRoute::BlockedByCanaryResultAcceptancePreflight
            } else {
                ToolExecutionLiveCutoverPreflightRoute::LiveCutoverPreflightReadyPendingApproval
            };
            let ready = matches!(
                route,
                ToolExecutionLiveCutoverPreflightRoute::LiveCutoverPreflightReadyPendingApproval
                    | ToolExecutionLiveCutoverPreflightRoute::PreflightOnlyNonSelectedCandidate
            )
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && (entry.preflight_only_non_selected_candidate
                    || (entry.canary_result_acceptance_pending_evidence
                        && entry.canary_acceptance_record_write_blocked
                        && entry.canary_acceptance_receipt_write_blocked));
            let live_cutover_blocked = ready
                && entry.selected_for_status_canary
                && !input.explicit_live_cutover_approval_present
                && !input.live_cutover_switch_enabled
                && !input.live_cutover_started;

            ToolExecutionLiveCutoverPreflightEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_acceptance_preflight_route: entry
                    .canary_result_acceptance_preflight_route,
                registry_guard_route: entry.registry_guard_route,
                selected_for_status_canary: entry.selected_for_status_canary,
                preflight_only_non_selected_candidate: entry.preflight_only_non_selected_candidate,
                live_cutover_preflight_route: route,
                live_cutover_preflight_ready: ready,
                explicit_live_cutover_approval_required: entry.selected_for_status_canary,
                explicit_live_cutover_approval_present: input
                    .explicit_live_cutover_approval_present,
                live_cutover_blocked,
                operator_identity_binding_present: input.operator_identity_binding_present,
                rollback_anchor_present: input.rollback_anchor_present,
                kill_switch_present: input.kill_switch_present,
                observability_readback_required: input.observability_readback_required,
                live_cutover_switch_enabled: input.live_cutover_switch_enabled,
                adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
                tool_invocation_execution_switch_enabled: input
                    .tool_invocation_execution_switch_enabled,
                live_cutover_started: input.live_cutover_started,
                live_cutover_acceptance_record_written: input
                    .live_cutover_acceptance_record_written,
                result_receipt_written: input.result_receipt_written,
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
        .filter(|entry| entry.live_cutover_preflight_ready)
        .count();
    let blocked_count = entries
        .iter()
        .filter(|entry| entry.live_cutover_blocked)
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
    let rollback_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_present)
        .count();
    let kill_switch_count = entries
        .iter()
        .filter(|entry| entry.kill_switch_present)
        .count();
    let observability_count = entries
        .iter()
        .filter(|entry| entry.observability_readback_required)
        .count();
    let selected_status_canary_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary)
        .count();
    let preflight_only_non_selected_count = entries
        .iter()
        .filter(|entry| entry.preflight_only_non_selected_candidate)
        .count();
    let all_acceptance_preflight_entries_bound_to_live_cutover_preflight = ready_count
        == entries.len()
        && selected_status_canary_count == 1
        && preflight_only_non_selected_count + selected_status_canary_count == entries.len()
        && blocked_count == selected_status_canary_count
        && approval_required_count == selected_status_canary_count
        && approval_missing_count == selected_status_canary_count
        && rollback_count == entries.len()
        && kill_switch_count == entries.len()
        && observability_count == entries.len();
    let all_live_cutover_entries_keep_no_invocation_guard = entries.iter().all(|entry| {
        if matches!(
            entry.live_cutover_preflight_route,
            ToolExecutionLiveCutoverPreflightRoute::LiveCutoverPreflightReadyPendingApproval
                | ToolExecutionLiveCutoverPreflightRoute::PreflightOnlyNonSelectedCandidate
        ) {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.explicit_live_cutover_approval_present
                && !entry.live_cutover_switch_enabled
                && !entry.adapter_dispatch_switch_enabled
                && !entry.tool_invocation_execution_switch_enabled
                && !entry.live_cutover_started
                && !entry.live_cutover_acceptance_record_written
                && !entry.result_receipt_written
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
    let tool_execution_live_cutover_preflight_ready = acceptance
        .tool_execution_canary_result_acceptance_preflight_ready
        && !acceptance.tool_execution_canary_result_acceptance_allowed
        && !acceptance.tool_execution_live_cutover_allowed
        && input.operator_identity_binding_present
        && !input.explicit_live_cutover_approval_present
        && input.rollback_anchor_present
        && input.kill_switch_present
        && input.observability_readback_required
        && !input.live_cutover_switch_enabled
        && !input.adapter_dispatch_switch_enabled
        && !input.tool_invocation_execution_switch_enabled
        && !input.live_cutover_started
        && !input.live_cutover_acceptance_record_written
        && !input.result_receipt_written
        && all_acceptance_preflight_entries_bound_to_live_cutover_preflight
        && all_live_cutover_entries_keep_no_invocation_guard;

    ToolExecutionLiveCutoverPreflightPlan {
        runtime: "hepta",
        surface: "tool_execution_live_cutover_preflight",
        plugin_id: acceptance.plugin_id,
        status: if tool_execution_live_cutover_preflight_ready {
            "ready"
        } else {
            "blocked"
        },
        source_acceptance_preflight_surface: acceptance.surface,
        source_acceptance_preflight_ready: acceptance
            .tool_execution_canary_result_acceptance_preflight_ready,
        source_canary_result_acceptance_allowed: acceptance
            .tool_execution_canary_result_acceptance_allowed,
        source_live_cutover_allowed: acceptance.tool_execution_live_cutover_allowed,
        operator_identity_binding_present: input.operator_identity_binding_present,
        explicit_live_cutover_approval_present: input.explicit_live_cutover_approval_present,
        rollback_anchor_present: input.rollback_anchor_present,
        kill_switch_present: input.kill_switch_present,
        observability_readback_required: input.observability_readback_required,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        adapter_dispatch_switch_enabled: input.adapter_dispatch_switch_enabled,
        tool_invocation_execution_switch_enabled: input.tool_invocation_execution_switch_enabled,
        live_cutover_started: input.live_cutover_started,
        live_cutover_acceptance_record_written: input.live_cutover_acceptance_record_written,
        result_receipt_written: input.result_receipt_written,
        candidate_count: entries.len(),
        live_cutover_preflight_ready_count: ready_count,
        live_cutover_preflight_blocked_count: entries.len() - ready_count,
        explicit_live_cutover_approval_required_count: approval_required_count,
        explicit_live_cutover_approval_missing_count: approval_missing_count,
        rollback_anchor_present_count: rollback_count,
        kill_switch_present_count: kill_switch_count,
        observability_readback_required_count: observability_count,
        selected_status_canary_count,
        preflight_only_non_selected_count,
        all_acceptance_preflight_entries_bound_to_live_cutover_preflight,
        all_live_cutover_entries_keep_no_invocation_guard,
        tool_execution_live_cutover_preflight_ready,
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
        next_migration_step: "restore_tool_execution_live_cutover_operator_packet_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_cutover_preflight_collects_blocker_matrix() {
        let plan = hepta_system_tool_execution_live_cutover_preflight_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_acceptance_preflight_surface,
            "tool_execution_canary_result_acceptance_preflight"
        );
        assert!(plan.source_acceptance_preflight_ready);
        assert!(!plan.source_canary_result_acceptance_allowed);
        assert!(!plan.source_live_cutover_allowed);
        assert!(plan.operator_identity_binding_present);
        assert!(!plan.explicit_live_cutover_approval_present);
        assert!(plan.rollback_anchor_present);
        assert!(plan.kill_switch_present);
        assert!(plan.observability_readback_required);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.live_cutover_preflight_ready_count, 2);
        assert_eq!(plan.live_cutover_preflight_blocked_count, 0);
        assert_eq!(plan.explicit_live_cutover_approval_required_count, 1);
        assert_eq!(plan.explicit_live_cutover_approval_missing_count, 1);
        assert_eq!(plan.rollback_anchor_present_count, 2);
        assert_eq!(plan.kill_switch_present_count, 2);
        assert_eq!(plan.observability_readback_required_count, 2);
        assert_eq!(plan.selected_status_canary_count, 1);
        assert_eq!(plan.preflight_only_non_selected_count, 1);
        assert!(plan.all_acceptance_preflight_entries_bound_to_live_cutover_preflight);
        assert!(plan.all_live_cutover_entries_keep_no_invocation_guard);
        assert!(plan.tool_execution_live_cutover_preflight_ready);
        assert!(!plan.tool_execution_live_cutover_allowed);

        let selected = plan
            .entries
            .iter()
            .find(|entry| entry.selected_for_status_canary)
            .expect("selected status canary live preflight entry");
        assert_eq!(
            selected.live_cutover_preflight_route,
            ToolExecutionLiveCutoverPreflightRoute::LiveCutoverPreflightReadyPendingApproval
        );
        assert!(selected.live_cutover_blocked);

        let preflight_only = plan
            .entries
            .iter()
            .find(|entry| entry.preflight_only_non_selected_candidate)
            .expect("non-selected preflight-only live preflight entry");
        assert_eq!(
            preflight_only.live_cutover_preflight_route,
            ToolExecutionLiveCutoverPreflightRoute::PreflightOnlyNonSelectedCandidate
        );
        assert!(!preflight_only.live_cutover_blocked);
    }

    #[test]
    fn live_cutover_preflight_does_not_enable_execution() {
        let plan = hepta_system_tool_execution_live_cutover_preflight_plan();

        assert!(plan.tool_execution_live_cutover_preflight_ready);
        assert!(!plan.tool_execution_live_cutover_allowed);
        assert!(!plan.live_cutover_switch_enabled);
        assert!(!plan.adapter_dispatch_switch_enabled);
        assert!(!plan.tool_invocation_execution_switch_enabled);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn live_cutover_preflight_fails_closed_without_rollback_anchor() {
        let acceptance = hepta_system_tool_execution_canary_result_acceptance_preflight_plan();
        let input = ToolExecutionLiveCutoverPreflightInput {
            rollback_anchor_present: false,
            ..ToolExecutionLiveCutoverPreflightInput::default()
        };

        let plan = tool_execution_live_cutover_preflight_plan(&acceptance, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.rollback_anchor_present);
        assert_eq!(plan.live_cutover_preflight_ready_count, 0);
        assert_eq!(plan.live_cutover_preflight_blocked_count, 2);
        assert!(!plan.tool_execution_live_cutover_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_preflight_route
                == ToolExecutionLiveCutoverPreflightRoute::BlockedByMissingRollbackAnchor
        }));
    }

    #[test]
    fn live_cutover_preflight_fails_closed_on_premature_mutation() {
        let acceptance = hepta_system_tool_execution_canary_result_acceptance_preflight_plan();
        let input = ToolExecutionLiveCutoverPreflightInput {
            live_cutover_started: true,
            live_cutover_acceptance_record_written: true,
            result_receipt_written: true,
            ..ToolExecutionLiveCutoverPreflightInput::default()
        };

        let plan = tool_execution_live_cutover_preflight_plan(&acceptance, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.live_cutover_started);
        assert!(plan.live_cutover_acceptance_record_written);
        assert!(plan.result_receipt_written);
        assert!(!plan.tool_execution_live_cutover_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_preflight_route
                == ToolExecutionLiveCutoverPreflightRoute::BlockedByPrematureLiveCutoverMutation
        }));
    }

    #[test]
    fn live_cutover_preflight_fails_closed_when_execution_switch_enabled() {
        let acceptance = hepta_system_tool_execution_canary_result_acceptance_preflight_plan();
        let input = ToolExecutionLiveCutoverPreflightInput {
            live_cutover_switch_enabled: true,
            adapter_dispatch_switch_enabled: true,
            tool_invocation_execution_switch_enabled: true,
            ..ToolExecutionLiveCutoverPreflightInput::default()
        };

        let plan = tool_execution_live_cutover_preflight_plan(&acceptance, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.live_cutover_switch_enabled);
        assert!(plan.adapter_dispatch_switch_enabled);
        assert!(plan.tool_invocation_execution_switch_enabled);
        assert!(!plan.tool_execution_live_cutover_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.live_cutover_preflight_route
                == ToolExecutionLiveCutoverPreflightRoute::BlockedByExecutionSwitch
        }));
    }
}
