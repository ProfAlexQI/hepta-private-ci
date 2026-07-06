use serde::Serialize;

use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;
use crate::status_canary_runner_adapter::STATUS_CANARY_RUNNER_ADAPTER_ID;
use crate::status_canary_runner_adapter::StatusCanaryRunnerAdapterPlan;
use crate::status_canary_runner_adapter::StatusCanaryRunnerAdapterSideEffects;
use crate::status_canary_runner_adapter::status_canary_runner_adapter_plan;

pub const STATUS_CANARY_RUNNER_START_SURFACE_SCHEMA_VERSION: &str =
    "status_canary_runner_start_surface_v1";
pub const STATUS_CANARY_RUNNER_START_SURFACE_ID: &str =
    "status-canary-runner-start-surface/hepta-system-status/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCanaryRunnerStartSurfaceInput {
    pub runner_start_request_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryRunnerStartSurfacePlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub schema_version: &'static str,
    pub surface_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_runner_adapter_id: &'static str,
    pub source_runner_adapter_route: &'static str,
    pub source_runner_adapter_bound: bool,
    pub source_runner_adapter_request_present: bool,
    pub source_runner_adapter_blocked: bool,
    pub source_runner_adapter_allowed: bool,
    pub source_start_request_gate_bound: bool,
    pub source_start_request_gate_reason_audit_ready: bool,
    pub source_start_request_present: bool,
    pub source_requested_tool_id: &'static str,
    pub source_requested_selected_status_canary: bool,
    pub source_requested_preflight_only_connector: bool,
    pub source_runtime_boundaries_closed: bool,
    pub source_side_effects_closed: bool,
    pub runner_start_request_present: bool,
    pub runner_start_surface_blocked: bool,
    pub runner_start_surface_allowed: bool,
    pub surface_route: &'static str,
    pub side_effects: StatusCanaryRunnerStartSurfaceSideEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryRunnerStartSurfaceSideEffects {
    pub runner_started: bool,
    pub runner_command_enqueued: bool,
    pub start_request_persisted: bool,
    pub adapter_plan_persisted: bool,
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

pub fn status_canary_runner_start_surface_plan() -> StatusCanaryRunnerStartSurfacePlan {
    let runner_adapter = status_canary_runner_adapter_plan();
    status_canary_runner_start_surface_plan_from_runner_adapter(
        &runner_adapter,
        StatusCanaryRunnerStartSurfaceInput::default(),
    )
}

pub fn status_canary_runner_start_surface_plan_from_runner_adapter(
    runner_adapter: &StatusCanaryRunnerAdapterPlan,
    input: StatusCanaryRunnerStartSurfaceInput,
) -> StatusCanaryRunnerStartSurfacePlan {
    let side_effects = StatusCanaryRunnerStartSurfaceSideEffects::none();
    let source_side_effects_closed =
        runner_adapter.side_effects == StatusCanaryRunnerAdapterSideEffects::none();
    let source_runner_adapter_bound = status_canary_runner_start_surface_source_adapter_bound(
        runner_adapter,
        source_side_effects_closed,
    );
    let runner_start_surface_allowed = input.runner_start_request_present
        && source_runner_adapter_bound
        && runner_adapter.runner_adapter_plan_allowed
        && source_side_effects_closed
        && runner_adapter.source_runtime_boundaries_closed
        && side_effects == StatusCanaryRunnerStartSurfaceSideEffects::none();

    StatusCanaryRunnerStartSurfacePlan {
        runtime: "hepta",
        surface: "status_canary_runner_start_surface",
        schema_version: STATUS_CANARY_RUNNER_START_SURFACE_SCHEMA_VERSION,
        surface_id: STATUS_CANARY_RUNNER_START_SURFACE_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_runner_adapter_id: runner_adapter.adapter_id,
        source_runner_adapter_route: runner_adapter.adapter_route,
        source_runner_adapter_bound,
        source_runner_adapter_request_present: runner_adapter.runner_adapter_request_present,
        source_runner_adapter_blocked: runner_adapter.runner_adapter_plan_blocked,
        source_runner_adapter_allowed: runner_adapter.runner_adapter_plan_allowed,
        source_start_request_gate_bound: runner_adapter.source_start_request_gate_bound,
        source_start_request_gate_reason_audit_ready: runner_adapter
            .source_start_request_gate_reason_audit_ready,
        source_start_request_present: runner_adapter.source_start_request_present,
        source_requested_tool_id: runner_adapter.source_requested_tool_id,
        source_requested_selected_status_canary: runner_adapter
            .source_requested_selected_status_canary,
        source_requested_preflight_only_connector: runner_adapter
            .source_requested_preflight_only_connector,
        source_runtime_boundaries_closed: runner_adapter.source_runtime_boundaries_closed,
        source_side_effects_closed,
        runner_start_request_present: input.runner_start_request_present,
        runner_start_surface_blocked: !runner_start_surface_allowed,
        runner_start_surface_allowed,
        surface_route: status_canary_runner_start_surface_route(
            input.runner_start_request_present,
            source_runner_adapter_bound,
            runner_adapter.runner_adapter_plan_allowed,
            source_side_effects_closed,
            runner_adapter.source_runtime_boundaries_closed,
            runner_start_surface_allowed,
        ),
        side_effects,
    }
}

fn status_canary_runner_start_surface_source_adapter_bound(
    runner_adapter: &StatusCanaryRunnerAdapterPlan,
    source_side_effects_closed: bool,
) -> bool {
    runner_adapter.adapter_id == STATUS_CANARY_RUNNER_ADAPTER_ID
        && runner_adapter.selected_status_canary_tool_id == SELECTED_STATUS_CANARY_TOOL_ID
        && runner_adapter.preflight_only_connector_tool_id == PREFLIGHT_ONLY_CONNECTOR_TOOL_ID
        && runner_adapter.source_start_request_gate_bound
        && runner_adapter.source_start_request_gate_reason_audit_ready
        && runner_adapter.source_requested_tool_id == SELECTED_STATUS_CANARY_TOOL_ID
        && runner_adapter.source_requested_selected_status_canary
        && !runner_adapter.source_requested_preflight_only_connector
        && runner_adapter.source_runtime_boundaries_closed
        && source_side_effects_closed
}

fn status_canary_runner_start_surface_route(
    runner_start_request_present: bool,
    source_runner_adapter_bound: bool,
    source_runner_adapter_allowed: bool,
    source_side_effects_closed: bool,
    source_runtime_boundaries_closed: bool,
    runner_start_surface_allowed: bool,
) -> &'static str {
    if !runner_start_request_present {
        "status_canary_runner_start_surface_blocked_no_start_request"
    } else if !source_side_effects_closed {
        "status_canary_runner_start_surface_blocked_runner_adapter_side_effects_open"
    } else if !source_runtime_boundaries_closed {
        "status_canary_runner_start_surface_blocked_runtime_boundary_open"
    } else if !source_runner_adapter_bound {
        "status_canary_runner_start_surface_blocked_runner_adapter_not_bound"
    } else if !source_runner_adapter_allowed {
        "status_canary_runner_start_surface_blocked_runner_adapter"
    } else if runner_start_surface_allowed {
        "status_canary_runner_start_surface_would_start_canary"
    } else {
        "status_canary_runner_start_surface_blocked_unknown"
    }
}

