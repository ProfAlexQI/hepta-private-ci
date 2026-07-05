use crate::controlled_live_required_evidence_collection_plan::ControlledLiveRequiredEvidenceCollectionPlanEntry;
use crate::controlled_live_required_evidence_collection_plan::controlled_live_required_evidence_collection_plan_report;
use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;
use crate::status_canary_evidence_packet::StatusCanaryEvidenceDecision;
use crate::status_canary_evidence_packet::StatusCanaryEvidenceDecisionOverride;
use crate::status_canary_evidence_packet::status_canary_evidence_packet_from_decisions;
use serde::Serialize;

pub const STATUS_CANARY_EVIDENCE_ACCEPTANCE_PACKET_SCHEMA_VERSION: &str =
    "status_canary_evidence_acceptance_packet_v1";
pub const STATUS_CANARY_EVIDENCE_ACCEPTANCE_PACKET_ID: &str =
    "status-canary-evidence-acceptance-packet/hepta-system-status/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceAcceptancePacket {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub schema_version: &'static str,
    pub packet_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_required_evidence_collection_plan_ready: bool,
    pub checklist_item_count: usize,
    pub request_count: usize,
    pub known_request_count: usize,
    pub unknown_request_count: usize,
    pub duplicate_request_count: usize,
    pub request_source_validator_bound_count: usize,
    pub request_source_validator_contract_audit_ready_count: usize,
    pub request_reason_audit_count: usize,
    pub request_reason_audit_ready_count: usize,
    pub request_reason_audit_rejected_count: usize,
    pub accepted_decision_count: usize,
    pub rejected_decision_count: usize,
    pub generated_override_count: usize,
    pub generated_override_reason_audit_ready_count: usize,
    pub generated_missing_decision_count: usize,
    pub generated_recorded_decision_count: usize,
    pub generated_waived_decision_count: usize,
    pub generated_expired_decision_count: usize,
    pub generated_invalid_decision_count: usize,
    pub source_evidence_packet_ready: bool,
    pub source_evidence_packet_complete: bool,
    pub source_evidence_packet_missing_count: usize,
    pub source_evidence_packet_recorded_count: usize,
    pub source_evidence_packet_waived_count: usize,
    pub source_evidence_packet_expired_count: usize,
    pub source_evidence_packet_invalid_count: usize,
    pub source_evidence_packet_guard_route: &'static str,
    pub acceptance_packet_ready: bool,
    pub default_missing_decisions: bool,
    pub acceptance_packet_route: &'static str,
    pub entries: Vec<StatusCanaryEvidenceAcceptancePacketEntry>,
    pub decision_overrides: Vec<StatusCanaryEvidenceDecisionOverride>,
    pub side_effects: StatusCanaryEvidenceAcceptancePacketSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceAcceptancePacketEntry {
    pub source_blocker_id: &'static str,
    pub packet_key: &'static str,
    pub packet_route: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub request_present: bool,
    pub duplicate_request: bool,
    pub requested_decision: StatusCanaryEvidenceDecision,
    pub generated_decision: StatusCanaryEvidenceDecision,
    pub evidence_state: &'static str,
    pub decision_source_route: &'static str,
    pub decision_source_route_present: bool,
    pub evidence_artifact_present: bool,
    pub evidence_digest_present: bool,
    pub operator_authority_present: bool,
    pub freshness_attested: bool,
    pub waiver_reason_present: bool,
    pub expiry_attested: bool,
    pub invalidity_reason_present: bool,
    pub source_validator_bound: bool,
    pub source_validator_contract_audit_ready: bool,
    pub source_validator_reason_audit_bound: bool,
    pub source_validator_reason_audit_ready: bool,
    pub source_readback_reason_packet_route: &'static str,
    pub source_readback_fixture_reason_audit_rejection_reason: &'static str,
    pub request_reason_audit_ready: bool,
    pub decision_source_valid: bool,
    pub decision_accepted: bool,
    pub decision_rejected: bool,
    pub rejection_reason: &'static str,
    pub generated_override: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub selected_status_canary_bound: bool,
    pub preflight_only_connector_excluded: bool,
    pub evidence_recording_allowed: bool,
    pub waiver_recording_allowed: bool,
    pub credential_read_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceDecisionRequest {
    pub source_blocker_id: &'static str,
    pub requested_decision: StatusCanaryEvidenceDecision,
    pub decision_source_route: &'static str,
    pub evidence_artifact_present: bool,
    pub evidence_digest_present: bool,
    pub operator_authority_present: bool,
    pub freshness_attested: bool,
    pub waiver_reason_present: bool,
    pub expiry_attested: bool,
    pub invalidity_reason_present: bool,
    pub source_validator_bound: bool,
    pub source_validator_contract_audit_ready: bool,
    pub source_validator_reason_audit_bound: bool,
    pub source_validator_reason_audit_ready: bool,
    pub source_readback_reason_packet_route: &'static str,
    pub source_readback_fixture_reason_audit_rejection_reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceAcceptancePacketSideEffects {
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
    pub tool_invoked: bool,
    pub connector_started: bool,
    pub canary_started: bool,
    pub live_execution_started: bool,
}

pub fn status_canary_evidence_acceptance_packet() -> StatusCanaryEvidenceAcceptancePacket {
    status_canary_evidence_acceptance_packet_from_requests(&[])
}

pub fn status_canary_evidence_acceptance_packet_from_requests(
    requests: &[StatusCanaryEvidenceDecisionRequest],
) -> StatusCanaryEvidenceAcceptancePacket {
    let source = controlled_live_required_evidence_collection_plan_report();
    status_canary_evidence_acceptance_packet_from_plan(
        source.evidence_collection_plan_ready,
        source.entries,
        requests,
    )
}

pub fn status_canary_evidence_acceptance_packet_from_plan(
    source_required_evidence_collection_plan_ready: bool,
    source_entries: Vec<ControlledLiveRequiredEvidenceCollectionPlanEntry>,
    requests: &[StatusCanaryEvidenceDecisionRequest],
) -> StatusCanaryEvidenceAcceptancePacket {
    let known_request_count = requests
        .iter()
        .filter(|request| {
            source_entries
                .iter()
                .any(|entry| entry.source_blocker_id == request.source_blocker_id)
        })
        .count();
    let unknown_request_count = requests.len().saturating_sub(known_request_count);
    let entries = source_entries
        .into_iter()
        .map(|entry| status_canary_evidence_acceptance_packet_entry(entry, requests))
        .collect::<Vec<_>>();
    let duplicate_request_count = entries
        .iter()
        .filter(|entry| entry.duplicate_request)
        .count();
    let request_source_validator_bound_count = requests
        .iter()
        .filter(|request| request.source_validator_bound)
        .count();
    let request_source_validator_contract_audit_ready_count = requests
        .iter()
        .filter(|request| {
            request.source_validator_bound && request.source_validator_contract_audit_ready
        })
        .count();
    let request_reason_audit_count = requests
        .iter()
        .filter(|request| request.source_validator_reason_audit_bound)
        .count();
    let request_reason_audit_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.request_present
                && entry.source_validator_reason_audit_bound
                && entry.request_reason_audit_ready
        })
        .count();
    let request_reason_audit_rejected_count = entries
        .iter()
        .filter(|entry| {
            entry.request_present
                && entry.source_validator_reason_audit_bound
                && !entry.request_reason_audit_ready
        })
        .count();
    let accepted_decision_count = entries
        .iter()
        .filter(|entry| entry.decision_accepted)
        .count();
    let rejected_decision_count = entries
        .iter()
        .filter(|entry| entry.decision_rejected)
        .count();
    let decision_overrides = entries
        .iter()
        .filter(|entry| entry.generated_override)
        .map(|entry| StatusCanaryEvidenceDecisionOverride {
            source_blocker_id: entry.source_blocker_id,
            decision: entry.generated_decision,
            source_acceptance_packet_bound: true,
            source_acceptance_request_source_validator_bound: entry.source_validator_bound,
            source_acceptance_request_reason_audit_bound: entry.source_validator_reason_audit_bound,
            source_acceptance_request_reason_audit_ready: entry.request_reason_audit_ready,
            source_readback_reason_packet_route: entry.source_readback_reason_packet_route,
            source_readback_fixture_reason_audit_rejection_reason: entry
                .source_readback_fixture_reason_audit_rejection_reason,
        })
        .collect::<Vec<_>>();
    let generated_override_reason_audit_ready_count = decision_overrides
        .iter()
        .filter(|decision| {
            decision.source_acceptance_request_reason_audit_bound
                && decision.source_acceptance_request_reason_audit_ready
        })
        .count();
    let source_evidence_packet = status_canary_evidence_packet_from_decisions(&decision_overrides);
    let side_effects = StatusCanaryEvidenceAcceptancePacketSideEffects::none();
    let acceptance_packet_ready = source_required_evidence_collection_plan_ready
        && entries.len() == 7
        && unknown_request_count == 0
        && duplicate_request_count == 0
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.selected_status_canary_bound
                && entry.preflight_only_connector_excluded
                && !entry.evidence_recording_allowed
                && !entry.waiver_recording_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        })
        && side_effects == StatusCanaryEvidenceAcceptancePacketSideEffects::none();

    StatusCanaryEvidenceAcceptancePacket {
        runtime: "hepta",
        surface: "status_canary_evidence_acceptance_packet",
        schema_version: STATUS_CANARY_EVIDENCE_ACCEPTANCE_PACKET_SCHEMA_VERSION,
        packet_id: STATUS_CANARY_EVIDENCE_ACCEPTANCE_PACKET_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_required_evidence_collection_plan_ready,
        checklist_item_count: entries.len(),
        request_count: requests.len(),
        known_request_count,
        unknown_request_count,
        duplicate_request_count,
        request_source_validator_bound_count,
        request_source_validator_contract_audit_ready_count,
        request_reason_audit_count,
        request_reason_audit_ready_count,
        request_reason_audit_rejected_count,
        accepted_decision_count,
        rejected_decision_count,
        generated_override_count: decision_overrides.len(),
        generated_override_reason_audit_ready_count,
        generated_missing_decision_count: source_evidence_packet.missing_item_count,
        generated_recorded_decision_count: source_evidence_packet.recorded_item_count,
        generated_waived_decision_count: source_evidence_packet.waived_item_count,
        generated_expired_decision_count: source_evidence_packet.expired_item_count,
        generated_invalid_decision_count: source_evidence_packet.invalid_item_count,
        source_evidence_packet_ready: source_evidence_packet.packet_ready,
        source_evidence_packet_complete: source_evidence_packet.evidence_complete,
        source_evidence_packet_missing_count: source_evidence_packet.missing_item_count,
        source_evidence_packet_recorded_count: source_evidence_packet.recorded_item_count,
        source_evidence_packet_waived_count: source_evidence_packet.waived_item_count,
        source_evidence_packet_expired_count: source_evidence_packet.expired_item_count,
        source_evidence_packet_invalid_count: source_evidence_packet.invalid_item_count,
        source_evidence_packet_guard_route: source_evidence_packet.guard_route,
        acceptance_packet_ready,
        default_missing_decisions: requests.is_empty()
            && decision_overrides.is_empty()
            && source_evidence_packet.missing_item_count == 7,
        acceptance_packet_route: status_canary_evidence_acceptance_packet_route(
            source_required_evidence_collection_plan_ready,
            requests.len(),
            unknown_request_count,
            duplicate_request_count,
            rejected_decision_count,
            accepted_decision_count,
        ),
        entries,
        decision_overrides,
        side_effects,
    }
}

