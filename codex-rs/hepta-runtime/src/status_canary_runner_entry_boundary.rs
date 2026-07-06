use serde::Serialize;

use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;
use crate::status_canary_runner_start_surface::STATUS_CANARY_RUNNER_START_SURFACE_ID;
use crate::status_canary_runner_start_surface::StatusCanaryRunnerStartSurfacePlan;
use crate::status_canary_runner_start_surface::StatusCanaryRunnerStartSurfaceSideEffects;
use crate::status_canary_runner_start_surface::status_canary_runner_start_surface_plan;

pub const STATUS_CANARY_RUNNER_ENTRY_BOUNDARY_SCHEMA_VERSION: &str =
    "status_canary_runner_entry_boundary_v1";
pub const STATUS_CANARY_RUNNER_ENTRY_BOUNDARY_ID: &str =
    "status-canary-runner-entry-boundary/hepta-system-status/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCanaryRunnerEntryBoundaryInput {
    pub runner_entry_request_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryRunnerEntryBoundaryPlan {
    pub runtime: &'static str,
    pub boundary: &'static str,
    pub schema_version: &'static str,
    pub boundary_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_start_surface_id: &'static str,
    pub source_start_surface_route: &'static str,
    pub source_start_surface_bound: bool,
    pub source_runner_start_request_present: bool,
    pub source_runner_start_surface_blocked: bool,
    pub source_runner_start_surface_allowed: bool,
    pub source_runner_adapter_bound: bool,
    pub source_runner_adapter_allowed: bool,
    pub source_start_request_gate_bound: bool,
    pub source_start_request_gate_reason_audit_ready: bool,
    pub source_requested_tool_id: &'static str,
    pub source_requested_selected_status_canary: bool,
    pub source_requested_preflight_only_connector: bool,
    pub source_runtime_boundaries_closed: bool,
    pub source_side_effects_closed: bool,
    pub runner_entry_request_present: bool,
    pub runner_entry_boundary_blocked: bool,
    pub runner_entry_boundary_allowed: bool,
    pub boundary_route: &'static str,
    pub side_effects: StatusCanaryRunnerEntryBoundarySideEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryRunnerEntryBoundarySideEffects {
    pub runner_entered: bool,
    pub runner_started: bool,
    pub runner_command_enqueued: bool,
    pub runner_start_surface_persisted: bool,
    pub entry_request_persisted: bool,
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

pub fn status_canary_runner_entry_boundary_plan() -> StatusCanaryRunnerEntryBoundaryPlan {
    let start_surface = status_canary_runner_start_surface_plan();
    status_canary_runner_entry_boundary_plan_from_start_surface(
        &start_surface,
        StatusCanaryRunnerEntryBoundaryInput::default(),
    )
}

pub fn status_canary_runner_entry_boundary_plan_from_start_surface(
    start_surface: &StatusCanaryRunnerStartSurfacePlan,
    input: StatusCanaryRunnerEntryBoundaryInput,
) -> StatusCanaryRunnerEntryBoundaryPlan {
    let side_effects = StatusCanaryRunnerEntryBoundarySideEffects::none();
    let source_side_effects_closed =
        start_surface.side_effects == StatusCanaryRunnerStartSurfaceSideEffects::none();
    let source_start_surface_bound = status_canary_runner_entry_boundary_source_start_surface_bound(
        start_surface,
        source_side_effects_closed,
    );
    let runner_entry_boundary_allowed = input.runner_entry_request_present
        && source_start_surface_bound
        && start_surface.runner_start_surface_allowed
        && source_side_effects_closed
        && start_surface.source_runtime_boundaries_closed
        && side_effects == StatusCanaryRunnerEntryBoundarySideEffects::none();

    StatusCanaryRunnerEntryBoundaryPlan {
        runtime: "hepta",
        boundary: "status_canary_runner_entry_boundary",
        schema_version: STATUS_CANARY_RUNNER_ENTRY_BOUNDARY_SCHEMA_VERSION,
        boundary_id: STATUS_CANARY_RUNNER_ENTRY_BOUNDARY_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_start_surface_id: start_surface.surface_id,
        source_start_surface_route: start_surface.surface_route,
        source_start_surface_bound,
        source_runner_start_request_present: start_surface.runner_start_request_present,
        source_runner_start_surface_blocked: start_surface.runner_start_surface_blocked,
        source_runner_start_surface_allowed: start_surface.runner_start_surface_allowed,
        source_runner_adapter_bound: start_surface.source_runner_adapter_bound,
        source_runner_adapter_allowed: start_surface.source_runner_adapter_allowed,
        source_start_request_gate_bound: start_surface.source_start_request_gate_bound,
        source_start_request_gate_reason_audit_ready: start_surface
            .source_start_request_gate_reason_audit_ready,
        source_requested_tool_id: start_surface.source_requested_tool_id,
        source_requested_selected_status_canary: start_surface
            .source_requested_selected_status_canary,
        source_requested_preflight_only_connector: start_surface
            .source_requested_preflight_only_connector,
        source_runtime_boundaries_closed: start_surface.source_runtime_boundaries_closed,
        source_side_effects_closed,
        runner_entry_request_present: input.runner_entry_request_present,
        runner_entry_boundary_blocked: !runner_entry_boundary_allowed,
        runner_entry_boundary_allowed,
        boundary_route: status_canary_runner_entry_boundary_route(
            input.runner_entry_request_present,
            source_start_surface_bound,
            start_surface.runner_start_surface_allowed,
            source_side_effects_closed,
            start_surface.source_runtime_boundaries_closed,
            runner_entry_boundary_allowed,
        ),
        side_effects,
    }
}

fn status_canary_runner_entry_boundary_source_start_surface_bound(
    start_surface: &StatusCanaryRunnerStartSurfacePlan,
    source_side_effects_closed: bool,
) -> bool {
    start_surface.surface_id == STATUS_CANARY_RUNNER_START_SURFACE_ID
        && start_surface.selected_status_canary_tool_id == SELECTED_STATUS_CANARY_TOOL_ID
        && start_surface.preflight_only_connector_tool_id == PREFLIGHT_ONLY_CONNECTOR_TOOL_ID
        && start_surface.source_runner_adapter_bound
        && start_surface.source_start_request_gate_bound
        && start_surface.source_start_request_gate_reason_audit_ready
        && start_surface.source_requested_tool_id == SELECTED_STATUS_CANARY_TOOL_ID
        && start_surface.source_requested_selected_status_canary
        && !start_surface.source_requested_preflight_only_connector
        && start_surface.source_runtime_boundaries_closed
        && source_side_effects_closed
}

fn status_canary_runner_entry_boundary_route(
    runner_entry_request_present: bool,
    source_start_surface_bound: bool,
    source_start_surface_allowed: bool,
    source_side_effects_closed: bool,
    source_runtime_boundaries_closed: bool,
    runner_entry_boundary_allowed: bool,
) -> &'static str {
    if !runner_entry_request_present {
        "status_canary_runner_entry_boundary_blocked_no_entry_request"
    } else if !source_side_effects_closed {
        "status_canary_runner_entry_boundary_blocked_start_surface_side_effects_open"
    } else if !source_runtime_boundaries_closed {
        "status_canary_runner_entry_boundary_blocked_runtime_boundary_open"
    } else if !source_start_surface_bound {
        "status_canary_runner_entry_boundary_blocked_start_surface_not_bound"
    } else if !source_start_surface_allowed {
        "status_canary_runner_entry_boundary_blocked_start_surface"
    } else if runner_entry_boundary_allowed {
        "status_canary_runner_entry_boundary_would_enter_runner"
    } else {
        "status_canary_runner_entry_boundary_blocked_unknown"
    }
}

impl Default for StatusCanaryRunnerEntryBoundaryInput {
    fn default() -> Self {
        Self {
            runner_entry_request_present: false,
        }
    }
}

impl StatusCanaryRunnerEntryBoundarySideEffects {
    pub const fn none() -> Self {
        Self {
            runner_entered: false,
            runner_started: false,
            runner_command_enqueued: false,
            runner_start_surface_persisted: false,
            entry_request_persisted: false,
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
    use crate::status_canary_runner_start_surface::StatusCanaryRunnerStartSurfaceInput;
    use crate::status_canary_runner_start_surface::status_canary_runner_start_surface_plan_from_runner_adapter;
    use crate::status_canary_start_request_gate::StatusCanaryStartRequestGateInput;
    use crate::status_canary_start_request_gate::status_canary_start_request_gate;
    use crate::status_canary_start_request_gate::status_canary_start_request_gate_from_readiness;

    fn allowed_start_surface() -> StatusCanaryRunnerStartSurfacePlan {
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

        status_canary_runner_start_surface_plan_from_runner_adapter(
            &runner_adapter,
            StatusCanaryRunnerStartSurfaceInput {
                runner_start_request_present: true,
            },
        )
    }

    #[test]
    fn default_boundary_blocks_without_runner_entry_request() {
        let plan = status_canary_runner_entry_boundary_plan();

        assert_eq!(plan.boundary_id, STATUS_CANARY_RUNNER_ENTRY_BOUNDARY_ID);
        assert_eq!(
            plan.source_start_surface_id,
            STATUS_CANARY_RUNNER_START_SURFACE_ID
        );
        assert!(plan.source_start_surface_bound);
        assert!(plan.source_start_request_gate_reason_audit_ready);
        assert!(!plan.runner_entry_request_present);
        assert!(!plan.source_runner_start_request_present);
        assert!(plan.source_requested_selected_status_canary);
        assert!(!plan.source_requested_preflight_only_connector);
        assert!(plan.source_runner_start_surface_blocked);
        assert!(!plan.source_runner_start_surface_allowed);
        assert!(plan.runner_entry_boundary_blocked);
        assert!(!plan.runner_entry_boundary_allowed);
        assert_eq!(
            plan.boundary_route,
            "status_canary_runner_entry_boundary_blocked_no_entry_request"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerEntryBoundarySideEffects::none()
        );
    }

    #[test]
    fn runner_entry_request_is_blocked_by_start_surface() {
        let start_surface = status_canary_runner_start_surface_plan();
        let plan = status_canary_runner_entry_boundary_plan_from_start_surface(
            &start_surface,
            StatusCanaryRunnerEntryBoundaryInput {
                runner_entry_request_present: true,
            },
        );

        assert!(plan.runner_entry_request_present);
        assert!(plan.source_start_surface_bound);
        assert!(plan.source_runner_start_surface_blocked);
        assert!(!plan.source_runner_start_surface_allowed);
        assert!(plan.runner_entry_boundary_blocked);
        assert!(!plan.runner_entry_boundary_allowed);
        assert_eq!(
            plan.boundary_route,
            "status_canary_runner_entry_boundary_blocked_start_surface"
        );
        assert!(!plan.side_effects.runner_entered);
        assert!(!plan.side_effects.runner_started);
        assert!(!plan.side_effects.canary_started);
    }

    #[test]
    fn start_surface_reason_audit_gap_is_not_bound() {
        let mut start_surface = allowed_start_surface();
        start_surface.source_start_request_gate_reason_audit_ready = false;
        let plan = status_canary_runner_entry_boundary_plan_from_start_surface(
            &start_surface,
            StatusCanaryRunnerEntryBoundaryInput {
                runner_entry_request_present: true,
            },
        );

        assert!(start_surface.runner_start_surface_allowed);
        assert!(!plan.source_start_request_gate_reason_audit_ready);
        assert!(!plan.source_start_surface_bound);
        assert!(plan.runner_entry_boundary_blocked);
        assert!(!plan.runner_entry_boundary_allowed);
        assert_eq!(
            plan.boundary_route,
            "status_canary_runner_entry_boundary_blocked_start_surface_not_bound"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerEntryBoundarySideEffects::none()
        );
    }

    #[test]
    fn preflight_only_connector_start_surface_source_is_not_bound() {
        let start_request_gate = status_canary_start_request_gate();
        let runner_adapter = status_canary_runner_adapter_plan_from_start_request_gate(
            &start_request_gate,
            StatusCanaryRunnerAdapterInput {
                runner_adapter_request_present: true,
            },
        );
        let mut start_surface = status_canary_runner_start_surface_plan_from_runner_adapter(
            &runner_adapter,
            StatusCanaryRunnerStartSurfaceInput {
                runner_start_request_present: true,
            },
        );
        start_surface.source_requested_tool_id = PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
        start_surface.source_requested_selected_status_canary = false;
        start_surface.source_requested_preflight_only_connector = true;

        let plan = status_canary_runner_entry_boundary_plan_from_start_surface(
            &start_surface,
            StatusCanaryRunnerEntryBoundaryInput {
                runner_entry_request_present: true,
            },
        );

        assert!(!plan.source_start_surface_bound);
        assert!(plan.source_requested_preflight_only_connector);
        assert!(plan.runner_entry_boundary_blocked);
        assert!(!plan.runner_entry_boundary_allowed);
        assert_eq!(
            plan.boundary_route,
            "status_canary_runner_entry_boundary_blocked_start_surface_not_bound"
        );
        assert!(!plan.side_effects.connector_started);
        assert!(!plan.side_effects.tool_invoked);
        assert!(!plan.side_effects.canary_started);
    }

    #[test]
    fn allowed_start_surface_only_allows_entry_boundary_plan_without_side_effects() {
        let start_surface = allowed_start_surface();
        let plan = status_canary_runner_entry_boundary_plan_from_start_surface(
            &start_surface,
            StatusCanaryRunnerEntryBoundaryInput {
                runner_entry_request_present: true,
            },
        );

        assert!(plan.runner_entry_request_present);
        assert!(plan.source_start_surface_bound);
        assert!(plan.source_runner_start_request_present);
        assert!(plan.source_requested_selected_status_canary);
        assert!(!plan.source_requested_preflight_only_connector);
        assert!(plan.source_runner_start_surface_allowed);
        assert!(!plan.source_runner_start_surface_blocked);
        assert!(plan.runner_entry_boundary_allowed);
        assert!(!plan.runner_entry_boundary_blocked);
        assert_eq!(
            plan.boundary_route,
            "status_canary_runner_entry_boundary_would_enter_runner"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerEntryBoundarySideEffects::none()
        );
        assert!(!plan.side_effects.runner_entered);
        assert!(!plan.side_effects.runner_started);
        assert!(!plan.side_effects.runner_command_enqueued);
        assert!(!plan.side_effects.entry_request_persisted);
        assert!(!plan.side_effects.runner_start_surface_persisted);
        assert!(!plan.side_effects.registry_mutated);
        assert!(!plan.side_effects.tool_invoked);
        assert!(!plan.side_effects.connector_started);
        assert!(!plan.side_effects.ledger_written);
        assert!(!plan.side_effects.receipt_written);
        assert!(!plan.side_effects.canary_started);
        assert!(!plan.side_effects.live_execution_started);
    }
}
