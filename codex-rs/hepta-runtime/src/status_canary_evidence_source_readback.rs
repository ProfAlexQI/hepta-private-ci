use crate::controlled_live_required_evidence_collection_plan::ControlledLiveRequiredEvidenceCollectionPlanEntry;
use crate::controlled_live_required_evidence_collection_plan::controlled_live_required_evidence_collection_plan_report;
use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;
use crate::status_canary_evidence_packet::StatusCanaryEvidenceDecision;
use crate::status_canary_evidence_source_reason_packet::STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_ID;
use crate::status_canary_evidence_source_reason_packet::StatusCanaryEvidenceSourceReasonPacketReport;
use crate::status_canary_evidence_source_validator::StatusCanaryEvidenceSourceObservation;
use serde::Serialize;

pub const STATUS_CANARY_EVIDENCE_SOURCE_READBACK_SCHEMA_VERSION: &str =
    "status_canary_evidence_source_readback_v1";
pub const STATUS_CANARY_EVIDENCE_SOURCE_READBACK_ID: &str =
    "status-canary-evidence-source-readback/hepta-system-status/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub schema_version: &'static str,
    pub readback_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_required_evidence_collection_plan_ready: bool,
    pub source_item_count: usize,
    pub fixture_input_count: usize,
    pub known_fixture_count: usize,
    pub unknown_fixture_count: usize,
    pub duplicate_fixture_count: usize,
    pub observation_count: usize,
    pub recorded_observation_count: usize,
    pub waived_observation_count: usize,
    pub expired_observation_count: usize,
    pub invalid_observation_count: usize,
    pub missing_observation_count: usize,
    pub rejected_fixture_count: usize,
    pub source_contract_audit_count: usize,
    pub source_contract_audit_ready_count: usize,
    pub fixture_contract_audit_ready_count: usize,
    pub source_reason_packet_bound: bool,
    pub source_reason_packet_ready: bool,
    pub source_reason_packet_route: &'static str,
    pub fixture_reason_audit_count: usize,
    pub fixture_reason_audit_ready_count: usize,
    pub fixture_reason_audit_rejected_count: usize,
    pub source_readback_ready: bool,
    pub source_readback_route: &'static str,
    pub entries: Vec<StatusCanaryEvidenceSourceReadbackEntry>,
    pub observations: Vec<StatusCanaryEvidenceSourceObservation>,
    pub side_effects: StatusCanaryEvidenceSourceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceReadbackEntry {
    pub source_blocker_id: &'static str,
    pub source_readback_key: &'static str,
    pub source_readback_route: &'static str,
    pub source_kind: &'static str,
    pub source_readback_contract_audit_route: &'static str,
    pub source_readback_contract_audit_ready: bool,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub fixture_present: bool,
    pub duplicate_fixture: bool,
    pub requested_decision: StatusCanaryEvidenceDecision,
    pub fixture_source_route: &'static str,
    pub fixture_source_route_present: bool,
    pub source_adapter_metadata_contract_route: &'static str,
    pub source_adapter_readback_fixture_contract_route: &'static str,
    pub source_adapter_metadata_contract_ready: bool,
    pub source_adapter_readback_fixture_contract_ready: bool,
    pub fixture_contract_audit_ready: bool,
    pub source_reason_packet_bound: bool,
    pub source_reason_packet_ready: bool,
    pub fixture_reason_audit_ready: bool,
    pub fixture_reason_audit_rejection_reason: &'static str,
    pub evidence_artifact_present: bool,
    pub evidence_digest_present: bool,
    pub operator_authority_present: bool,
    pub freshness_attested: bool,
    pub waiver_reason_present: bool,
    pub expiry_attested: bool,
    pub invalidity_reason_present: bool,
    pub readback_missing: bool,
    pub readback_rejected: bool,
    pub rejection_reason: &'static str,
    pub observation_generated: bool,
    pub generated_observation: Option<StatusCanaryEvidenceSourceObservation>,
    pub operator_visible: bool,
    pub queryable: bool,
    pub selected_status_canary_bound: bool,
    pub preflight_only_connector_excluded: bool,
    pub source_read_persistence_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub waiver_recording_allowed: bool,
    pub credential_read_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceReadbackFixture {
    pub source_blocker_id: &'static str,
    pub requested_decision: StatusCanaryEvidenceDecision,
    pub source_route: &'static str,
    pub source_adapter_metadata_contract_route: &'static str,
    pub source_adapter_readback_fixture_contract_route: &'static str,
    pub source_adapter_metadata_contract_ready: bool,
    pub source_adapter_readback_fixture_contract_ready: bool,
    pub evidence_artifact_present: bool,
    pub evidence_digest_present: bool,
    pub operator_authority_present: bool,
    pub freshness_attested: bool,
    pub waiver_reason_present: bool,
    pub expiry_attested: bool,
    pub invalidity_reason_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceReadbackSideEffects {
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

pub fn status_canary_evidence_source_readback() -> StatusCanaryEvidenceSourceReadbackReport {
    status_canary_evidence_source_readback_from_fixtures(&[])
}

pub fn status_canary_evidence_source_readback_from_fixtures(
    fixtures: &[StatusCanaryEvidenceSourceReadbackFixture],
) -> StatusCanaryEvidenceSourceReadbackReport {
    let source = controlled_live_required_evidence_collection_plan_report();
    status_canary_evidence_source_readback_from_plan_and_reason_packet(
        source.evidence_collection_plan_ready,
        source.entries,
        fixtures,
        None,
    )
}

pub fn status_canary_evidence_source_readback_from_fixtures_and_reason_packet(
    fixtures: &[StatusCanaryEvidenceSourceReadbackFixture],
    reason_packet: &StatusCanaryEvidenceSourceReasonPacketReport,
) -> StatusCanaryEvidenceSourceReadbackReport {
    let source = controlled_live_required_evidence_collection_plan_report();
    status_canary_evidence_source_readback_from_plan_and_reason_packet(
        source.evidence_collection_plan_ready,
        source.entries,
        fixtures,
        Some(reason_packet),
    )
}

pub fn status_canary_evidence_source_readback_from_plan(
    source_required_evidence_collection_plan_ready: bool,
    source_entries: Vec<ControlledLiveRequiredEvidenceCollectionPlanEntry>,
    fixtures: &[StatusCanaryEvidenceSourceReadbackFixture],
) -> StatusCanaryEvidenceSourceReadbackReport {
    status_canary_evidence_source_readback_from_plan_and_reason_packet(
        source_required_evidence_collection_plan_ready,
        source_entries,
        fixtures,
        None,
    )
}

fn status_canary_evidence_source_readback_from_plan_and_reason_packet(
    source_required_evidence_collection_plan_ready: bool,
    source_entries: Vec<ControlledLiveRequiredEvidenceCollectionPlanEntry>,
    fixtures: &[StatusCanaryEvidenceSourceReadbackFixture],
    reason_packet: Option<&StatusCanaryEvidenceSourceReasonPacketReport>,
) -> StatusCanaryEvidenceSourceReadbackReport {
    let known_fixture_count = fixtures
        .iter()
        .filter(|fixture| {
            source_entries
                .iter()
                .any(|entry| entry.source_blocker_id == fixture.source_blocker_id)
        })
        .count();
    let unknown_fixture_count = fixtures.len().saturating_sub(known_fixture_count);
    let source_reason_packet_bound = reason_packet
        .map(|packet| packet.reason_packet_id == STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_ID)
        .unwrap_or(false);
    let source_reason_packet_ready = reason_packet
        .map(|packet| source_reason_packet_bound && packet.reason_packet_ready)
        .unwrap_or(false);
    let source_reason_packet_route = reason_packet
        .map(|packet| packet.reason_packet_route)
        .unwrap_or("status_canary_evidence_source_reason_packet_not_bound_to_readback");
    let entries = source_entries
        .into_iter()
        .map(|entry| status_canary_evidence_source_readback_entry(entry, fixtures, reason_packet))
        .collect::<Vec<_>>();
    let duplicate_fixture_count = entries
        .iter()
        .filter(|entry| entry.duplicate_fixture)
        .count();
    let rejected_fixture_count = entries
        .iter()
        .filter(|entry| entry.readback_rejected)
        .count();
    let source_contract_audit_count = entries.len();
    let source_contract_audit_ready_count = entries
        .iter()
        .filter(|entry| entry.source_readback_contract_audit_ready)
        .count();
    let fixture_contract_audit_ready_count = entries
        .iter()
        .filter(|entry| entry.fixture_contract_audit_ready)
        .count();
    let fixture_reason_audit_count = entries
        .iter()
        .filter(|entry| entry.fixture_present && entry.source_reason_packet_bound)
        .count();
    let fixture_reason_audit_ready_count = entries
        .iter()
        .filter(|entry| entry.fixture_reason_audit_ready)
        .count();
    let fixture_reason_audit_rejected_count = entries
        .iter()
        .filter(|entry| {
            entry.fixture_present
                && entry.source_reason_packet_bound
                && !entry.fixture_reason_audit_ready
        })
        .count();
    let observations = entries
        .iter()
        .filter_map(|entry| entry.generated_observation)
        .collect::<Vec<_>>();
    let missing_observation_count = entries.len().saturating_sub(observations.len());
    let side_effects = StatusCanaryEvidenceSourceReadbackSideEffects::none();
    let source_readback_ready = source_required_evidence_collection_plan_ready
        && entries.len() == 7
        && unknown_fixture_count == 0
        && duplicate_fixture_count == 0
        && rejected_fixture_count == 0
        && source_contract_audit_count == 7
        && source_contract_audit_ready_count == 7
        && reason_packet
            .map(|_| source_reason_packet_bound && source_reason_packet_ready)
            .unwrap_or(true)
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.source_readback_contract_audit_ready
                && entry.selected_status_canary_bound
                && entry.preflight_only_connector_excluded
                && !entry.source_read_persistence_allowed
                && !entry.evidence_recording_allowed
                && !entry.waiver_recording_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        })
        && side_effects == StatusCanaryEvidenceSourceReadbackSideEffects::none();

    StatusCanaryEvidenceSourceReadbackReport {
        runtime: "hepta",
        surface: "status_canary_evidence_source_readback",
        schema_version: STATUS_CANARY_EVIDENCE_SOURCE_READBACK_SCHEMA_VERSION,
        readback_id: STATUS_CANARY_EVIDENCE_SOURCE_READBACK_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_required_evidence_collection_plan_ready,
        source_item_count: entries.len(),
        fixture_input_count: fixtures.len(),
        known_fixture_count,
        unknown_fixture_count,
        duplicate_fixture_count,
        observation_count: observations.len(),
        recorded_observation_count: observations
            .iter()
            .filter(|observation| {
                observation.requested_decision == StatusCanaryEvidenceDecision::Recorded
            })
            .count(),
        waived_observation_count: observations
            .iter()
            .filter(|observation| {
                observation.requested_decision == StatusCanaryEvidenceDecision::Waived
            })
            .count(),
        expired_observation_count: observations
            .iter()
            .filter(|observation| {
                observation.requested_decision == StatusCanaryEvidenceDecision::Expired
            })
            .count(),
        invalid_observation_count: observations
            .iter()
            .filter(|observation| {
                observation.requested_decision == StatusCanaryEvidenceDecision::Invalid
            })
            .count(),
        missing_observation_count,
        rejected_fixture_count,
        source_contract_audit_count,
        source_contract_audit_ready_count,
        fixture_contract_audit_ready_count,
        source_reason_packet_bound,
        source_reason_packet_ready,
        source_reason_packet_route,
        fixture_reason_audit_count,
        fixture_reason_audit_ready_count,
        fixture_reason_audit_rejected_count,
        source_readback_ready,
        source_readback_route: status_canary_evidence_source_readback_route(
            source_required_evidence_collection_plan_ready,
            fixtures.len(),
            unknown_fixture_count,
            duplicate_fixture_count,
            rejected_fixture_count,
            observations.len(),
        ),
        entries,
        observations,
        side_effects,
    }
}

fn status_canary_evidence_source_readback_entry(
    source: ControlledLiveRequiredEvidenceCollectionPlanEntry,
    fixtures: &[StatusCanaryEvidenceSourceReadbackFixture],
    reason_packet: Option<&StatusCanaryEvidenceSourceReasonPacketReport>,
) -> StatusCanaryEvidenceSourceReadbackEntry {
    let matching_fixtures = fixtures
        .iter()
        .filter(|fixture| fixture.source_blocker_id == source.source_blocker_id)
        .collect::<Vec<_>>();
    let duplicate_fixture = matching_fixtures.len() > 1;
    let fixture = matching_fixtures.first().copied().copied();
    let reason_audit = status_canary_evidence_source_readback_reason_audit(fixture, reason_packet);
    let source_readback_contract_audit_route =
        status_canary_evidence_source_readback_contract_audit_route(source.source_blocker_id);
    let source_readback_contract_audit_ready =
        status_canary_evidence_source_readback_contract_audit_ready(source.source_blocker_id);
    let validation = fixture
        .map(|fixture| {
            status_canary_evidence_source_readback_validation(
                fixture,
                duplicate_fixture,
                source_readback_contract_audit_ready,
                reason_audit.source_reason_packet_bound,
                reason_audit.fixture_reason_audit_ready,
                reason_audit.rejection_reason,
            )
        })
        .unwrap_or(StatusCanaryEvidenceSourceReadbackValidation {
            valid: false,
            rejection_reason: "no_source_fixture",
        });
    let fixture_contract_audit_ready = fixture
        .map(status_canary_evidence_source_readback_fixture_contract_audit_ready)
        .unwrap_or(false);
    let observation_generated = fixture.is_some() && validation.valid;
    let generated_observation = if observation_generated {
        fixture.map(|fixture| StatusCanaryEvidenceSourceObservation {
            source_blocker_id: source.source_blocker_id,
            requested_decision: fixture.requested_decision,
            source_route: fixture.source_route,
            source_adapter_metadata_contract_route: fixture.source_adapter_metadata_contract_route,
            source_adapter_readback_fixture_contract_route: fixture
                .source_adapter_readback_fixture_contract_route,
            source_readback_contract_audit_route,
            source_adapter_contract_audit_ready: fixture_contract_audit_ready,
            source_readback_contract_audit_ready,
            source_readback_reason_packet_bound: reason_audit.source_reason_packet_bound,
            source_readback_reason_packet_ready: reason_audit.source_reason_packet_ready,
            source_readback_reason_packet_route: reason_audit.source_reason_packet_route,
            source_readback_fixture_reason_audit_ready: reason_audit.fixture_reason_audit_ready,
            source_readback_fixture_reason_audit_rejection_reason: reason_audit.rejection_reason,
            evidence_artifact_present: fixture.evidence_artifact_present,
            evidence_digest_present: fixture.evidence_digest_present,
            operator_authority_present: fixture.operator_authority_present,
            freshness_attested: fixture.freshness_attested,
            waiver_reason_present: fixture.waiver_reason_present,
            expiry_attested: fixture.expiry_attested,
            invalidity_reason_present: fixture.invalidity_reason_present,
        })
    } else {
        None
    };

    StatusCanaryEvidenceSourceReadbackEntry {
        source_blocker_id: source.source_blocker_id,
        source_readback_key: status_canary_evidence_source_readback_key(source.source_blocker_id),
        source_readback_route: status_canary_evidence_source_readback_entry_route(
            source.source_blocker_id,
        ),
        source_kind: status_canary_evidence_source_kind(source.source_blocker_id),
        source_readback_contract_audit_route,
        source_readback_contract_audit_ready,
        operator_label: source.operator_label,
        required_evidence: source.required_evidence,
        fixture_present: fixture.is_some(),
        duplicate_fixture,
        requested_decision: fixture
            .map(|fixture| fixture.requested_decision)
            .unwrap_or(StatusCanaryEvidenceDecision::Missing),
        fixture_source_route: fixture
            .map(|fixture| fixture.source_route)
            .unwrap_or("memory://status-canary/evidence-source/readback/no-fixture"),
        fixture_source_route_present: fixture
            .map(|fixture| !fixture.source_route.is_empty())
            .unwrap_or(false),
        source_adapter_metadata_contract_route: fixture
            .map(|fixture| fixture.source_adapter_metadata_contract_route)
            .unwrap_or("metadata://status-canary/evidence-source-adapter-contract/no-fixture"),
        source_adapter_readback_fixture_contract_route: fixture
            .map(|fixture| fixture.source_adapter_readback_fixture_contract_route)
            .unwrap_or(
                "metadata://status-canary/evidence-source-readback-fixture-contract/no-fixture",
            ),
        source_adapter_metadata_contract_ready: fixture
            .map(|fixture| fixture.source_adapter_metadata_contract_ready)
            .unwrap_or(false),
        source_adapter_readback_fixture_contract_ready: fixture
            .map(|fixture| fixture.source_adapter_readback_fixture_contract_ready)
            .unwrap_or(false),
        fixture_contract_audit_ready,
        source_reason_packet_bound: reason_audit.source_reason_packet_bound,
        source_reason_packet_ready: reason_audit.source_reason_packet_ready,
        fixture_reason_audit_ready: reason_audit.fixture_reason_audit_ready,
        fixture_reason_audit_rejection_reason: reason_audit.rejection_reason,
        evidence_artifact_present: fixture
            .map(|fixture| fixture.evidence_artifact_present)
            .unwrap_or(false),
        evidence_digest_present: fixture
            .map(|fixture| fixture.evidence_digest_present)
            .unwrap_or(false),
        operator_authority_present: fixture
            .map(|fixture| fixture.operator_authority_present)
            .unwrap_or(false),
        freshness_attested: fixture
            .map(|fixture| fixture.freshness_attested)
            .unwrap_or(false),
        waiver_reason_present: fixture
            .map(|fixture| fixture.waiver_reason_present)
            .unwrap_or(false),
        expiry_attested: fixture
            .map(|fixture| fixture.expiry_attested)
            .unwrap_or(false),
        invalidity_reason_present: fixture
            .map(|fixture| fixture.invalidity_reason_present)
            .unwrap_or(false),
        readback_missing: fixture.is_none(),
        readback_rejected: fixture.is_some() && !validation.valid,
        rejection_reason: validation.rejection_reason,
        observation_generated,
        generated_observation,
        operator_visible: true,
        queryable: true,
        selected_status_canary_bound: true,
        preflight_only_connector_excluded: true,
        source_read_persistence_allowed: false,
        evidence_recording_allowed: false,
        waiver_recording_allowed: false,
        credential_read_allowed: false,
        transport_mutation_allowed: false,
        persistence_allowed: false,
        live_mutation_allowed: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StatusCanaryEvidenceSourceReadbackValidation {
    valid: bool,
    rejection_reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StatusCanaryEvidenceSourceReadbackReasonAudit {
    source_reason_packet_bound: bool,
    source_reason_packet_ready: bool,
    source_reason_packet_route: &'static str,
    fixture_reason_audit_ready: bool,
    rejection_reason: &'static str,
}

fn status_canary_evidence_source_readback_reason_audit(
    fixture: Option<StatusCanaryEvidenceSourceReadbackFixture>,
    reason_packet: Option<&StatusCanaryEvidenceSourceReasonPacketReport>,
) -> StatusCanaryEvidenceSourceReadbackReasonAudit {
    let Some(reason_packet) = reason_packet else {
        return StatusCanaryEvidenceSourceReadbackReasonAudit {
            source_reason_packet_bound: false,
            source_reason_packet_ready: false,
            source_reason_packet_route: "status_canary_evidence_source_reason_packet_not_bound_to_readback",
            fixture_reason_audit_ready: false,
            rejection_reason: "source_reason_packet_not_bound_to_readback",
        };
    };

    let source_reason_packet_bound =
        reason_packet.reason_packet_id == STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_ID;
    let source_reason_packet_ready =
        source_reason_packet_bound && reason_packet.reason_packet_ready;
    let Some(fixture) = fixture else {
        return StatusCanaryEvidenceSourceReadbackReasonAudit {
            source_reason_packet_bound,
            source_reason_packet_ready,
            source_reason_packet_route: reason_packet.reason_packet_route,
            fixture_reason_audit_ready: false,
            rejection_reason: "no_source_fixture",
        };
    };

    if !source_reason_packet_bound {
        return StatusCanaryEvidenceSourceReadbackReasonAudit {
            source_reason_packet_bound,
            source_reason_packet_ready,
            source_reason_packet_route: reason_packet.reason_packet_route,
            fixture_reason_audit_ready: false,
            rejection_reason: "source_reason_packet_id_not_bound",
        };
    }
    if !source_reason_packet_ready {
        return StatusCanaryEvidenceSourceReadbackReasonAudit {
            source_reason_packet_bound,
            source_reason_packet_ready,
            source_reason_packet_route: reason_packet.reason_packet_route,
            fixture_reason_audit_ready: false,
            rejection_reason: "source_reason_packet_not_ready",
        };
    }

    let reason = reason_packet.entries.iter().find(|entry| {
        entry.source_blocker_id == fixture.source_blocker_id
            && entry.requested_decision == fixture.requested_decision
    });
    let Some(reason) = reason else {
        return StatusCanaryEvidenceSourceReadbackReasonAudit {
            source_reason_packet_bound,
            source_reason_packet_ready,
            source_reason_packet_route: reason_packet.reason_packet_route,
            fixture_reason_audit_ready: false,
            rejection_reason: "source_reason_packet_entry_missing",
        };
    };

    StatusCanaryEvidenceSourceReadbackReasonAudit {
        source_reason_packet_bound,
        source_reason_packet_ready,
        source_reason_packet_route: reason_packet.reason_packet_route,
        fixture_reason_audit_ready: reason.fixture_generation_allowed,
        rejection_reason: reason.fixture_generation_blocker_reason,
    }
}

fn status_canary_evidence_source_readback_validation(
    fixture: StatusCanaryEvidenceSourceReadbackFixture,
    duplicate_fixture: bool,
    source_readback_contract_audit_ready: bool,
    fixture_reason_audit_required: bool,
    fixture_reason_audit_ready: bool,
    fixture_reason_audit_rejection_reason: &'static str,
) -> StatusCanaryEvidenceSourceReadbackValidation {
    if duplicate_fixture {
        return StatusCanaryEvidenceSourceReadbackValidation {
            valid: false,
            rejection_reason: "duplicate_source_fixture",
        };
    }
    if !source_readback_contract_audit_ready {
        return StatusCanaryEvidenceSourceReadbackValidation {
            valid: false,
            rejection_reason: "source_readback_contract_audit_not_ready",
        };
    }
    if !status_canary_evidence_source_readback_fixture_contract_audit_ready(fixture) {
        return StatusCanaryEvidenceSourceReadbackValidation {
            valid: false,
            rejection_reason: "source_adapter_contract_audit_not_ready",
        };
    }
    if fixture.source_route.is_empty() {
        return StatusCanaryEvidenceSourceReadbackValidation {
            valid: false,
            rejection_reason: "missing_source_fixture_route",
        };
    }
    if fixture.requested_decision == StatusCanaryEvidenceDecision::Missing {
        return StatusCanaryEvidenceSourceReadbackValidation {
            valid: false,
            rejection_reason: "missing_is_default_not_source_fixture",
        };
    }
    if fixture_reason_audit_required && !fixture_reason_audit_ready {
        return StatusCanaryEvidenceSourceReadbackValidation {
            valid: false,
            rejection_reason: fixture_reason_audit_rejection_reason,
        };
    }

    StatusCanaryEvidenceSourceReadbackValidation {
        valid: true,
        rejection_reason: "accepted",
    }
}

fn status_canary_evidence_source_readback_fixture_contract_audit_ready(
    fixture: StatusCanaryEvidenceSourceReadbackFixture,
) -> bool {
    fixture.source_adapter_metadata_contract_ready
        && fixture.source_adapter_readback_fixture_contract_ready
        && fixture
            .source_adapter_metadata_contract_route
            .starts_with("metadata://status-canary/evidence-source-adapter-contract/")
        && fixture
            .source_adapter_readback_fixture_contract_route
            .starts_with("metadata://status-canary/evidence-source-readback-fixture-contract/")
        && fixture.source_adapter_metadata_contract_route
            != "metadata://status-canary/evidence-source-adapter-contract/unknown"
        && fixture.source_adapter_readback_fixture_contract_route
            != "metadata://status-canary/evidence-source-readback-fixture-contract/unknown"
}

fn status_canary_evidence_source_readback_contract_audit_route(
    source_blocker_id: &str,
) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "audit://status-canary/evidence-source-readback-contract/clean-worktree-snapshot"
        }
        "operator_live_approval_missing" => {
            "audit://status-canary/evidence-source-readback-contract/operator-live-approval"
        }
        "fresh_soak_readback_missing" => {
            "audit://status-canary/evidence-source-readback-contract/fresh-status-canary-soak"
        }
        "credential_boundary_attestation_missing" => {
            "audit://status-canary/evidence-source-readback-contract/credential-boundary-attestation"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "audit://status-canary/evidence-source-readback-contract/transport-boundary-approval"
        }
        "rollback_rehearsal_missing" => {
            "audit://status-canary/evidence-source-readback-contract/rollback-rehearsal"
        }
        "kill_switch_rehearsal_missing" => {
            "audit://status-canary/evidence-source-readback-contract/kill-switch-rehearsal"
        }
        _ => "audit://status-canary/evidence-source-readback-contract/unknown",
    }
}

fn status_canary_evidence_source_readback_contract_audit_ready(source_blocker_id: &str) -> bool {
    status_canary_evidence_source_kind(source_blocker_id)
        != "unknown_status_canary_evidence_source_readback"
        && status_canary_evidence_source_readback_contract_audit_route(source_blocker_id)
            != "audit://status-canary/evidence-source-readback-contract/unknown"
}

fn status_canary_evidence_source_readback_route(
    source_required_evidence_collection_plan_ready: bool,
    fixture_input_count: usize,
    unknown_fixture_count: usize,
    duplicate_fixture_count: usize,
    rejected_fixture_count: usize,
    observation_count: usize,
) -> &'static str {
    if !source_required_evidence_collection_plan_ready {
        "status_canary_evidence_source_readback_blocked_source_plan_not_ready"
    } else if unknown_fixture_count > 0 {
        "status_canary_evidence_source_readback_blocked_unknown_fixture"
    } else if duplicate_fixture_count > 0 {
        "status_canary_evidence_source_readback_blocked_duplicate_fixture"
    } else if rejected_fixture_count > 0 {
        "status_canary_evidence_source_readback_blocked_rejected_fixture"
    } else if fixture_input_count == 0 {
        "status_canary_evidence_source_readback_ready_no_fixtures"
    } else if observation_count > 0 {
        "status_canary_evidence_source_readback_ready_observations_generated"
    } else {
        "status_canary_evidence_source_readback_blocked_unknown"
    }
}