fn status_canary_evidence_acceptance_packet_entry(
    source: ControlledLiveRequiredEvidenceCollectionPlanEntry,
    requests: &[StatusCanaryEvidenceDecisionRequest],
) -> StatusCanaryEvidenceAcceptancePacketEntry {
    let matching_requests = requests
        .iter()
        .filter(|request| request.source_blocker_id == source.source_blocker_id)
        .collect::<Vec<_>>();
    let duplicate_request = matching_requests.len() > 1;
    let request = matching_requests.first().copied().copied();
    let validation = request
        .map(|request| status_canary_evidence_decision_validation(request, duplicate_request))
        .unwrap_or(StatusCanaryEvidenceDecisionValidation {
            valid: false,
            rejection_reason: "no_decision_request",
        });
    let requested_decision = request
        .map(|request| request.requested_decision)
        .unwrap_or(StatusCanaryEvidenceDecision::Missing);
    let request_reason_audit_ready = request
        .map(status_canary_evidence_decision_request_reason_audit_ready)
        .unwrap_or(false);
    let decision_accepted = request.is_some() && validation.valid;
    let decision_rejected = request.is_some() && !validation.valid;
    let generated_decision = if decision_accepted {
        requested_decision
    } else {
        StatusCanaryEvidenceDecision::Missing
    };

    StatusCanaryEvidenceAcceptancePacketEntry {
        source_blocker_id: source.source_blocker_id,
        packet_key: status_canary_evidence_acceptance_packet_key(source.source_blocker_id),
        packet_route: status_canary_evidence_acceptance_packet_entry_route(
            source.source_blocker_id,
        ),
        operator_label: source.operator_label,
        required_evidence: source.required_evidence,
        request_present: request.is_some(),
        duplicate_request,
        requested_decision,
        generated_decision,
        evidence_state: generated_decision.evidence_state(),
        decision_source_route: request
            .map(|request| request.decision_source_route)
            .unwrap_or("memory://status-canary/evidence-acceptance/no-decision-request"),
        decision_source_route_present: request
            .map(|request| !request.decision_source_route.is_empty())
            .unwrap_or(false),
        evidence_artifact_present: request
            .map(|request| request.evidence_artifact_present)
            .unwrap_or(false),
        evidence_digest_present: request
            .map(|request| request.evidence_digest_present)
            .unwrap_or(false),
        operator_authority_present: request
            .map(|request| request.operator_authority_present)
            .unwrap_or(false),
        freshness_attested: request
            .map(|request| request.freshness_attested)
            .unwrap_or(false),
        waiver_reason_present: request
            .map(|request| request.waiver_reason_present)
            .unwrap_or(false),
        expiry_attested: request
            .map(|request| request.expiry_attested)
            .unwrap_or(false),
        invalidity_reason_present: request
            .map(|request| request.invalidity_reason_present)
            .unwrap_or(false),
        source_validator_bound: request
            .map(|request| request.source_validator_bound)
            .unwrap_or(false),
        source_validator_contract_audit_ready: request
            .map(|request| request.source_validator_contract_audit_ready)
            .unwrap_or(false),
        source_validator_reason_audit_bound: request
            .map(|request| request.source_validator_reason_audit_bound)
            .unwrap_or(false),
        source_validator_reason_audit_ready: request
            .map(|request| request.source_validator_reason_audit_ready)
            .unwrap_or(false),
        source_readback_reason_packet_route: request
            .map(|request| request.source_readback_reason_packet_route)
            .unwrap_or("status_canary_evidence_source_reason_packet_not_bound_to_acceptance"),
        source_readback_fixture_reason_audit_rejection_reason: request
            .map(|request| request.source_readback_fixture_reason_audit_rejection_reason)
            .unwrap_or("source_reason_packet_not_bound_to_acceptance"),
        request_reason_audit_ready,
        decision_source_valid: validation.valid,
        decision_accepted,
        decision_rejected,
        rejection_reason: validation.rejection_reason,
        generated_override: decision_accepted,
        operator_visible: true,
        queryable: true,
        selected_status_canary_bound: true,
        preflight_only_connector_excluded: true,
        evidence_recording_allowed: false,
        waiver_recording_allowed: false,
        credential_read_allowed: false,
        transport_mutation_allowed: false,
        persistence_allowed: false,
        live_mutation_allowed: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StatusCanaryEvidenceDecisionValidation {
    valid: bool,
    rejection_reason: &'static str,
}

fn status_canary_evidence_decision_validation(
    request: StatusCanaryEvidenceDecisionRequest,
    duplicate_request: bool,
) -> StatusCanaryEvidenceDecisionValidation {
    if duplicate_request {
        return StatusCanaryEvidenceDecisionValidation {
            valid: false,
            rejection_reason: "duplicate_decision_request",
        };
    }
    if request.decision_source_route.is_empty() {
        return StatusCanaryEvidenceDecisionValidation {
            valid: false,
            rejection_reason: "missing_decision_source_route",
        };
    }
    if request.source_validator_bound && !request.source_validator_contract_audit_ready {
        return StatusCanaryEvidenceDecisionValidation {
            valid: false,
            rejection_reason: "source_validator_contract_audit_not_ready",
        };
    }
    if request.source_validator_reason_audit_bound
        && !status_canary_evidence_decision_request_reason_audit_ready(request)
    {
        return StatusCanaryEvidenceDecisionValidation {
            valid: false,
            rejection_reason: request.source_readback_fixture_reason_audit_rejection_reason,
        };
    }

    match request.requested_decision {
        StatusCanaryEvidenceDecision::Missing => StatusCanaryEvidenceDecisionValidation {
            valid: false,
            rejection_reason: "missing_decision_is_default_not_acceptance_request",
        },
        StatusCanaryEvidenceDecision::Recorded => {
            if !request.evidence_artifact_present {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_recorded_evidence_artifact",
                }
            } else if !request.evidence_digest_present {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_recorded_evidence_digest",
                }
            } else if !request.operator_authority_present {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_recorded_operator_authority",
                }
            } else if !request.freshness_attested {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_recorded_freshness_attestation",
                }
            } else {
                StatusCanaryEvidenceDecisionValidation {
                    valid: true,
                    rejection_reason: "accepted",
                }
            }
        }
        StatusCanaryEvidenceDecision::Waived => {
            if !request.operator_authority_present {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_waiver_operator_authority",
                }
            } else if !request.waiver_reason_present {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_waiver_reason",
                }
            } else {
                StatusCanaryEvidenceDecisionValidation {
                    valid: true,
                    rejection_reason: "accepted",
                }
            }
        }
        StatusCanaryEvidenceDecision::Expired => {
            if !request.evidence_artifact_present {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_expired_evidence_artifact",
                }
            } else if !request.evidence_digest_present {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_expired_evidence_digest",
                }
            } else if !request.expiry_attested {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_expiry_attestation",
                }
            } else {
                StatusCanaryEvidenceDecisionValidation {
                    valid: true,
                    rejection_reason: "accepted",
                }
            }
        }
        StatusCanaryEvidenceDecision::Invalid => {
            if !request.evidence_artifact_present {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_invalid_evidence_artifact",
                }
            } else if !request.evidence_digest_present {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_invalid_evidence_digest",
                }
            } else if !request.invalidity_reason_present {
                StatusCanaryEvidenceDecisionValidation {
                    valid: false,
                    rejection_reason: "missing_invalidity_reason",
                }
            } else {
                StatusCanaryEvidenceDecisionValidation {
                    valid: true,
                    rejection_reason: "accepted",
                }
            }
        }
    }
}

