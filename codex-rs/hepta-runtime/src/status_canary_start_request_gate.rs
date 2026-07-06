use serde::Serialize;

use crate::controlled_canary_readiness_plan::ControlledCanaryReadinessPlanReport;
use crate::controlled_canary_readiness_plan::ControlledCanaryReadinessPlanSideEffects;
use crate::controlled_canary_readiness_plan::controlled_canary_readiness_plan_report;
use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;

pub const STATUS_CANARY_START_REQUEST_GATE_SCHEMA_VERSION: &str =
    "status_canary_start_request_gate_v1";
pub const STATUS_CANARY_START_REQUEST_GATE_ID: &str =
    "status-canary-start-request-gate/hepta-system-status/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCanaryStartRequestGateInput {
    pub start_request_present: bool,
    pub requested_tool_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryStartRequestGatePlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub schema_version: &'static str,
    pub gate_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_controlled_canary_gate: &'static str,
    pub source_controlled_canary_ready: bool,
    pub source_controlled_canary_activation_ready: bool,
    pub source_status_canary_start_guard_bound: bool,
    pub source_status_canary_start_guard_id: &'static str,
    pub source_status_canary_start_guard_route: &'static str,
    pub source_status_canary_start_guard_switch_enabled: bool,
    pub source_status_canary_start_guard_evidence_complete: bool,
    pub source_status_canary_start_guard_missing_evidence_count: usize,
    pub source_status_canary_start_guard_reason_audit_count: usize,
    pub source_status_canary_start_guard_reason_audit_ready_count: usize,
    pub source_status_canary_start_guard_reason_audit_rejected_count: usize,
    pub source_status_canary_start_guard_reason_audit_ready: bool,
    pub source_status_canary_start_guard_blocked: bool,
    pub source_status_canary_start_guard_allowed: bool,
    pub source_controlled_canary_side_effects_closed: bool,
    pub source_runtime_boundaries_closed: bool,
    pub start_request_present: bool,
    pub requested_tool_id: &'static str,
    pub requested_selected_status_canary: bool,
    pub requested_preflight_only_connector: bool,
    pub start_request_allowed: bool,
    pub start_request_blocked: bool,
    pub gate_route: &'static str,
    pub side_effects: StatusCanaryStartRequestGateSideEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryStartRequestGateSideEffects {
    pub start_request_persisted: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub waiver_recorded: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub credential_read: bool,
    pub transport_mutated: bool,
    pub ledger_written: bool,
    pub receipt_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub registry_mutated: bool,
    pub tool_invoked: bool,
    pub connector_started: bool,
    pub canary_started: bool,
    pub live_execution_started: bool,
}

pub fn status_canary_start_request_gate() -> StatusCanaryStartRequestGatePlan {
    let readiness = controlled_canary_readiness_plan_report();
    status_canary_start_request_gate_from_readiness(
        &readiness,
        StatusCanaryStartRequestGateInput::default(),
    )
}

