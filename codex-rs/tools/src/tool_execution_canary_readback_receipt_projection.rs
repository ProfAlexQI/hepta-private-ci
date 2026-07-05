use crate::ToolExecutionCanaryCutoverPlan;
use crate::ToolExecutionCanaryCutoverPlanRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_canary_cutover_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionCanaryReadbackReceiptProjectionRoute {
    CanaryReadbackReceiptProjectionReady,
    PreflightOnlyNonSelectedCandidate,
    BlockedByCanaryPlan,
    BlockedByMissingReadbackChannel,
    BlockedByMissingReceiptDigest,
    BlockedByMissingTraceCorrelation,
    BlockedByMissingRollbackReadback,
    BlockedByPrematureReceiptMutation,
    BlockedByCutoverSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionCanaryReadbackReceiptProjectionInput {
    pub canary_readback_channel_declared: bool,
    pub canary_result_receipt_digest_required: bool,
    pub canary_trace_correlation_required: bool,
    pub rollback_readback_required: bool,
    pub operator_summary_required: bool,
    pub canary_cutover_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
    pub canary_execution_started: bool,
    pub canary_result_receipt_written: bool,
    pub canary_readback_projection_written: bool,
    pub rollback_executed: bool,
}

impl Default for ToolExecutionCanaryReadbackReceiptProjectionInput {
    fn default() -> Self {
        Self {
            canary_readback_channel_declared: true,
            canary_result_receipt_digest_required: true,
            canary_trace_correlation_required: true,
            rollback_readback_required: true,
            operator_summary_required: true,
            canary_cutover_switch_enabled: false,
            live_cutover_switch_enabled: false,
            canary_execution_started: false,
            canary_result_receipt_written: false,
            canary_readback_projection_written: false,
            rollback_executed: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionCanaryReadbackReceiptProjectionEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_canary_cutover_plan_route: ToolExecutionCanaryCutoverPlanRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub selected_for_status_canary: bool,
    pub preflight_only_non_selected_candidate: bool,
    pub canary_readback_receipt_projection_route: ToolExecutionCanaryReadbackReceiptProjectionRoute,
    pub canary_readback_receipt_projection_ready: bool,
    pub canary_readback_channel_declared: bool,
    pub canary_result_receipt_digest_required: bool,
    pub canary_trace_correlation_required: bool,
    pub rollback_readback_required: bool,
    pub operator_summary_required: bool,
    pub canary_result_receipt_write_blocked: bool,
    pub canary_cutover_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
    pub canary_execution_started: bool,
    pub canary_result_receipt_written: bool,
    pub canary_readback_projection_written: bool,
    pub rollback_executed: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub execution_adapter_dispatch_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub operator_decision_record_write_enabled: bool,
    pub operator_decision_receipt_write_enabled: bool,
    pub result_receipt_write_enabled: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionCanaryReadbackReceiptProjectionPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_canary_cutover_plan_surface: &'static str,
    pub source_canary_cutover_plan_ready: bool,
    pub source_canary_cutover_start_allowed: bool,
    pub source_canary_result_receipt_write_allowed: bool,
    pub source_live_cutover_allowed: bool,
    pub canary_readback_channel_declared: bool,
    pub canary_result_receipt_digest_required: bool,
    pub canary_trace_correlation_required: bool,
    pub rollback_readback_required: bool,
    pub operator_summary_required: bool,
    pub canary_cutover_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
    pub canary_execution_started: bool,
    pub canary_result_receipt_written: bool,
    pub canary_readback_projection_written: bool,
    pub rollback_executed: bool,
    pub candidate_count: usize,
    pub canary_readback_receipt_projection_ready_count: usize,
    pub canary_readback_receipt_projection_blocked_count: usize,
    pub canary_readback_channel_declared_count: usize,
    pub canary_result_receipt_digest_required_count: usize,
    pub canary_trace_correlation_required_count: usize,
    pub rollback_readback_required_count: usize,
    pub operator_summary_required_count: usize,
    pub canary_result_receipt_write_blocked_count: usize,
    pub selected_status_canary_count: usize,
    pub preflight_only_non_selected_count: usize,
    pub all_canary_plan_entries_bound_to_readback_projection: bool,
    pub all_canary_readback_entries_keep_no_invocation_guard: bool,
    pub tool_execution_canary_readback_receipt_projection_ready: bool,
    pub tool_execution_canary_result_receipt_write_allowed: bool,
    pub tool_execution_canary_result_acceptance_allowed: bool,
    pub tool_execution_live_cutover_allowed: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub execution_adapter_dispatched: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub operator_decision_record_written: bool,
    pub operator_decision_receipt_written: bool,
    pub result_receipt_written: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<ToolExecutionCanaryReadbackReceiptProjectionEntry>,
}

pub fn hepta_system_tool_execution_canary_readback_receipt_projection_plan()
-> ToolExecutionCanaryReadbackReceiptProjectionPlan {
    let canary_plan = hepta_system_tool_execution_canary_cutover_plan();
    tool_execution_canary_readback_receipt_projection_plan(
        &canary_plan,
        &ToolExecutionCanaryReadbackReceiptProjectionInput::default(),
    )
}

pub fn tool_execution_canary_readback_receipt_projection_plan(
    canary_plan: &ToolExecutionCanaryCutoverPlan,
    input: &ToolExecutionCanaryReadbackReceiptProjectionInput,
) -> ToolExecutionCanaryReadbackReceiptProjectionPlan {
    let entries = canary_plan
        .entries
        .iter()
        .map(|entry| {
            let route = if input.canary_cutover_switch_enabled || input.live_cutover_switch_enabled
            {
                ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByCutoverSwitch
            } else if input.canary_execution_started
                || input.canary_result_receipt_written
                || input.canary_readback_projection_written
                || input.rollback_executed
            {
                ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByPrematureReceiptMutation
            } else if !input.canary_readback_channel_declared {
                ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByMissingReadbackChannel
            } else if !input.canary_result_receipt_digest_required {
                ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByMissingReceiptDigest
            } else if !input.canary_trace_correlation_required {
                ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByMissingTraceCorrelation
            } else if !input.rollback_readback_required || !input.operator_summary_required {
                ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByMissingRollbackReadback
            } else if !entry.canary_cutover_plan_ready
                || !matches!(
                    entry.canary_cutover_plan_route,
                    ToolExecutionCanaryCutoverPlanRoute::CanaryCutoverPlanReady
                        | ToolExecutionCanaryCutoverPlanRoute::PreflightOnlyNonSelectedCandidate
                )
            {
                ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByCanaryPlan
            } else if entry.preflight_only_non_selected_candidate {
                ToolExecutionCanaryReadbackReceiptProjectionRoute::PreflightOnlyNonSelectedCandidate
            } else {
                ToolExecutionCanaryReadbackReceiptProjectionRoute::CanaryReadbackReceiptProjectionReady
            };
            let ready = matches!(
                route,
                ToolExecutionCanaryReadbackReceiptProjectionRoute::CanaryReadbackReceiptProjectionReady
                    | ToolExecutionCanaryReadbackReceiptProjectionRoute::PreflightOnlyNonSelectedCandidate
            )
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && entry.canary_readback_receipt_required
                && entry.canary_result_receipt_schema_present;
            let canary_result_receipt_write_blocked = ready
                && entry.selected_for_status_canary
                && entry.canary_start_blocked
                && !input.canary_execution_started
                && !input.canary_result_receipt_written
                && !input.canary_readback_projection_written;

            ToolExecutionCanaryReadbackReceiptProjectionEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_canary_cutover_plan_route: entry.canary_cutover_plan_route,
                registry_guard_route: entry.registry_guard_route,
                selected_for_status_canary: entry.selected_for_status_canary,
                preflight_only_non_selected_candidate: entry.preflight_only_non_selected_candidate,
                canary_readback_receipt_projection_route: route,
                canary_readback_receipt_projection_ready: ready,
                canary_readback_channel_declared: input.canary_readback_channel_declared,
                canary_result_receipt_digest_required: input
                    .canary_result_receipt_digest_required,
                canary_trace_correlation_required: input.canary_trace_correlation_required,
                rollback_readback_required: input.rollback_readback_required,
                operator_summary_required: input.operator_summary_required,
                canary_result_receipt_write_blocked,
                canary_cutover_switch_enabled: input.canary_cutover_switch_enabled,
                live_cutover_switch_enabled: input.live_cutover_switch_enabled,
                canary_execution_started: input.canary_execution_started,
                canary_result_receipt_written: input.canary_result_receipt_written,
                canary_readback_projection_written: input.canary_readback_projection_written,
                rollback_executed: input.rollback_executed,
                router_registration_lookup_enabled: false,
                registry_lookup_executed: false,
                registry_source_of_truth_enabled: false,
                tool_registration_enabled: false,
                execution_adapter_dispatch_enabled: false,
                tool_invocation_enabled: false,
                ledger_write_enabled: false,
                approval_request_enabled: false,
                operator_decision_record_write_enabled: false,
                operator_decision_receipt_write_enabled: false,
                result_receipt_write_enabled: false,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let ready_count = entries
        .iter()
        .filter(|entry| entry.canary_readback_receipt_projection_ready)
        .count();
    let channel_count = entries
        .iter()
        .filter(|entry| entry.canary_readback_channel_declared)
        .count();
    let digest_count = entries
        .iter()
        .filter(|entry| entry.canary_result_receipt_digest_required)
        .count();
    let trace_count = entries
        .iter()
        .filter(|entry| entry.canary_trace_correlation_required)
        .count();
    let rollback_count = entries
        .iter()
        .filter(|entry| entry.rollback_readback_required)
        .count();
    let summary_count = entries
        .iter()
        .filter(|entry| entry.operator_summary_required)
        .count();
    let write_blocked_count = entries
        .iter()
        .filter(|entry| entry.canary_result_receipt_write_blocked)
        .count();
    let selected_status_canary_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary)
        .count();
    let preflight_only_non_selected_count = entries
        .iter()
        .filter(|entry| entry.preflight_only_non_selected_candidate)
        .count();
    let all_canary_plan_entries_bound_to_readback_projection = ready_count == entries.len()
        && channel_count == entries.len()
        && digest_count == entries.len()
        && trace_count == entries.len()
        && rollback_count == entries.len()
        && summary_count == entries.len()
        && selected_status_canary_count == 1
        && preflight_only_non_selected_count + selected_status_canary_count == entries.len()
        && write_blocked_count == selected_status_canary_count;
    let all_canary_readback_entries_keep_no_invocation_guard = entries.iter().all(|entry| {
        if matches!(
            entry.canary_readback_receipt_projection_route,
            ToolExecutionCanaryReadbackReceiptProjectionRoute::CanaryReadbackReceiptProjectionReady
                | ToolExecutionCanaryReadbackReceiptProjectionRoute::PreflightOnlyNonSelectedCandidate
        ) {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.canary_cutover_switch_enabled
                && !entry.live_cutover_switch_enabled
                && !entry.canary_execution_started
                && !entry.canary_result_receipt_written
                && !entry.canary_readback_projection_written
                && !entry.rollback_executed
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.execution_adapter_dispatch_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && !entry.operator_decision_record_write_enabled
                && !entry.operator_decision_receipt_write_enabled
                && !entry.result_receipt_write_enabled
        } else {
            true
        }
    });
    let tool_execution_canary_readback_receipt_projection_ready = canary_plan
        .tool_execution_canary_cutover_plan_ready
        && !canary_plan.tool_execution_canary_cutover_start_allowed
        && !canary_plan.tool_execution_canary_result_receipt_write_allowed
        && !canary_plan.tool_execution_live_cutover_allowed
        && input.canary_readback_channel_declared
        && input.canary_result_receipt_digest_required
        && input.canary_trace_correlation_required
        && input.rollback_readback_required
        && input.operator_summary_required
        && !input.canary_cutover_switch_enabled
        && !input.live_cutover_switch_enabled
        && !input.canary_execution_started
        && !input.canary_result_receipt_written
        && !input.canary_readback_projection_written
        && !input.rollback_executed
        && all_canary_plan_entries_bound_to_readback_projection
        && all_canary_readback_entries_keep_no_invocation_guard;

    ToolExecutionCanaryReadbackReceiptProjectionPlan {
        runtime: "hepta",
        surface: "tool_execution_canary_readback_receipt_projection",
        plugin_id: canary_plan.plugin_id,
        status: if tool_execution_canary_readback_receipt_projection_ready {
            "ready"
        } else {
            "blocked"
        },
        source_canary_cutover_plan_surface: canary_plan.surface,
        source_canary_cutover_plan_ready: canary_plan.tool_execution_canary_cutover_plan_ready,
        source_canary_cutover_start_allowed: canary_plan
            .tool_execution_canary_cutover_start_allowed,
        source_canary_result_receipt_write_allowed: canary_plan
            .tool_execution_canary_result_receipt_write_allowed,
        source_live_cutover_allowed: canary_plan.tool_execution_live_cutover_allowed,
        canary_readback_channel_declared: input.canary_readback_channel_declared,
        canary_result_receipt_digest_required: input.canary_result_receipt_digest_required,
        canary_trace_correlation_required: input.canary_trace_correlation_required,
        rollback_readback_required: input.rollback_readback_required,
        operator_summary_required: input.operator_summary_required,
        canary_cutover_switch_enabled: input.canary_cutover_switch_enabled,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        canary_execution_started: input.canary_execution_started,
        canary_result_receipt_written: input.canary_result_receipt_written,
        canary_readback_projection_written: input.canary_readback_projection_written,
        rollback_executed: input.rollback_executed,
        candidate_count: entries.len(),
        canary_readback_receipt_projection_ready_count: ready_count,
        canary_readback_receipt_projection_blocked_count: entries.len() - ready_count,
        canary_readback_channel_declared_count: channel_count,
        canary_result_receipt_digest_required_count: digest_count,
        canary_trace_correlation_required_count: trace_count,
        rollback_readback_required_count: rollback_count,
        operator_summary_required_count: summary_count,
        canary_result_receipt_write_blocked_count: write_blocked_count,
        selected_status_canary_count,
        preflight_only_non_selected_count,
        all_canary_plan_entries_bound_to_readback_projection,
        all_canary_readback_entries_keep_no_invocation_guard,
        tool_execution_canary_readback_receipt_projection_ready,
        tool_execution_canary_result_receipt_write_allowed: false,
        tool_execution_canary_result_acceptance_allowed: false,
        tool_execution_live_cutover_allowed: false,
        router_registration_lookup_enabled: false,
        registry_lookup_executed: false,
        registry_source_of_truth_enabled: false,
        tool_registration_enabled: false,
        execution_adapter_dispatched: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        operator_decision_record_written: false,
        operator_decision_receipt_written: false,
        result_receipt_written: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_execution_canary_result_acceptance_preflight_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canary_readback_receipt_projection_collects_receipt_requirements() {
        let plan = hepta_system_tool_execution_canary_readback_receipt_projection_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_canary_cutover_plan_surface,
            "tool_execution_canary_cutover_plan"
        );
        assert!(plan.source_canary_cutover_plan_ready);
        assert!(!plan.source_canary_cutover_start_allowed);
        assert!(!plan.source_canary_result_receipt_write_allowed);
        assert!(!plan.source_live_cutover_allowed);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.canary_readback_receipt_projection_ready_count, 2);
        assert_eq!(plan.canary_readback_receipt_projection_blocked_count, 0);
        assert_eq!(plan.canary_readback_channel_declared_count, 2);
        assert_eq!(plan.canary_result_receipt_digest_required_count, 2);
        assert_eq!(plan.canary_trace_correlation_required_count, 2);
        assert_eq!(plan.rollback_readback_required_count, 2);
        assert_eq!(plan.operator_summary_required_count, 2);
        assert_eq!(plan.canary_result_receipt_write_blocked_count, 1);
        assert_eq!(plan.selected_status_canary_count, 1);
        assert_eq!(plan.preflight_only_non_selected_count, 1);
        assert!(plan.all_canary_plan_entries_bound_to_readback_projection);
        assert!(plan.all_canary_readback_entries_keep_no_invocation_guard);
        assert!(plan.tool_execution_canary_readback_receipt_projection_ready);
        assert!(!plan.tool_execution_canary_result_receipt_write_allowed);
        assert!(!plan.tool_execution_canary_result_acceptance_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);

        let selected = plan
            .entries
            .iter()
            .find(|entry| entry.selected_for_status_canary)
            .expect("selected status canary readback entry");
        assert_eq!(
            selected.canary_readback_receipt_projection_route,
            ToolExecutionCanaryReadbackReceiptProjectionRoute::CanaryReadbackReceiptProjectionReady
        );
        assert!(selected.canary_result_receipt_write_blocked);

        let preflight_only = plan
            .entries
            .iter()
            .find(|entry| entry.preflight_only_non_selected_candidate)
            .expect("non-selected preflight-only readback entry");
        assert_eq!(
            preflight_only.canary_readback_receipt_projection_route,
            ToolExecutionCanaryReadbackReceiptProjectionRoute::PreflightOnlyNonSelectedCandidate
        );
        assert!(!preflight_only.canary_result_receipt_write_blocked);
    }

    #[test]
    fn canary_readback_receipt_projection_does_not_write_receipts() {
        let plan = hepta_system_tool_execution_canary_readback_receipt_projection_plan();

        assert!(plan.tool_execution_canary_readback_receipt_projection_ready);
        assert!(!plan.canary_execution_started);
        assert!(!plan.canary_result_receipt_written);
        assert!(!plan.canary_readback_projection_written);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.result_receipt_written);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn canary_readback_receipt_projection_fails_closed_without_readback_channel() {
        let canary_plan = hepta_system_tool_execution_canary_cutover_plan();
        let input = ToolExecutionCanaryReadbackReceiptProjectionInput {
            canary_readback_channel_declared: false,
            ..ToolExecutionCanaryReadbackReceiptProjectionInput::default()
        };

        let plan = tool_execution_canary_readback_receipt_projection_plan(&canary_plan, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.canary_readback_channel_declared);
        assert_eq!(plan.canary_readback_receipt_projection_ready_count, 0);
        assert_eq!(plan.canary_readback_receipt_projection_blocked_count, 2);
        assert!(!plan.tool_execution_canary_readback_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_readback_receipt_projection_route
                == ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByMissingReadbackChannel
        }));
    }

    #[test]
    fn canary_readback_receipt_projection_fails_closed_without_digest() {
        let canary_plan = hepta_system_tool_execution_canary_cutover_plan();
        let input = ToolExecutionCanaryReadbackReceiptProjectionInput {
            canary_result_receipt_digest_required: false,
            ..ToolExecutionCanaryReadbackReceiptProjectionInput::default()
        };

        let plan = tool_execution_canary_readback_receipt_projection_plan(&canary_plan, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.canary_result_receipt_digest_required);
        assert_eq!(plan.canary_result_receipt_digest_required_count, 0);
        assert!(!plan.tool_execution_canary_readback_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_readback_receipt_projection_route
                == ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByMissingReceiptDigest
        }));
    }

    #[test]
    fn canary_readback_receipt_projection_fails_closed_on_premature_receipt_mutation() {
        let canary_plan = hepta_system_tool_execution_canary_cutover_plan();
        let input = ToolExecutionCanaryReadbackReceiptProjectionInput {
            canary_execution_started: true,
            canary_result_receipt_written: true,
            canary_readback_projection_written: true,
            rollback_executed: true,
            ..ToolExecutionCanaryReadbackReceiptProjectionInput::default()
        };

        let plan = tool_execution_canary_readback_receipt_projection_plan(&canary_plan, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.canary_execution_started);
        assert!(plan.canary_result_receipt_written);
        assert!(plan.canary_readback_projection_written);
        assert!(plan.rollback_executed);
        assert!(!plan.tool_execution_canary_readback_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_readback_receipt_projection_route
                == ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByPrematureReceiptMutation
        }));
    }

    #[test]
    fn canary_readback_receipt_projection_fails_closed_when_cutover_switch_enabled() {
        let canary_plan = hepta_system_tool_execution_canary_cutover_plan();
        let input = ToolExecutionCanaryReadbackReceiptProjectionInput {
            canary_cutover_switch_enabled: true,
            live_cutover_switch_enabled: true,
            ..ToolExecutionCanaryReadbackReceiptProjectionInput::default()
        };

        let plan = tool_execution_canary_readback_receipt_projection_plan(&canary_plan, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.canary_cutover_switch_enabled);
        assert!(plan.live_cutover_switch_enabled);
        assert!(!plan.tool_execution_canary_readback_receipt_projection_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_readback_receipt_projection_route
                == ToolExecutionCanaryReadbackReceiptProjectionRoute::BlockedByCutoverSwitch
        }));
    }
}
