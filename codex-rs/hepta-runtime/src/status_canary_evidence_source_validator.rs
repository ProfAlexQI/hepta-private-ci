use crate::controlled_live_required_evidence_collection_plan::ControlledLiveRequiredEvidenceCollectionPlanEntry;
use crate::controlled_live_required_evidence_collection_plan::controlled_live_required_evidence_collection_plan_report;
use crate::status_canary_evidence_acceptance_packet::StatusCanaryEvidenceDecisionRequest;
use crate::status_canary_evidence_acceptance_packet::status_canary_evidence_acceptance_packet_from_requests;
use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;
use crate::status_canary_evidence_packet::StatusCanaryEvidenceDecision;
use serde::Serialize;

pub const STATUS_CANARY_EVIDENCE_SOURCE_VALIDATOR_SCHEMA_VERSION: &str =
    "status_canary_evidence_source_validator_v1";
pub const STATUS_CANARY_EVIDENCE_SOURCE_VALIDATOR_ID: &str =
    "status-canary-evidence-source-validator/hepta-system-status/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceValidatorReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub schema_version: &'static str,
    pub validator_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_required_evidence_collection_plan_ready: bool,
    pub source_item_count: usize,
    pub observation_count: usize,
    pub known_observation_count: usize,
    pub unknown_observation_count: usize,
    pub duplicate_observation_count: usize,
    pub observation_contract_audit_count: usize,
    pub observation_contract_audit_ready_count: usize,
    pub observation_contract_audit_rejected_count: usize,
    pub observation_reason_audit_count: usize,
    pub observation_reason_audit_ready_count: usize,
    pub observation_reason_audit_rejected_count: usize,
    pub source_missing_count: usize,
    pub source_validated_count: usize,
    pub source_rejected_count: usize,
    pub generated_request_count: usize,
    pub generated_recorded_request_count: usize,
    pub generated_waived_request_count: usize,
    pub generated_expired_request_count: usize,
    pub generated_invalid_request_count: usize,
    pub source_acceptance_packet_ready: bool,
    pub source_acceptance_packet_route: &'static str,
    pub source_acceptance_request_count: usize,
    pub source_acceptance_accepted_decision_count: usize,
    pub source_acceptance_rejected_decision_count: usize,
    pub source_acceptance_generated_override_count: usize,
    pub source_acceptance_evidence_complete: bool,
    pub source_validator_ready: bool,
    pub source_validator_route: &'static str,
    pub entries: Vec<StatusCanaryEvidenceSourceValidatorEntry>,
    pub decision_requests: Vec<StatusCanaryEvidenceDecisionRequest>,
    pub side_effects: StatusCanaryEvidenceSourceValidatorSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceValidatorEntry {
    pub source_blocker_id: &'static str,
    pub validator_key: &'static str,
    pub validator_route: &'static str,
    pub source_kind: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub observation_present: bool,
    pub duplicate_observation: bool,
    pub requested_decision: StatusCanaryEvidenceDecision,
    pub observation_source_route: &'static str,
    pub observation_source_route_present: bool,
    pub source_adapter_metadata_contract_route: &'static str,
    pub source_adapter_readback_fixture_contract_route: &'static str,
    pub source_readback_contract_audit_route: &'static str,
    pub source_adapter_contract_audit_ready: bool,
    pub source_readback_contract_audit_ready: bool,
    pub observation_contract_audit_ready: bool,
    pub source_readback_reason_packet_bound: bool,
    pub source_readback_reason_packet_ready: bool,
    pub source_readback_reason_packet_route: &'static str,
    pub source_readback_fixture_reason_audit_ready: bool,
    pub source_readback_fixture_reason_audit_rejection_reason: &'static str,
    pub observation_reason_audit_ready: bool,
    pub evidence_artifact_present: bool,
    pub evidence_digest_present: bool,
    pub operator_authority_present: bool,
    pub freshness_attested: bool,
    pub waiver_reason_present: bool,
    pub expiry_attested: bool,
    pub invalidity_reason_present: bool,
    pub source_valid: bool,
    pub source_missing: bool,
    pub source_rejected: bool,
    pub rejection_reason: &'static str,
    pub request_generated: bool,
    pub generated_request: Option<StatusCanaryEvidenceDecisionRequest>,
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
pub struct StatusCanaryEvidenceSourceObservation {
    pub source_blocker_id: &'static str,
    pub requested_decision: StatusCanaryEvidenceDecision,
    pub source_route: &'static str,
    pub source_adapter_metadata_contract_route: &'static str,
    pub source_adapter_readback_fixture_contract_route: &'static str,
    pub source_readback_contract_audit_route: &'static str,
    pub source_adapter_contract_audit_ready: bool,
    pub source_readback_contract_audit_ready: bool,
    pub source_readback_reason_packet_bound: bool,
    pub source_readback_reason_packet_ready: bool,
    pub source_readback_reason_packet_route: &'static str,
    pub source_readback_fixture_reason_audit_ready: bool,
    pub source_readback_fixture_reason_audit_rejection_reason: &'static str,
    pub evidence_artifact_present: bool,
    pub evidence_digest_present: bool,
    pub operator_authority_present: bool,
    pub freshness_attested: bool,
    pub waiver_reason_present: bool,
    pub expiry_attested: bool,
    pub invalidity_reason_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceValidatorSideEffects {
    pub source_read_persisted: bool,
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

pub fn status_canary_evidence_source_validator() -> StatusCanaryEvidenceSourceValidatorReport {
    status_canary_evidence_source_validator_from_observations(&[])
}

pub fn status_canary_evidence_source_validator_from_observations(
    observations: &[StatusCanaryEvidenceSourceObservation],
) -> StatusCanaryEvidenceSourceValidatorReport {
    let source = controlled_live_required_evidence_collection_plan_report();
    status_canary_evidence_source_validator_from_plan(
        source.evidence_collection_plan_ready,
        source.entries,
        observations,
    )
}

pub fn status_canary_evidence_source_validator_from_plan(
    source_required_evidence_collection_plan_ready: bool,
    source_entries: Vec<ControlledLiveRequiredEvidenceCollectionPlanEntry>,
    observations: &[StatusCanaryEvidenceSourceObservation],
) -> StatusCanaryEvidenceSourceValidatorReport {
    let known_observation_count = observations
        .iter()
        .filter(|observation| {
            source_entries
                .iter()
                .any(|entry| entry.source_blocker_id == observation.source_blocker_id)
        })
        .count();
    let unknown_observation_count = observations.len().saturating_sub(known_observation_count);
    let entries = source_entries
        .into_iter()
        .map(|entry| status_canary_evidence_source_validator_entry(entry, observations))
        .collect::<Vec<_>>();
    let duplicate_observation_count = entries
        .iter()
        .filter(|entry| entry.duplicate_observation)
        .count();
    let observation_contract_audit_count = observations.len();
    let observation_contract_audit_ready_count = entries
        .iter()
        .filter(|entry| entry.observation_contract_audit_ready)
        .count();
    let observation_contract_audit_rejected_count = entries
        .iter()
        .filter(|entry| entry.observation_present && !entry.observation_contract_audit_ready)
        .count();
    let observation_reason_audit_count = observations
        .iter()
        .filter(|observation| observation.source_readback_reason_packet_bound)
        .count();
    let observation_reason_audit_ready_count = entries
        .iter()
        .filter(|entry| entry.observation_reason_audit_ready)
        .count();
    let observation_reason_audit_rejected_count = entries
        .iter()
        .filter(|entry| {
            entry.observation_present
                && entry.source_readback_reason_packet_bound
                && !entry.observation_reason_audit_ready
        })
        .count();
    let source_missing_count = entries.iter().filter(|entry| entry.source_missing).count();
    let source_validated_count = entries.iter().filter(|entry| entry.source_valid).count();
    let source_rejected_count = entries.iter().filter(|entry| entry.source_rejected).count();
    let decision_requests = entries
        .iter()
        .filter_map(|entry| entry.generated_request)
        .collect::<Vec<_>>();
    let acceptance_packet =
        status_canary_evidence_acceptance_packet_from_requests(&decision_requests);
    let side_effects = StatusCanaryEvidenceSourceValidatorSideEffects::none();
    let source_validator_ready = source_required_evidence_collection_plan_ready
        && entries.len() == 7
        && unknown_observation_count == 0
        && duplicate_observation_count == 0
        && observation_contract_audit_ready_count == observation_contract_audit_count
        && observation_reason_audit_ready_count == observation_reason_audit_count
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
        && side_effects == StatusCanaryEvidenceSourceValidatorSideEffects::none();

    StatusCanaryEvidenceSourceValidatorReport {
        runtime: "hepta",
        surface: "status_canary_evidence_source_validator",
        schema_version: STATUS_CANARY_EVIDENCE_SOURCE_VALIDATOR_SCHEMA_VERSION,
        validator_id: STATUS_CANARY_EVIDENCE_SOURCE_VALIDATOR_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_required_evidence_collection_plan_ready,
        source_item_count: entries.len(),
        observation_count: observations.len(),
        known_observation_count,
        unknown_observation_count,
        duplicate_observation_count,
        observation_contract_audit_count,
        observation_contract_audit_ready_count,
        observation_contract_audit_rejected_count,
        observation_reason_audit_count,
        observation_reason_audit_ready_count,
        observation_reason_audit_rejected_count,
        source_missing_count,
        source_validated_count,
        source_rejected_count,
        generated_request_count: decision_requests.len(),
        generated_recorded_request_count: decision_requests
            .iter()
            .filter(|request| request.requested_decision == StatusCanaryEvidenceDecision::Recorded)
            .count(),
        generated_waived_request_count: decision_requests
            .iter()
            .filter(|request| request.requested_decision == StatusCanaryEvidenceDecision::Waived)
            .count(),
        generated_expired_request_count: decision_requests
            .iter()
            .filter(|request| request.requested_decision == StatusCanaryEvidenceDecision::Expired)
            .count(),
        generated_invalid_request_count: decision_requests
            .iter()
            .filter(|request| request.requested_decision == StatusCanaryEvidenceDecision::Invalid)
            .count(),
        source_acceptance_packet_ready: acceptance_packet.acceptance_packet_ready,
        source_acceptance_packet_route: acceptance_packet.acceptance_packet_route,
        source_acceptance_request_count: acceptance_packet.request_count,
        source_acceptance_accepted_decision_count: acceptance_packet.accepted_decision_count,
        source_acceptance_rejected_decision_count: acceptance_packet.rejected_decision_count,
        source_acceptance_generated_override_count: acceptance_packet.generated_override_count,
        source_acceptance_evidence_complete: acceptance_packet.source_evidence_packet_complete,
        source_validator_ready,
        source_validator_route: status_canary_evidence_source_validator_route(
            source_required_evidence_collection_plan_ready,
            observations.len(),
            unknown_observation_count,
            duplicate_observation_count,
            source_rejected_count,
            source_validated_count,
        ),
        entries,
        decision_requests,
        side_effects,
    }
}

fn status_canary_evidence_source_validator_entry(
    source: ControlledLiveRequiredEvidenceCollectionPlanEntry,
    observations: &[StatusCanaryEvidenceSourceObservation],
) -> StatusCanaryEvidenceSourceValidatorEntry {
    let matching_observations = observations
        .iter()
        .filter(|observation| observation.source_blocker_id == source.source_blocker_id)
        .collect::<Vec<_>>();
    let duplicate_observation = matching_observations.len() > 1;
    let observation = matching_observations.first().copied().copied();
    let validation = observation
        .map(|observation| {
            status_canary_evidence_source_validation(observation, duplicate_observation)
        })
        .unwrap_or(StatusCanaryEvidenceSourceValidation {
            valid: false,
            rejection_reason: "no_source_observation",
        });
    let requested_decision = observation
        .map(|observation| observation.requested_decision)
        .unwrap_or(StatusCanaryEvidenceDecision::Missing);
    let observation_contract_audit_ready = observation
        .map(status_canary_evidence_source_observation_contract_audit_ready)
        .unwrap_or(false);
    let observation_reason_audit_ready = observation
        .map(status_canary_evidence_source_observation_reason_audit_ready)
        .unwrap_or(false);
    let source_valid = observation.is_some() && validation.valid;
    let source_rejected = observation.is_some() && !validation.valid;
    let generated_request = if source_valid {
        observation.map(|observation| StatusCanaryEvidenceDecisionRequest {
            source_blocker_id: source.source_blocker_id,
            requested_decision: observation.requested_decision,
            decision_source_route: observation.source_route,
            evidence_artifact_present: observation.evidence_artifact_present,
            evidence_digest_present: observation.evidence_digest_present,
            operator_authority_present: observation.operator_authority_present,
            freshness_attested: observation.freshness_attested,
            waiver_reason_present: observation.waiver_reason_present,
            expiry_attested: observation.expiry_attested,
            invalidity_reason_present: observation.invalidity_reason_present,
            source_validator_bound: true,
            source_validator_contract_audit_ready: observation_contract_audit_ready,
            source_validator_reason_audit_bound: observation.source_readback_reason_packet_bound,
            source_validator_reason_audit_ready: observation_reason_audit_ready,
            source_readback_reason_packet_route: observation.source_readback_reason_packet_route,
            source_readback_fixture_reason_audit_rejection_reason: observation
                .source_readback_fixture_reason_audit_rejection_reason,
        })
    } else {
        None
    };

    StatusCanaryEvidenceSourceValidatorEntry {
        source_blocker_id: source.source_blocker_id,
        validator_key: status_canary_evidence_source_validator_key(source.source_blocker_id),
        validator_route: status_canary_evidence_source_validator_entry_route(
            source.source_blocker_id,
        ),
        source_kind: status_canary_evidence_source_kind(source.source_blocker_id),
        operator_label: source.operator_label,
        required_evidence: source.required_evidence,
        observation_present: observation.is_some(),
        duplicate_observation,
        requested_decision,
        observation_source_route: observation
            .map(|observation| observation.source_route)
            .unwrap_or("memory://status-canary/evidence-source/no-observation"),
        observation_source_route_present: observation
            .map(|observation| !observation.source_route.is_empty())
            .unwrap_or(false),
        source_adapter_metadata_contract_route: observation
            .map(|observation| observation.source_adapter_metadata_contract_route)
            .unwrap_or("metadata://status-canary/evidence-source-adapter-contract/no-observation"),
        source_adapter_readback_fixture_contract_route: observation
            .map(|observation| observation.source_adapter_readback_fixture_contract_route)
            .unwrap_or(
                "metadata://status-canary/evidence-source-readback-fixture-contract/no-observation",
            ),
        source_readback_contract_audit_route: observation
            .map(|observation| observation.source_readback_contract_audit_route)
            .unwrap_or("audit://status-canary/evidence-source-readback-contract/no-observation"),
        source_adapter_contract_audit_ready: observation
            .map(|observation| observation.source_adapter_contract_audit_ready)
            .unwrap_or(false),
        source_readback_contract_audit_ready: observation
            .map(|observation| observation.source_readback_contract_audit_ready)
            .unwrap_or(false),
        observation_contract_audit_ready,
        source_readback_reason_packet_bound: observation
            .map(|observation| observation.source_readback_reason_packet_bound)
            .unwrap_or(false),
        source_readback_reason_packet_ready: observation
            .map(|observation| observation.source_readback_reason_packet_ready)
            .unwrap_or(false),
        source_readback_reason_packet_route: observation
            .map(|observation| observation.source_readback_reason_packet_route)
            .unwrap_or("status_canary_evidence_source_reason_packet_not_bound_to_observation"),
        source_readback_fixture_reason_audit_ready: observation
            .map(|observation| observation.source_readback_fixture_reason_audit_ready)
            .unwrap_or(false),
        source_readback_fixture_reason_audit_rejection_reason: observation
            .map(|observation| observation.source_readback_fixture_reason_audit_rejection_reason)
            .unwrap_or("source_reason_packet_not_bound_to_observation"),
        observation_reason_audit_ready,
        evidence_artifact_present: observation
            .map(|observation| observation.evidence_artifact_present)
            .unwrap_or(false),
        evidence_digest_present: observation
            .map(|observation| observation.evidence_digest_present)
            .unwrap_or(false),
        operator_authority_present: observation
            .map(|observation| observation.operator_authority_present)
            .unwrap_or(false),
        freshness_attested: observation
            .map(|observation| observation.freshness_attested)
            .unwrap_or(false),
        waiver_reason_present: observation
            .map(|observation| observation.waiver_reason_present)
            .unwrap_or(false),
        expiry_attested: observation
            .map(|observation| observation.expiry_attested)
            .unwrap_or(false),
        invalidity_reason_present: observation
            .map(|observation| observation.invalidity_reason_present)
            .unwrap_or(false),
        source_valid,
        source_missing: observation.is_none(),
        source_rejected,
        rejection_reason: validation.rejection_reason,
        request_generated: generated_request.is_some(),
        generated_request,
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
struct StatusCanaryEvidenceSourceValidation {
    valid: bool,
    rejection_reason: &'static str,
}

fn status_canary_evidence_source_validation(
    observation: StatusCanaryEvidenceSourceObservation,
    duplicate_observation: bool,
) -> StatusCanaryEvidenceSourceValidation {
    if duplicate_observation {
        return StatusCanaryEvidenceSourceValidation {
            valid: false,
            rejection_reason: "duplicate_source_observation",
        };
    }
    if observation.source_route.is_empty() {
        return StatusCanaryEvidenceSourceValidation {
            valid: false,
            rejection_reason: "missing_source_route",
        };
    }
    if !status_canary_evidence_source_observation_contract_audit_ready(observation) {
        return StatusCanaryEvidenceSourceValidation {
            valid: false,
            rejection_reason: "source_observation_contract_audit_not_ready",
        };
    }
    if observation.source_readback_reason_packet_bound
        && !status_canary_evidence_source_observation_reason_audit_ready(observation)
    {
        return StatusCanaryEvidenceSourceValidation {
            valid: false,
            rejection_reason: observation.source_readback_fixture_reason_audit_rejection_reason,
        };
    }

    match observation.requested_decision {
        StatusCanaryEvidenceDecision::Missing => StatusCanaryEvidenceSourceValidation {
            valid: false,
            rejection_reason: "missing_is_default_not_source_observation",
        },
        StatusCanaryEvidenceDecision::Recorded => {
            if !observation.evidence_artifact_present {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_recorded_source_artifact",
                }
            } else if !observation.evidence_digest_present {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_recorded_source_digest",
                }
            } else if !observation.operator_authority_present {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_recorded_source_authority",
                }
            } else if !observation.freshness_attested {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_recorded_source_freshness",
                }
            } else {
                StatusCanaryEvidenceSourceValidation {
                    valid: true,
                    rejection_reason: "accepted",
                }
            }
        }
        StatusCanaryEvidenceDecision::Waived => {
            if !observation.operator_authority_present {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_waiver_source_authority",
                }
            } else if !observation.waiver_reason_present {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_waiver_source_reason",
                }
            } else {
                StatusCanaryEvidenceSourceValidation {
                    valid: true,
                    rejection_reason: "accepted",
                }
            }
        }
        StatusCanaryEvidenceDecision::Expired => {
            if !observation.evidence_artifact_present {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_expired_source_artifact",
                }
            } else if !observation.evidence_digest_present {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_expired_source_digest",
                }
            } else if !observation.expiry_attested {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_expired_source_attestation",
                }
            } else {
                StatusCanaryEvidenceSourceValidation {
                    valid: true,
                    rejection_reason: "accepted",
                }
            }
        }
        StatusCanaryEvidenceDecision::Invalid => {
            if !observation.evidence_artifact_present {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_invalid_source_artifact",
                }
            } else if !observation.evidence_digest_present {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_invalid_source_digest",
                }
            } else if !observation.invalidity_reason_present {
                StatusCanaryEvidenceSourceValidation {
                    valid: false,
                    rejection_reason: "missing_invalid_source_reason",
                }
            } else {
                StatusCanaryEvidenceSourceValidation {
                    valid: true,
                    rejection_reason: "accepted",
                }
            }
        }
    }
}

