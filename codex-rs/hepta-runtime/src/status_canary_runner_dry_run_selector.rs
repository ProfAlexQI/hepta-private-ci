use serde::Serialize;

use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;
use crate::status_canary_runner_binding_guard::STATUS_CANARY_RUNNER_BINDING_GUARD_ID;
use crate::status_canary_runner_binding_guard::StatusCanaryRunnerBindingGuardPlan;
use crate::status_canary_runner_binding_guard::StatusCanaryRunnerBindingGuardSideEffects;
use crate::status_canary_runner_binding_guard::status_canary_runner_binding_guard_plan;

pub const STATUS_CANARY_RUNNER_DRY_RUN_SELECTOR_SCHEMA_VERSION: &str =
    "status_canary_runner_dry_run_selector_v1";
pub const STATUS_CANARY_RUNNER_DRY_RUN_SELECTOR_ID: &str =
    "status-canary-runner-dry-run-selector/hepta-system-status/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCanaryRunnerDryRunSelectorInput {
    pub runner_dry_run_selector_request_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryRunnerDryRunSelectorPlan {
    pub runtime: &'static str,
    pub selector: &'static str,
    pub schema_version: &'static str,
    pub selector_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_binding_guard_id: &'static str,
    pub source_binding_guard_route: &'static str,
    pub source_binding_guard_bound: bool,
    pub source_runner_binding_request_present: bool,
    pub source_runner_binding_guard_blocked: bool,
    pub source_runner_binding_guard_allowed: bool,
    pub source_entry_adapter_bound: bool,
    pub source_entry_adapter_allowed: bool,
    pub source_start_request_gate_reason_audit_ready: bool,
    pub source_requested_tool_id: &'static str,
    pub source_requested_selected_status_canary: bool,
    pub source_requested_preflight_only_connector: bool,
    pub source_runtime_boundaries_closed: bool,
    pub source_side_effects_closed: bool,
    pub runner_dry_run_selector_request_present: bool,
    pub runner_dry_run_selector_blocked: bool,
    pub runner_dry_run_selector_allowed: bool,
    pub selector_route: &'static str,
    pub side_effects: StatusCanaryRunnerDryRunSelectorSideEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryRunnerDryRunSelectorSideEffects {
    pub runner_dry_run_selected: bool,
    pub runner_dry_run_executed: bool,
    pub dry_run_request_persisted: bool,
    pub runner_bound: bool,
    pub runner_entered: bool,
    pub runner_started: bool,
    pub runner_command_enqueued: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub waiver_recorded: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub credential_read: bool,
    pub transport_mutated: bool,
    pub registry_mutated: bool,
    pub tool_invoked: bool,
    pub connector_started: bool,
    pub ledger_written: bool,
    pub receipt_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub canary_started: bool,
    pub live_execution_started: bool,
}

pub fn status_canary_runner_dry_run_selector_plan() -> StatusCanaryRunnerDryRunSelectorPlan {
    let binding_guard = status_canary_runner_binding_guard_plan();
    status_canary_runner_dry_run_selector_plan_from_binding_guard(
        &binding_guard,
        StatusCanaryRunnerDryRunSelectorInput::default(),
    )
}

pub fn status_canary_runner_dry_run_selector_plan_from_binding_guard(
    binding_guard: &StatusCanaryRunnerBindingGuardPlan,
    input: StatusCanaryRunnerDryRunSelectorInput,
) -> StatusCanaryRunnerDryRunSelectorPlan {
    let side_effects = StatusCanaryRunnerDryRunSelectorSideEffects::none();
    let source_side_effects_closed =
        binding_guard.side_effects == StatusCanaryRunnerBindingGuardSideEffects::none();
    let source_binding_guard_bound = status_canary_runner_dry_run_selector_source_binding_bound(
        binding_guard,
        source_side_effects_closed,
    );
    let runner_dry_run_selector_allowed = input.runner_dry_run_selector_request_present
        && source_binding_guard_bound
        && binding_guard.runner_binding_guard_allowed
        && binding_guard.source_runtime_boundaries_closed
        && source_side_effects_closed
        && side_effects == StatusCanaryRunnerDryRunSelectorSideEffects::none();

    StatusCanaryRunnerDryRunSelectorPlan {
        runtime: "hepta",
        selector: "status_canary_runner_dry_run_selector",
        schema_version: STATUS_CANARY_RUNNER_DRY_RUN_SELECTOR_SCHEMA_VERSION,
        selector_id: STATUS_CANARY_RUNNER_DRY_RUN_SELECTOR_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_binding_guard_id: binding_guard.guard_id,
        source_binding_guard_route: binding_guard.guard_route,
        source_binding_guard_bound,
        source_runner_binding_request_present: binding_guard.runner_binding_request_present,
        source_runner_binding_guard_blocked: binding_guard.runner_binding_guard_blocked,
        source_runner_binding_guard_allowed: binding_guard.runner_binding_guard_allowed,
        source_entry_adapter_bound: binding_guard.source_entry_adapter_bound,
        source_entry_adapter_allowed: binding_guard.source_runner_entry_adapter_allowed,
        source_start_request_gate_reason_audit_ready: binding_guard
            .source_start_request_gate_reason_audit_ready,
        source_requested_tool_id: binding_guard.source_requested_tool_id,
        source_requested_selected_status_canary: binding_guard
            .source_requested_selected_status_canary,
        source_requested_preflight_only_connector: binding_guard
            .source_requested_preflight_only_connector,
        source_runtime_boundaries_closed: binding_guard.source_runtime_boundaries_closed,
        source_side_effects_closed,
        runner_dry_run_selector_request_present: input.runner_dry_run_selector_request_present,
        runner_dry_run_selector_blocked: !runner_dry_run_selector_allowed,
        runner_dry_run_selector_allowed,
        selector_route: status_canary_runner_dry_run_selector_route(
            input.runner_dry_run_selector_request_present,
            source_binding_guard_bound,
            binding_guard.runner_binding_guard_allowed,
            source_side_effects_closed,
            binding_guard.source_runtime_boundaries_closed,
            runner_dry_run_selector_allowed,
        ),
        side_effects,
    }
}

fn status_canary_runner_dry_run_selector_source_binding_bound(
    binding_guard: &StatusCanaryRunnerBindingGuardPlan,
    source_side_effects_closed: bool,
) -> bool {
    binding_guard.guard_id == STATUS_CANARY_RUNNER_BINDING_GUARD_ID
        && binding_guard.selected_status_canary_tool_id == SELECTED_STATUS_CANARY_TOOL_ID
        && binding_guard.preflight_only_connector_tool_id == PREFLIGHT_ONLY_CONNECTOR_TOOL_ID
        && binding_guard.source_entry_adapter_bound
        && binding_guard.source_start_request_gate_reason_audit_ready
        && binding_guard.source_requested_tool_id == SELECTED_STATUS_CANARY_TOOL_ID
        && binding_guard.source_requested_selected_status_canary
        && !binding_guard.source_requested_preflight_only_connector
        && binding_guard.source_runtime_boundaries_closed
        && source_side_effects_closed
}

fn status_canary_runner_dry_run_selector_route(
    runner_dry_run_selector_request_present: bool,
    source_binding_guard_bound: bool,
    source_binding_guard_allowed: bool,
    source_side_effects_closed: bool,
    source_runtime_boundaries_closed: bool,
    runner_dry_run_selector_allowed: bool,
) -> &'static str {
    if !runner_dry_run_selector_request_present {
        "status_canary_runner_dry_run_selector_blocked_no_selector_request"
    } else if !source_side_effects_closed {
        "status_canary_runner_dry_run_selector_blocked_binding_guard_side_effects_open"
    } else if !source_runtime_boundaries_closed {
        "status_canary_runner_dry_run_selector_blocked_runtime_boundary_open"
    } else if !source_binding_guard_bound {
        "status_canary_runner_dry_run_selector_blocked_binding_guard_not_bound"
    } else if !source_binding_guard_allowed {
        "status_canary_runner_dry_run_selector_blocked_binding_guard"
    } else if runner_dry_run_selector_allowed {
        "status_canary_runner_dry_run_selector_would_select_dry_run"
    } else {
        "status_canary_runner_dry_run_selector_blocked_unknown"
    }
}

