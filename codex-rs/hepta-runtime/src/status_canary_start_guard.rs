use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;
use crate::status_canary_evidence_packet::StatusCanaryEvidencePacket;
use crate::status_canary_evidence_packet::StatusCanaryEvidencePacketSideEffects;
use crate::status_canary_evidence_packet::status_canary_evidence_packet;
use serde::Serialize;

pub const STATUS_CANARY_START_GUARD_SCHEMA_VERSION: &str = "status_canary_start_guard_v1";
pub const STATUS_CANARY_START_GUARD_ID: &str = "status-canary-start-guard/hepta-system-status/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCanaryStartGuardInput {
    pub canary_start_switch_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryStartGuardPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub schema_version: &'static str,
    pub guard_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_evidence_packet_id: &'static str,
    pub source_evidence_packet_ready: bool,
    pub source_evidence_packet_complete: bool,
    pub source_evidence_packet_missing_count: usize,
    pub source_evidence_packet_recorded_count: usize,
    pub source_evidence_packet_waived_count: usize,
    pub source_evidence_packet_expired_count: usize,
    pub source_evidence_packet_invalid_count: usize,
    pub source_evidence_packet_decision_reason_audit_count: usize,
    pub source_evidence_packet_decision_reason_audit_ready_count: usize,
    pub source_evidence_packet_decision_reason_audit_rejected_count: usize,
    pub source_evidence_packet_reason_audit_ready: bool,
    pub source_evidence_packet_guard_route: &'static str,
    pub canary_start_switch_enabled: bool,
    pub canary_start_allowed: bool,
    pub canary_start_blocked: bool,
    pub guard_route: &'static str,
    pub side_effects: StatusCanaryStartGuardSideEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryStartGuardSideEffects {
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

pub fn status_canary_start_guard() -> StatusCanaryStartGuardPlan {
    let evidence_packet = status_canary_evidence_packet();
    status_canary_start_guard_from_packet(&evidence_packet, StatusCanaryStartGuardInput::default())
}

pub fn status_canary_start_guard_from_packet(
    evidence_packet: &StatusCanaryEvidencePacket,
    input: StatusCanaryStartGuardInput,
) -> StatusCanaryStartGuardPlan {
    let side_effects = StatusCanaryStartGuardSideEffects::none();
    let source_side_effects_closed =
        evidence_packet.side_effects == StatusCanaryEvidencePacketSideEffects::none();
    let source_evidence_packet_reason_audit_ready = evidence_packet.decision_reason_audit_count
        == evidence_packet.decision_reason_audit_ready_count
        && evidence_packet.decision_reason_audit_rejected_count == 0;
    let canary_start_allowed = evidence_packet.packet_ready
        && evidence_packet.evidence_complete
        && source_evidence_packet_reason_audit_ready
        && input.canary_start_switch_enabled
        && source_side_effects_closed
        && side_effects == StatusCanaryStartGuardSideEffects::none();

    StatusCanaryStartGuardPlan {
        runtime: "hepta",
        surface: "status_canary_start_guard",
        schema_version: STATUS_CANARY_START_GUARD_SCHEMA_VERSION,
        guard_id: STATUS_CANARY_START_GUARD_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_evidence_packet_id: evidence_packet.packet_id,
        source_evidence_packet_ready: evidence_packet.packet_ready,
        source_evidence_packet_complete: evidence_packet.evidence_complete,
        source_evidence_packet_missing_count: evidence_packet.missing_item_count,
        source_evidence_packet_recorded_count: evidence_packet.recorded_item_count,
        source_evidence_packet_waived_count: evidence_packet.waived_item_count,
        source_evidence_packet_expired_count: evidence_packet.expired_item_count,
        source_evidence_packet_invalid_count: evidence_packet.invalid_item_count,
        source_evidence_packet_decision_reason_audit_count: evidence_packet
            .decision_reason_audit_count,
        source_evidence_packet_decision_reason_audit_ready_count: evidence_packet
            .decision_reason_audit_ready_count,
        source_evidence_packet_decision_reason_audit_rejected_count: evidence_packet
            .decision_reason_audit_rejected_count,
        source_evidence_packet_reason_audit_ready,
        source_evidence_packet_guard_route: evidence_packet.guard_route,
        canary_start_switch_enabled: input.canary_start_switch_enabled,
        canary_start_allowed,
        canary_start_blocked: !canary_start_allowed,
        guard_route: status_canary_start_guard_route(
            evidence_packet.packet_ready,
            evidence_packet.evidence_complete,
            evidence_packet.missing_item_count,
            evidence_packet.expired_item_count,
            evidence_packet.invalid_item_count,
            source_evidence_packet_reason_audit_ready,
            input.canary_start_switch_enabled,
            source_side_effects_closed,
            canary_start_allowed,
        ),
        side_effects,
    }
}

fn status_canary_start_guard_route(
    source_evidence_packet_ready: bool,
    source_evidence_packet_complete: bool,
    source_evidence_packet_missing_count: usize,
    source_evidence_packet_expired_count: usize,
    source_evidence_packet_invalid_count: usize,
    source_evidence_packet_reason_audit_ready: bool,
    canary_start_switch_enabled: bool,
    source_side_effects_closed: bool,
    canary_start_allowed: bool,
) -> &'static str {
    if !source_evidence_packet_ready {
        "status_canary_start_blocked_evidence_packet_not_ready"
    } else if !source_side_effects_closed {
        "status_canary_start_blocked_evidence_packet_side_effects_open"
    } else if !source_evidence_packet_reason_audit_ready {
        "status_canary_start_blocked_evidence_packet_reason_audit"
    } else if !source_evidence_packet_complete && source_evidence_packet_missing_count > 0 {
        "status_canary_start_blocked_missing_evidence_packet"
    } else if !source_evidence_packet_complete && source_evidence_packet_expired_count > 0 {
        "status_canary_start_blocked_expired_evidence_packet"
    } else if !source_evidence_packet_complete && source_evidence_packet_invalid_count > 0 {
        "status_canary_start_blocked_invalid_evidence_packet"
    } else if !source_evidence_packet_complete {
        "status_canary_start_blocked_incomplete_evidence_packet"
    } else if !canary_start_switch_enabled {
        "status_canary_start_blocked_switch_closed"
    } else if canary_start_allowed {
        "status_canary_start_guard_would_allow_start"
    } else {
        "status_canary_start_blocked_unknown"
    }
}