fn status_canary_evidence_decision_request_reason_audit_ready(
    request: StatusCanaryEvidenceDecisionRequest,
) -> bool {
    if !request.source_validator_reason_audit_bound {
        return false;
    }

    request.source_validator_bound
        && request.source_validator_contract_audit_ready
        && request.source_validator_reason_audit_ready
        && request.source_readback_reason_packet_route
            != "status_canary_evidence_source_reason_packet_not_bound_to_acceptance"
        && request.source_readback_fixture_reason_audit_rejection_reason
            == "fixture_generation_allowed"
}

fn status_canary_evidence_acceptance_packet_route(
    source_required_evidence_collection_plan_ready: bool,
    request_count: usize,
    unknown_request_count: usize,
    duplicate_request_count: usize,
    rejected_decision_count: usize,
    accepted_decision_count: usize,
) -> &'static str {
    if !source_required_evidence_collection_plan_ready {
        "status_canary_evidence_acceptance_packet_blocked_source_plan_not_ready"
    } else if unknown_request_count > 0 {
        "status_canary_evidence_acceptance_packet_blocked_unknown_request_source"
    } else if duplicate_request_count > 0 {
        "status_canary_evidence_acceptance_packet_blocked_duplicate_request"
    } else if rejected_decision_count > 0 {
        "status_canary_evidence_acceptance_packet_blocked_rejected_decision"
    } else if request_count == 0 {
        "status_canary_evidence_acceptance_packet_ready_no_decision_requests"
    } else if accepted_decision_count > 0 {
        "status_canary_evidence_acceptance_packet_ready_validated_decisions"
    } else {
        "status_canary_evidence_acceptance_packet_blocked_unknown"
    }
}