fn status_canary_evidence_source_readback_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "status_canary.evidence_readback.dirty_worktree_boundary",
        "operator_live_approval_missing" => {
            "status_canary.evidence_readback.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "status_canary.evidence_readback.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "status_canary.evidence_readback.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "status_canary.evidence_readback.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "status_canary.evidence_readback.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "status_canary.evidence_readback.kill_switch_rehearsal_missing"
        }
        _ => "status_canary.evidence_readback.unknown",
    }
}

fn status_canary_evidence_source_readback_entry_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "readback://status-canary/evidence-source-readback/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "readback://status-canary/evidence-source-readback/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "readback://status-canary/evidence-source-readback/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "readback://status-canary/evidence-source-readback/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "readback://status-canary/evidence-source-readback/transport-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "readback://status-canary/evidence-source-readback/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "readback://status-canary/evidence-source-readback/kill-switch-rehearsal-missing"
        }
        _ => "readback://status-canary/evidence-source-readback/unknown",
    }
}

fn status_canary_evidence_source_kind(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "clean_worktree_snapshot_source_readback",
        "operator_live_approval_missing" => "operator_live_approval_packet_source_readback",
        "fresh_soak_readback_missing" => "fresh_status_canary_soak_readback_source_readback",
        "credential_boundary_attestation_missing" => {
            "credential_boundary_attestation_source_readback"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "transport_boundary_approval_source_readback"
        }
        "rollback_rehearsal_missing" => "rollback_rehearsal_packet_source_readback",
        "kill_switch_rehearsal_missing" => "kill_switch_rehearsal_packet_source_readback",
        _ => "unknown_status_canary_evidence_source_readback",
    }
}