fn status_canary_evidence_source_observation_contract_audit_ready(
    observation: StatusCanaryEvidenceSourceObservation,
) -> bool {
    observation.source_adapter_contract_audit_ready
        && observation.source_readback_contract_audit_ready
        && observation
            .source_adapter_metadata_contract_route
            .starts_with("metadata://status-canary/evidence-source-adapter-contract/")
        && observation
            .source_adapter_readback_fixture_contract_route
            .starts_with("metadata://status-canary/evidence-source-readback-fixture-contract/")
        && observation
            .source_readback_contract_audit_route
            .starts_with("audit://status-canary/evidence-source-readback-contract/")
        && observation.source_adapter_metadata_contract_route
            != "metadata://status-canary/evidence-source-adapter-contract/unknown"
        && observation.source_adapter_readback_fixture_contract_route
            != "metadata://status-canary/evidence-source-readback-fixture-contract/unknown"
        && observation.source_readback_contract_audit_route
            != "audit://status-canary/evidence-source-readback-contract/unknown"
}

fn status_canary_evidence_source_observation_reason_audit_ready(
    observation: StatusCanaryEvidenceSourceObservation,
) -> bool {
    if !observation.source_readback_reason_packet_bound {
        return false;
    }

    observation.source_readback_reason_packet_ready
        && observation.source_readback_reason_packet_route
            != "status_canary_evidence_source_reason_packet_not_bound_to_observation"
        && observation.source_readback_fixture_reason_audit_ready
        && observation.source_readback_fixture_reason_audit_rejection_reason
            == "fixture_generation_allowed"
}

