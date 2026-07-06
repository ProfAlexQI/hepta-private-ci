use crate::controlled_live_required_evidence_collection_plan::ControlledLiveRequiredEvidenceCollectionPlanEntry;
use crate::controlled_live_required_evidence_collection_plan::controlled_live_required_evidence_collection_plan_report;
use serde::Serialize;

pub const STATUS_CANARY_EVIDENCE_PACKET_SCHEMA_VERSION: &str = "status_canary_evidence_packet_v1";
pub const STATUS_CANARY_EVIDENCE_PACKET_ID: &str =
    "status-canary-evidence-packet/hepta-system-status/v1";
pub const SELECTED_STATUS_CANARY_TOOL_ID: &str =
    "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp";
pub const PREFLIGHT_ONLY_CONNECTOR_TOOL_ID: &str =
    "preview:connector:hepta-system@hepta-local:hepta_system_local_app";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidencePacket {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub schema_version: &'static str,
    pub packet_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_required_evidence_collection_plan_ready: bool,
    pub checklist_item_count: usize,
    pub missing_item_count: usize,
    pub recorded_item_count: usize,
    pub waived_item_count: usize,
    pub expired_item_count: usize,
    pub invalid_item_count: usize,
    pub decision_reason_audit_count: usize,
    pub decision_reason_audit_ready_count: usize,
    pub decision_reason_audit_rejected_count: usize,
    pub action_required_count: usize,
    pub evidence_complete: bool,
    pub packet_ready: bool,
    pub canary_start_switch_enabled: bool,
    pub canary_start_blocked: bool,
    pub canary_start_allowed: bool,
    pub guard_route: &'static str,
    pub entries: Vec<StatusCanaryEvidenceChecklistItem>,
    pub side_effects: StatusCanaryEvidencePacketSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceChecklistItem {
    pub source_blocker_id: &'static str,
    pub packet_key: &'static str,
    pub packet_route: &'static str,
    pub action_kind: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub decision: StatusCanaryEvidenceDecision,
    pub evidence_state: &'static str,
    pub source_acceptance_packet_bound: bool,
    pub source_acceptance_request_source_validator_bound: bool,
    pub source_acceptance_request_reason_audit_bound: bool,
    pub source_acceptance_request_reason_audit_ready: bool,
    pub source_readback_reason_packet_route: &'static str,
    pub source_readback_fixture_reason_audit_rejection_reason: &'static str,
    pub decision_reason_audit_ready: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub action_required: bool,
    pub canary_start_blocked: bool,
    pub evidence_recorded: bool,
    pub evidence_waived: bool,
    pub evidence_expired: bool,
    pub evidence_invalid: bool,
    pub evidence_recording_allowed: bool,
    pub waiver_allowed: bool,
    pub credential_read_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusCanaryEvidenceDecision {
    Missing,
    Recorded,
    Waived,
    Expired,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceDecisionOverride {
    pub source_blocker_id: &'static str,
    pub decision: StatusCanaryEvidenceDecision,
    pub source_acceptance_packet_bound: bool,
    pub source_acceptance_request_source_validator_bound: bool,
    pub source_acceptance_request_reason_audit_bound: bool,
    pub source_acceptance_request_reason_audit_ready: bool,
    pub source_readback_reason_packet_route: &'static str,
    pub source_readback_fixture_reason_audit_rejection_reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidencePacketSideEffects {
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub waiver_recorded: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub credential_read: bool,
    pub transport_mutated: bool,
    pub ledger_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub tool_invoked: bool,
    pub connector_started: bool,
    pub live_execution_started: bool,
}

pub fn status_canary_evidence_packet() -> StatusCanaryEvidencePacket {
    let source = controlled_live_required_evidence_collection_plan_report();
    status_canary_evidence_packet_from_plan(
        source.evidence_collection_plan_ready,
        source.entries,
        &[],
    )
}

pub fn status_canary_evidence_packet_from_decisions(
    decisions: &[StatusCanaryEvidenceDecisionOverride],
) -> StatusCanaryEvidencePacket {
    let source = controlled_live_required_evidence_collection_plan_report();
    status_canary_evidence_packet_from_plan(
        source.evidence_collection_plan_ready,
        source.entries,
        decisions,
    )
}

pub fn status_canary_evidence_packet_from_plan(
    source_required_evidence_collection_plan_ready: bool,
    source_entries: Vec<ControlledLiveRequiredEvidenceCollectionPlanEntry>,
    decisions: &[StatusCanaryEvidenceDecisionOverride],
) -> StatusCanaryEvidencePacket {
    let entries = source_entries
        .into_iter()
        .map(|entry| status_canary_evidence_checklist_item(entry, decisions))
        .collect::<Vec<_>>();
    let missing_item_count = entries
        .iter()
        .filter(|entry| entry.decision == StatusCanaryEvidenceDecision::Missing)
        .count();
    let recorded_item_count = entries
        .iter()
        .filter(|entry| entry.decision == StatusCanaryEvidenceDecision::Recorded)
        .count();
    let waived_item_count = entries
        .iter()
        .filter(|entry| entry.decision == StatusCanaryEvidenceDecision::Waived)
        .count();
    let expired_item_count = entries
        .iter()
        .filter(|entry| entry.decision == StatusCanaryEvidenceDecision::Expired)
        .count();
    let invalid_item_count = entries
        .iter()
        .filter(|entry| entry.decision == StatusCanaryEvidenceDecision::Invalid)
        .count();
    let decision_reason_audit_count = decisions
        .iter()
        .filter(|decision| decision.source_acceptance_request_reason_audit_bound)
        .count();
    let decision_reason_audit_ready_count = decisions
        .iter()
        .filter(|decision| {
            decision.source_acceptance_request_reason_audit_bound
                && status_canary_evidence_decision_override_reason_audit_ready(**decision)
        })
        .count();
    let decision_reason_audit_rejected_count =
        decision_reason_audit_count.saturating_sub(decision_reason_audit_ready_count);
    let action_required_count = entries.iter().filter(|entry| entry.action_required).count();
    let side_effects = StatusCanaryEvidencePacketSideEffects::none();
    let evidence_complete = entries.len() == 7
        && missing_item_count == 0
        && expired_item_count == 0
        && invalid_item_count == 0
        && recorded_item_count + waived_item_count == 7;
    let packet_ready = source_required_evidence_collection_plan_ready
        && entries.len() == 7
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && !entry.evidence_recording_allowed
                && !entry.waiver_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        })
        && side_effects == StatusCanaryEvidencePacketSideEffects::none();
    let canary_start_switch_enabled = false;
    let canary_start_allowed = packet_ready && evidence_complete && canary_start_switch_enabled;
    let canary_start_blocked = !canary_start_allowed;

    StatusCanaryEvidencePacket {
        runtime: "hepta",
        surface: "status_canary_evidence_packet",
        schema_version: STATUS_CANARY_EVIDENCE_PACKET_SCHEMA_VERSION,
        packet_id: STATUS_CANARY_EVIDENCE_PACKET_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_required_evidence_collection_plan_ready,
        checklist_item_count: entries.len(),
        missing_item_count,
        recorded_item_count,
        waived_item_count,
        expired_item_count,
        invalid_item_count,
        decision_reason_audit_count,
        decision_reason_audit_ready_count,
        decision_reason_audit_rejected_count,
        action_required_count,
        evidence_complete,
        packet_ready,
        canary_start_switch_enabled,
        canary_start_blocked,
        canary_start_allowed,
        guard_route: status_canary_evidence_packet_guard_route(
            packet_ready,
            evidence_complete,
            missing_item_count,
            expired_item_count,
            invalid_item_count,
            canary_start_switch_enabled,
        ),
        entries,
        side_effects,
    }
}

fn status_canary_evidence_checklist_item(
    source: ControlledLiveRequiredEvidenceCollectionPlanEntry,
    decisions: &[StatusCanaryEvidenceDecisionOverride],
) -> StatusCanaryEvidenceChecklistItem {
    let decision = decisions
        .iter()
        .find(|decision| decision.source_blocker_id == source.source_blocker_id)
        .filter(|decision| status_canary_evidence_decision_override_valid(**decision))
        .map(|decision| decision.decision)
        .unwrap_or(StatusCanaryEvidenceDecision::Missing);
    let decision_override = decisions
        .iter()
        .find(|decision| decision.source_blocker_id == source.source_blocker_id)
        .copied();
    let decision_reason_audit_ready = decision_override
        .map(status_canary_evidence_decision_override_reason_audit_ready)
        .unwrap_or(false);

    StatusCanaryEvidenceChecklistItem {
        source_blocker_id: source.source_blocker_id,
        packet_key: status_canary_evidence_packet_key(source.source_blocker_id),
        packet_route: status_canary_evidence_packet_route(source.source_blocker_id),
        action_kind: status_canary_evidence_action_kind(source.source_blocker_id),
        operator_label: source.operator_label,
        required_evidence: source.required_evidence,
        decision,
        evidence_state: decision.evidence_state(),
        source_acceptance_packet_bound: decision_override
            .map(|decision| decision.source_acceptance_packet_bound)
            .unwrap_or(false),
        source_acceptance_request_source_validator_bound: decision_override
            .map(|decision| decision.source_acceptance_request_source_validator_bound)
            .unwrap_or(false),
        source_acceptance_request_reason_audit_bound: decision_override
            .map(|decision| decision.source_acceptance_request_reason_audit_bound)
            .unwrap_or(false),
        source_acceptance_request_reason_audit_ready: decision_override
            .map(|decision| decision.source_acceptance_request_reason_audit_ready)
            .unwrap_or(false),
        source_readback_reason_packet_route: decision_override
            .map(|decision| decision.source_readback_reason_packet_route)
            .unwrap_or("status_canary_evidence_source_reason_packet_not_bound_to_evidence_packet"),
        source_readback_fixture_reason_audit_rejection_reason: decision_override
            .map(|decision| decision.source_readback_fixture_reason_audit_rejection_reason)
            .unwrap_or("source_reason_packet_not_bound_to_evidence_packet"),
        decision_reason_audit_ready,
        operator_visible: true,
        queryable: true,
        action_required: decision.action_required(),
        canary_start_blocked: !decision.is_complete(),
        evidence_recorded: decision == StatusCanaryEvidenceDecision::Recorded,
        evidence_waived: decision == StatusCanaryEvidenceDecision::Waived,
        evidence_expired: decision == StatusCanaryEvidenceDecision::Expired,
        evidence_invalid: decision == StatusCanaryEvidenceDecision::Invalid,
        evidence_recording_allowed: false,
        waiver_allowed: false,
        credential_read_allowed: false,
        transport_mutation_allowed: false,
        persistence_allowed: false,
        live_mutation_allowed: false,
    }
}

fn status_canary_evidence_decision_override_valid(
    decision: StatusCanaryEvidenceDecisionOverride,
) -> bool {
    if !decision.source_acceptance_packet_bound {
        return true;
    }

    if !decision.source_acceptance_request_reason_audit_bound {
        return true;
    }

    status_canary_evidence_decision_override_reason_audit_ready(decision)
}

fn status_canary_evidence_decision_override_reason_audit_ready(
    decision: StatusCanaryEvidenceDecisionOverride,
) -> bool {
    if !decision.source_acceptance_request_reason_audit_bound {
        return false;
    }

    decision.source_acceptance_request_reason_audit_ready
        && decision.source_readback_reason_packet_route
            != "status_canary_evidence_source_reason_packet_not_bound_to_evidence_packet"
        && decision.source_readback_fixture_reason_audit_rejection_reason
            == "fixture_generation_allowed"
}

fn status_canary_evidence_packet_guard_route(
    packet_ready: bool,
    evidence_complete: bool,
    missing_item_count: usize,
    expired_item_count: usize,
    invalid_item_count: usize,
    canary_start_switch_enabled: bool,
) -> &'static str {
    if !packet_ready {
        "status_canary_evidence_packet_blocked_not_ready"
    } else if !evidence_complete && missing_item_count > 0 {
        "status_canary_evidence_packet_blocked_missing_evidence"
    } else if !evidence_complete && expired_item_count > 0 {
        "status_canary_evidence_packet_blocked_expired_evidence"
    } else if !evidence_complete && invalid_item_count > 0 {
        "status_canary_evidence_packet_blocked_invalid_evidence"
    } else if !evidence_complete {
        "status_canary_evidence_packet_blocked_incomplete_evidence"
    } else if !canary_start_switch_enabled {
        "status_canary_evidence_packet_ready_start_switch_closed"
    } else {
        "status_canary_evidence_packet_allows_start"
    }
}

fn status_canary_evidence_packet_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "status_canary.evidence_packet.dirty_worktree_boundary",
        "operator_live_approval_missing" => {
            "status_canary.evidence_packet.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "status_canary.evidence_packet.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "status_canary.evidence_packet.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "status_canary.evidence_packet.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => "status_canary.evidence_packet.rollback_rehearsal_missing",
        "kill_switch_rehearsal_missing" => {
            "status_canary.evidence_packet.kill_switch_rehearsal_missing"
        }
        _ => "status_canary.evidence_packet.unknown",
    }
}