impl StatusCanaryEvidenceSourceReadbackFixture {
    pub const fn recorded(source_blocker_id: &'static str) -> Self {
        Self {
            source_blocker_id,
            requested_decision: StatusCanaryEvidenceDecision::Recorded,
            source_route: "readback://status-canary/evidence-source-readback/test-recorded",
            source_adapter_metadata_contract_route: "metadata://status-canary/evidence-source-adapter-contract/test-recorded",
            source_adapter_readback_fixture_contract_route: "metadata://status-canary/evidence-source-readback-fixture-contract/test-recorded",
            source_adapter_metadata_contract_ready: true,
            source_adapter_readback_fixture_contract_ready: true,
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
            source_route: "readback://status-canary/evidence-source-readback/test-waived",
            source_adapter_metadata_contract_route: "metadata://status-canary/evidence-source-adapter-contract/test-waived",
            source_adapter_readback_fixture_contract_route: "metadata://status-canary/evidence-source-readback-fixture-contract/test-waived",
            source_adapter_metadata_contract_ready: true,
            source_adapter_readback_fixture_contract_ready: true,
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
            source_route: "readback://status-canary/evidence-source-readback/test-expired",
            source_adapter_metadata_contract_route: "metadata://status-canary/evidence-source-adapter-contract/test-expired",
            source_adapter_readback_fixture_contract_route: "metadata://status-canary/evidence-source-readback-fixture-contract/test-expired",
            source_adapter_metadata_contract_ready: true,
            source_adapter_readback_fixture_contract_ready: true,
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
            source_route: "readback://status-canary/evidence-source-readback/test-invalid",
            source_adapter_metadata_contract_route: "metadata://status-canary/evidence-source-adapter-contract/test-invalid",
            source_adapter_readback_fixture_contract_route: "metadata://status-canary/evidence-source-readback-fixture-contract/test-invalid",
            source_adapter_metadata_contract_ready: true,
            source_adapter_readback_fixture_contract_ready: true,
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

impl StatusCanaryEvidenceSourceReadbackSideEffects {
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
    use crate::status_canary_evidence_source_adapter::StatusCanaryEvidenceSourceAdapterInput;
    use crate::status_canary_evidence_source_adapter::status_canary_evidence_source_adapter;
    use crate::status_canary_evidence_source_adapter::status_canary_evidence_source_adapter_from_inputs;
    use crate::status_canary_evidence_source_reason_packet::status_canary_evidence_source_reason_packet_from_adapter;
    use crate::status_canary_evidence_source_validator::status_canary_evidence_source_validator_from_observations;
    use crate::status_canary_start_guard::StatusCanaryStartGuardInput;
    use crate::status_canary_start_guard::status_canary_start_guard_from_packet;

    fn blocker_ids() -> Vec<&'static str> {
        controlled_live_required_evidence_collection_plan_report()
            .entries
            .iter()
            .map(|entry| entry.source_blocker_id)
            .collect()
    }

    #[test]
    fn default_readback_has_no_fixtures_and_no_observations() {
        let readback = status_canary_evidence_source_readback();

        assert_eq!(
            readback.readback_id,
            STATUS_CANARY_EVIDENCE_SOURCE_READBACK_ID
        );
        assert!(readback.source_readback_ready);
        assert_eq!(
            readback.source_readback_route,
            "status_canary_evidence_source_readback_ready_no_fixtures"
        );
        assert_eq!(readback.source_item_count, 7);
        assert_eq!(readback.fixture_input_count, 0);
        assert_eq!(readback.known_fixture_count, 0);
        assert_eq!(readback.unknown_fixture_count, 0);
        assert_eq!(readback.duplicate_fixture_count, 0);
        assert_eq!(readback.observation_count, 0);
        assert_eq!(readback.missing_observation_count, 7);
        assert_eq!(readback.source_contract_audit_count, 7);
        assert_eq!(readback.source_contract_audit_ready_count, 7);
        assert_eq!(readback.fixture_contract_audit_ready_count, 0);
        assert!(!readback.source_reason_packet_bound);
        assert!(!readback.source_reason_packet_ready);
        assert_eq!(readback.fixture_reason_audit_count, 0);
        assert_eq!(readback.fixture_reason_audit_ready_count, 0);
        assert_eq!(readback.fixture_reason_audit_rejected_count, 0);
        assert_eq!(
            readback.side_effects,
            StatusCanaryEvidenceSourceReadbackSideEffects::none()
        );
        assert!(readback.entries.iter().all(|entry| {
            !entry.fixture_present
                && entry.readback_missing
                && !entry.readback_rejected
                && !entry.observation_generated
                && entry.generated_observation.is_none()
                && entry.source_readback_contract_audit_ready
                && !entry.fixture_contract_audit_ready
                && !entry.source_read_persistence_allowed
                && !entry.evidence_recording_allowed
                && !entry.waiver_recording_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        }));
    }

    #[test]
    fn recorded_and_waived_fixtures_generate_validator_observations() {
        let ids = blocker_ids();
        let fixtures = ids
            .iter()
            .enumerate()
            .map(|(index, source_blocker_id)| {
                if index < 4 {
                    StatusCanaryEvidenceSourceReadbackFixture::recorded(source_blocker_id)
                } else {
                    StatusCanaryEvidenceSourceReadbackFixture::waived(source_blocker_id)
                }
            })
            .collect::<Vec<_>>();
        let readback = status_canary_evidence_source_readback_from_fixtures(&fixtures);
        let validator =
            status_canary_evidence_source_validator_from_observations(&readback.observations);
        let acceptance =
            status_canary_evidence_acceptance_packet_from_requests(&validator.decision_requests);

        assert!(readback.source_readback_ready);
        assert_eq!(readback.fixture_input_count, 7);
        assert_eq!(readback.observation_count, 7);
        assert_eq!(readback.source_contract_audit_ready_count, 7);
        assert_eq!(readback.fixture_contract_audit_ready_count, 7);
        assert_eq!(readback.recorded_observation_count, 4);
        assert_eq!(readback.waived_observation_count, 3);
        assert_eq!(
            readback.source_readback_route,
            "status_canary_evidence_source_readback_ready_observations_generated"
        );
        assert_eq!(validator.source_validated_count, 7);
        assert_eq!(validator.observation_contract_audit_ready_count, 7);
        assert_eq!(validator.observation_reason_audit_count, 0);
        assert_eq!(validator.observation_reason_audit_ready_count, 0);
        assert_eq!(validator.observation_reason_audit_rejected_count, 0);
        assert_eq!(validator.generated_request_count, 7);
        assert_eq!(acceptance.request_count, 7);
        assert_eq!(acceptance.generated_override_count, 7);
        assert!(acceptance.source_evidence_packet_complete);
    }

    #[test]
    fn adapter_reason_packet_allows_matching_fixture_readback() {
        let ids = blocker_ids();
        let inputs = ids
            .iter()
            .enumerate()
            .map(|(index, source_blocker_id)| {
                if index < 4 {
                    StatusCanaryEvidenceSourceAdapterInput::recorded(source_blocker_id)
                } else {
                    StatusCanaryEvidenceSourceAdapterInput::waived(source_blocker_id)
                }
            })
            .collect::<Vec<_>>();
        let adapter = status_canary_evidence_source_adapter_from_inputs(&inputs);
        let reason_packet = status_canary_evidence_source_reason_packet_from_adapter(&adapter);
        let readback = status_canary_evidence_source_readback_from_fixtures_and_reason_packet(
            &adapter.generated_fixtures,
            &reason_packet,
        );
        let validator =
            status_canary_evidence_source_validator_from_observations(&readback.observations);

        assert!(adapter.source_adapter_ready);
        assert!(reason_packet.reason_packet_ready);
        assert!(readback.source_readback_ready);
        assert!(readback.source_reason_packet_bound);
        assert!(readback.source_reason_packet_ready);
        assert_eq!(readback.fixture_input_count, 7);
        assert_eq!(readback.fixture_reason_audit_count, 7);
        assert_eq!(readback.fixture_reason_audit_ready_count, 7);
        assert_eq!(readback.fixture_reason_audit_rejected_count, 0);
        assert_eq!(readback.observation_count, 7);
        assert_eq!(validator.observation_reason_audit_count, 7);
        assert_eq!(validator.observation_reason_audit_ready_count, 7);
        assert_eq!(validator.observation_reason_audit_rejected_count, 0);
        assert_eq!(validator.generated_request_count, 7);
        assert!(readback.entries.iter().all(|entry| {
            entry.fixture_reason_audit_ready
                && entry.fixture_reason_audit_rejection_reason == "fixture_generation_allowed"
        }));
    }

    #[test]
    fn reason_packet_blocks_fixture_without_matching_adapter_input() {
        let adapter = status_canary_evidence_source_adapter();
        let reason_packet = status_canary_evidence_source_reason_packet_from_adapter(&adapter);
        let fixture =
            StatusCanaryEvidenceSourceReadbackFixture::recorded("dirty_worktree_boundary");
        let readback = status_canary_evidence_source_readback_from_fixtures_and_reason_packet(
            &[fixture],
            &reason_packet,
        );

        assert!(adapter.source_adapter_ready);
        assert!(reason_packet.reason_packet_ready);
        assert!(!readback.source_readback_ready);
        assert!(readback.source_reason_packet_bound);
        assert!(readback.source_reason_packet_ready);
        assert_eq!(readback.fixture_reason_audit_count, 1);
        assert_eq!(readback.fixture_reason_audit_ready_count, 0);
        assert_eq!(readback.fixture_reason_audit_rejected_count, 1);
        assert_eq!(readback.rejected_fixture_count, 1);
        assert_eq!(readback.observation_count, 0);
        assert!(readback.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.fixture_present
                && entry.source_reason_packet_bound
                && !entry.fixture_reason_audit_ready
                && entry.fixture_reason_audit_rejection_reason
                    == "source_adapter_input_missing_for_decision"
                && entry.readback_rejected
                && entry.rejection_reason == "source_adapter_input_missing_for_decision"
                && !entry.observation_generated
        }));
    }