fn status_canary_evidence_source_validator_route(
    source_required_evidence_collection_plan_ready: bool,
    observation_count: usize,
    unknown_observation_count: usize,
    duplicate_observation_count: usize,
    source_rejected_count: usize,
    source_validated_count: usize,
) -> &'static str {
    if !source_required_evidence_collection_plan_ready {
        "status_canary_evidence_source_validator_blocked_source_plan_not_ready"
    } else if unknown_observation_count > 0 {
        "status_canary_evidence_source_validator_blocked_unknown_source"
    } else if duplicate_observation_count > 0 {
        "status_canary_evidence_source_validator_blocked_duplicate_source"
    } else if source_rejected_count > 0 {
        "status_canary_evidence_source_validator_blocked_rejected_source"
    } else if observation_count == 0 {
        "status_canary_evidence_source_validator_ready_no_observations"
    } else if source_validated_count > 0 {
        "status_canary_evidence_source_validator_ready_validated_sources"
    } else {
        "status_canary_evidence_source_validator_blocked_unknown"
    }
}

fn status_canary_evidence_source_validator_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "status_canary.evidence_source.dirty_worktree_boundary",
        "operator_live_approval_missing" => {
            "status_canary.evidence_source.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "status_canary.evidence_source.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "status_canary.evidence_source.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "status_canary.evidence_source.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => "status_canary.evidence_source.rollback_rehearsal_missing",
        "kill_switch_rehearsal_missing" => {
            "status_canary.evidence_source.kill_switch_rehearsal_missing"
        }
        _ => "status_canary.evidence_source.unknown",
    }
}

