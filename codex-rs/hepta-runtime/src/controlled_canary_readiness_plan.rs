use serde::Serialize;

use crate::ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentKillSwitchRehearsalBoundaryReadbackReport;
use crate::HeptaSystemStatusOperatorApprovalProtocolReport;
use crate::controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_report;
use crate::hepta_system_status_operator_approval_protocol_report;
use crate::status_canary_start_guard::STATUS_CANARY_START_GUARD_ID;
use crate::status_canary_start_guard::StatusCanaryStartGuardPlan;
use crate::status_canary_start_guard::StatusCanaryStartGuardSideEffects;
use crate::status_canary_start_guard::status_canary_start_guard;

pub const CONTROLLED_CANARY_READINESS_PLAN_GATE: &str = "controlled_canary_readiness_plan_gate";
pub const CONTROLLED_CANARY_READINESS_PLAN_SCHEMA_VERSION: &str =
    "controlled_canary_readiness_plan_v1";
pub const CONTROLLED_CANARY_READINESS_PLAN_RECOMMENDED_NEXT_GATE: &str =
    "phase11_dirty_worktree_release_boundary_inventory_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledCanaryReadinessPlanReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_approval_protocol_gate: &'static str,
    pub source_approval_protocol_ready: bool,
    pub source_approval_packet_count: usize,
    pub source_approval_request_sent: bool,
    pub source_approval_accepted: bool,
    pub source_approval_recorded: bool,
    pub source_boundary_gate: &'static str,
    pub source_boundary_ready: bool,
    pub source_boundary_entry_count: usize,
    pub source_boundary_ready_count: usize,
    pub source_boundary_missing_evidence_count: usize,
    pub canary_scope: ControlledCanaryReadinessScope,
    pub canary_plan_entry_count: usize,
    pub canary_plan_ready_count: usize,
    pub blocker_reference_count: usize,
    pub missing_blocker_count: usize,
    pub approval_protocol_bound: bool,
    pub operator_readback_bound: bool,
    pub dirty_worktree_blocker_preserved: bool,
    pub soak_readback_required: bool,
    pub rollback_rehearsal_required: bool,
    pub kill_switch_rehearsal_required: bool,
    pub gateway_native_telegram_boundary_closed: bool,
    pub credential_boundary_closed: bool,
    pub persistence_boundary_closed: bool,
    pub status_canary_start_guard_bound: bool,
    pub status_canary_start_guard_id: &'static str,
    pub status_canary_start_guard_route: &'static str,
    pub status_canary_start_guard_switch_enabled: bool,
    pub status_canary_start_guard_evidence_complete: bool,
    pub status_canary_start_guard_missing_evidence_count: usize,
    pub status_canary_start_guard_evidence_packet_reason_audit_count: usize,
    pub status_canary_start_guard_evidence_packet_reason_audit_ready_count: usize,
    pub status_canary_start_guard_evidence_packet_reason_audit_rejected_count: usize,
    pub status_canary_start_guard_evidence_packet_reason_audit_ready: bool,
    pub status_canary_start_guard_side_effects_closed: bool,
    pub status_canary_start_guard_blocked: bool,
    pub status_canary_start_guard_allowed: bool,
    pub controlled_canary_readiness_plan_ready: bool,
    pub controlled_canary_activation_ready: bool,
    pub approval_request_sent: bool,
    pub approval_request_allowed: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub approval_broker_write_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub credential_read_allowed: bool,
    pub gateway_or_auth_mutation_allowed: bool,
    pub native_post_mutation_allowed: bool,
    pub telegram_transport_mutation_allowed: bool,
    pub channel_send_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub canary_persistence_allowed: bool,
    pub canary_receipt_persisted: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub provider_invocation_allowed: bool,
    pub model_invocation_allowed: bool,
    pub package_or_release_allowed: bool,
    pub public_ga_allowed: bool,
    pub live_activation_allowed: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<ControlledCanaryReadinessPlanEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledCanaryReadinessPlanSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledCanaryReadinessScope {
    pub canary_id: &'static str,
    pub scope_route: &'static str,
    pub approval_subject: &'static str,
    pub source_packet_id: &'static str,
    pub selected_candidate_tool_id: &'static str,
    pub allowed_surface: &'static str,
    pub activation_mode: &'static str,
    pub gateway_auth_boundary: &'static str,
    pub native_post_boundary: &'static str,
    pub telegram_transport_boundary: &'static str,
    pub channel_send_boundary: &'static str,
    pub credential_boundary: &'static str,
    pub persistence_boundary: &'static str,
    pub live_activation_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledCanaryReadinessPlanEntry {
    pub source_blocker_id: &'static str,
    pub plan_key: &'static str,
    pub plan_route: &'static str,
    pub source_boundary_route: &'static str,
    pub owner: &'static str,
    pub risk_bucket: &'static str,
    pub operator_status: &'static str,
    pub evidence_state: &'static str,
    pub required_canary_evidence: &'static str,
    pub readiness_state: &'static str,
    pub operator_visible: bool,
    pub queryable: bool,
    pub canary_plan_ready: bool,
    pub blocks_canary_activation: bool,
    pub evidence_recording_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub waiver_allowed: bool,
    pub credential_read_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub persistence_allowed: bool,
    pub live_activation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledCanaryReadinessPlanSideEffects {
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub approval_broker_written: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub blocker_waived: bool,
    pub credential_read: bool,
    pub gateway_or_auth_mutated: bool,
    pub native_post_mutation_performed: bool,
    pub telegram_transport_mutated: bool,
    pub channel_send_performed: bool,
    pub packet_persisted: bool,
    pub canary_receipt_persisted: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_activation_started: bool,
    pub live_execution_started: bool,
}

pub fn controlled_canary_readiness_plan_report() -> ControlledCanaryReadinessPlanReport {
    let approval = hepta_system_status_operator_approval_protocol_report();
    let boundary =
        controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_report();
    let start_guard = status_canary_start_guard();
    controlled_canary_readiness_plan_report_from_sources_and_start_guard(
        &approval,
        &boundary,
        &start_guard,
    )
}

pub fn controlled_canary_readiness_plan_report_from_sources(
    approval: &HeptaSystemStatusOperatorApprovalProtocolReport,
    boundary: &ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentKillSwitchRehearsalBoundaryReadbackReport,
) -> ControlledCanaryReadinessPlanReport {
    let start_guard = status_canary_start_guard();
    controlled_canary_readiness_plan_report_from_sources_and_start_guard(
        approval,
        boundary,
        &start_guard,
    )
}

pub fn controlled_canary_readiness_plan_report_from_sources_and_start_guard(
    approval: &HeptaSystemStatusOperatorApprovalProtocolReport,
    boundary: &ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentKillSwitchRehearsalBoundaryReadbackReport,
    start_guard: &StatusCanaryStartGuardPlan,
) -> ControlledCanaryReadinessPlanReport {
    let canary_scope = controlled_canary_readiness_scope(approval);
    let entries = controlled_canary_readiness_plan_entries(boundary);
    let canary_plan_ready_count = entries
        .iter()
        .filter(|entry| entry.canary_plan_ready)
        .count();
    let missing_blocker_count = entries
        .iter()
        .filter(|entry| entry.evidence_state == "missing")
        .count();
    let dirty_worktree_blocker_preserved = entries
        .iter()
        .any(|entry| entry.source_blocker_id == "dirty_worktree_boundary");
    let soak_readback_required = entries
        .iter()
        .any(|entry| entry.source_blocker_id == "fresh_soak_readback_missing");
    let rollback_rehearsal_required = entries
        .iter()
        .any(|entry| entry.source_blocker_id == "rollback_rehearsal_missing");
    let kill_switch_rehearsal_required = entries
        .iter()
        .any(|entry| entry.source_blocker_id == "kill_switch_rehearsal_missing");
    let gateway_native_telegram_boundary_closed = entries.iter().any(|entry| {
        entry.source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing"
    }) && !boundary.transport_mutation_allowed;
    let credential_boundary_closed = entries
        .iter()
        .any(|entry| entry.source_blocker_id == "credential_boundary_attestation_missing")
        && !boundary.credential_read_allowed;
    let persistence_boundary_closed = !boundary.packet_persisted
        && !boundary.attachment_persisted
        && !boundary.readback_persisted;
    let status_canary_start_guard_side_effects_closed =
        start_guard.side_effects == StatusCanaryStartGuardSideEffects::none();
    let status_canary_start_guard_evidence_packet_reason_audit_ready = start_guard
        .source_evidence_packet_reason_audit_ready
        && start_guard.source_evidence_packet_decision_reason_audit_count
            == start_guard.source_evidence_packet_decision_reason_audit_ready_count
        && start_guard.source_evidence_packet_decision_reason_audit_rejected_count == 0;
    let status_canary_start_guard_bound = start_guard.guard_id == STATUS_CANARY_START_GUARD_ID
        && start_guard.source_evidence_packet_ready
        && !start_guard.source_evidence_packet_complete
        && start_guard.source_evidence_packet_missing_count == 7
        && status_canary_start_guard_evidence_packet_reason_audit_ready
        && !start_guard.canary_start_switch_enabled
        && start_guard.canary_start_blocked
        && !start_guard.canary_start_allowed
        && start_guard.guard_route == "status_canary_start_blocked_missing_evidence_packet"
        && status_canary_start_guard_side_effects_closed;

    let controlled_canary_readiness_plan_ready = approval.approval_protocol_ready
        && approval.approval_packet_count == 1
        && approval.explicit_accept_required
        && approval.non_acceptance_receipt_projected
        && !approval.approval_request_sent
        && !approval.approval_accepted
        && !approval.approval_recorded
        && !approval.approval_broker_write_allowed
        && !approval.receipt_persisted
        && !approval.credential_read_allowed
        && !approval.external_network_allowed
        && !approval.ledger_write_allowed
        && !approval.transport_mutation_allowed
        && !approval.live_execution_allowed
        && boundary.kill_switch_rehearsal_boundary_readback_ready
        && boundary.kill_switch_rehearsal_boundary_entry_count == 7
        && boundary.kill_switch_rehearsal_boundary_ready_count == 7
        && boundary.kill_switch_rehearsal_evidence_missing_count == 7
        && !boundary.approval_request_sent
        && !boundary.approval_accepted
        && boundary.blocker_waived_count == 0
        && boundary.evidence_recorded_count == 0
        && entries.len() == 7
        && canary_plan_ready_count == 7
        && missing_blocker_count == 7
        && dirty_worktree_blocker_preserved
        && soak_readback_required
        && rollback_rehearsal_required
        && kill_switch_rehearsal_required
        && gateway_native_telegram_boundary_closed
        && credential_boundary_closed
        && persistence_boundary_closed
        && status_canary_start_guard_bound
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.blocks_canary_activation
                && !entry.evidence_recording_allowed
                && !entry.approval_acceptance_allowed
                && !entry.waiver_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_activation_allowed
        });

    ControlledCanaryReadinessPlanReport {
        runtime: "hepta",
        surface: "controlled_canary_readiness_plan",
        status: if controlled_canary_readiness_plan_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_CANARY_READINESS_PLAN_GATE,
        schema_version: CONTROLLED_CANARY_READINESS_PLAN_SCHEMA_VERSION,
        plugin_id: approval.plugin_id,
        source_approval_protocol_gate: approval.gate,
        source_approval_protocol_ready: approval.approval_protocol_ready,
        source_approval_packet_count: approval.approval_packet_count,
        source_approval_request_sent: approval.approval_request_sent,
        source_approval_accepted: approval.approval_accepted,
        source_approval_recorded: approval.approval_recorded,
        source_boundary_gate: boundary.gate,
        source_boundary_ready: boundary.kill_switch_rehearsal_boundary_readback_ready,
        source_boundary_entry_count: boundary.kill_switch_rehearsal_boundary_entry_count,
        source_boundary_ready_count: boundary.kill_switch_rehearsal_boundary_ready_count,
        source_boundary_missing_evidence_count: boundary
            .kill_switch_rehearsal_evidence_missing_count,
        canary_scope,
        canary_plan_entry_count: entries.len(),
        canary_plan_ready_count,
        blocker_reference_count: entries.len(),
        missing_blocker_count,
        approval_protocol_bound: approval.approval_protocol_ready,
        operator_readback_bound: boundary.kill_switch_rehearsal_boundary_readback_ready,
        dirty_worktree_blocker_preserved,
        soak_readback_required,
        rollback_rehearsal_required,
        kill_switch_rehearsal_required,
        gateway_native_telegram_boundary_closed,
        credential_boundary_closed,
        persistence_boundary_closed,
        status_canary_start_guard_bound,
        status_canary_start_guard_id: start_guard.guard_id,
        status_canary_start_guard_route: start_guard.guard_route,
        status_canary_start_guard_switch_enabled: start_guard.canary_start_switch_enabled,
        status_canary_start_guard_evidence_complete: start_guard.source_evidence_packet_complete,
        status_canary_start_guard_missing_evidence_count: start_guard
            .source_evidence_packet_missing_count,
        status_canary_start_guard_evidence_packet_reason_audit_count: start_guard
            .source_evidence_packet_decision_reason_audit_count,
        status_canary_start_guard_evidence_packet_reason_audit_ready_count: start_guard
            .source_evidence_packet_decision_reason_audit_ready_count,
        status_canary_start_guard_evidence_packet_reason_audit_rejected_count: start_guard
            .source_evidence_packet_decision_reason_audit_rejected_count,
        status_canary_start_guard_evidence_packet_reason_audit_ready,
        status_canary_start_guard_side_effects_closed,
        status_canary_start_guard_blocked: start_guard.canary_start_blocked,
        status_canary_start_guard_allowed: start_guard.canary_start_allowed,
        controlled_canary_readiness_plan_ready,
        controlled_canary_activation_ready: false,
        approval_request_sent: false,
        approval_request_allowed: false,
        approval_accepted: false,
        approval_recorded: false,
        approval_broker_write_allowed: false,
        evidence_recording_allowed: false,
        credential_read_allowed: false,
        gateway_or_auth_mutation_allowed: false,
        native_post_mutation_allowed: false,
        telegram_transport_mutation_allowed: false,
        channel_send_allowed: false,
        transport_mutation_allowed: false,
        canary_persistence_allowed: false,
        canary_receipt_persisted: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        provider_invocation_allowed: false,
        model_invocation_allowed: false,
        package_or_release_allowed: false,
        public_ga_allowed: false,
        live_activation_allowed: false,
        live_execution_allowed: false,
        entries,
        blockers: vec![
            "dirty_worktree_release_boundary_unresolved",
            "operator_live_approval_missing",
            "fresh_soak_readback_missing",
            "credential_boundary_attestation_missing",
            "gateway_native_telegram_post_boundary_approval_missing",
            "rollback_rehearsal_missing",
            "kill_switch_rehearsal_missing",
            "canary_activation_blocked",
            "gateway_native_telegram_boundaries_closed",
            "credential_boundary_closed",
            "persistence_boundary_closed",
            "status_canary_start_guard_blocked",
            "live_activation_closed",
        ],
        recommended_next_gate: CONTROLLED_CANARY_READINESS_PLAN_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledCanaryReadinessPlanSideEffects::none(),
    }
}