pub fn status_canary_start_request_gate_from_readiness(
    readiness: &ControlledCanaryReadinessPlanReport,
    input: StatusCanaryStartRequestGateInput,
) -> StatusCanaryStartRequestGatePlan {
    let side_effects = StatusCanaryStartRequestGateSideEffects::none();
    let source_controlled_canary_side_effects_closed =
        readiness.side_effects == ControlledCanaryReadinessPlanSideEffects::none();
    let source_runtime_boundaries_closed = controlled_canary_runtime_boundaries_closed(readiness);
    let requested_selected_status_canary =
        input.requested_tool_id == SELECTED_STATUS_CANARY_TOOL_ID;
    let requested_preflight_only_connector =
        input.requested_tool_id == PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
    let source_status_canary_start_guard_bound = readiness.status_canary_start_guard_bound
        && readiness.status_canary_start_guard_side_effects_closed;
    let source_status_canary_start_guard_reason_audit_ready =
        readiness.status_canary_start_guard_evidence_packet_reason_audit_ready;
    let source_status_canary_start_guard_ready = source_status_canary_start_guard_bound
        && source_status_canary_start_guard_reason_audit_ready;
    let start_request_allowed = input.start_request_present
        && requested_selected_status_canary
        && readiness.controlled_canary_readiness_plan_ready
        && source_status_canary_start_guard_ready
        && readiness.status_canary_start_guard_allowed
        && source_controlled_canary_side_effects_closed
        && source_runtime_boundaries_closed
        && side_effects == StatusCanaryStartRequestGateSideEffects::none();

    StatusCanaryStartRequestGatePlan {
        runtime: "hepta",
        surface: "status_canary_start_request_gate",
        schema_version: STATUS_CANARY_START_REQUEST_GATE_SCHEMA_VERSION,
        gate_id: STATUS_CANARY_START_REQUEST_GATE_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_controlled_canary_gate: readiness.gate,
        source_controlled_canary_ready: readiness.controlled_canary_readiness_plan_ready,
        source_controlled_canary_activation_ready: readiness.controlled_canary_activation_ready,
        source_status_canary_start_guard_bound: readiness.status_canary_start_guard_bound,
        source_status_canary_start_guard_id: readiness.status_canary_start_guard_id,
        source_status_canary_start_guard_route: readiness.status_canary_start_guard_route,
        source_status_canary_start_guard_switch_enabled: readiness
            .status_canary_start_guard_switch_enabled,
        source_status_canary_start_guard_evidence_complete: readiness
            .status_canary_start_guard_evidence_complete,
        source_status_canary_start_guard_missing_evidence_count: readiness
            .status_canary_start_guard_missing_evidence_count,
        source_status_canary_start_guard_reason_audit_count: readiness
            .status_canary_start_guard_evidence_packet_reason_audit_count,
        source_status_canary_start_guard_reason_audit_ready_count: readiness
            .status_canary_start_guard_evidence_packet_reason_audit_ready_count,
        source_status_canary_start_guard_reason_audit_rejected_count: readiness
            .status_canary_start_guard_evidence_packet_reason_audit_rejected_count,
        source_status_canary_start_guard_reason_audit_ready,
        source_status_canary_start_guard_blocked: readiness.status_canary_start_guard_blocked,
        source_status_canary_start_guard_allowed: readiness.status_canary_start_guard_allowed,
        source_controlled_canary_side_effects_closed,
        source_runtime_boundaries_closed,
        start_request_present: input.start_request_present,
        requested_tool_id: input.requested_tool_id,
        requested_selected_status_canary,
        requested_preflight_only_connector,
        start_request_allowed,
        start_request_blocked: !start_request_allowed,
        gate_route: status_canary_start_request_gate_route(
            input.start_request_present,
            requested_selected_status_canary,
            requested_preflight_only_connector,
            readiness.controlled_canary_readiness_plan_ready,
            source_status_canary_start_guard_bound,
            source_status_canary_start_guard_reason_audit_ready,
            readiness.status_canary_start_guard_allowed,
            source_controlled_canary_side_effects_closed,
            source_runtime_boundaries_closed,
            start_request_allowed,
        ),
        side_effects,
    }
}

fn controlled_canary_runtime_boundaries_closed(
    readiness: &ControlledCanaryReadinessPlanReport,
) -> bool {
    !readiness.approval_request_sent
        && !readiness.approval_request_allowed
        && !readiness.approval_accepted
        && !readiness.approval_recorded
        && !readiness.approval_broker_write_allowed
        && !readiness.evidence_recording_allowed
        && !readiness.credential_read_allowed
        && !readiness.gateway_or_auth_mutation_allowed
        && !readiness.native_post_mutation_allowed
        && !readiness.telegram_transport_mutation_allowed
        && !readiness.channel_send_allowed
        && !readiness.transport_mutation_allowed
        && !readiness.canary_persistence_allowed
        && !readiness.canary_receipt_persisted
        && !readiness.workflow_event_log_write_allowed
        && !readiness.sqlite_write_allowed
        && !readiness.provider_invocation_allowed
        && !readiness.model_invocation_allowed
        && !readiness.package_or_release_allowed
        && !readiness.public_ga_allowed
        && !readiness.live_activation_allowed
        && !readiness.live_execution_allowed
}