    #[test]
    fn expired_and_invalid_fixtures_reach_validator_but_keep_start_guard_blocked() {
        let ids = blocker_ids();
        let fixtures = ids
            .iter()
            .enumerate()
            .map(|(index, source_blocker_id)| match index {
                0 | 1 => StatusCanaryEvidenceSourceReadbackFixture::recorded(source_blocker_id),
                2 | 3 => StatusCanaryEvidenceSourceReadbackFixture::waived(source_blocker_id),
                4 | 5 => StatusCanaryEvidenceSourceReadbackFixture::expired(source_blocker_id),
                _ => StatusCanaryEvidenceSourceReadbackFixture::invalid(source_blocker_id),
            })
            .collect::<Vec<_>>();
        let readback = status_canary_evidence_source_readback_from_fixtures(&fixtures);
        let validator =
            status_canary_evidence_source_validator_from_observations(&readback.observations);
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

        assert!(readback.source_readback_ready);
        assert_eq!(readback.observation_count, 7);
        assert_eq!(readback.expired_observation_count, 2);
        assert_eq!(readback.invalid_observation_count, 1);
        assert_eq!(validator.generated_expired_request_count, 2);
        assert_eq!(validator.generated_invalid_request_count, 1);
        assert!(!acceptance.source_evidence_packet_complete);
        assert!(guard.canary_start_blocked);
        assert!(!guard.canary_start_allowed);
        assert_eq!(
            guard.guard_route,
            "status_canary_start_blocked_expired_evidence_packet"
        );
    }