impl Default for StatusCanaryStartGuardInput {
    fn default() -> Self {
        Self {
            canary_start_switch_enabled: false,
        }
    }
}

impl StatusCanaryStartGuardSideEffects {
    pub const fn none() -> Self {
        Self {
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
    use crate::controlled_live_required_evidence_collection_plan::controlled_live_required_evidence_collection_plan_report;
    use crate::status_canary_evidence_packet::StatusCanaryEvidenceDecision;
    use crate::status_canary_evidence_packet::StatusCanaryEvidenceDecisionOverride;
    use crate::status_canary_evidence_packet::status_canary_evidence_packet_from_decisions;

    fn complete_evidence_packet() -> StatusCanaryEvidencePacket {
        let decisions = controlled_live_required_evidence_collection_plan_report()
            .entries
            .iter()
            .map(|entry| StatusCanaryEvidenceDecisionOverride::recorded(entry.source_blocker_id))
            .collect::<Vec<_>>();

        status_canary_evidence_packet_from_decisions(&decisions)
    }

    fn reason_audited_recorded_decision(
        source_blocker_id: &'static str,
    ) -> StatusCanaryEvidenceDecisionOverride {
        let mut decision = StatusCanaryEvidenceDecisionOverride::recorded(source_blocker_id);
        decision.source_acceptance_packet_bound = true;
        decision.source_acceptance_request_source_validator_bound = true;
        decision.source_acceptance_request_reason_audit_bound = true;
        decision.source_acceptance_request_reason_audit_ready = true;
        decision.source_readback_reason_packet_route =
            "status_canary_evidence_source_reason_packet_ready_inputs_valid";
        decision.source_readback_fixture_reason_audit_rejection_reason =
            "fixture_generation_allowed";
        decision
    }

    fn complete_reason_audited_evidence_packet() -> StatusCanaryEvidencePacket {
        let decisions = controlled_live_required_evidence_collection_plan_report()
            .entries
            .iter()
            .map(|entry| reason_audited_recorded_decision(entry.source_blocker_id))
            .collect::<Vec<_>>();

        status_canary_evidence_packet_from_decisions(&decisions)
    }

    #[test]
    fn default_guard_blocks_on_missing_evidence_packet() {
        let guard = status_canary_start_guard();

        assert_eq!(guard.guard_id, STATUS_CANARY_START_GUARD_ID);
        assert_eq!(
            guard.selected_status_canary_tool_id,
            SELECTED_STATUS_CANARY_TOOL_ID
        );
        assert_eq!(
            guard.preflight_only_connector_tool_id,
            PREFLIGHT_ONLY_CONNECTOR_TOOL_ID
        );
        assert!(guard.source_evidence_packet_ready);
        assert!(!guard.source_evidence_packet_complete);
        assert_eq!(guard.source_evidence_packet_missing_count, 7);
        assert_eq!(guard.source_evidence_packet_recorded_count, 0);
        assert_eq!(guard.source_evidence_packet_waived_count, 0);
        assert_eq!(guard.source_evidence_packet_expired_count, 0);
        assert_eq!(guard.source_evidence_packet_invalid_count, 0);
        assert_eq!(guard.source_evidence_packet_decision_reason_audit_count, 0);
        assert_eq!(
            guard.source_evidence_packet_decision_reason_audit_ready_count,
            0
        );
        assert_eq!(
            guard.source_evidence_packet_decision_reason_audit_rejected_count,
            0
        );
        assert!(guard.source_evidence_packet_reason_audit_ready);
        assert!(!guard.canary_start_switch_enabled);
        assert!(guard.canary_start_blocked);
        assert!(!guard.canary_start_allowed);
        assert_eq!(
            guard.guard_route,
            "status_canary_start_blocked_missing_evidence_packet"
        );
        assert_eq!(
            guard.side_effects,
            StatusCanaryStartGuardSideEffects::none()
        );
    }

    #[test]
    fn complete_evidence_still_blocks_when_start_switch_is_closed() {
        let packet = complete_evidence_packet();
        let guard =
            status_canary_start_guard_from_packet(&packet, StatusCanaryStartGuardInput::default());

        assert!(guard.source_evidence_packet_ready);
        assert!(guard.source_evidence_packet_complete);
        assert_eq!(guard.source_evidence_packet_missing_count, 0);
        assert_eq!(guard.source_evidence_packet_recorded_count, 7);
        assert_eq!(guard.source_evidence_packet_waived_count, 0);
        assert_eq!(guard.source_evidence_packet_expired_count, 0);
        assert_eq!(guard.source_evidence_packet_invalid_count, 0);
        assert_eq!(guard.source_evidence_packet_decision_reason_audit_count, 0);
        assert_eq!(
            guard.source_evidence_packet_decision_reason_audit_ready_count,
            0
        );
        assert_eq!(
            guard.source_evidence_packet_decision_reason_audit_rejected_count,
            0
        );
        assert!(guard.source_evidence_packet_reason_audit_ready);
        assert!(!guard.canary_start_switch_enabled);
        assert!(guard.canary_start_blocked);
        assert!(!guard.canary_start_allowed);
        assert_eq!(
            guard.guard_route,
            "status_canary_start_blocked_switch_closed"
        );
        assert_eq!(
            guard.side_effects,
            StatusCanaryStartGuardSideEffects::none()
        );
    }

    #[test]
    fn complete_evidence_and_open_switch_only_allow_the_guard_plan() {
        let packet = complete_evidence_packet();
        let guard = status_canary_start_guard_from_packet(
            &packet,
            StatusCanaryStartGuardInput {
                canary_start_switch_enabled: true,
            },
        );

        assert!(guard.source_evidence_packet_ready);
        assert!(guard.source_evidence_packet_complete);
        assert!(guard.source_evidence_packet_reason_audit_ready);
        assert!(guard.canary_start_switch_enabled);
        assert!(!guard.canary_start_blocked);
        assert!(guard.canary_start_allowed);
        assert_eq!(
            guard.guard_route,
            "status_canary_start_guard_would_allow_start"
        );
        assert_eq!(
            guard.side_effects,
            StatusCanaryStartGuardSideEffects::none()
        );
        assert!(!guard.side_effects.canary_started);
        assert!(!guard.side_effects.tool_invoked);
        assert!(!guard.side_effects.ledger_written);
    }

    #[test]
    fn complete_reason_audited_evidence_and_open_switch_allow_the_guard_plan() {
        let packet = complete_reason_audited_evidence_packet();
        let guard = status_canary_start_guard_from_packet(
            &packet,
            StatusCanaryStartGuardInput {
                canary_start_switch_enabled: true,
            },
        );

        assert!(guard.source_evidence_packet_ready);
        assert!(guard.source_evidence_packet_complete);
        assert_eq!(guard.source_evidence_packet_missing_count, 0);
        assert_eq!(guard.source_evidence_packet_recorded_count, 7);
        assert_eq!(guard.source_evidence_packet_decision_reason_audit_count, 7);
        assert_eq!(
            guard.source_evidence_packet_decision_reason_audit_ready_count,
            7
        );
        assert_eq!(
            guard.source_evidence_packet_decision_reason_audit_rejected_count,
            0
        );
        assert!(guard.source_evidence_packet_reason_audit_ready);
        assert!(guard.canary_start_switch_enabled);
        assert!(!guard.canary_start_blocked);
        assert!(guard.canary_start_allowed);
        assert_eq!(
            guard.guard_route,
            "status_canary_start_guard_would_allow_start"
        );
        assert_eq!(
            guard.side_effects,
            StatusCanaryStartGuardSideEffects::none()
        );
    }

    #[test]
    fn expired_and_invalid_evidence_block_even_when_start_switch_is_open() {
        let decisions = controlled_live_required_evidence_collection_plan_report()
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                StatusCanaryEvidenceDecisionOverride::with_decision(
                    entry.source_blocker_id,
                    match index {
                        0 | 1 => StatusCanaryEvidenceDecision::Recorded,
                        2 | 3 => StatusCanaryEvidenceDecision::Waived,
                        4 | 5 => StatusCanaryEvidenceDecision::Expired,
                        _ => StatusCanaryEvidenceDecision::Invalid,
                    },
                )
            })
            .collect::<Vec<_>>();
        let packet = status_canary_evidence_packet_from_decisions(&decisions);
        let guard = status_canary_start_guard_from_packet(
            &packet,
            StatusCanaryStartGuardInput {
                canary_start_switch_enabled: true,
            },
        );

        assert!(guard.source_evidence_packet_ready);
        assert!(!guard.source_evidence_packet_complete);
        assert_eq!(guard.source_evidence_packet_missing_count, 0);
        assert_eq!(guard.source_evidence_packet_recorded_count, 2);
        assert_eq!(guard.source_evidence_packet_waived_count, 2);
        assert_eq!(guard.source_evidence_packet_expired_count, 2);
        assert_eq!(guard.source_evidence_packet_invalid_count, 1);
        assert_eq!(guard.source_evidence_packet_decision_reason_audit_count, 0);
        assert_eq!(
            guard.source_evidence_packet_decision_reason_audit_ready_count,
            0
        );
        assert_eq!(
            guard.source_evidence_packet_decision_reason_audit_rejected_count,
            0
        );
        assert!(guard.source_evidence_packet_reason_audit_ready);
        assert!(guard.canary_start_switch_enabled);
        assert!(guard.canary_start_blocked);
        assert!(!guard.canary_start_allowed);
        assert_eq!(
            guard.guard_route,
            "status_canary_start_blocked_expired_evidence_packet"
        );
        assert_eq!(
            guard.side_effects,
            StatusCanaryStartGuardSideEffects::none()
        );
        assert!(!guard.side_effects.canary_started);
        assert!(!guard.side_effects.tool_invoked);
        assert!(!guard.side_effects.ledger_written);
    }

