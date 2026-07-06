use crate::ToolExecutionCanaryReadbackReceiptProjectionPlan;
use crate::ToolExecutionCanaryReadbackReceiptProjectionRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_canary_readback_receipt_projection_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionCanaryResultAcceptancePreflightRoute {
    CanaryResultAcceptancePendingEvidence,
    PreflightOnlyNonSelectedCandidate,
    BlockedByReadbackProjection,
    BlockedByMissingAcceptancePolicy,
    BlockedByMissingOperatorIdentityBinding,
    BlockedByPrematureAcceptanceMutation,
    BlockedByLiveCutoverSwitch,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionCanaryResultAcceptancePreflightInput {
    pub canary_result_acceptance_policy_present: bool,
    pub operator_identity_binding_present: bool,
    pub canary_result_receipt_present: bool,
    pub canary_readback_evidence_present: bool,
    pub operator_canary_result_acceptance_present: bool,
    pub canary_acceptance_record_written: bool,
    pub canary_acceptance_receipt_written: bool,
    pub live_cutover_switch_enabled: bool,
}

impl Default for ToolExecutionCanaryResultAcceptancePreflightInput {
    fn default() -> Self {
        Self {
            canary_result_acceptance_policy_present: true,
            operator_identity_binding_present: true,
            canary_result_receipt_present: false,
            canary_readback_evidence_present: false,
            operator_canary_result_acceptance_present: false,
            canary_acceptance_record_written: false,
            canary_acceptance_receipt_written: false,
            live_cutover_switch_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionCanaryResultAcceptancePreflightEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub execution_adapter_kind: &'static str,
    pub source_readback_projection_route: ToolExecutionCanaryReadbackReceiptProjectionRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub selected_for_status_canary: bool,
    pub preflight_only_non_selected_candidate: bool,
    pub canary_result_acceptance_preflight_route: ToolExecutionCanaryResultAcceptancePreflightRoute,
    pub canary_result_acceptance_preflight_ready: bool,
    pub canary_result_acceptance_pending_evidence: bool,
    pub canary_result_receipt_required: bool,
    pub canary_readback_evidence_required: bool,
    pub canary_acceptance_record_write_blocked: bool,
    pub canary_acceptance_receipt_write_blocked: bool,
    pub canary_result_acceptance_policy_present: bool,
    pub operator_identity_binding_present: bool,
    pub canary_result_receipt_present: bool,
    pub canary_readback_evidence_present: bool,
    pub operator_canary_result_acceptance_present: bool,
    pub canary_acceptance_record_written: bool,
    pub canary_acceptance_receipt_written: bool,
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
pub struct ToolExecutionCanaryResultAcceptancePreflightPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_readback_projection_surface: &'static str,
    pub source_readback_projection_ready: bool,
    pub source_canary_result_receipt_write_allowed: bool,
    pub source_canary_result_acceptance_allowed: bool,
    pub source_live_cutover_allowed: bool,
    pub canary_result_acceptance_policy_present: bool,
    pub operator_identity_binding_present: bool,
    pub canary_result_receipt_present: bool,
    pub canary_readback_evidence_present: bool,
    pub operator_canary_result_acceptance_present: bool,
    pub canary_acceptance_record_written: bool,
    pub canary_acceptance_receipt_written: bool,
    pub live_cutover_switch_enabled: bool,
    pub candidate_count: usize,
    pub canary_result_acceptance_preflight_ready_count: usize,
    pub canary_result_acceptance_preflight_blocked_count: usize,
    pub canary_result_acceptance_pending_evidence_count: usize,
    pub canary_result_receipt_required_count: usize,
    pub canary_readback_evidence_required_count: usize,
    pub canary_acceptance_record_write_blocked_count: usize,
    pub canary_acceptance_receipt_write_blocked_count: usize,
    pub selected_status_canary_count: usize,
    pub preflight_only_non_selected_count: usize,
    pub all_readback_projections_bound_to_acceptance_preflight: bool,
    pub all_acceptance_preflight_entries_keep_no_invocation_guard: bool,
    pub tool_execution_canary_result_acceptance_preflight_ready: bool,
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
    pub result_receipt_written: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<ToolExecutionCanaryResultAcceptancePreflightEntry>,
}

pub fn hepta_system_tool_execution_canary_result_acceptance_preflight_plan()
-> ToolExecutionCanaryResultAcceptancePreflightPlan {
    let projection = hepta_system_tool_execution_canary_readback_receipt_projection_plan();
    tool_execution_canary_result_acceptance_preflight_plan(
        &projection,
        &ToolExecutionCanaryResultAcceptancePreflightInput::default(),
    )
}

pub fn tool_execution_canary_result_acceptance_preflight_plan(
    projection: &ToolExecutionCanaryReadbackReceiptProjectionPlan,
    input: &ToolExecutionCanaryResultAcceptancePreflightInput,
) -> ToolExecutionCanaryResultAcceptancePreflightPlan {
    let entries = projection
        .entries
        .iter()
        .map(|entry| {
            let route = if input.live_cutover_switch_enabled {
                ToolExecutionCanaryResultAcceptancePreflightRoute::BlockedByLiveCutoverSwitch
            } else if input.canary_result_receipt_present
                || input.canary_readback_evidence_present
                || input.operator_canary_result_acceptance_present
                || input.canary_acceptance_record_written
                || input.canary_acceptance_receipt_written
            {
                ToolExecutionCanaryResultAcceptancePreflightRoute::BlockedByPrematureAcceptanceMutation
            } else if !input.canary_result_acceptance_policy_present {
                ToolExecutionCanaryResultAcceptancePreflightRoute::BlockedByMissingAcceptancePolicy
            } else if !input.operator_identity_binding_present {
                ToolExecutionCanaryResultAcceptancePreflightRoute::BlockedByMissingOperatorIdentityBinding
            } else if entry.preflight_only_non_selected_candidate
                && entry.canary_readback_receipt_projection_route
                    == ToolExecutionCanaryReadbackReceiptProjectionRoute::PreflightOnlyNonSelectedCandidate
            {
                ToolExecutionCanaryResultAcceptancePreflightRoute::PreflightOnlyNonSelectedCandidate
            } else if !entry.canary_readback_receipt_projection_ready
                || entry.canary_readback_receipt_projection_route
                    != ToolExecutionCanaryReadbackReceiptProjectionRoute::CanaryReadbackReceiptProjectionReady
            {
                ToolExecutionCanaryResultAcceptancePreflightRoute::BlockedByReadbackProjection
            } else {
                ToolExecutionCanaryResultAcceptancePreflightRoute::CanaryResultAcceptancePendingEvidence
            };
            let ready = matches!(
                route,
                ToolExecutionCanaryResultAcceptancePreflightRoute::CanaryResultAcceptancePendingEvidence
                    | ToolExecutionCanaryResultAcceptancePreflightRoute::PreflightOnlyNonSelectedCandidate
            )
                && entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && (entry.preflight_only_non_selected_candidate
                    || entry.canary_result_receipt_write_blocked);
            let canary_result_acceptance_pending_evidence = ready
                && route
                    == ToolExecutionCanaryResultAcceptancePreflightRoute::CanaryResultAcceptancePendingEvidence;
            let canary_acceptance_record_write_blocked = ready
                && entry.selected_for_status_canary
                && !input.canary_result_receipt_present
                && !input.operator_canary_result_acceptance_present
                && !input.canary_acceptance_record_written;
            let canary_acceptance_receipt_write_blocked = ready
                && entry.selected_for_status_canary
                && !input.canary_readback_evidence_present
                && !input.canary_acceptance_receipt_written;

            ToolExecutionCanaryResultAcceptancePreflightEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                execution_adapter_kind: entry.execution_adapter_kind,
                source_readback_projection_route: entry
                    .canary_readback_receipt_projection_route,
                registry_guard_route: entry.registry_guard_route,
                selected_for_status_canary: entry.selected_for_status_canary,
                preflight_only_non_selected_candidate: entry.preflight_only_non_selected_candidate,
                canary_result_acceptance_preflight_route: route,
                canary_result_acceptance_preflight_ready: ready,
                canary_result_acceptance_pending_evidence,
                canary_result_receipt_required: entry.selected_for_status_canary,
                canary_readback_evidence_required: entry.selected_for_status_canary,
                canary_acceptance_record_write_blocked,
                canary_acceptance_receipt_write_blocked,
                canary_result_acceptance_policy_present: input
                    .canary_result_acceptance_policy_present,
                operator_identity_binding_present: input.operator_identity_binding_present,
                canary_result_receipt_present: input.canary_result_receipt_present,
                canary_readback_evidence_present: input.canary_readback_evidence_present,
                operator_canary_result_acceptance_present: input
                    .operator_canary_result_acceptance_present,
                canary_acceptance_record_written: input.canary_acceptance_record_written,
                canary_acceptance_receipt_written: input.canary_acceptance_receipt_written,
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

    let ready_count = entries
        .iter()
        .filter(|entry| entry.canary_result_acceptance_preflight_ready)
        .count();
    let pending_count = entries
        .iter()
        .filter(|entry| entry.canary_result_acceptance_pending_evidence)
        .count();
    let receipt_required_count = entries
        .iter()
        .filter(|entry| entry.canary_result_receipt_required)
        .count();
    let readback_required_count = entries
        .iter()
        .filter(|entry| entry.canary_readback_evidence_required)
        .count();
    let record_write_blocked_count = entries
        .iter()
        .filter(|entry| entry.canary_acceptance_record_write_blocked)
        .count();
    let receipt_write_blocked_count = entries
        .iter()
        .filter(|entry| entry.canary_acceptance_receipt_write_blocked)
        .count();
    let selected_status_canary_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary)
        .count();
    let preflight_only_non_selected_count = entries
        .iter()
        .filter(|entry| entry.preflight_only_non_selected_candidate)
        .count();
    let all_readback_projections_bound_to_acceptance_preflight = ready_count == entries.len()
        && selected_status_canary_count == 1
        && preflight_only_non_selected_count + selected_status_canary_count == entries.len()
        && pending_count == selected_status_canary_count
        && receipt_required_count == selected_status_canary_count
        && readback_required_count == selected_status_canary_count
        && record_write_blocked_count == selected_status_canary_count
        && receipt_write_blocked_count == selected_status_canary_count;
    let all_acceptance_preflight_entries_keep_no_invocation_guard = entries.iter().all(|entry| {
        if matches!(
            entry.canary_result_acceptance_preflight_route,
            ToolExecutionCanaryResultAcceptancePreflightRoute::CanaryResultAcceptancePendingEvidence
                | ToolExecutionCanaryResultAcceptancePreflightRoute::PreflightOnlyNonSelectedCandidate
        ) {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.canary_result_receipt_present
                && !entry.canary_readback_evidence_present
                && !entry.operator_canary_result_acceptance_present
                && !entry.canary_acceptance_record_written
                && !entry.canary_acceptance_receipt_written
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
    let tool_execution_canary_result_acceptance_preflight_ready = projection
        .tool_execution_canary_readback_receipt_projection_ready
        && !projection.tool_execution_canary_result_receipt_write_allowed
        && !projection.tool_execution_canary_result_acceptance_allowed
        && !projection.tool_execution_live_cutover_allowed
        && input.canary_result_acceptance_policy_present
        && input.operator_identity_binding_present
        && !input.canary_result_receipt_present
        && !input.canary_readback_evidence_present
        && !input.operator_canary_result_acceptance_present
        && !input.canary_acceptance_record_written
        && !input.canary_acceptance_receipt_written
        && !input.live_cutover_switch_enabled
        && all_readback_projections_bound_to_acceptance_preflight
        && all_acceptance_preflight_entries_keep_no_invocation_guard;

    ToolExecutionCanaryResultAcceptancePreflightPlan {
        runtime: "hepta",
        surface: "tool_execution_canary_result_acceptance_preflight",
        plugin_id: projection.plugin_id,
        status: if tool_execution_canary_result_acceptance_preflight_ready {
            "ready"
        } else {
            "blocked"
        },
        source_readback_projection_surface: projection.surface,
        source_readback_projection_ready: projection
            .tool_execution_canary_readback_receipt_projection_ready,
        source_canary_result_receipt_write_allowed: projection
            .tool_execution_canary_result_receipt_write_allowed,
        source_canary_result_acceptance_allowed: projection
            .tool_execution_canary_result_acceptance_allowed,
        source_live_cutover_allowed: projection.tool_execution_live_cutover_allowed,
        canary_result_acceptance_policy_present: input.canary_result_acceptance_policy_present,
        operator_identity_binding_present: input.operator_identity_binding_present,
        canary_result_receipt_present: input.canary_result_receipt_present,
        canary_readback_evidence_present: input.canary_readback_evidence_present,
        operator_canary_result_acceptance_present: input.operator_canary_result_acceptance_present,
        canary_acceptance_record_written: input.canary_acceptance_record_written,
        canary_acceptance_receipt_written: input.canary_acceptance_receipt_written,
        live_cutover_switch_enabled: input.live_cutover_switch_enabled,
        candidate_count: entries.len(),
        canary_result_acceptance_preflight_ready_count: ready_count,
        canary_result_acceptance_preflight_blocked_count: entries.len() - ready_count,
        canary_result_acceptance_pending_evidence_count: pending_count,
        canary_result_receipt_required_count: receipt_required_count,
        canary_readback_evidence_required_count: readback_required_count,
        canary_acceptance_record_write_blocked_count: record_write_blocked_count,
        canary_acceptance_receipt_write_blocked_count: receipt_write_blocked_count,
        selected_status_canary_count,
        preflight_only_non_selected_count,
        all_readback_projections_bound_to_acceptance_preflight,
        all_acceptance_preflight_entries_keep_no_invocation_guard,
        tool_execution_canary_result_acceptance_preflight_ready,
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
        result_receipt_written: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_execution_live_cutover_preflight_without_invocation",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canary_result_acceptance_preflight_collects_pending_evidence() {
        let plan = hepta_system_tool_execution_canary_result_acceptance_preflight_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_readback_projection_surface,
            "tool_execution_canary_readback_receipt_projection"
        );
        assert!(plan.source_readback_projection_ready);
        assert!(!plan.source_canary_result_receipt_write_allowed);
        assert!(!plan.source_canary_result_acceptance_allowed);
        assert!(!plan.source_live_cutover_allowed);
        assert!(plan.canary_result_acceptance_policy_present);
        assert!(plan.operator_identity_binding_present);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.canary_result_acceptance_preflight_ready_count, 2);
        assert_eq!(plan.canary_result_acceptance_preflight_blocked_count, 0);
        assert_eq!(plan.canary_result_acceptance_pending_evidence_count, 1);
        assert_eq!(plan.canary_result_receipt_required_count, 1);
        assert_eq!(plan.canary_readback_evidence_required_count, 1);
        assert_eq!(plan.canary_acceptance_record_write_blocked_count, 1);
        assert_eq!(plan.canary_acceptance_receipt_write_blocked_count, 1);
        assert_eq!(plan.selected_status_canary_count, 1);
        assert_eq!(plan.preflight_only_non_selected_count, 1);
        assert!(plan.all_readback_projections_bound_to_acceptance_preflight);
        assert!(plan.all_acceptance_preflight_entries_keep_no_invocation_guard);
        assert!(plan.tool_execution_canary_result_acceptance_preflight_ready);
        assert!(!plan.tool_execution_canary_result_acceptance_allowed);
        assert!(!plan.tool_execution_live_cutover_allowed);

        let selected = plan
            .entries
            .iter()
            .find(|entry| entry.selected_for_status_canary)
            .expect("selected status canary acceptance entry");
        assert_eq!(
            selected.canary_result_acceptance_preflight_route,
            ToolExecutionCanaryResultAcceptancePreflightRoute::CanaryResultAcceptancePendingEvidence
        );
        assert!(selected.canary_result_acceptance_pending_evidence);

        let preflight_only = plan
            .entries
            .iter()
            .find(|entry| entry.preflight_only_non_selected_candidate)
            .expect("non-selected preflight-only acceptance entry");
        assert_eq!(
            preflight_only.canary_result_acceptance_preflight_route,
            ToolExecutionCanaryResultAcceptancePreflightRoute::PreflightOnlyNonSelectedCandidate
        );
        assert!(!preflight_only.canary_result_acceptance_pending_evidence);
    }

    #[test]
    fn canary_result_acceptance_preflight_does_not_accept_or_cutover() {
        let plan = hepta_system_tool_execution_canary_result_acceptance_preflight_plan();

        assert!(plan.tool_execution_canary_result_acceptance_preflight_ready);
        assert!(!plan.canary_result_receipt_present);
        assert!(!plan.canary_readback_evidence_present);
        assert!(!plan.operator_canary_result_acceptance_present);
        assert!(!plan.canary_acceptance_record_written);
        assert!(!plan.canary_acceptance_receipt_written);
        assert!(!plan.tool_execution_canary_result_acceptance_allowed);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.result_receipt_written);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
    }

    #[test]
    fn canary_result_acceptance_preflight_fails_closed_without_policy() {
        let projection = hepta_system_tool_execution_canary_readback_receipt_projection_plan();
        let input = ToolExecutionCanaryResultAcceptancePreflightInput {
            canary_result_acceptance_policy_present: false,
            ..ToolExecutionCanaryResultAcceptancePreflightInput::default()
        };

        let plan = tool_execution_canary_result_acceptance_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.canary_result_acceptance_policy_present);
        assert_eq!(plan.canary_result_acceptance_preflight_ready_count, 0);
        assert_eq!(plan.canary_result_acceptance_preflight_blocked_count, 2);
        assert!(!plan.tool_execution_canary_result_acceptance_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_result_acceptance_preflight_route
                == ToolExecutionCanaryResultAcceptancePreflightRoute::BlockedByMissingAcceptancePolicy
        }));
    }

    #[test]
    fn canary_result_acceptance_preflight_fails_closed_on_premature_acceptance_mutation() {
        let projection = hepta_system_tool_execution_canary_readback_receipt_projection_plan();
        let input = ToolExecutionCanaryResultAcceptancePreflightInput {
            canary_result_receipt_present: true,
            canary_readback_evidence_present: true,
            operator_canary_result_acceptance_present: true,
            canary_acceptance_record_written: true,
            canary_acceptance_receipt_written: true,
            ..ToolExecutionCanaryResultAcceptancePreflightInput::default()
        };

        let plan = tool_execution_canary_result_acceptance_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.canary_result_receipt_present);
        assert!(plan.canary_readback_evidence_present);
        assert!(plan.operator_canary_result_acceptance_present);
        assert!(plan.canary_acceptance_record_written);
        assert!(plan.canary_acceptance_receipt_written);
        assert!(!plan.tool_execution_canary_result_acceptance_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_result_acceptance_preflight_route
                == ToolExecutionCanaryResultAcceptancePreflightRoute::BlockedByPrematureAcceptanceMutation
        }));
    }

    #[test]
    fn canary_result_acceptance_preflight_fails_closed_when_live_cutover_switch_enabled() {
        let projection = hepta_system_tool_execution_canary_readback_receipt_projection_plan();
        let input = ToolExecutionCanaryResultAcceptancePreflightInput {
            live_cutover_switch_enabled: true,
            ..ToolExecutionCanaryResultAcceptancePreflightInput::default()
        };

        let plan = tool_execution_canary_result_acceptance_preflight_plan(&projection, &input);

        assert_eq!(plan.status, "blocked");
        assert!(plan.live_cutover_switch_enabled);
        assert!(!plan.tool_execution_canary_result_acceptance_preflight_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_result_acceptance_preflight_route
                == ToolExecutionCanaryResultAcceptancePreflightRoute::BlockedByLiveCutoverSwitch
        }));
    }
}