fn status_canary_evidence_acceptance_packet_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "status_canary.evidence_acceptance.dirty_worktree_boundary",
        "operator_live_approval_missing" => {
            "status_canary.evidence_acceptance.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "status_canary.evidence_acceptance.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "status_canary.evidence_acceptance.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "status_canary.evidence_acceptance.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "status_canary.evidence_acceptance.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "status_canary.evidence_acceptance.kill_switch_rehearsal_missing"
        }
        _ => "status_canary.evidence_acceptance.unknown",
    }
}

fn status_canary_evidence_acceptance_packet_entry_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "memory://status-canary/evidence-acceptance/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "memory://status-canary/evidence-acceptance/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "memory://status-canary/evidence-acceptance/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "memory://status-canary/evidence-acceptance/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "memory://status-canary/evidence-acceptance/transport-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "memory://status-canary/evidence-acceptance/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "memory://status-canary/evidence-acceptance/kill-switch-rehearsal-missing"
        }
        _ => "memory://status-canary/evidence-acceptance/unknown",
    }
}

impl StatusCanaryEvidenceDecisionRequest {
    pub const fn recorded(source_blocker_id: &'static str) -> Self {
        Self {
            source_blocker_id,
            requested_decision: StatusCanaryEvidenceDecision::Recorded,
            decision_source_route: "memory://status-canary/evidence-acceptance/test-recorded",
            evidence_artifact_present: true,
            evidence_digest_present: true,
            operator_authority_present: true,
            freshness_attested: true,
            waiver_reason_present: false,
            expiry_attested: false,
            invalidity_reason_present: false,
            source_validator_bound: false,
            source_validator_contract_audit_ready: false,
            source_validator_reason_audit_bound: false,
            source_validator_reason_audit_ready: false,
            source_readback_reason_packet_route: "status_canary_evidence_source_reason_packet_not_bound_to_acceptance",
            source_readback_fixture_reason_audit_rejection_reason: "source_reason_packet_not_bound_to_acceptance",
        }
    }