    #[test]
    fn packet_not_ready_fails_closed_even_with_complete_evidence_and_open_switch() {
        let mut packet = complete_evidence_packet();
        packet.packet_ready = false;
        packet.guard_route = "status_canary_evidence_packet_blocked_not_ready";
        let guard = status_canary_start_guard_from_packet(
            &packet,
            StatusCanaryStartGuardInput {
                canary_start_switch_enabled: true,
            },
        );

        assert!(!guard.source_evidence_packet_ready);
        assert!(guard.source_evidence_packet_complete);
        assert!(guard.canary_start_switch_enabled);
        assert!(guard.canary_start_blocked);
        assert!(!guard.canary_start_allowed);
        assert_eq!(
            guard.guard_route,
            "status_canary_start_blocked_evidence_packet_not_ready"
        );
        assert_eq!(
            guard.side_effects,
            StatusCanaryStartGuardSideEffects::none()
        );
    }

    #[test]
    fn reason_audit_rejection_blocks_before_missing_evidence_route() {
        let mut decision = reason_audited_recorded_decision("dirty_worktree_boundary");
        decision.source_acceptance_request_reason_audit_ready = false;
        decision.source_readback_fixture_reason_audit_rejection_reason =
            "source_adapter_input_missing_for_decision";
        let packet = status_canary_evidence_packet_from_decisions(&[decision]);
        let guard = status_canary_start_guard_from_packet(
            &packet,
            StatusCanaryStartGuardInput {
                canary_start_switch_enabled: true,
            },
        );

        assert!(guard.source_evidence_packet_ready);
        assert!(!guard.source_evidence_packet_complete);
        assert_eq!(guard.source_evidence_packet_missing_count, 7);
        assert_eq!(guard.source_evidence_packet_decision_reason_audit_count, 1);
        assert_eq!(
            guard.source_evidence_packet_decision_reason_audit_ready_count,
            0
        );
        assert_eq!(
            guard.source_evidence_packet_decision_reason_audit_rejected_count,
            1
        );
        assert!(!guard.source_evidence_packet_reason_audit_ready);
        assert!(guard.canary_start_blocked);
        assert!(!guard.canary_start_allowed);
        assert_eq!(
            guard.guard_route,
            "status_canary_start_blocked_evidence_packet_reason_audit"
        );
        assert_eq!(
            guard.side_effects,
            StatusCanaryStartGuardSideEffects::none()
        );
    }
}