impl Default for StatusCanaryRunnerDryRunSelectorInput {
    fn default() -> Self {
        Self {
            runner_dry_run_selector_request_present: false,
        }
    }
}

impl StatusCanaryRunnerDryRunSelectorSideEffects {
    pub const fn none() -> Self {
        Self {
            runner_dry_run_selected: false,
            runner_dry_run_executed: false,
            dry_run_request_persisted: false,
            runner_bound: false,
            runner_entered: false,
            runner_started: false,
            runner_command_enqueued: false,
            evidence_recorded: false,
            evidence_persisted: false,
            waiver_recorded: false,
            approval_requested: false,
            approval_accepted: false,
            credential_read: false,
            transport_mutated: false,
            registry_mutated: false,
            tool_invoked: false,
            connector_started: false,
            ledger_written: false,
            receipt_written: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            canary_started: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controlled_canary_readiness_plan::controlled_canary_readiness_plan_report;
    use crate::status_canary_runner_adapter::StatusCanaryRunnerAdapterInput;
    use crate::status_canary_runner_adapter::status_canary_runner_adapter_plan_from_start_request_gate;
    use crate::status_canary_runner_binding_guard::StatusCanaryRunnerBindingGuardInput;
    use crate::status_canary_runner_binding_guard::status_canary_runner_binding_guard_plan_from_entry_adapter;
    use crate::status_canary_runner_entry_adapter::StatusCanaryRunnerEntryAdapterInput;
    use crate::status_canary_runner_entry_adapter::status_canary_runner_entry_adapter_plan_from_entry_boundary;
    use crate::status_canary_runner_entry_boundary::StatusCanaryRunnerEntryBoundaryInput;
    use crate::status_canary_runner_entry_boundary::status_canary_runner_entry_boundary_plan_from_start_surface;
    use crate::status_canary_runner_start_surface::StatusCanaryRunnerStartSurfaceInput;
    use crate::status_canary_runner_start_surface::status_canary_runner_start_surface_plan_from_runner_adapter;
    use crate::status_canary_start_request_gate::StatusCanaryStartRequestGateInput;
    use crate::status_canary_start_request_gate::status_canary_start_request_gate_from_readiness;

    fn allowed_binding_guard() -> StatusCanaryRunnerBindingGuardPlan {
        let mut readiness = controlled_canary_readiness_plan_report();
        readiness.status_canary_start_guard_route = "status_canary_start_guard_would_allow_start";
        readiness.status_canary_start_guard_switch_enabled = true;
        readiness.status_canary_start_guard_evidence_complete = true;
        readiness.status_canary_start_guard_missing_evidence_count = 0;
        readiness.status_canary_start_guard_blocked = false;
        readiness.status_canary_start_guard_allowed = true;

        let start_request_gate = status_canary_start_request_gate_from_readiness(
            &readiness,
            StatusCanaryStartRequestGateInput {
                start_request_present: true,
                requested_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
            },
        );
        let runner_adapter = status_canary_runner_adapter_plan_from_start_request_gate(
            &start_request_gate,
            StatusCanaryRunnerAdapterInput {
                runner_adapter_request_present: true,
            },
        );
        let start_surface = status_canary_runner_start_surface_plan_from_runner_adapter(
            &runner_adapter,
            StatusCanaryRunnerStartSurfaceInput {
                runner_start_request_present: true,
            },
        );
        let entry_boundary = status_canary_runner_entry_boundary_plan_from_start_surface(
            &start_surface,
            StatusCanaryRunnerEntryBoundaryInput {
                runner_entry_request_present: true,
            },
        );
        let entry_adapter = status_canary_runner_entry_adapter_plan_from_entry_boundary(
            &entry_boundary,
            StatusCanaryRunnerEntryAdapterInput {
                runner_entry_adapter_request_present: true,
            },
        );

        status_canary_runner_binding_guard_plan_from_entry_adapter(
            &entry_adapter,
            StatusCanaryRunnerBindingGuardInput {
                runner_binding_request_present: true,
            },
        )
    }

    #[test]
    fn default_selector_blocks_without_selector_request() {
        let plan = status_canary_runner_dry_run_selector_plan();

        assert_eq!(plan.selector_id, STATUS_CANARY_RUNNER_DRY_RUN_SELECTOR_ID);
        assert_eq!(
            plan.source_binding_guard_id,
            STATUS_CANARY_RUNNER_BINDING_GUARD_ID
        );
        assert!(plan.source_binding_guard_bound);
        assert!(plan.source_start_request_gate_reason_audit_ready);
        assert!(!plan.runner_dry_run_selector_request_present);
        assert!(!plan.source_runner_binding_request_present);
        assert!(plan.source_requested_selected_status_canary);
        assert!(!plan.source_requested_preflight_only_connector);
        assert!(plan.source_runner_binding_guard_blocked);
        assert!(!plan.source_runner_binding_guard_allowed);
        assert!(plan.runner_dry_run_selector_blocked);
        assert!(!plan.runner_dry_run_selector_allowed);
        assert_eq!(
            plan.selector_route,
            "status_canary_runner_dry_run_selector_blocked_no_selector_request"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerDryRunSelectorSideEffects::none()
        );
    }

    #[test]
    fn selector_request_is_blocked_by_binding_guard() {
        let binding_guard = status_canary_runner_binding_guard_plan();
        let plan = status_canary_runner_dry_run_selector_plan_from_binding_guard(
            &binding_guard,
            StatusCanaryRunnerDryRunSelectorInput {
                runner_dry_run_selector_request_present: true,
            },
        );

        assert!(plan.runner_dry_run_selector_request_present);
        assert!(plan.source_binding_guard_bound);
        assert!(plan.source_runner_binding_guard_blocked);
        assert!(!plan.source_runner_binding_guard_allowed);
        assert!(plan.runner_dry_run_selector_blocked);
        assert!(!plan.runner_dry_run_selector_allowed);
        assert_eq!(
            plan.selector_route,
            "status_canary_runner_dry_run_selector_blocked_binding_guard"
        );
        assert!(!plan.side_effects.runner_dry_run_selected);
        assert!(!plan.side_effects.runner_dry_run_executed);
        assert!(!plan.side_effects.canary_started);
    }

    #[test]
    fn binding_guard_reason_audit_gap_is_not_bound() {
        let mut binding_guard = allowed_binding_guard();
        binding_guard.source_start_request_gate_reason_audit_ready = false;
        let plan = status_canary_runner_dry_run_selector_plan_from_binding_guard(
            &binding_guard,
            StatusCanaryRunnerDryRunSelectorInput {
                runner_dry_run_selector_request_present: true,
            },
        );

        assert!(binding_guard.runner_binding_guard_allowed);
        assert!(!plan.source_start_request_gate_reason_audit_ready);
        assert!(!plan.source_binding_guard_bound);
        assert!(plan.runner_dry_run_selector_blocked);
        assert!(!plan.runner_dry_run_selector_allowed);
        assert_eq!(
            plan.selector_route,
            "status_canary_runner_dry_run_selector_blocked_binding_guard_not_bound"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerDryRunSelectorSideEffects::none()
        );
    }

    #[test]
    fn preflight_only_connector_binding_guard_source_is_not_bound() {
        let mut binding_guard = allowed_binding_guard();
        binding_guard.source_requested_tool_id = PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
        binding_guard.source_requested_selected_status_canary = false;
        binding_guard.source_requested_preflight_only_connector = true;

        let plan = status_canary_runner_dry_run_selector_plan_from_binding_guard(
            &binding_guard,
            StatusCanaryRunnerDryRunSelectorInput {
                runner_dry_run_selector_request_present: true,
            },
        );

        assert!(!plan.source_binding_guard_bound);
        assert!(plan.source_requested_preflight_only_connector);
        assert!(plan.runner_dry_run_selector_blocked);
        assert!(!plan.runner_dry_run_selector_allowed);
        assert_eq!(
            plan.selector_route,
            "status_canary_runner_dry_run_selector_blocked_binding_guard_not_bound"
        );
        assert!(!plan.side_effects.connector_started);
        assert!(!plan.side_effects.tool_invoked);
        assert!(!plan.side_effects.canary_started);
    }

    #[test]
    fn allowed_binding_guard_only_allows_dry_run_selector_plan_without_side_effects() {
        let binding_guard = allowed_binding_guard();
        let plan = status_canary_runner_dry_run_selector_plan_from_binding_guard(
            &binding_guard,
            StatusCanaryRunnerDryRunSelectorInput {
                runner_dry_run_selector_request_present: true,
            },
        );

        assert!(plan.runner_dry_run_selector_request_present);
        assert!(plan.source_binding_guard_bound);
        assert!(plan.source_runner_binding_request_present);
        assert!(plan.source_requested_selected_status_canary);
        assert!(!plan.source_requested_preflight_only_connector);
        assert!(plan.source_runner_binding_guard_allowed);
        assert!(!plan.source_runner_binding_guard_blocked);
        assert!(plan.runner_dry_run_selector_allowed);
        assert!(!plan.runner_dry_run_selector_blocked);
        assert_eq!(
            plan.selector_route,
            "status_canary_runner_dry_run_selector_would_select_dry_run"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerDryRunSelectorSideEffects::none()
        );
        assert!(!plan.side_effects.runner_dry_run_selected);
        assert!(!plan.side_effects.runner_dry_run_executed);
        assert!(!plan.side_effects.dry_run_request_persisted);
        assert!(!plan.side_effects.runner_bound);
        assert!(!plan.side_effects.runner_entered);
        assert!(!plan.side_effects.runner_started);
        assert!(!plan.side_effects.runner_command_enqueued);
        assert!(!plan.side_effects.registry_mutated);
        assert!(!plan.side_effects.tool_invoked);
        assert!(!plan.side_effects.connector_started);
        assert!(!plan.side_effects.ledger_written);
        assert!(!plan.side_effects.receipt_written);
        assert!(!plan.side_effects.canary_started);
        assert!(!plan.side_effects.live_execution_started);
    }
}