pub fn controlled_canary_readiness_scope(
    approval: &HeptaSystemStatusOperatorApprovalProtocolReport,
) -> ControlledCanaryReadinessScope {
    ControlledCanaryReadinessScope {
        canary_id: "controlled-canary.hepta-system-status.internal-read-only.v1",
        scope_route: "canary://hepta-system/status/internal-read-only/readiness-plan",
        approval_subject: approval.approval_subject,
        source_packet_id: approval.packet.packet_id,
        selected_candidate_tool_id: approval.selected_candidate_tool_id,
        allowed_surface: "internal_read_only_status_payload",
        activation_mode: "plan_only_no_activation",
        gateway_auth_boundary: "closed",
        native_post_boundary: "closed",
        telegram_transport_boundary: "closed",
        channel_send_boundary: "closed",
        credential_boundary: "closed",
        persistence_boundary: "closed",
        live_activation_boundary: "closed",
    }
}

pub fn controlled_canary_readiness_plan_entries(
    boundary: &ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentKillSwitchRehearsalBoundaryReadbackReport,
) -> Vec<ControlledCanaryReadinessPlanEntry> {
    boundary
        .entries
        .iter()
        .map(|entry| ControlledCanaryReadinessPlanEntry {
            source_blocker_id: entry.source_blocker_id,
            plan_key: canary_plan_key(entry.source_blocker_id),
            plan_route: canary_plan_route(entry.source_blocker_id),
            source_boundary_route: entry.kill_switch_rehearsal_boundary_route,
            owner: entry.owner,
            risk_bucket: entry.risk_bucket,
            operator_status: entry.operator_status,
            evidence_state: entry.kill_switch_rehearsal_evidence_state,
            required_canary_evidence: entry.required_evidence,
            readiness_state: "missing_required_evidence",
            operator_visible: entry.operator_visible,
            queryable: entry.kill_switch_rehearsal_boundary_readback_visible,
            canary_plan_ready: entry.operator_visible
                && entry.kill_switch_rehearsal_boundary_readback_visible,
            blocks_canary_activation: true,
            evidence_recording_allowed: false,
            approval_acceptance_allowed: false,
            waiver_allowed: false,
            credential_read_allowed: false,
            transport_mutation_allowed: false,
            persistence_allowed: false,
            live_activation_allowed: false,
        })
        .collect()
}