fn status_canary_evidence_packet_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "memory://status-canary/evidence-packet/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "memory://status-canary/evidence-packet/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "memory://status-canary/evidence-packet/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "memory://status-canary/evidence-packet/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "memory://status-canary/evidence-packet/transport-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "memory://status-canary/evidence-packet/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "memory://status-canary/evidence-packet/kill-switch-rehearsal-missing"
        }
        _ => "memory://status-canary/evidence-packet/unknown",
    }
}

fn status_canary_evidence_action_kind(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "clean_worktree_snapshot_required",
        "operator_live_approval_missing" => "operator_live_approval_packet_required",
        "fresh_soak_readback_missing" => "fresh_status_canary_soak_readback_required",
        "credential_boundary_attestation_missing" => "credential_boundary_attestation_required",
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "transport_boundary_approval_required"
        }
        "rollback_rehearsal_missing" => "rollback_rehearsal_packet_required",
        "kill_switch_rehearsal_missing" => "kill_switch_rehearsal_packet_required",
        _ => "unknown_status_canary_evidence_action_required",
    }
}

impl StatusCanaryEvidenceDecision {
    pub const fn evidence_state(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Recorded => "recorded",
            Self::Waived => "waived",
            Self::Expired => "expired",
            Self::Invalid => "invalid",
        }
    }

    pub const fn action_required(self) -> bool {
        matches!(self, Self::Missing | Self::Expired | Self::Invalid)
    }

    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Recorded | Self::Waived)
    }
}