    pub const fn waived(source_blocker_id: &'static str) -> Self {
        Self {
            source_blocker_id,
            requested_decision: StatusCanaryEvidenceDecision::Waived,
            decision_source_route: "memory://status-canary/evidence-acceptance/test-waived",
            evidence_artifact_present: false,
            evidence_digest_present: false,
            operator_authority_present: true,
            freshness_attested: false,
            waiver_reason_present: true,
            expiry_attested: false,
            invalidity_reason_present: false,
            source_validator_bound: false,
            source_validator_contract_audit_ready: false,
            source_validator_reason_audit_bound: false,
            source_validator_reason_audit_ready: false,
            source_readback_reason_packet_route: "status_canary_evidence_source_reason_packet_not_bound_to_acceptance",
            source_readback_fixture_reason_audit_rejection_reason: "source_reason_packet_not_bound_to_acceptance",
        }
    }

    pub const fn expired(source_blocker_id: &'static str) -> Self {
        Self {
            source_blocker_id,
            requested_decision: StatusCanaryEvidenceDecision::Expired,
            decision_source_route: "memory://status-canary/evidence-acceptance/test-expired",
            evidence_artifact_present: true,
            evidence_digest_present: true,
            operator_authority_present: false,
            freshness_attested: false,
            waiver_reason_present: false,
            expiry_attested: true,
            invalidity_reason_present: false,
            source_validator_bound: false,
            source_validator_contract_audit_ready: false,
            source_validator_reason_audit_bound: false,
            source_validator_reason_audit_ready: false,
            source_readback_reason_packet_route: "status_canary_evidence_source_reason_packet_not_bound_to_acceptance",
            source_readback_fixture_reason_audit_rejection_reason: "source_reason_packet_not_bound_to_acceptance",
        }
    }