fn status_canary_evidence_source_validator_entry_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "readback://status-canary/evidence-source/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "readback://status-canary/evidence-source/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "readback://status-canary/evidence-source/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "readback://status-canary/evidence-source/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "readback://status-canary/evidence-source/transport-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "readback://status-canary/evidence-source/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "readback://status-canary/evidence-source/kill-switch-rehearsal-missing"
        }
        _ => "readback://status-canary/evidence-source/unknown",
    }
}

fn status_canary_evidence_source_kind(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "clean_worktree_snapshot_source",
        "operator_live_approval_missing" => "operator_live_approval_packet_source",
        "fresh_soak_readback_missing" => "fresh_status_canary_soak_readback_source",
        "credential_boundary_attestation_missing" => "credential_boundary_attestation_source",
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "transport_boundary_approval_source"
        }
        "rollback_rehearsal_missing" => "rollback_rehearsal_packet_source",
        "kill_switch_rehearsal_missing" => "kill_switch_rehearsal_packet_source",
        _ => "unknown_status_canary_evidence_source",
    }
}

impl StatusCanaryEvidenceSourceObservation {
    pub const fn recorded(source_blocker_id: &'static str) -> Self {
        Self {
            source_blocker_id,
            requested_decision: StatusCanaryEvidenceDecision::Recorded,
            source_route: "readback://status-canary/evidence-source/test-recorded",
            source_adapter_metadata_contract_route: "metadata://status-canary/evidence-source-adapter-contract/test-recorded",
            source_adapter_readback_fixture_contract_route: "metadata://status-canary/evidence-source-readback-fixture-contract/test-recorded",
            source_readback_contract_audit_route: "audit://status-canary/evidence-source-readback-contract/test-recorded",
            source_adapter_contract_audit_ready: true,
            source_readback_contract_audit_ready: true,
            source_readback_reason_packet_bound: false,
            source_readback_reason_packet_ready: false,
            source_readback_reason_packet_route: "status_canary_evidence_source_reason_packet_not_bound_to_observation",
            source_readback_fixture_reason_audit_ready: false,
            source_readback_fixture_reason_audit_rejection_reason: "source_reason_packet_not_bound_to_observation",
            evidence_artifact_present: true,
            evidence_digest_present: true,
            operator_authority_present: true,
            freshness_attested: true,
            waiver_reason_present: false,
            expiry_attested: false,
            invalidity_reason_present: false,
        }
    }