    #[test]
    fn unknown_and_duplicate_fixtures_fail_closed_without_observations() {
        let duplicate =
            StatusCanaryEvidenceSourceReadbackFixture::recorded("dirty_worktree_boundary");
        let unknown = StatusCanaryEvidenceSourceReadbackFixture::recorded("unknown_blocker");
        let readback =
            status_canary_evidence_source_readback_from_fixtures(&[duplicate, duplicate, unknown]);

        assert!(!readback.source_readback_ready);
        assert_eq!(readback.fixture_input_count, 3);
        assert_eq!(readback.known_fixture_count, 2);
        assert_eq!(readback.unknown_fixture_count, 1);
        assert_eq!(readback.duplicate_fixture_count, 1);
        assert_eq!(readback.observation_count, 0);
        assert_eq!(
            readback.source_readback_route,
            "status_canary_evidence_source_readback_blocked_unknown_fixture"
        );
        assert!(readback.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.duplicate_fixture
                && entry.readback_rejected
                && entry.rejection_reason == "duplicate_source_fixture"
                && !entry.observation_generated
        }));
    }

    #[test]
    fn malformed_fixture_can_reach_validator_and_fail_source_validation() {
        let mut fixture =
            StatusCanaryEvidenceSourceReadbackFixture::recorded("operator_live_approval_missing");
        fixture.evidence_digest_present = false;
        let readback = status_canary_evidence_source_readback_from_fixtures(&[fixture]);
        let validator =
            status_canary_evidence_source_validator_from_observations(&readback.observations);

        assert!(readback.source_readback_ready);
        assert_eq!(readback.observation_count, 1);
        assert_eq!(
            readback.source_readback_route,
            "status_canary_evidence_source_readback_ready_observations_generated"
        );
        assert_eq!(validator.source_validated_count, 0);
        assert_eq!(validator.source_rejected_count, 1);
        assert_eq!(validator.generated_request_count, 0);
        assert!(validator.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.rejection_reason == "missing_recorded_source_digest"
        }));
    }

    #[test]
    fn fixture_without_contract_audit_fails_before_observation_generation() {
        let mut fixture =
            StatusCanaryEvidenceSourceReadbackFixture::recorded("operator_live_approval_missing");
        fixture.source_adapter_metadata_contract_ready = false;
        let readback = status_canary_evidence_source_readback_from_fixtures(&[fixture]);

        assert!(!readback.source_readback_ready);
        assert_eq!(readback.fixture_input_count, 1);
        assert_eq!(readback.observation_count, 0);
        assert_eq!(readback.rejected_fixture_count, 1);
        assert_eq!(readback.fixture_contract_audit_ready_count, 0);
        assert_eq!(
            readback.source_readback_route,
            "status_canary_evidence_source_readback_blocked_rejected_fixture"
        );
        assert!(readback.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.readback_rejected
                && !entry.fixture_contract_audit_ready
                && entry.rejection_reason == "source_adapter_contract_audit_not_ready"
                && !entry.observation_generated
        }));
    }
}