fn status_canary_start_request_gate_route(
    start_request_present: bool,
    requested_selected_status_canary: bool,
    requested_preflight_only_connector: bool,
    source_controlled_canary_ready: bool,
    source_status_canary_start_guard_bound: bool,
    source_status_canary_start_guard_reason_audit_ready: bool,
    source_status_canary_start_guard_allowed: bool,
    source_controlled_canary_side_effects_closed: bool,
    source_runtime_boundaries_closed: bool,
    start_request_allowed: bool,
) -> &'static str {
    if !start_request_present {
        "status_canary_start_request_blocked_no_request"
    } else if requested_preflight_only_connector {
        "status_canary_start_request_blocked_preflight_only_connector"
    } else if !requested_selected_status_canary {
        "status_canary_start_request_blocked_unknown_candidate"
    } else if !source_controlled_canary_ready {
        "status_canary_start_request_blocked_readiness_not_ready"
    } else if !source_controlled_canary_side_effects_closed {
        "status_canary_start_request_blocked_readiness_side_effects_open"
    } else if !source_runtime_boundaries_closed {
        "status_canary_start_request_blocked_runtime_boundary_open"
    } else if !source_status_canary_start_guard_bound {
        "status_canary_start_request_blocked_start_guard_not_bound"
    } else if !source_status_canary_start_guard_reason_audit_ready {
        "status_canary_start_request_blocked_start_guard_reason_audit"
    } else if !source_status_canary_start_guard_allowed {
        "status_canary_start_request_blocked_start_guard"
    } else if start_request_allowed {
        "status_canary_start_request_gate_would_allow_start"
    } else {
        "status_canary_start_request_blocked_unknown"
    }
}

impl Default for StatusCanaryStartRequestGateInput {
    fn default() -> Self {
        Self {
            start_request_present: false,
            requested_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        }
    }
}