fn canary_plan_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "controlled_canary.plan.dirty_worktree_boundary",
        "operator_live_approval_missing" => "controlled_canary.plan.operator_live_approval_missing",
        "fresh_soak_readback_missing" => "controlled_canary.plan.fresh_soak_readback_missing",
        "credential_boundary_attestation_missing" => {
            "controlled_canary.plan.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_canary.plan.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => "controlled_canary.plan.rollback_rehearsal_missing",
        "kill_switch_rehearsal_missing" => "controlled_canary.plan.kill_switch_rehearsal_missing",
        _ => "controlled_canary.plan.unknown",
    }
}

fn canary_plan_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "canary://hepta-system/status/readiness/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "canary://hepta-system/status/readiness/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "canary://hepta-system/status/readiness/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "canary://hepta-system/status/readiness/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "canary://hepta-system/status/readiness/gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "canary://hepta-system/status/readiness/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "canary://hepta-system/status/readiness/kill-switch-rehearsal-missing"
        }
        _ => "canary://hepta-system/status/readiness/unknown",
    }
}

impl ControlledCanaryReadinessPlanSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            approval_broker_written: false,
            evidence_recorded: false,
            evidence_persisted: false,
            blocker_waived: false,
            credential_read: false,
            gateway_or_auth_mutated: false,
            native_post_mutation_performed: false,
            telegram_transport_mutated: false,
            channel_send_performed: false,
            packet_persisted: false,
            canary_receipt_persisted: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            provider_invoked: false,
            model_invoked: false,
            package_or_release_written: false,
            public_ga_promoted: false,
            live_activation_started: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canary_readiness_plan_binds_approval_protocol_and_boundary() {
        let report = controlled_canary_readiness_plan_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_approval_protocol_ready);
        assert_eq!(report.source_approval_packet_count, 1);
        assert!(report.source_boundary_ready);
        assert_eq!(report.source_boundary_entry_count, 7);
        assert_eq!(report.canary_plan_entry_count, 7);
        assert_eq!(report.canary_plan_ready_count, 7);
        assert!(report.approval_protocol_bound);
        assert!(report.operator_readback_bound);
        assert!(report.status_canary_start_guard_bound);
        assert_eq!(
            report.status_canary_start_guard_id,
            STATUS_CANARY_START_GUARD_ID
        );
        assert_eq!(
            report.status_canary_start_guard_route,
            "status_canary_start_blocked_missing_evidence_packet"
        );
        assert_eq!(report.status_canary_start_guard_missing_evidence_count, 7);
        assert_eq!(
            report.status_canary_start_guard_evidence_packet_reason_audit_count,
            0
        );
        assert_eq!(
            report.status_canary_start_guard_evidence_packet_reason_audit_ready_count,
            0
        );
        assert_eq!(
            report.status_canary_start_guard_evidence_packet_reason_audit_rejected_count,
            0
        );
        assert!(report.status_canary_start_guard_evidence_packet_reason_audit_ready);
        assert_eq!(
            report.canary_scope.activation_mode,
            "plan_only_no_activation"
        );
    }

    #[test]
    fn canary_readiness_plan_preserves_required_blockers() {
        let report = controlled_canary_readiness_plan_report();

        assert!(report.dirty_worktree_blocker_preserved);
        assert!(report.soak_readback_required);
        assert!(report.rollback_rehearsal_required);
        assert!(report.kill_switch_rehearsal_required);
        assert!(report.gateway_native_telegram_boundary_closed);
        assert!(report.credential_boundary_closed);
        assert!(report.persistence_boundary_closed);
        assert!(report.status_canary_start_guard_side_effects_closed);
        assert!(report.status_canary_start_guard_evidence_packet_reason_audit_ready);
        assert!(report.status_canary_start_guard_blocked);
        assert!(!report.status_canary_start_guard_allowed);
        assert_eq!(report.missing_blocker_count, 7);
        assert!(report.entries.iter().all(|entry| {
            entry.blocks_canary_activation
                && entry.readiness_state == "missing_required_evidence"
                && entry.evidence_state == "missing"
        }));
    }

    #[test]
    fn canary_readiness_plan_keeps_activation_transport_and_persistence_closed() {
        let report = controlled_canary_readiness_plan_report();

        assert!(report.controlled_canary_readiness_plan_ready);
        assert!(!report.controlled_canary_activation_ready);
        assert!(!report.status_canary_start_guard_switch_enabled);
        assert!(!report.status_canary_start_guard_evidence_complete);
        assert!(!report.status_canary_start_guard_allowed);
        assert!(!report.approval_request_sent);
        assert!(!report.approval_accepted);
        assert!(!report.approval_recorded);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.gateway_or_auth_mutation_allowed);
        assert!(!report.native_post_mutation_allowed);
        assert!(!report.telegram_transport_mutation_allowed);
        assert!(!report.channel_send_allowed);
        assert!(!report.transport_mutation_allowed);
        assert!(!report.canary_persistence_allowed);
        assert!(!report.canary_receipt_persisted);
        assert!(!report.package_or_release_allowed);
        assert!(!report.public_ga_allowed);
        assert!(!report.live_activation_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledCanaryReadinessPlanSideEffects::none()
        );
    }
}