    pub const fn invalid(source_blocker_id: &'static str) -> Self {
        Self {
            source_blocker_id,
            requested_decision: StatusCanaryEvidenceDecision::Invalid,
            decision_source_route: "memory://status-canary/evidence-acceptance/test-invalid",
            evidence_artifact_present: true,
            evidence_digest_present: true,
            operator_authority_present: false,
            freshness_attested: false,
            waiver_reason_present: false,
            expiry_attested: false,
            invalidity_reason_present: true,
            source_validator_bound: false,
            source_validator_contract_audit_ready: false,
            source_validator_reason_audit_bound: false,
            source_validator_reason_audit_ready: false,
            source_readback_reason_packet_route: "status_canary_evidence_source_reason_packet_not_bound_to_acceptance",
            source_readback_fixture_reason_audit_rejection_reason: "source_reason_packet_not_bound_to_acceptance",
        }
    }
}

impl StatusCanaryEvidenceAcceptancePacketSideEffects {
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
    use crate::status_canary_start_guard::StatusCanaryStartGuardInput;
    use crate::status_canary_start_guard::StatusCanaryStartGuardSideEffects;
    use crate::status_canary_start_guard::status_canary_start_guard_from_packet;

    fn blocker_ids() -> Vec<&'static str> {
        controlled_live_required_evidence_collection_plan_report()
            .entries
            .iter()
            .map(|entry| entry.source_blocker_id)
            .collect()
    }

    #[test]
    fn default_packet_has_no_requests_and_generates_no_overrides() {
        let packet = status_canary_evidence_acceptance_packet();

        assert_eq!(
            packet.packet_id,
            STATUS_CANARY_EVIDENCE_ACCEPTANCE_PACKET_ID
        );
        assert_eq!(
            packet.selected_status_canary_tool_id,
            SELECTED_STATUS_CANARY_TOOL_ID
        );
        assert_eq!(
            packet.preflight_only_connector_tool_id,
            PREFLIGHT_ONLY_CONNECTOR_TOOL_ID
        );
        assert!(packet.acceptance_packet_ready);
        assert!(packet.default_missing_decisions);
        assert_eq!(packet.checklist_item_count, 7);
        assert_eq!(packet.request_count, 0);
        assert_eq!(packet.known_request_count, 0);
        assert_eq!(packet.unknown_request_count, 0);
        assert_eq!(packet.duplicate_request_count, 0);
        assert_eq!(packet.request_source_validator_bound_count, 0);
        assert_eq!(
            packet.request_source_validator_contract_audit_ready_count,
            0
        );
        assert_eq!(packet.request_reason_audit_count, 0);
        assert_eq!(packet.request_reason_audit_ready_count, 0);
        assert_eq!(packet.request_reason_audit_rejected_count, 0);
        assert_eq!(packet.accepted_decision_count, 0);
        assert_eq!(packet.rejected_decision_count, 0);
        assert_eq!(packet.generated_override_count, 0);
        assert_eq!(packet.generated_override_reason_audit_ready_count, 0);
        assert_eq!(packet.source_evidence_packet_missing_count, 7);
        assert_eq!(
            packet.acceptance_packet_route,
            "status_canary_evidence_acceptance_packet_ready_no_decision_requests"
        );
        assert_eq!(
            packet.side_effects,
            StatusCanaryEvidenceAcceptancePacketSideEffects::none()
        );
        assert!(packet.entries.iter().all(|entry| {
            !entry.request_present
                && !entry.decision_source_valid
                && !entry.decision_accepted
                && !entry.decision_rejected
                && !entry.generated_override
                && entry.generated_decision == StatusCanaryEvidenceDecision::Missing
                && !entry.evidence_recording_allowed
                && !entry.waiver_recording_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        }));
    }

    #[test]
    fn valid_recorded_and_waived_requests_generate_complete_overlay_without_side_effects() {
        let ids = blocker_ids();
        let requests = ids
            .iter()
            .enumerate()
            .map(|(index, source_blocker_id)| {
                if index < 4 {
                    StatusCanaryEvidenceDecisionRequest::recorded(source_blocker_id)
                } else {
                    StatusCanaryEvidenceDecisionRequest::waived(source_blocker_id)
                }
            })
            .collect::<Vec<_>>();
        let packet = status_canary_evidence_acceptance_packet_from_requests(&requests);

        assert!(packet.acceptance_packet_ready);
        assert!(!packet.default_missing_decisions);
        assert_eq!(packet.request_count, 7);
        assert_eq!(packet.request_source_validator_bound_count, 0);
        assert_eq!(packet.request_reason_audit_count, 0);
        assert_eq!(packet.request_reason_audit_ready_count, 0);
        assert_eq!(packet.request_reason_audit_rejected_count, 0);
        assert_eq!(packet.accepted_decision_count, 7);
        assert_eq!(packet.rejected_decision_count, 0);
        assert_eq!(packet.generated_override_count, 7);
        assert_eq!(packet.generated_override_reason_audit_ready_count, 0);
        assert_eq!(packet.source_evidence_packet_missing_count, 0);
        assert_eq!(packet.source_evidence_packet_recorded_count, 4);
        assert_eq!(packet.source_evidence_packet_waived_count, 3);
        assert!(packet.source_evidence_packet_complete);
        assert_eq!(
            packet.acceptance_packet_route,
            "status_canary_evidence_acceptance_packet_ready_validated_decisions"
        );
        assert!(packet.entries.iter().all(|entry| {
            entry.request_present
                && entry.decision_source_valid
                && entry.decision_accepted
                && !entry.decision_rejected
                && entry.generated_override
                && (entry.generated_decision == StatusCanaryEvidenceDecision::Recorded
                    || entry.generated_decision == StatusCanaryEvidenceDecision::Waived)
        }));
        assert_eq!(
            packet.side_effects,
            StatusCanaryEvidenceAcceptancePacketSideEffects::none()
        );
    }

    #[test]
    fn expired_and_invalid_requests_validate_but_keep_start_guard_blocked() {
        let ids = blocker_ids();
        let requests = ids
            .iter()
            .enumerate()
            .map(|(index, source_blocker_id)| match index {
                0 | 1 => StatusCanaryEvidenceDecisionRequest::recorded(source_blocker_id),
                2 | 3 => StatusCanaryEvidenceDecisionRequest::waived(source_blocker_id),
                4 | 5 => StatusCanaryEvidenceDecisionRequest::expired(source_blocker_id),
                _ => StatusCanaryEvidenceDecisionRequest::invalid(source_blocker_id),
            })
            .collect::<Vec<_>>();
        let acceptance_packet = status_canary_evidence_acceptance_packet_from_requests(&requests);
        let evidence_packet =
            status_canary_evidence_packet_from_decisions(&acceptance_packet.decision_overrides);
        let guard = status_canary_start_guard_from_packet(
            &evidence_packet,
            StatusCanaryStartGuardInput {
                canary_start_switch_enabled: true,
            },
        );

        assert!(acceptance_packet.acceptance_packet_ready);
        assert_eq!(acceptance_packet.accepted_decision_count, 7);
        assert_eq!(acceptance_packet.rejected_decision_count, 0);
        assert_eq!(acceptance_packet.request_reason_audit_count, 0);
        assert_eq!(acceptance_packet.request_reason_audit_ready_count, 0);
        assert_eq!(acceptance_packet.request_reason_audit_rejected_count, 0);
        assert_eq!(acceptance_packet.source_evidence_packet_expired_count, 2);
        assert_eq!(acceptance_packet.source_evidence_packet_invalid_count, 1);
        assert!(!acceptance_packet.source_evidence_packet_complete);
        assert_eq!(
            acceptance_packet.source_evidence_packet_guard_route,
            "status_canary_evidence_packet_blocked_expired_evidence"
        );
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
    }

    #[test]
    fn malformed_recorded_request_is_rejected_without_generating_override() {
        let mut request =
            StatusCanaryEvidenceDecisionRequest::recorded("operator_live_approval_missing");
        request.evidence_digest_present = false;
        let packet = status_canary_evidence_acceptance_packet_from_requests(&[request]);

        assert!(packet.acceptance_packet_ready);
        assert_eq!(packet.request_count, 1);
        assert_eq!(packet.accepted_decision_count, 0);
        assert_eq!(packet.rejected_decision_count, 1);
        assert_eq!(packet.request_reason_audit_count, 0);
        assert_eq!(packet.request_reason_audit_ready_count, 0);
        assert_eq!(packet.request_reason_audit_rejected_count, 0);
        assert_eq!(packet.generated_override_count, 0);
        assert_eq!(packet.source_evidence_packet_missing_count, 7);
        assert_eq!(
            packet.acceptance_packet_route,
            "status_canary_evidence_acceptance_packet_blocked_rejected_decision"
        );
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.decision_rejected
                && entry.rejection_reason == "missing_recorded_evidence_digest"
                && !entry.generated_override
        }));
        assert_eq!(
            packet.side_effects,
            StatusCanaryEvidenceAcceptancePacketSideEffects::none()
        );
    }

    #[test]
    fn unknown_and_duplicate_requests_fail_closed_without_acceptance() {
        let duplicate = StatusCanaryEvidenceDecisionRequest::recorded("dirty_worktree_boundary");
        let unknown = StatusCanaryEvidenceDecisionRequest::recorded("unknown_blocker");
        let packet = status_canary_evidence_acceptance_packet_from_requests(&[
            duplicate, duplicate, unknown,
        ]);

        assert!(!packet.acceptance_packet_ready);
        assert_eq!(packet.request_count, 3);
        assert_eq!(packet.known_request_count, 2);
        assert_eq!(packet.unknown_request_count, 1);
        assert_eq!(packet.duplicate_request_count, 1);
        assert_eq!(packet.request_reason_audit_count, 0);
        assert_eq!(packet.request_reason_audit_ready_count, 0);
        assert_eq!(packet.request_reason_audit_rejected_count, 0);
        assert_eq!(packet.accepted_decision_count, 0);
        assert_eq!(packet.rejected_decision_count, 1);
        assert_eq!(packet.generated_override_count, 0);
        assert_eq!(packet.source_evidence_packet_missing_count, 7);
        assert_eq!(
            packet.acceptance_packet_route,
            "status_canary_evidence_acceptance_packet_blocked_unknown_request_source"
        );
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.duplicate_request
                && entry.decision_rejected
                && entry.rejection_reason == "duplicate_decision_request"
        }));
        assert_eq!(
            packet.side_effects,
            StatusCanaryEvidenceAcceptancePacketSideEffects::none()
        );
    }

    #[test]
    fn reason_bound_request_without_ready_audit_is_rejected_without_override() {
        let mut request =
            StatusCanaryEvidenceDecisionRequest::recorded("operator_live_approval_missing");
        request.source_validator_bound = true;
        request.source_validator_contract_audit_ready = true;
        request.source_validator_reason_audit_bound = true;
        request.source_validator_reason_audit_ready = false;
        request.source_readback_reason_packet_route =
            "status_canary_evidence_source_reason_packet_ready_inputs_valid";
        request.source_readback_fixture_reason_audit_rejection_reason =
            "source_adapter_input_missing_for_decision";

        let packet = status_canary_evidence_acceptance_packet_from_requests(&[request]);

        assert!(packet.acceptance_packet_ready);
        assert_eq!(packet.request_count, 1);
        assert_eq!(packet.request_source_validator_bound_count, 1);
        assert_eq!(
            packet.request_source_validator_contract_audit_ready_count,
            1
        );
        assert_eq!(packet.request_reason_audit_count, 1);
        assert_eq!(packet.request_reason_audit_ready_count, 0);
        assert_eq!(packet.request_reason_audit_rejected_count, 1);
        assert_eq!(packet.accepted_decision_count, 0);
        assert_eq!(packet.rejected_decision_count, 1);
        assert_eq!(packet.generated_override_count, 0);
        assert_eq!(packet.generated_override_reason_audit_ready_count, 0);
        assert_eq!(packet.source_evidence_packet_missing_count, 7);
        assert_eq!(
            packet.acceptance_packet_route,
            "status_canary_evidence_acceptance_packet_blocked_rejected_decision"
        );
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.source_validator_bound
                && entry.source_validator_reason_audit_bound
                && !entry.request_reason_audit_ready
                && entry.decision_rejected
                && entry.rejection_reason == "source_adapter_input_missing_for_decision"
                && !entry.generated_override
        }));
    }
}