    pub const fn waived(source_blocker_id: &'static str) -> Self {
        Self {
            source_blocker_id,
            requested_decision: StatusCanaryEvidenceDecision::Waived,
            source_route: "readback://status-canary/evidence-source/test-waived",
            source_adapter_metadata_contract_route: "metadata://status-canary/evidence-source-adapter-contract/test-waived",
            source_adapter_readback_fixture_contract_route: "metadata://status-canary/evidence-source-readback-fixture-contract/test-waived",
            source_readback_contract_audit_route: "audit://status-canary/evidence-source-readback-contract/test-waived",
            source_adapter_contract_audit_ready: true,
            source_readback_contract_audit_ready: true,
            source_readback_reason_packet_bound: false,
            source_readback_reason_packet_ready: false,
            source_readback_reason_packet_route: "status_canary_evidence_source_reason_packet_not_bound_to_observation",
            source_readback_fixture_reason_audit_ready: false,
            source_readback_fixture_reason_audit_rejection_reason: "source_reason_packet_not_bound_to_observation",
            evidence_artifact_present: false,
            evidence_digest_present: false,
            operator_authority_present: true,
            freshness_attested: false,
            waiver_reason_present: true,
            expiry_attested: false,
            invalidity_reason_present: false,
        }
    }

    pub const fn expired(source_blocker_id: &'static str) -> Self {
        Self {
            source_blocker_id,
            requested_decision: StatusCanaryEvidenceDecision::Expired,
            source_route: "readback://status-canary/evidence-source/test-expired",
            source_adapter_metadata_contract_route: "metadata://status-canary/evidence-source-adapter-contract/test-expired",
            source_adapter_readback_fixture_contract_route: "metadata://status-canary/evidence-source-readback-fixture-contract/test-expired",
            source_readback_contract_audit_route: "audit://status-canary/evidence-source-readback-contract/test-expired",
            source_adapter_contract_audit_ready: true,
            source_readback_contract_audit_ready: true,
            source_readback_reason_packet_bound: false,
            source_readback_reason_packet_ready: false,
            source_readback_reason_packet_route: "status_canary_evidence_source_reason_packet_not_bound_to_observation",
            source_readback_fixture_reason_audit_ready: false,
            source_readback_fixture_reason_audit_rejection_reason: "source_reason_packet_not_bound_to_observation",
            evidence_artifact_present: true,
            evidence_digest_present: true,
            operator_authority_present: false,
            freshness_attested: false,
            waiver_reason_present: false,
            expiry_attested: true,
            invalidity_reason_present: false,
        }
    }