impl Default for StatusCanaryRunnerStartSurfaceInput {
    fn default() -> Self {
        Self {
            runner_start_request_present: false,
        }
    }
}

impl StatusCanaryRunnerStartSurfaceSideEffects {
    pub const fn none() -> Self {
        Self {
            runner_started: false,
            runner_command_enqueued: false,
            start_request_persisted: false,
            adapter_plan_persisted: false,
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
    use crate::status_canary_start_request_gate::StatusCanaryStartRequestGateInput;
    use crate::status_canary_start_request_gate::status_canary_start_request_gate;
    use crate::status_canary_start_request_gate::status_canary_start_request_gate_from_readiness;

    fn allowed_runner_adapter() -> StatusCanaryRunnerAdapterPlan {
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

        status_canary_runner_adapter_plan_from_start_request_gate(
            &start_request_gate,
            StatusCanaryRunnerAdapterInput {
                runner_adapter_request_present: true,
            },
        )
    }

    #[test]
    fn default_surface_blocks_without_runner_start_request() {
        let plan = status_canary_runner_start_surface_plan();

        assert_eq!(plan.surface_id, STATUS_CANARY_RUNNER_START_SURFACE_ID);
        assert_eq!(
            plan.source_runner_adapter_id,
            STATUS_CANARY_RUNNER_ADAPTER_ID
        );
        assert!(plan.source_runner_adapter_bound);
        assert!(plan.source_start_request_gate_reason_audit_ready);
        assert!(!plan.runner_start_request_present);
        assert!(!plan.source_start_request_present);
        assert!(plan.source_requested_selected_status_canary);
        assert!(!plan.source_requested_preflight_only_connector);
        assert!(plan.source_runner_adapter_blocked);
        assert!(!plan.source_runner_adapter_allowed);
        assert!(plan.runner_start_surface_blocked);
        assert!(!plan.runner_start_surface_allowed);
        assert_eq!(
            plan.surface_route,
            "status_canary_runner_start_surface_blocked_no_start_request"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerStartSurfaceSideEffects::none()
        );
    }

    #[test]
    fn runner_start_request_is_blocked_by_runner_adapter() {
        let runner_adapter = status_canary_runner_adapter_plan();
        let plan = status_canary_runner_start_surface_plan_from_runner_adapter(
            &runner_adapter,
            StatusCanaryRunnerStartSurfaceInput {
                runner_start_request_present: true,
            },
        );

        assert!(plan.runner_start_request_present);
        assert!(plan.source_runner_adapter_bound);
        assert!(plan.source_runner_adapter_blocked);
        assert!(!plan.source_runner_adapter_allowed);
        assert!(plan.runner_start_surface_blocked);
        assert!(!plan.runner_start_surface_allowed);
        assert_eq!(
            plan.surface_route,
            "status_canary_runner_start_surface_blocked_runner_adapter"
        );
        assert!(!plan.side_effects.runner_started);
        assert!(!plan.side_effects.canary_started);
    }

    #[test]
    fn runner_adapter_reason_audit_gap_is_not_bound() {
        let mut runner_adapter = allowed_runner_adapter();
        runner_adapter.source_start_request_gate_reason_audit_ready = false;
        let plan = status_canary_runner_start_surface_plan_from_runner_adapter(
            &runner_adapter,
            StatusCanaryRunnerStartSurfaceInput {
                runner_start_request_present: true,
            },
        );

        assert!(runner_adapter.runner_adapter_plan_allowed);
        assert!(!plan.source_start_request_gate_reason_audit_ready);
        assert!(!plan.source_runner_adapter_bound);
        assert!(plan.runner_start_surface_blocked);
        assert!(!plan.runner_start_surface_allowed);
        assert_eq!(
            plan.surface_route,
            "status_canary_runner_start_surface_blocked_runner_adapter_not_bound"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerStartSurfaceSideEffects::none()
        );
    }

    #[test]
    fn preflight_only_connector_adapter_source_is_not_bound() {
        let start_request_gate = status_canary_start_request_gate();
        let mut runner_adapter = status_canary_runner_adapter_plan_from_start_request_gate(
            &start_request_gate,
            StatusCanaryRunnerAdapterInput {
                runner_adapter_request_present: true,
            },
        );
        runner_adapter.source_requested_tool_id = PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
        runner_adapter.source_requested_selected_status_canary = false;
        runner_adapter.source_requested_preflight_only_connector = true;

        let plan = status_canary_runner_start_surface_plan_from_runner_adapter(
            &runner_adapter,
            StatusCanaryRunnerStartSurfaceInput {
                runner_start_request_present: true,
            },
        );

        assert!(!plan.source_runner_adapter_bound);
        assert!(plan.source_requested_preflight_only_connector);
        assert!(plan.runner_start_surface_blocked);
        assert!(!plan.runner_start_surface_allowed);
        assert_eq!(
            plan.surface_route,
            "status_canary_runner_start_surface_blocked_runner_adapter_not_bound"
        );
        assert!(!plan.side_effects.connector_started);
        assert!(!plan.side_effects.tool_invoked);
        assert!(!plan.side_effects.canary_started);
    }

    #[test]
    fn allowed_runner_adapter_only_allows_start_surface_plan_without_side_effects() {
        let runner_adapter = allowed_runner_adapter();
        let plan = status_canary_runner_start_surface_plan_from_runner_adapter(
            &runner_adapter,
            StatusCanaryRunnerStartSurfaceInput {
                runner_start_request_present: true,
            },
        );

        assert!(plan.runner_start_request_present);
        assert!(plan.source_runner_adapter_bound);
        assert!(plan.source_start_request_present);
        assert!(plan.source_requested_selected_status_canary);
        assert!(!plan.source_requested_preflight_only_connector);
        assert!(plan.source_runner_adapter_allowed);
        assert!(!plan.source_runner_adapter_blocked);
        assert!(plan.runner_start_surface_allowed);
        assert!(!plan.runner_start_surface_blocked);
        assert_eq!(
            plan.surface_route,
            "status_canary_runner_start_surface_would_start_canary"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerStartSurfaceSideEffects::none()
        );
        assert!(!plan.side_effects.runner_started);
        assert!(!plan.side_effects.runner_command_enqueued);
        assert!(!plan.side_effects.start_request_persisted);
        assert!(!plan.side_effects.adapter_plan_persisted);
        assert!(!plan.side_effects.registry_mutated);
        assert!(!plan.side_effects.tool_invoked);
        assert!(!plan.side_effects.connector_started);
        assert!(!plan.side_effects.ledger_written);
        assert!(!plan.side_effects.receipt_written);
        assert!(!plan.side_effects.canary_started);
        assert!(!plan.side_effects.live_execution_started);
    }
}
