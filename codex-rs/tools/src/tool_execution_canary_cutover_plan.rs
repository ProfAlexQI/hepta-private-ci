use crate::HEPTA_SYSTEM_STATUS_CANARY_TOOL_ID;
use crate::ToolExecutionOperatorApprovalDecisionPreflightPlan;
use crate::ToolExecutionOperatorApprovalDecisionPreflightRoute;
use crate::ToolExecutionStatusCanarySpecPlan;
use crate::ToolExecutionStatusCanarySpecRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_operator_approval_decision_preflight_plan;
use crate::hepta_system_tool_execution_status_canary_spec_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionCanaryCutoverPlanRoute {
    CanaryCutoverPlanReady,
    PreflightOnlyNonSelectedCandidate,
    BlockedByDecisionPreflight,
    BlockedByStatusCanarySpec,
    BlockedByMissingCanaryScope,
    BlockedByMissingCanaryBudget,
    BlockedByMissingRollbackPlan,
    BlockedByMissingReadbackReceipt,
    BlockedByOperatorAcceptance,
    BlockedByPrematureCanaryMutation,
    BlockedByCutoverSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionCanaryCutoverPlanInput {
    pub canary_scope_declared: bool,
    pub canary_budget_declared: bool,
    pub rollback_plan_declared: bool,
    pub canary_readback_receipt_required: bool,
    pub canary_result_receipt_schema_present: bool,
    pub operator_acceptance_present: bool,
    pub canary_cutover_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
    pub canary_execution_started: bool,
    pub canary_result_receipt_written: bool,
    pub rollback_executed: bool,
}

impl Default for ToolExecutionCanaryCutoverPlanInput {
    fn default() -> Self {
        Self {
            canary_scope_declared: true,
            canary_budget_declared: true,
            rollback_plan_declared: true,
            canary_readback_receipt_required: true,
            canary_result_receipt_schema_present: true,
            operator_acceptance_present: false,
            canary_cutover_switch_enabled: false,
            live_cutover_switch_enabled: false,
            canary_execution_started: false,
            canary_result_receipt_written: false,
            rollback_executed: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionCanaryCutoverPlanEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_decision_preflight_route: ToolExecutionOperatorApprovalDecisionPreflightRoute,
    pub source_status_canary_spec_route: Option<ToolExecutionStatusCanarySpecRoute>,
    pub source_status_canary_spec_ready: bool,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub selected_for_status_canary: bool,
    pub preflight_only_non_selected_candidate: bool,
    pub canary_cutover_plan_route: ToolExecutionCanaryCutoverPlanRoute,
    pub canary_cutover_plan_ready: bool,
    pub canary_scope_declared: bool,
    pub canary_budget_declared: bool,
    pub rollback_plan_required: bool,
    pub rollback_plan_declared: bool,
    pub canary_readback_receipt_required: bool,
    pub canary_result_receipt_schema_present: bool,
    pub canary_start_blocked: bool,
    pub operator_acceptance_present: bool,
    pub canary_cutover_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
    pub canary_execution_started: bool,
    pub canary_result_receipt_written: bool,
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
pub struct ToolExecutionCanaryCutoverPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_decision_preflight_surface: &'static str,
    pub source_decision_preflight_ready: bool,
    pub source_operator_decision_acceptance_allowed: bool,
    pub source_canary_cutover_allowed: bool,
    pub source_live_cutover_allowed: bool,
    pub source_status_canary_spec_surface: &'static str,
    pub source_status_canary_spec_ready: bool,
    pub source_status_canary_start_allowed: bool,
    pub status_canary_tool_id: &'static str,
    pub canary_scope_declared: bool,
    pub canary_budget_declared: bool,
    pub rollback_plan_declared: bool,
    pub canary_readback_receipt_required: bool,
    pub canary_result_receipt_schema_present: bool,
    pub operator_acceptance_present: bool,
    pub canary_cutover_switch_enabled: bool,
    pub live_cutover_switch_enabled: bool,
    pub canary_execution_started: bool,
    pub canary_result_receipt_written: bool,
    pub rollback_executed: bool,
    pub candidate_count: usize,
    pub canary_cutover_plan_ready_count: usize,
    pub canary_cutover_plan_blocked_count: usize,
    pub canary_scope_declared_count: usize,
    pub canary_budget_declared_count: usize,
    pub rollback_plan_required_count: usize,
    pub canary_readback_receipt_required_count: usize,
    pub canary_start_blocked_count: usize,
    pub selected_status_canary_count: usize,
    pub preflight_only_non_selected_count: usize,
    pub all_decision_preflight_entries_bound_to_canary_plan: bool,
    pub all_status_canary_spec_entries_bound_to_canary_plan: bool,
    pub all_canary_plan_entries_keep_no_invocation_guard: bool,
    pub tool_execution_canary_cutover_plan_ready: bool,
    pub tool_execution_canary_cutover_start_allowed: bool,
    pub tool_execution_canary_result_receipt_write_allowed: bool,
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
    pub entries: Vec<ToolExecutionCanaryCutoverPlanEntry>,
}

pub fn hepta_system_tool_execution_canary_cutover_plan() -> ToolExecutionCanaryCutoverPlan {
    let decision = hepta_system_tool_execution_operator_approval_decision_preflight_plan();
    let status_canary = hepta_system_tool_execution_status_canary_spec_plan();
    tool_execution_canary_cutover_plan_with_status_canary_spec(
        &decision,
        &status_canary,
        &ToolExecutionCanaryCutoverPlanInput::default(),
    )
}

pub fn tool_execution_canary_cutover_plan(
    decision: &ToolExecutionOperatorApprovalDecisionPreflightPlan,
    input: &ToolExecutionCanaryCutoverPlanInput,
) -> ToolExecutionCanaryCutoverPlan {
    let status_canary = hepta_system_tool_execution_status_canary_spec_plan();
    tool_execution_canary_cutover_plan_with_status_canary_spec(decision, &status_canary, input)
}

pub fn tool_execution_canary_cutover_plan_with_status_canary_spec(
    decision: &ToolExecutionOperatorApprovalDecisionPreflightPlan,
    status_canary: &ToolExecutionStatusCanarySpecPlan,
    input: &ToolExecutionCanaryCutoverPlanInput,
) -> ToolExecutionCanaryCutoverPlan {
    let entries = decision
        .entries
        .iter()
        .map(|entry| {
            let status_canary_entry = status_canary
                .entries
                .iter()
                .find(|status_entry| status_entry.candidate_tool_id == entry.candidate_tool_id);
            let source_status_canary_spec_route =
                status_canary_entry.map(|status_entry| status_entry.canary_spec_route);
            let selected_for_status_canary = status_canary_entry
                .is_some_and(|status_entry| status_entry.selected_for_status_canary);
            let preflight_only_non_selected_candidate = status_canary_entry.is_some_and(
                |status_entry| {
                    !status_entry.selected_for_status_canary
                        && status_entry.canary_spec_route
                            == ToolExecutionStatusCanarySpecRoute::PreflightOnlyNonSelectedCandidate
                },
            );
            let source_status_canary_spec_ready = status_canary.status_canary_spec_ready
                && status_canary_entry.is_some_and(|status_entry| status_entry.canary_spec_ready);
            let route = if input.canary_cutover_switch_enabled || input.live_cutover_switch_enabled
            {
                ToolExecutionCanaryCutoverPlanRoute::BlockedByCutoverSwitch
            } else if input.canary_execution_started
                || input.canary_result_receipt_written
                || input.rollback_executed
            {
                ToolExecutionCanaryCutoverPlanRoute::BlockedByPrematureCanaryMutation
            } else if input.operator_acceptance_present {
                ToolExecutionCanaryCutoverPlanRoute::BlockedByOperatorAcceptance
            } else if !input.canary_scope_declared {
                ToolExecutionCanaryCutoverPlanRoute::BlockedByMissingCanaryScope
            } else if !input.canary_budget_declared {
                ToolExecutionCanaryCutoverPlanRoute::BlockedByMissingCanaryBudget
            } else if !input.rollback_plan_declared {
                ToolExecutionCanaryCutoverPlanRoute::BlockedByMissingRollbackPlan
            } else if !input.canary_readback_receipt_required
                || !input.canary_result_receipt_schema_present
            {
                ToolExecutionCanaryCutoverPlanRoute::BlockedByMissingReadbackReceipt
            } else if !entry.operator_approval_decision_preflight_ready
                || entry.operator_approval_decision_preflight_route
                    != ToolExecutionOperatorApprovalDecisionPreflightRoute::OperatorApprovalDecisionPendingExplicitApproval
            {
                ToolExecutionCanaryCutoverPlanRoute::BlockedByDecisionPreflight
            } else if !source_status_canary_spec_ready {
                ToolExecutionCanaryCutoverPlanRoute::BlockedByStatusCanarySpec
            } else if preflight_only_non_selected_candidate {
                ToolExecutionCanaryCutoverPlanRoute::PreflightOnlyNonSelectedCandidate
            } else {
                ToolExecutionCanaryCutoverPlanRoute::CanaryCutoverPlanReady
            };
            let ready = matches!(
                route,
                ToolExecutionCanaryCutoverPlanRoute::CanaryCutoverPlanReady
                    | ToolExecutionCanaryCutoverPlanRoute::PreflightOnlyNonSelectedCandidate
            )
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && entry.operator_decision_pending
                && entry.operator_decision_write_blocked
                && entry.operator_acceptance_blocked;
            let canary_start_blocked = selected_for_status_canary
                && ready
                && !input.operator_acceptance_present
                && !input.canary_cutover_switch_enabled
                && !input.canary_execution_started;

            ToolExecutionCanaryCutoverPlanEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_decision_preflight_route: entry
                    .operator_approval_decision_preflight_route,
                source_status_canary_spec_route,
                source_status_canary_spec_ready,
                registry_guard_route: entry.registry_guard_route,
                selected_for_status_canary,
                preflight_only_non_selected_candidate,
                canary_cutover_plan_route: route,
                canary_cutover_plan_ready: ready,
                canary_scope_declared: input.canary_scope_declared,
                canary_budget_declared: input.canary_budget_declared,
                rollback_plan_required: true,
                rollback_plan_declared: input.rollback_plan_declared,
                canary_readback_receipt_required: input.canary_readback_receipt_required,
                canary_result_receipt_schema_present: input.canary_result_receipt_schema_present,
                canary_start_blocked,
                operator_acceptance_present: input.operator_acceptance_present,
                canary_cutover_switch_enabled: input.canary_cutover_switch_enabled,
                live_cutover_switch_enabled: input.live_cutover_switch_enabled,
                canary_execution_started: input.canary_execution_started,
                canary_result_receipt_written: input.canary_result_receipt_written,
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
        .filter(|entry| entry.canary_cutover_plan_ready)
        .count();
    let scope_count = entries
        .iter()
        .filter(|entry| entry.canary_scope_declared)
        .count();
    let budget_count = entries
        .iter()
        .filter(|entry| entry.canary_budget_declared)
        .count();
    let rollback_count = entries
        .iter()
        .filter(|entry| entry.rollback_plan_required && entry.rollback_plan_declared)
        .count();
    let readback_count = entries
        .iter()
        .filter(|entry| {
            entry.canary_readback_receipt_required && entry.canary_result_receipt_schema_present
        })
        .count();
    let start_blocked_count = entries
        .iter()
        .filter(|entry| entry.canary_start_blocked)
        .count();
    let selected_status_canary_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary)
        .count();
    let preflight_only_non_selected_count = entries
        .iter()
        .filter(|entry| entry.preflight_only_non_selected_candidate)
        .count();
    let all_decision_preflight_entries_bound_to_canary_plan = ready_count == entries.len()
        && scope_count == entries.len()
        && budget_count == entries.len()
        && rollback_count == entries.len()
        && readback_count == entries.len()
        && start_blocked_count == selected_status_canary_count;
    let all_status_canary_spec_entries_bound_to_canary_plan = status_canary
        .status_canary_spec_ready
        && selected_status_canary_count == 1
        && preflight_only_non_selected_count + selected_status_canary_count == entries.len()
        && entries.iter().all(|entry| {
            entry.source_status_canary_spec_ready
                && (entry.selected_for_status_canary || entry.preflight_only_non_selected_candidate)
        });
    let all_canary_plan_entries_keep_no_invocation_guard = entries.iter().all(|entry| {
        if matches!(
            entry.canary_cutover_plan_route,
            ToolExecutionCanaryCutoverPlanRoute::CanaryCutoverPlanReady
                | ToolExecutionCanaryCutoverPlanRoute::PreflightOnlyNonSelectedCandidate
        ) {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.operator_acceptance_present
                && !entry.canary_cutover_switch_enabled
                && !entry.live_cutover_switch_enabled
                && !entry.canary_execution_started
                && !entry.canary_result_receipt_written
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
    let tool_execution_canary_cutover_plan_ready = decision
        .tool_execution_operator_approval_decision_preflight_ready
        && !decision.tool_execution_operator_decision_acceptance_allowed
        && !decision.tool_execution_canary_cutover_allowed
        && !decision.tool_execution_live_cutover_allowed
        && status_canary.status_canary_spec_ready
        && !status_canary.status_canary_start_allowed
        && input.canary_scope_declared
        && input.canary_budget_declared
        && input.rollback_plan_declared
        && input.canary_readback_receipt_required
        && input.canary_result_receipt_schema_present
        && !input.operator_acceptance_present
        && !input.canary_cutover_switch_enabled
        && !input.live_cutover_switch_enabled
        && !input.canary_execution_started
        && !input.canary_result_receipt_written
        && !input.rollback_executed
        && all_decision_preflight_entries_bound_to_canary_plan
        && all_status_canary_spec_entries_bound_to_canary_plan
        && all_canary_plan_entries_keep_no_invocation_guard;

    ToolExecutionCanaryCutoverPlan {
        runtime: "hepta",
        surface: "tool_execution_canary_cutover_plan",
        plugin_id: decision.plugin_id,
        status: if tool_execution_canary_cutover_plan_ready {
            "ready"
        } else {
            "blocked"
        },
        source_decision_preflight_surface: decision.surface,
        source_decision_preflight_ready: decision
            .tool_execution_operator_approval_decision_preflight_ready,
        source_operator_decision_acceptance_allowed: decision
            .tool_execution_operator_decision_acceptance_allowed,
        source_canary_cutover_allowed: decision.tool_execution_canary_cutover_allowed,
        source_live_cutover_allowed: decision.tool_execution_live_cutover_allowed,
        source_status_canary_spec_surface: status_canary.surface,
        source_status_canary_spec_ready: status_canary.status_canary_spec_ready,
        source_status_canary_start_allowed: status_canary.status_canary_start_allowed,
        status_canary_tool_id: HEPTA_SYSTEM_STATUS_CANARY_TOOL_ID,
        canary_scope_declared: input.canary_scope_declared,
        canary_budget_declared: input.canary_budget_declared,
        rollback_plan_declared: input.rollback_plan_declared,
        canary_readback_receipt_required: input.canary_readback_receipt_required,
        canary_result_receipt_schema_present: input.canary_result_receipt_schema_present,
        operator_acceptance_present: input.operator_acceptance_present,
        canary_cutover_switch_enabled: input.canary_cutover_switch_enabled,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        canary_execution_started: input.canary_execution_started,
        canary_result_receipt_written: input.canary_result_receipt_written,
        rollback_executed: input.rollback_executed,
        candidate_count: entries.len(),
        canary_cutover_plan_ready_count: ready_count,
        canary_cutover_plan_blocked_count: entries.len() - ready_count,
        canary_scope_declared_count: scope_count,
        canary_budget_declared_count: budget_count,
        rollback_plan_required_count: rollback_count,
        canary_readback_receipt_required_count: readback_count,
        canary_start_blocked_count: start_blocked_count,
        selected_status_canary_count,
        preflight_only_non_selected_count,
        all_decision_preflight_entries_bound_to_canary_plan,
        all_status_canary_spec_entries_bound_to_canary_plan,
        all_canary_plan_entries_keep_no_invocation_guard,
        tool_execution_canary_cutover_plan_ready,
        tool_execution_canary_cutover_start_allowed: false,
        tool_execution_canary_result_receipt_write_allowed: false,
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
        next_migration_step: "restore_tool_execution_canary_readback_receipt_projection_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_system_plugin_tool_contribution_inventory_preview_plan;
    use crate::hepta_system_tool_execution_adapter_preflight_plan;
    use crate::hepta_system_tool_registry_read_only_dispatch_preflight_plan;
    use crate::tool_execution_dispatch_shadow_plan_with_registry_shadow_pipeline;
    use crate::tool_execution_status_canary_spec_plan;
    use crate::tool_registry_shadow_pipeline_plan;

    #[test]
    fn tool_execution_canary_cutover_plan_collects_canary_scope() {
        let plan = hepta_system_tool_execution_canary_cutover_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_decision_preflight_surface,
            "tool_execution_operator_approval_decision_preflight"
        );
        assert!(plan.source_decision_preflight_ready);
        assert!(!plan.source_operator_decision_acceptance_allowed);
        assert!(!plan.source_canary_cutover_allowed);
        assert!(!plan.source_live_cutover_allowed);
        assert_eq!(
            plan.source_status_canary_spec_surface,
            "tool_execution_status_canary_spec"
        );
        assert!(plan.source_status_canary_spec_ready);
        assert!(!plan.source_status_canary_start_allowed);
        assert_eq!(
            plan.status_canary_tool_id,
            HEPTA_SYSTEM_STATUS_CANARY_TOOL_ID
        );
        assert!(plan.canary_scope_declared);
        assert!(plan.canary_budget_declared);
        assert!(plan.rollback_plan_declared);
        assert!(plan.canary_readback_receipt_required);
        assert!(plan.canary_result_receipt_schema_present);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.canary_cutover_plan_ready_count, 2);
        assert_eq!(plan.canary_cutover_plan_blocked_count, 0);
        assert_eq!(plan.canary_scope_declared_count, 2);
        assert_eq!(plan.canary_budget_declared_count, 2);
        assert_eq!(plan.rollback_plan_required_count, 2);
        assert_eq!(plan.canary_readback_receipt_required_count, 2);
        assert_eq!(plan.canary_start_blocked_count, 1);
        assert_eq!(plan.selected_status_canary_count, 1);
        assert_eq!(plan.preflight_only_non_selected_count, 1);
        assert!(plan.all_decision_preflight_entries_bound_to_canary_plan);
        assert!(plan.all_status_canary_spec_entries_bound_to_canary_plan);
        assert!(plan.all_canary_plan_entries_keep_no_invocation_guard);
        assert!(plan.tool_execution_canary_cutover_plan_ready);
        assert!(!plan.tool_execution_canary_cutover_start_allowed);
        assert!(!plan.tool_execution_canary_result_receipt_write_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);

        let selected = plan
            .entries
            .iter()
            .find(|entry| entry.selected_for_status_canary)
            .expect("selected status canary cutover entry");
        assert_eq!(
            selected.candidate_tool_id,
            HEPTA_SYSTEM_STATUS_CANARY_TOOL_ID
        );
        assert_eq!(
            selected.canary_cutover_plan_route,
            ToolExecutionCanaryCutoverPlanRoute::CanaryCutoverPlanReady
        );
        assert!(selected.canary_start_blocked);

        let preflight_only = plan
            .entries
            .iter()
            .find(|entry| entry.preflight_only_non_selected_candidate)
            .expect("non-selected preflight-only candidate");
        assert_eq!(
            preflight_only.canary_cutover_plan_route,
            ToolExecutionCanaryCutoverPlanRoute::PreflightOnlyNonSelectedCandidate
        );
        assert!(!preflight_only.canary_start_blocked);
    }

    #[test]
    fn tool_execution_canary_cutover_plan_does_not_start_execution() {
        let plan = hepta_system_tool_execution_canary_cutover_plan();

        assert!(plan.tool_execution_canary_cutover_plan_ready);
        assert!(!plan.tool_execution_canary_cutover_start_allowed);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.operator_decision_record_written);
        assert!(!plan.operator_decision_receipt_written);
        assert!(!plan.result_receipt_written);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn tool_execution_canary_cutover_plan_fails_closed_without_canary_scope() {
        let decision = hepta_system_tool_execution_operator_approval_decision_preflight_plan();
        let input = ToolExecutionCanaryCutoverPlanInput {
            canary_scope_declared: false,
            ..ToolExecutionCanaryCutoverPlanInput::default()
        };

        let plan = tool_execution_canary_cutover_plan(&decision, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.canary_scope_declared);
        assert_eq!(plan.canary_cutover_plan_ready_count, 0);
        assert_eq!(plan.canary_cutover_plan_blocked_count, 2);
        assert!(!plan.tool_execution_canary_cutover_plan_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_cutover_plan_route
                == ToolExecutionCanaryCutoverPlanRoute::BlockedByMissingCanaryScope
        }));
    }

    #[test]
    fn tool_execution_canary_cutover_plan_fails_closed_when_status_canary_spec_blocks() {
        let mut contributions = hepta_system_plugin_tool_contribution_inventory_preview_plan();
        let duplicate = contributions.candidate_inventory_entries[0].clone();
        contributions.candidate_inventory_entries.push(duplicate);
        let dispatch_preflight = hepta_system_tool_registry_read_only_dispatch_preflight_plan();
        let pipeline = tool_registry_shadow_pipeline_plan(&contributions, &dispatch_preflight);
        let adapter = hepta_system_tool_execution_adapter_preflight_plan();
        let dispatch = tool_execution_dispatch_shadow_plan_with_registry_shadow_pipeline(
            &adapter,
            &pipeline,
            &Default::default(),
        );
        let status_canary = tool_execution_status_canary_spec_plan(&dispatch, &Default::default());
        let decision = hepta_system_tool_execution_operator_approval_decision_preflight_plan();

        let plan = tool_execution_canary_cutover_plan_with_status_canary_spec(
            &decision,
            &status_canary,
            &Default::default(),
        );

        assert_eq!(status_canary.status, "blocked");
        assert_eq!(plan.status, "blocked");
        assert!(!plan.source_status_canary_spec_ready);
        assert_eq!(plan.canary_cutover_plan_ready_count, 0);
        assert_eq!(plan.canary_cutover_plan_blocked_count, 2);
        assert_eq!(plan.selected_status_canary_count, 1);
        assert_eq!(plan.preflight_only_non_selected_count, 0);
        assert!(!plan.all_status_canary_spec_entries_bound_to_canary_plan);
        assert!(!plan.tool_execution_canary_cutover_plan_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_cutover_plan_route
                == ToolExecutionCanaryCutoverPlanRoute::BlockedByStatusCanarySpec
                && !entry.canary_cutover_plan_ready
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.result_receipt_write_enabled
        }));
    }

    #[test]
    fn tool_execution_canary_cutover_plan_fails_closed_without_rollback() {
        let decision = hepta_system_tool_execution_operator_approval_decision_preflight_plan();
        let input = ToolExecutionCanaryCutoverPlanInput {
            rollback_plan_declared: false,
            ..ToolExecutionCanaryCutoverPlanInput::default()
        };

        let plan = tool_execution_canary_cutover_plan(&decision, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.rollback_plan_declared);
        assert_eq!(plan.rollback_plan_required_count, 0);
        assert!(!plan.tool_execution_canary_cutover_plan_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_cutover_plan_route
                == ToolExecutionCanaryCutoverPlanRoute::BlockedByMissingRollbackPlan
        }));
    }

    #[test]
    fn tool_execution_canary_cutover_plan_fails_closed_on_premature_canary_mutation() {
        let decision = hepta_system_tool_execution_operator_approval_decision_preflight_plan();
        let input = ToolExecutionCanaryCutoverPlanInput {
            canary_execution_started: true,
            canary_result_receipt_written: true,
            rollback_executed: true,
            ..ToolExecutionCanaryCutoverPlanInput::default()
        };

        let plan = tool_execution_canary_cutover_plan(&decision, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.canary_execution_started);
        assert!(plan.canary_result_receipt_written);
        assert!(plan.rollback_executed);
        assert!(!plan.tool_execution_canary_cutover_plan_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_cutover_plan_route
                == ToolExecutionCanaryCutoverPlanRoute::BlockedByPrematureCanaryMutation
        }));
    }

    #[test]
    fn tool_execution_canary_cutover_plan_fails_closed_when_cutover_switch_enabled() {
        let decision = hepta_system_tool_execution_operator_approval_decision_preflight_plan();
        let input = ToolExecutionCanaryCutoverPlanInput {
            canary_cutover_switch_enabled: true,
            live_cutover_switch_enabled: true,
            ..ToolExecutionCanaryCutoverPlanInput::default()
        };

        let plan = tool_execution_canary_cutover_plan(&decision, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.canary_cutover_switch_enabled);
        assert!(plan.live_cutover_switch_enabled);
        assert!(!plan.tool_execution_canary_cutover_plan_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_cutover_plan_route
                == ToolExecutionCanaryCutoverPlanRoute::BlockedByCutoverSwitch
        }));
    }
}