impl StatusCanaryEvidenceDecisionOverride {
    pub const fn recorded(source_blocker_id: &'static str) -> Self {
        Self::with_decision(source_blocker_id, StatusCanaryEvidenceDecision::Recorded)
    }

    pub const fn waived(source_blocker_id: &'static str) -> Self {
        Self::with_decision(source_blocker_id, StatusCanaryEvidenceDecision::Waived)
    }

    pub const fn expired(source_blocker_id: &'static str) -> Self {
        Self::with_decision(source_blocker_id, StatusCanaryEvidenceDecision::Expired)
    }

    pub const fn invalid(source_blocker_id: &'static str) -> Self {
        Self::with_decision(source_blocker_id, StatusCanaryEvidenceDecision::Invalid)
    }

    pub const fn with_decision(
        source_blocker_id: &'static str,
        decision: StatusCanaryEvidenceDecision,
    ) -> Self {
        Self {
            source_blocker_id,
            decision,
            source_acceptance_packet_bound: false,
            source_acceptance_request_source_validator_bound: false,
            source_acceptance_request_reason_audit_bound: false,
            source_acceptance_request_reason_audit_ready: false,
            source_readback_reason_packet_route: "status_canary_evidence_source_reason_packet_not_bound_to_evidence_packet",
            source_readback_fixture_reason_audit_rejection_reason: "source_reason_packet_not_bound_to_evidence_packet",
        }
    }
}