impl StatusCanaryStartRequestGateSideEffects {
    pub const fn none() -> Self {
        Self {
            start_request_persisted: false,
            evidence_recorded: false,
            evidence_persisted: false,
            waiver_recorded: false,
            approval_requested: false,
            approval_accepted: false,
            credential_read: false,
            transport_mutated: false,
            ledger_written: false,
            receipt_written: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            registry_mutated: false,
            tool_invoked: false,
            connector_started: false,
            canary_started: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn readiness_with_allowed_start_guard() -> ControlledCanaryReadinessPlanReport {
        let mut readiness = controlled_canary_readiness_plan_report();
        readiness.status_canary_start_guard_route = "status_canary_start_guard_would_allow_start";
        readiness.status_canary_start_guard_switch_enabled = true;
        readiness.status_canary_start_guard_evidence_complete = true;
        readiness.status_canary_start_guard_missing_evidence_count = 0;
        readiness.status_canary_start_guard_blocked = false;
        readiness.status_canary_start_guard_allowed = true;
        readiness
    }

    #[test]
    fn default_gate_blocks_without_request_and_missing_evidence() {
        let gate = status_canary_start_request_gate();

        assert_eq!(gate.gate_id, STATUS_CANARY_START_REQUEST_GATE_ID);
        assert_eq!(
            gate.selected_status_canary_tool_id,
            SELECTED_STATUS_CANARY_TOOL_ID
        );
        assert!(!gate.start_request_present);
        assert!(gate.requested_selected_status_canary);
        assert!(gate.source_controlled_canary_ready);
        assert!(gate.source_status_canary_start_guard_bound);
        assert_eq!(
            gate.source_status_canary_start_guard_route,
            "status_canary_start_blocked_missing_evidence_packet"
        );
        assert_eq!(
            gate.source_status_canary_start_guard_missing_evidence_count,
            7
        );
        assert_eq!(gate.source_status_canary_start_guard_reason_audit_count, 0);
        assert_eq!(
            gate.source_status_canary_start_guard_reason_audit_ready_count,
            0
        );
        assert_eq!(
            gate.source_status_canary_start_guard_reason_audit_rejected_count,
            0
        );
        assert!(gate.source_status_canary_start_guard_reason_audit_ready);
        assert!(gate.source_status_canary_start_guard_blocked);
        assert!(!gate.source_status_canary_start_guard_allowed);
        assert!(gate.start_request_blocked);
        assert!(!gate.start_request_allowed);
        assert_eq!(
            gate.gate_route,
            "status_canary_start_request_blocked_no_request"
        );
        assert_eq!(
            gate.side_effects,
            StatusCanaryStartRequestGateSideEffects::none()
        );
    }

    #[test]
    fn selected_status_canary_request_is_blocked_by_start_guard() {
        let readiness = controlled_canary_readiness_plan_report();
        let gate = status_canary_start_request_gate_from_readiness(
            &readiness,
            StatusCanaryStartRequestGateInput {
                start_request_present: true,
                requested_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
            },
        );

        assert!(gate.start_request_present);
        assert!(gate.requested_selected_status_canary);
        assert!(!gate.requested_preflight_only_connector);
        assert!(gate.source_status_canary_start_guard_bound);
        assert!(gate.source_status_canary_start_guard_reason_audit_ready);
        assert!(gate.source_status_canary_start_guard_blocked);
        assert!(!gate.source_status_canary_start_guard_allowed);
        assert!(gate.start_request_blocked);
        assert!(!gate.start_request_allowed);
        assert_eq!(
            gate.gate_route,
            "status_canary_start_request_blocked_start_guard"
        );
        assert_eq!(
            gate.side_effects,
            StatusCanaryStartRequestGateSideEffects::none()
        );
    }

    #[test]
    fn selected_status_canary_request_is_blocked_by_start_guard_reason_audit() {
        let mut readiness = readiness_with_allowed_start_guard();
        readiness.status_canary_start_guard_route =
            "status_canary_start_blocked_evidence_packet_reason_audit";
        readiness.status_canary_start_guard_evidence_packet_reason_audit_count = 1;
        readiness.status_canary_start_guard_evidence_packet_reason_audit_ready_count = 0;
        readiness.status_canary_start_guard_evidence_packet_reason_audit_rejected_count = 1;
        readiness.status_canary_start_guard_evidence_packet_reason_audit_ready = false;
        readiness.status_canary_start_guard_blocked = true;
        readiness.status_canary_start_guard_allowed = false;
        let gate = status_canary_start_request_gate_from_readiness(
            &readiness,
            StatusCanaryStartRequestGateInput {
                start_request_present: true,
                requested_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
            },
        );

        assert!(gate.start_request_present);
        assert!(gate.requested_selected_status_canary);
        assert!(gate.source_status_canary_start_guard_bound);
        assert_eq!(gate.source_status_canary_start_guard_reason_audit_count, 1);
        assert_eq!(
            gate.source_status_canary_start_guard_reason_audit_ready_count,
            0
        );
        assert_eq!(
            gate.source_status_canary_start_guard_reason_audit_rejected_count,
            1
        );
        assert!(!gate.source_status_canary_start_guard_reason_audit_ready);
        assert!(gate.source_status_canary_start_guard_blocked);
        assert!(!gate.source_status_canary_start_guard_allowed);
        assert!(gate.start_request_blocked);
        assert!(!gate.start_request_allowed);
        assert_eq!(
            gate.gate_route,
            "status_canary_start_request_blocked_start_guard_reason_audit"
        );
        assert_eq!(
            gate.side_effects,
            StatusCanaryStartRequestGateSideEffects::none()
        );
    }

    #[test]
    fn preflight_only_connector_request_fails_closed_before_start_guard() {
        let readiness = readiness_with_allowed_start_guard();
        let gate = status_canary_start_request_gate_from_readiness(
            &readiness,
            StatusCanaryStartRequestGateInput {
                start_request_present: true,
                requested_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
            },
        );

        assert!(gate.start_request_present);
        assert!(!gate.requested_selected_status_canary);
        assert!(gate.requested_preflight_only_connector);
        assert!(gate.source_status_canary_start_guard_allowed);
        assert!(gate.start_request_blocked);
        assert!(!gate.start_request_allowed);
        assert_eq!(
            gate.gate_route,
            "status_canary_start_request_blocked_preflight_only_connector"
        );
        assert!(!gate.side_effects.connector_started);
        assert!(!gate.side_effects.tool_invoked);
        assert!(!gate.side_effects.canary_started);
    }

    #[test]
    fn allowed_guard_only_allows_the_start_request_gate_plan() {
        let readiness = readiness_with_allowed_start_guard();
        let gate = status_canary_start_request_gate_from_readiness(
            &readiness,
            StatusCanaryStartRequestGateInput {
                start_request_present: true,
                requested_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
            },
        );

        assert!(gate.start_request_present);
        assert!(gate.requested_selected_status_canary);
        assert!(gate.source_controlled_canary_ready);
        assert!(gate.source_status_canary_start_guard_bound);
        assert!(gate.source_status_canary_start_guard_switch_enabled);
        assert!(gate.source_status_canary_start_guard_evidence_complete);
        assert_eq!(
            gate.source_status_canary_start_guard_missing_evidence_count,
            0
        );
        assert!(gate.source_status_canary_start_guard_reason_audit_ready);
        assert!(gate.source_status_canary_start_guard_allowed);
        assert!(!gate.start_request_blocked);
        assert!(gate.start_request_allowed);
        assert_eq!(
            gate.gate_route,
            "status_canary_start_request_gate_would_allow_start"
        );
        assert_eq!(
            gate.side_effects,
            StatusCanaryStartRequestGateSideEffects::none()
        );
        assert!(!gate.side_effects.start_request_persisted);
        assert!(!gate.side_effects.canary_started);
        assert!(!gate.side_effects.tool_invoked);
        assert!(!gate.side_effects.ledger_written);
        assert!(!gate.side_effects.receipt_written);
    }
}
