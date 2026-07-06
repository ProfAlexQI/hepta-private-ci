use serde::Serialize;

use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;
use crate::status_canary_runner_entry_adapter::STATUS_CANARY_RUNNER_ENTRY_ADAPTER_ID;
use crate::status_canary_runner_entry_adapter::StatusCanaryRunnerEntryAdapterPlan;
use crate::status_canary_runner_entry_adapter::StatusCanaryRunnerEntryAdapterSideEffects;
use crate::status_canary_runner_entry_adapter::status_canary_runner_entry_adapter_plan;

pub const STATUS_CANARY_RUNNER_BINDING_GUARD_SCHEMA_VERSION: &str =
    "status_canary_runner_binding_guard_v1";
pub const STATUS_CANARY_RUNNER_BINDING_GUARD_ID: &str =
    "status-canary-runner-binding-guard/hepta-system-status/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCanaryRunnerBindingGuardInput {
    pub runner_binding_request_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryRunnerBindingGuardPlan {
    pub runtime: &'static str,
    pub guard: &'static str,
    pub schema_version: &'static str,
    pub guard_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_entry_adapter_id: &'static str,
    pub source_entry_adapter_route: &'static str,
    pub source_entry_adapter_bound: bool,
    pub source_runner_entry_adapter_request_present: bool,
    pub source_runner_entry_adapter_blocked: bool,
    pub source_runner_entry_adapter_allowed: bool,
    pub source_entry_boundary_bound: bool,
    pub source_entry_boundary_allowed: bool,
    pub source_start_surface_bound: bool,
    pub source_start_surface_allowed: bool,
    pub source_start_request_gate_reason_audit_ready: bool,
    pub source_requested_tool_id: &'static str,
    pub source_requested_selected_status_canary: bool,
    pub source_requested_preflight_only_connector: bool,
    pub source_runtime_boundaries_closed: bool,
    pub source_side_effects_closed: bool,
    pub runner_binding_request_present: bool,
    pub runner_binding_guard_blocked: bool,
    pub runner_binding_guard_allowed: bool,
    pub guard_route: &'static str,
    pub side_effects: StatusCanaryRunnerBindingGuardSideEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryRunnerBindingGuardSideEffects {
    pub runner_bound: bool,
    pub runner_entered: bool,
    pub runner_started: bool,
    pub runner_command_enqueued: bool,
    pub runner_binding_persisted: bool,
    pub binding_request_persisted: bool,
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

pub fn status_canary_runner_binding_guard_plan() -> StatusCanaryRunnerBindingGuardPlan {
    let entry_adapter = status_canary_runner_entry_adapter_plan();
    status_canary_runner_binding_guard_plan_from_entry_adapter(
        &entry_adapter,
        StatusCanaryRunnerBindingGuardInput::default(),
    )
}

pub fn status_canary_runner_binding_guard_plan_from_entry_adapter(
    entry_adapter: &StatusCanaryRunnerEntryAdapterPlan,
    input: StatusCanaryRunnerBindingGuardInput,
) -> StatusCanaryRunnerBindingGuardPlan {
    let side_effects = StatusCanaryRunnerBindingGuardSideEffects::none();
    let source_side_effects_closed =
        entry_adapter.side_effects == StatusCanaryRunnerEntryAdapterSideEffects::none();
    let source_entry_adapter_bound = status_canary_runner_binding_guard_source_entry_adapter_bound(
        entry_adapter,
        source_side_effects_closed,
    );
    let runner_binding_guard_allowed = input.runner_binding_request_present
        && source_entry_adapter_bound
        && entry_adapter.runner_entry_adapter_plan_allowed
        && source_side_effects_closed
        && entry_adapter.source_runtime_boundaries_closed
        && side_effects == StatusCanaryRunnerBindingGuardSideEffects::none();

    StatusCanaryRunnerBindingGuardPlan {
        runtime: "hepta",
        guard: "status_canary_runner_binding_guard",
        schema_version: STATUS_CANARY_RUNNER_BINDING_GUARD_SCHEMA_VERSION,
        guard_id: STATUS_CANARY_RUNNER_BINDING_GUARD_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_entry_adapter_id: entry_adapter.adapter_id,
        source_entry_adapter_route: entry_adapter.adapter_route,
        source_entry_adapter_bound,
        source_runner_entry_adapter_request_present: entry_adapter
            .runner_entry_adapter_request_present,
        source_runner_entry_adapter_blocked: entry_adapter.runner_entry_adapter_plan_blocked,
        source_runner_entry_adapter_allowed: entry_adapter.runner_entry_adapter_plan_allowed,
        source_entry_boundary_bound: entry_adapter.source_entry_boundary_bound,
        source_entry_boundary_allowed: entry_adapter.source_runner_entry_boundary_allowed,
        source_start_surface_bound: entry_adapter.source_start_surface_bound,
        source_start_surface_allowed: entry_adapter.source_start_surface_allowed,
        source_start_request_gate_reason_audit_ready: entry_adapter
            .source_start_request_gate_reason_audit_ready,
        source_requested_tool_id: entry_adapter.source_requested_tool_id,
        source_requested_selected_status_canary: entry_adapter
            .source_requested_selected_status_canary,
        source_requested_preflight_only_connector: entry_adapter
            .source_requested_preflight_only_connector,
        source_runtime_boundaries_closed: entry_adapter.source_runtime_boundaries_closed,
        source_side_effects_closed,
        runner_binding_request_present: input.runner_binding_request_present,
        runner_binding_guard_blocked: !runner_binding_guard_allowed,
        runner_binding_guard_allowed,
        guard_route: status_canary_runner_binding_guard_route(
            input.runner_binding_request_present,
            source_entry_adapter_bound,
            entry_adapter.runner_entry_adapter_plan_allowed,
            source_side_effects_closed,
            entry_adapter.source_runtime_boundaries_closed,
            runner_binding_guard_allowed,
        ),
        side_effects,
    }
}

fn status_canary_runner_binding_guard_source_entry_adapter_bound(
    entry_adapter: &StatusCanaryRunnerEntryAdapterPlan,
    source_side_effects_closed: bool,
) -> bool {
    entry_adapter.adapter_id == STATUS_CANARY_RUNNER_ENTRY_ADAPTER_ID
        && entry_adapter.selected_status_canary_tool_id == SELECTED_STATUS_CANARY_TOOL_ID
        && entry_adapter.preflight_only_connector_tool_id == PREFLIGHT_ONLY_CONNECTOR_TOOL_ID
        && entry_adapter.source_entry_boundary_bound
        && entry_adapter.source_start_surface_bound
        && entry_adapter.source_start_request_gate_reason_audit_ready
        && entry_adapter.source_requested_tool_id == SELECTED_STATUS_CANARY_TOOL_ID
        && entry_adapter.source_requested_selected_status_canary
        && !entry_adapter.source_requested_preflight_only_connector
        && entry_adapter.source_runtime_boundaries_closed
        && source_side_effects_closed
}

fn status_canary_runner_binding_guard_route(
    runner_binding_request_present: bool,
    source_entry_adapter_bound: bool,
    source_entry_adapter_allowed: bool,
    source_side_effects_closed: bool,
    source_runtime_boundaries_closed: bool,
    runner_binding_guard_allowed: bool,
) -> &'static str {
    if !runner_binding_request_present {
        "status_canary_runner_binding_guard_blocked_no_binding_request"
    } else if !source_side_effects_closed {
        "status_canary_runner_binding_guard_blocked_entry_adapter_side_effects_open"
    } else if !source_runtime_boundaries_closed {
        "status_canary_runner_binding_guard_blocked_runtime_boundary_open"
    } else if !source_entry_adapter_bound {
        "status_canary_runner_binding_guard_blocked_entry_adapter_not_bound"
    } else if !source_entry_adapter_allowed {
        "status_canary_runner_binding_guard_blocked_entry_adapter"
    } else if runner_binding_guard_allowed {
        "status_canary_runner_binding_guard_would_bind_runner"
    } else {
        "status_canary_runner_binding_guard_blocked_unknown"
    }
}

impl Default for StatusCanaryRunnerBindingGuardInput {
    fn default() -> Self {
        Self {
            runner_binding_request_present: false,
        }
    }
}

impl StatusCanaryRunnerBindingGuardSideEffects {
    pub const fn none() -> Self {
        Self {
            runner_bound: false,
            runner_entered: false,
            runner_started: false,
            runner_command_enqueued: false,
            runner_binding_persisted: false,
            binding_request_persisted: false,
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
    use crate::status_canary_runner_entry_adapter::StatusCanaryRunnerEntryAdapterInput;
    use crate::status_canary_runner_entry_adapter::status_canary_runner_entry_adapter_plan_from_entry_boundary;
    use crate::status_canary_runner_entry_boundary::StatusCanaryRunnerEntryBoundaryInput;
    use crate::status_canary_runner_entry_boundary::status_canary_runner_entry_boundary_plan_from_start_surface;
    use crate::status_canary_runner_start_surface::StatusCanaryRunnerStartSurfaceInput;
    use crate::status_canary_runner_start_surface::status_canary_runner_start_surface_plan_from_runner_adapter;
    use crate::status_canary_start_request_gate::StatusCanaryStartRequestGateInput;
    use crate::status_canary_start_request_gate::status_canary_start_request_gate_from_readiness;

    fn allowed_entry_adapter() -> StatusCanaryRunnerEntryAdapterPlan {
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

        status_canary_runner_entry_adapter_plan_from_entry_boundary(
            &entry_boundary,
            StatusCanaryRunnerEntryAdapterInput {
                runner_entry_adapter_request_present: true,
            },
        )
    }

    #[test]
    fn default_guard_blocks_without_binding_request() {
        let plan = status_canary_runner_binding_guard_plan();

        assert_eq!(plan.guard_id, STATUS_CANARY_RUNNER_BINDING_GUARD_ID);
        assert_eq!(
            plan.source_entry_adapter_id,
            STATUS_CANARY_RUNNER_ENTRY_ADAPTER_ID
        );
        assert!(plan.source_entry_adapter_bound);
        assert!(plan.source_start_request_gate_reason_audit_ready);
        assert!(!plan.runner_binding_request_present);
        assert!(!plan.source_runner_entry_adapter_request_present);
        assert!(plan.source_requested_selected_status_canary);
        assert!(!plan.source_requested_preflight_only_connector);
        assert!(plan.source_runner_entry_adapter_blocked);
        assert!(!plan.source_runner_entry_adapter_allowed);
        assert!(plan.runner_binding_guard_blocked);
        assert!(!plan.runner_binding_guard_allowed);
        assert_eq!(
            plan.guard_route,
            "status_canary_runner_binding_guard_blocked_no_binding_request"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerBindingGuardSideEffects::none()
        );
    }

    #[test]
    fn binding_request_is_blocked_by_entry_adapter() {
        let entry_adapter = status_canary_runner_entry_adapter_plan();
        let plan = status_canary_runner_binding_guard_plan_from_entry_adapter(
            &entry_adapter,
            StatusCanaryRunnerBindingGuardInput {
                runner_binding_request_present: true,
            },
        );

        assert!(plan.runner_binding_request_present);
        assert!(plan.source_entry_adapter_bound);
        assert!(plan.source_runner_entry_adapter_blocked);
        assert!(!plan.source_runner_entry_adapter_allowed);
        assert!(plan.runner_binding_guard_blocked);
        assert!(!plan.runner_binding_guard_allowed);
        assert_eq!(
            plan.guard_route,
            "status_canary_runner_binding_guard_blocked_entry_adapter"
        );
        assert!(!plan.side_effects.runner_bound);
        assert!(!plan.side_effects.runner_started);
        assert!(!plan.side_effects.canary_started);
    }

    #[test]
    fn entry_adapter_reason_audit_gap_is_not_bound() {
        let mut entry_adapter = allowed_entry_adapter();
        entry_adapter.source_start_request_gate_reason_audit_ready = false;
        let plan = status_canary_runner_binding_guard_plan_from_entry_adapter(
            &entry_adapter,
            StatusCanaryRunnerBindingGuardInput {
                runner_binding_request_present: true,
            },
        );

        assert!(entry_adapter.runner_entry_adapter_plan_allowed);
        assert!(!plan.source_start_request_gate_reason_audit_ready);
        assert!(!plan.source_entry_adapter_bound);
        assert!(plan.runner_binding_guard_blocked);
        assert!(!plan.runner_binding_guard_allowed);
        assert_eq!(
            plan.guard_route,
            "status_canary_runner_binding_guard_blocked_entry_adapter_not_bound"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerBindingGuardSideEffects::none()
        );
    }

    #[test]
    fn preflight_only_connector_entry_adapter_source_is_not_bound() {
        let mut entry_adapter = allowed_entry_adapter();
        entry_adapter.source_requested_tool_id = PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
        entry_adapter.source_requested_selected_status_canary = false;
        entry_adapter.source_requested_preflight_only_connector = true;

        let plan = status_canary_runner_binding_guard_plan_from_entry_adapter(
            &entry_adapter,
            StatusCanaryRunnerBindingGuardInput {
                runner_binding_request_present: true,
            },
        );

        assert!(!plan.source_entry_adapter_bound);
        assert!(plan.source_requested_preflight_only_connector);
        assert!(plan.runner_binding_guard_blocked);
        assert!(!plan.runner_binding_guard_allowed);
        assert_eq!(
            plan.guard_route,
            "status_canary_runner_binding_guard_blocked_entry_adapter_not_bound"
        );
        assert!(!plan.side_effects.connector_started);
        assert!(!plan.side_effects.tool_invoked);
        assert!(!plan.side_effects.canary_started);
    }

    #[test]
    fn allowed_entry_adapter_only_allows_binding_guard_plan_without_side_effects() {
        let entry_adapter = allowed_entry_adapter();
        let plan = status_canary_runner_binding_guard_plan_from_entry_adapter(
            &entry_adapter,
            StatusCanaryRunnerBindingGuardInput {
                runner_binding_request_present: true,
            },
        );

        assert!(plan.runner_binding_request_present);
        assert!(plan.source_entry_adapter_bound);
        assert!(plan.source_runner_entry_adapter_request_present);
        assert!(plan.source_requested_selected_status_canary);
        assert!(!plan.source_requested_preflight_only_connector);
        assert!(plan.source_runner_entry_adapter_allowed);
        assert!(!plan.source_runner_entry_adapter_blocked);
        assert!(plan.runner_binding_guard_allowed);
        assert!(!plan.runner_binding_guard_blocked);
        assert_eq!(
            plan.guard_route,
            "status_canary_runner_binding_guard_would_bind_runner"
        );
        assert_eq!(
            plan.side_effects,
            StatusCanaryRunnerBindingGuardSideEffects::none()
        );
        assert!(!plan.side_effects.runner_bound);
        assert!(!plan.side_effects.runner_entered);
        assert!(!plan.side_effects.runner_started);
        assert!(!plan.side_effects.runner_command_enqueued);
        assert!(!plan.side_effects.runner_binding_persisted);
        assert!(!plan.side_effects.binding_request_persisted);
        assert!(!plan.side_effects.registry_mutated);
        assert!(!plan.side_effects.tool_invoked);
        assert!(!plan.side_effects.connector_started);
        assert!(!plan.side_effects.ledger_written);
        assert!(!plan.side_effects.receipt_written);
        assert!(!plan.side_effects.canary_started);
        assert!(!plan.side_effects.live_execution_started);
    }
}