    pub const fn invalid(source_blocker_id: &'static str) -> Self {
        Self {
            source_blocker_id,
            requested_decision: StatusCanaryEvidenceDecision::Invalid,
            source_route: "readback://status-canary/evidence-source/test-invalid",
            source_adapter_metadata_contract_route: "metadata://status-canary/evidence-source-adapter-contract/test-invalid",
            source_adapter_readback_fixture_contract_route: "metadata://status-canary/evidence-source-readback-fixture-contract/test-invalid",
            source_readback_contract_audit_route: "audit://status-canary/evidence-source-readback-contract/test-invalid",
            source_adapter_contract_audit_ready: true,
            source_readback_contract_audit_ready: true,
            source_readback_reason_packet_bound: false,
            source_readback_reason_packet_ready: false,
            source_readback_reason_packet_route: "status_canary_evidence_source_reason_packet_not_bound_to_observation",
            source_readback_fixture_reason_audit_ready: false,
            source_readback_fixture_reason_audit_rejection_reason: "source_reason_packet_not_bound_to_observation",
            evidence_artifact_present: true,
            evidence_digest_present: true,
            operator_authority_present: false,
            freshness_attested: false,
            waiver_reason_present: false,
            expiry_attested: false,
            invalidity_reason_present: true,
        }
    }
}