impl StatusCanaryEvidencePacketSideEffects {
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
            workflow_event_log_written: false,
            sqlite_written: false,
            tool_invoked: false,
            connector_started: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_collects_seven_missing_evidence_actions_without_side_effects() {
        let packet = status_canary_evidence_packet();

        assert!(packet.packet_ready);
        assert!(!packet.evidence_complete);
        assert!(packet.canary_start_blocked);
        assert!(!packet.canary_start_allowed);
        assert_eq!(
            packet.guard_route,
            "status_canary_evidence_packet_blocked_missing_evidence"
        );
        assert_eq!(
            packet.selected_status_canary_tool_id,
            SELECTED_STATUS_CANARY_TOOL_ID
        );
        assert_eq!(
            packet.preflight_only_connector_tool_id,
            PREFLIGHT_ONLY_CONNECTOR_TOOL_ID
        );
        assert_eq!(packet.checklist_item_count, 7);
        assert_eq!(packet.missing_item_count, 7);
        assert_eq!(packet.recorded_item_count, 0);
        assert_eq!(packet.waived_item_count, 0);
        assert_eq!(packet.expired_item_count, 0);
        assert_eq!(packet.invalid_item_count, 0);
        assert_eq!(packet.decision_reason_audit_count, 0);
        assert_eq!(packet.decision_reason_audit_ready_count, 0);
        assert_eq!(packet.decision_reason_audit_rejected_count, 0);
        assert_eq!(packet.action_required_count, 7);
        assert_eq!(
            packet.side_effects,
            StatusCanaryEvidencePacketSideEffects::none()
        );
        assert!(packet.entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.action_required
                && entry.canary_start_blocked
                && entry.evidence_state == "missing"
                && !entry.evidence_recorded
                && !entry.evidence_waived
                && !entry.evidence_expired
                && !entry.evidence_invalid
                && !entry.evidence_recording_allowed
                && !entry.waiver_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        }));
    }

    #[test]
    fn packet_has_stable_action_keys_for_all_controlled_live_blockers() {
        let packet = status_canary_evidence_packet();

        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.action_kind == "clean_worktree_snapshot_required"
                && entry.packet_route
                    == "memory://status-canary/evidence-packet/dirty-worktree-boundary"
        }));
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.action_kind == "operator_live_approval_packet_required"
        }));
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "fresh_soak_readback_missing"
                && entry.action_kind == "fresh_status_canary_soak_readback_required"
        }));
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "credential_boundary_attestation_missing"
                && entry.action_kind == "credential_boundary_attestation_required"
        }));
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing"
                && entry.action_kind == "transport_boundary_approval_required"
        }));
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "rollback_rehearsal_missing"
                && entry.action_kind == "rollback_rehearsal_packet_required"
        }));
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "kill_switch_rehearsal_missing"
                && entry.action_kind == "kill_switch_rehearsal_packet_required"
        }));
    }

    #[test]
    fn complete_packet_still_cannot_start_canary_until_switch_opens() {
        let decisions = controlled_live_required_evidence_collection_plan_report()
            .entries
            .iter()
            .map(|entry| StatusCanaryEvidenceDecisionOverride::recorded(entry.source_blocker_id))
            .collect::<Vec<_>>();
        let packet = status_canary_evidence_packet_from_decisions(&decisions);

        assert!(packet.packet_ready);
        assert!(packet.evidence_complete);
        assert_eq!(packet.missing_item_count, 0);
        assert_eq!(packet.recorded_item_count, 7);
        assert_eq!(packet.waived_item_count, 0);
        assert_eq!(packet.expired_item_count, 0);
        assert_eq!(packet.invalid_item_count, 0);
        assert_eq!(packet.decision_reason_audit_count, 0);
        assert_eq!(packet.decision_reason_audit_ready_count, 0);
        assert_eq!(packet.decision_reason_audit_rejected_count, 0);
        assert_eq!(packet.action_required_count, 0);
        assert_eq!(
            packet.guard_route,
            "status_canary_evidence_packet_ready_start_switch_closed"
        );
        assert!(packet.canary_start_blocked);
        assert!(!packet.canary_start_allowed);
        assert!(!packet.canary_start_switch_enabled);
        assert_eq!(
            packet.side_effects,
            StatusCanaryEvidencePacketSideEffects::none()
        );
    }

    #[test]
    fn packet_overlay_treats_recorded_and_waived_as_complete_without_side_effects() {
        let decisions = controlled_live_required_evidence_collection_plan_report()
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                StatusCanaryEvidenceDecisionOverride::with_decision(
                    entry.source_blocker_id,
                    if index < 4 {
                        StatusCanaryEvidenceDecision::Recorded
                    } else {
                        StatusCanaryEvidenceDecision::Waived
                    },
                )
            })
            .collect::<Vec<_>>();
        let packet = status_canary_evidence_packet_from_decisions(&decisions);

        assert!(packet.packet_ready);
        assert!(packet.evidence_complete);
        assert_eq!(packet.missing_item_count, 0);
        assert_eq!(packet.recorded_item_count, 4);
        assert_eq!(packet.waived_item_count, 3);
        assert_eq!(packet.expired_item_count, 0);
        assert_eq!(packet.invalid_item_count, 0);
        assert_eq!(packet.decision_reason_audit_count, 0);
        assert_eq!(packet.decision_reason_audit_ready_count, 0);
        assert_eq!(packet.decision_reason_audit_rejected_count, 0);
        assert_eq!(packet.action_required_count, 0);
        assert_eq!(
            packet.guard_route,
            "status_canary_evidence_packet_ready_start_switch_closed"
        );
        assert!(packet.entries.iter().all(|entry| {
            !entry.action_required
                && !entry.canary_start_blocked
                && (entry.evidence_recorded || entry.evidence_waived)
                && !entry.evidence_expired
                && !entry.evidence_invalid
        }));
        assert_eq!(
            packet.side_effects,
            StatusCanaryEvidencePacketSideEffects::none()
        );
    }

    #[test]
    fn packet_overlay_blocks_on_expired_and_invalid_evidence_without_recording() {
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

        assert!(packet.packet_ready);
        assert!(!packet.evidence_complete);
        assert_eq!(packet.missing_item_count, 0);
        assert_eq!(packet.recorded_item_count, 2);
        assert_eq!(packet.waived_item_count, 2);
        assert_eq!(packet.expired_item_count, 2);
        assert_eq!(packet.invalid_item_count, 1);
        assert_eq!(packet.decision_reason_audit_count, 0);
        assert_eq!(packet.decision_reason_audit_ready_count, 0);
        assert_eq!(packet.decision_reason_audit_rejected_count, 0);
        assert_eq!(packet.action_required_count, 3);
        assert_eq!(
            packet.guard_route,
            "status_canary_evidence_packet_blocked_expired_evidence"
        );
        assert!(packet.canary_start_blocked);
        assert!(!packet.canary_start_allowed);
        assert_eq!(
            packet
                .entries
                .iter()
                .filter(|entry| entry.evidence_expired || entry.evidence_invalid)
                .count(),
            3
        );
        assert!(packet.entries.iter().all(|entry| {
            !entry.evidence_recording_allowed
                && !entry.waiver_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        }));
        assert_eq!(
            packet.side_effects,
            StatusCanaryEvidencePacketSideEffects::none()
        );
    }

    #[test]
    fn reason_bound_decision_overlay_without_ready_audit_fails_closed_as_missing() {
        let mut decision =
            StatusCanaryEvidenceDecisionOverride::recorded("dirty_worktree_boundary");
        decision.source_acceptance_packet_bound = true;
        decision.source_acceptance_request_source_validator_bound = true;
        decision.source_acceptance_request_reason_audit_bound = true;
        decision.source_acceptance_request_reason_audit_ready = false;
        decision.source_readback_reason_packet_route =
            "status_canary_evidence_source_reason_packet_ready_inputs_valid";
        decision.source_readback_fixture_reason_audit_rejection_reason =
            "source_adapter_input_missing_for_decision";

        let packet = status_canary_evidence_packet_from_decisions(&[decision]);

        assert!(packet.packet_ready);
        assert!(!packet.evidence_complete);
        assert_eq!(packet.missing_item_count, 7);
        assert_eq!(packet.recorded_item_count, 0);
        assert_eq!(packet.decision_reason_audit_count, 1);
        assert_eq!(packet.decision_reason_audit_ready_count, 0);
        assert_eq!(packet.decision_reason_audit_rejected_count, 1);
        assert_eq!(
            packet.guard_route,
            "status_canary_evidence_packet_blocked_missing_evidence"
        );
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.source_acceptance_packet_bound
                && entry.source_acceptance_request_reason_audit_bound
                && !entry.decision_reason_audit_ready
                && entry.decision == StatusCanaryEvidenceDecision::Missing
        }));
    }
}