impl StatusCanaryEvidenceSourceValidatorSideEffects {
    pub const fn none() -> Self {
        Self {
            source_read_persisted: false,
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
    use crate::status_canary_evidence_acceptance_packet::status_canary_evidence_acceptance_packet_from_requests;
    use crate::status_canary_evidence_packet::status_canary_evidence_packet_from_decisions;
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
    fn default_validator_has_no_source_observations_and_no_generated_requests() {
        let validator = status_canary_evidence_source_validator();

        assert_eq!(
            validator.validator_id,
            STATUS_CANARY_EVIDENCE_SOURCE_VALIDATOR_ID
        );
        assert!(validator.source_validator_ready);
        assert_eq!(
            validator.source_validator_route,
            "status_canary_evidence_source_validator_ready_no_observations"
        );
        assert_eq!(validator.source_item_count, 7);
        assert_eq!(validator.observation_count, 0);
        assert_eq!(validator.observation_contract_audit_count, 0);
        assert_eq!(validator.observation_contract_audit_ready_count, 0);
        assert_eq!(validator.observation_contract_audit_rejected_count, 0);
        assert_eq!(validator.observation_reason_audit_count, 0);
        assert_eq!(validator.observation_reason_audit_ready_count, 0);
        assert_eq!(validator.observation_reason_audit_rejected_count, 0);
        assert_eq!(validator.source_missing_count, 7);
        assert_eq!(validator.source_validated_count, 0);
        assert_eq!(validator.source_rejected_count, 0);
        assert_eq!(validator.generated_request_count, 0);
        assert_eq!(validator.source_acceptance_request_count, 0);
        assert_eq!(
            validator.source_acceptance_packet_route,
            "status_canary_evidence_acceptance_packet_ready_no_decision_requests"
        );
        assert_eq!(
            validator.side_effects,
            StatusCanaryEvidenceSourceValidatorSideEffects::none()
        );
        assert!(validator.entries.iter().all(|entry| {
            !entry.observation_present
                && entry.source_missing
                && !entry.observation_contract_audit_ready
                && !entry.observation_reason_audit_ready
                && !entry.source_valid
                && !entry.source_rejected
                && !entry.request_generated
                && entry.generated_request.is_none()
                && !entry.evidence_recording_allowed
                && !entry.waiver_recording_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        }));
    }

    #[test]
    fn valid_recorded_and_waived_sources_generate_acceptance_requests() {
        let ids = blocker_ids();
        let observations = ids
            .iter()
            .enumerate()
            .map(|(index, source_blocker_id)| {
                if index < 4 {
                    StatusCanaryEvidenceSourceObservation::recorded(source_blocker_id)
                } else {
                    StatusCanaryEvidenceSourceObservation::waived(source_blocker_id)
                }
            })
            .collect::<Vec<_>>();
        let validator = status_canary_evidence_source_validator_from_observations(&observations);
        let acceptance =
            status_canary_evidence_acceptance_packet_from_requests(&validator.decision_requests);

        assert!(validator.source_validator_ready);
        assert_eq!(validator.observation_count, 7);
        assert_eq!(validator.observation_contract_audit_count, 7);
        assert_eq!(validator.observation_contract_audit_ready_count, 7);
        assert_eq!(validator.observation_contract_audit_rejected_count, 0);
        assert_eq!(validator.observation_reason_audit_count, 0);
        assert_eq!(validator.observation_reason_audit_ready_count, 0);
        assert_eq!(validator.observation_reason_audit_rejected_count, 0);
        assert_eq!(validator.source_validated_count, 7);
        assert_eq!(validator.source_rejected_count, 0);
        assert_eq!(validator.generated_request_count, 7);
        assert_eq!(validator.generated_recorded_request_count, 4);
        assert_eq!(validator.generated_waived_request_count, 3);
        assert!(validator.decision_requests.iter().all(|request| {
            request.source_validator_bound
                && request.source_validator_contract_audit_ready
                && !request.source_validator_reason_audit_bound
                && !request.source_validator_reason_audit_ready
        }));
        assert_eq!(
            validator.source_validator_route,
            "status_canary_evidence_source_validator_ready_validated_sources"
        );
        assert_eq!(acceptance.request_count, 7);
        assert_eq!(acceptance.request_source_validator_bound_count, 7);
        assert_eq!(
            acceptance.request_source_validator_contract_audit_ready_count,
            7
        );
        assert_eq!(acceptance.request_reason_audit_count, 0);
        assert_eq!(acceptance.request_reason_audit_ready_count, 0);
        assert_eq!(acceptance.request_reason_audit_rejected_count, 0);
        assert_eq!(acceptance.accepted_decision_count, 7);
        assert_eq!(acceptance.rejected_decision_count, 0);
        assert_eq!(acceptance.generated_override_count, 7);
        assert!(acceptance.source_evidence_packet_complete);
        assert_eq!(
            validator.side_effects,
            StatusCanaryEvidenceSourceValidatorSideEffects::none()
        );
    }

    #[test]
    fn reason_bound_observation_with_ready_reason_audit_reaches_acceptance_overlay() {
        let mut observation =
            StatusCanaryEvidenceSourceObservation::recorded("operator_live_approval_missing");
        observation.source_readback_reason_packet_bound = true;
        observation.source_readback_reason_packet_ready = true;
        observation.source_readback_reason_packet_route =
            "status_canary_evidence_source_reason_packet_ready_inputs_valid";
        observation.source_readback_fixture_reason_audit_ready = true;
        observation.source_readback_fixture_reason_audit_rejection_reason =
            "fixture_generation_allowed";
        let validator = status_canary_evidence_source_validator_from_observations(&[observation]);
        let acceptance =
            status_canary_evidence_acceptance_packet_from_requests(&validator.decision_requests);
        let evidence_packet =
            status_canary_evidence_packet_from_decisions(&acceptance.decision_overrides);

        assert!(validator.source_validator_ready);
        assert_eq!(validator.observation_reason_audit_count, 1);
        assert_eq!(validator.observation_reason_audit_ready_count, 1);
        assert_eq!(validator.observation_reason_audit_rejected_count, 0);
        assert_eq!(validator.generated_request_count, 1);
        assert!(validator.decision_requests.iter().all(|request| {
            request.source_validator_bound
                && request.source_validator_contract_audit_ready
                && request.source_validator_reason_audit_bound
                && request.source_validator_reason_audit_ready
                && request.source_readback_fixture_reason_audit_rejection_reason
                    == "fixture_generation_allowed"
        }));
        assert_eq!(acceptance.request_reason_audit_count, 1);
        assert_eq!(acceptance.request_reason_audit_ready_count, 1);
        assert_eq!(acceptance.request_reason_audit_rejected_count, 0);
        assert_eq!(acceptance.accepted_decision_count, 1);
        assert_eq!(acceptance.rejected_decision_count, 0);
        assert_eq!(acceptance.generated_override_count, 1);
        assert_eq!(acceptance.generated_override_reason_audit_ready_count, 1);
        assert_eq!(evidence_packet.decision_reason_audit_count, 1);
        assert_eq!(evidence_packet.decision_reason_audit_ready_count, 1);
        assert_eq!(evidence_packet.decision_reason_audit_rejected_count, 0);
        assert_eq!(evidence_packet.recorded_item_count, 1);
        assert_eq!(evidence_packet.missing_item_count, 6);
    }

    #[test]
    fn expired_and_invalid_sources_generate_requests_but_keep_start_guard_blocked() {
        let ids = blocker_ids();
        let observations = ids
            .iter()
            .enumerate()
            .map(|(index, source_blocker_id)| match index {
                0 | 1 => StatusCanaryEvidenceSourceObservation::recorded(source_blocker_id),
                2 | 3 => StatusCanaryEvidenceSourceObservation::waived(source_blocker_id),
                4 | 5 => StatusCanaryEvidenceSourceObservation::expired(source_blocker_id),
                _ => StatusCanaryEvidenceSourceObservation::invalid(source_blocker_id),
            })
            .collect::<Vec<_>>();
        let validator = status_canary_evidence_source_validator_from_observations(&observations);
        let acceptance =
            status_canary_evidence_acceptance_packet_from_requests(&validator.decision_requests);
        let evidence_packet =
            status_canary_evidence_packet_from_decisions(&acceptance.decision_overrides);
        let guard = status_canary_start_guard_from_packet(
            &evidence_packet,
            StatusCanaryStartGuardInput {
                canary_start_switch_enabled: true,
            },
        );

        assert!(validator.source_validator_ready);
        assert_eq!(validator.source_validated_count, 7);
        assert_eq!(validator.generated_expired_request_count, 2);
        assert_eq!(validator.generated_invalid_request_count, 1);
        assert_eq!(acceptance.source_evidence_packet_expired_count, 2);
        assert_eq!(acceptance.source_evidence_packet_invalid_count, 1);
        assert!(!acceptance.source_evidence_packet_complete);
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
    fn malformed_source_is_rejected_before_acceptance_request_generation() {
        let mut observation =
            StatusCanaryEvidenceSourceObservation::recorded("operator_live_approval_missing");
        observation.evidence_digest_present = false;
        let validator = status_canary_evidence_source_validator_from_observations(&[observation]);

        assert!(validator.source_validator_ready);
        assert_eq!(validator.observation_count, 1);
        assert_eq!(validator.source_validated_count, 0);
        assert_eq!(validator.source_rejected_count, 1);
        assert_eq!(validator.generated_request_count, 0);
        assert_eq!(validator.source_acceptance_request_count, 0);
        assert_eq!(
            validator.source_validator_route,
            "status_canary_evidence_source_validator_blocked_rejected_source"
        );
        assert!(validator.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.observation_contract_audit_ready
                && entry.source_rejected
                && entry.rejection_reason == "missing_recorded_source_digest"
                && !entry.request_generated
        }));
    }

    #[test]
    fn observation_without_contract_audit_fails_before_acceptance_request_generation() {
        let mut observation =
            StatusCanaryEvidenceSourceObservation::recorded("operator_live_approval_missing");
        observation.source_readback_contract_audit_ready = false;
        let validator = status_canary_evidence_source_validator_from_observations(&[observation]);

        assert!(!validator.source_validator_ready);
        assert_eq!(validator.observation_count, 1);
        assert_eq!(validator.observation_contract_audit_count, 1);
        assert_eq!(validator.observation_contract_audit_ready_count, 0);
        assert_eq!(validator.observation_contract_audit_rejected_count, 1);
        assert_eq!(validator.source_validated_count, 0);
        assert_eq!(validator.source_rejected_count, 1);
        assert_eq!(validator.generated_request_count, 0);
        assert_eq!(validator.source_acceptance_request_count, 0);
        assert_eq!(
            validator.source_validator_route,
            "status_canary_evidence_source_validator_blocked_rejected_source"
        );
        assert!(validator.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && !entry.observation_contract_audit_ready
                && entry.source_rejected
                && entry.rejection_reason == "source_observation_contract_audit_not_ready"
                && !entry.request_generated
        }));
    }

    #[test]
    fn reason_bound_observation_without_reason_audit_fails_before_acceptance_request_generation() {
        let mut observation =
            StatusCanaryEvidenceSourceObservation::recorded("operator_live_approval_missing");
        observation.source_readback_reason_packet_bound = true;
        observation.source_readback_reason_packet_ready = true;
        observation.source_readback_reason_packet_route =
            "status_canary_evidence_source_reason_packet_ready_inputs_valid";
        observation.source_readback_fixture_reason_audit_ready = false;
        observation.source_readback_fixture_reason_audit_rejection_reason =
            "source_adapter_input_missing_for_decision";
        let validator = status_canary_evidence_source_validator_from_observations(&[observation]);

        assert!(!validator.source_validator_ready);
        assert_eq!(validator.observation_count, 1);
        assert_eq!(validator.observation_contract_audit_count, 1);
        assert_eq!(validator.observation_contract_audit_ready_count, 1);
        assert_eq!(validator.observation_contract_audit_rejected_count, 0);
        assert_eq!(validator.observation_reason_audit_count, 1);
        assert_eq!(validator.observation_reason_audit_ready_count, 0);
        assert_eq!(validator.observation_reason_audit_rejected_count, 1);
        assert_eq!(validator.source_validated_count, 0);
        assert_eq!(validator.source_rejected_count, 1);
        assert_eq!(validator.generated_request_count, 0);
        assert_eq!(validator.source_acceptance_request_count, 0);
        assert_eq!(
            validator.source_validator_route,
            "status_canary_evidence_source_validator_blocked_rejected_source"
        );
        assert!(validator.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.observation_contract_audit_ready
                && !entry.observation_reason_audit_ready
                && entry.source_rejected
                && entry.rejection_reason == "source_adapter_input_missing_for_decision"
                && !entry.request_generated
        }));
    }

    #[test]
    fn unknown_and_duplicate_sources_fail_closed_without_generated_requests() {
        let duplicate = StatusCanaryEvidenceSourceObservation::recorded("dirty_worktree_boundary");
        let unknown = StatusCanaryEvidenceSourceObservation::recorded("unknown_blocker");
        let validator = status_canary_evidence_source_validator_from_observations(&[
            duplicate, duplicate, unknown,
        ]);

        assert!(!validator.source_validator_ready);
        assert_eq!(validator.observation_count, 3);
        assert_eq!(validator.known_observation_count, 2);
        assert_eq!(validator.unknown_observation_count, 1);
        assert_eq!(validator.duplicate_observation_count, 1);
        assert_eq!(validator.source_validated_count, 0);
        assert_eq!(validator.source_rejected_count, 1);
        assert_eq!(validator.generated_request_count, 0);
        assert_eq!(
            validator.source_validator_route,
            "status_canary_evidence_source_validator_blocked_unknown_source"
        );
        assert!(validator.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.duplicate_observation
                && entry.source_rejected
                && entry.rejection_reason == "duplicate_source_observation"
        }));
        assert_eq!(
            validator.side_effects,
            StatusCanaryEvidenceSourceValidatorSideEffects::none()
        );
    }
}
