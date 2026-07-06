use crate::controlled_live_required_evidence_collection_plan::ControlledLiveRequiredEvidenceCollectionPlanEntry;
use crate::controlled_live_required_evidence_collection_plan::controlled_live_required_evidence_collection_plan_report;
use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;
use crate::status_canary_evidence_packet::StatusCanaryEvidenceDecision;
use crate::status_canary_evidence_source_readback::StatusCanaryEvidenceSourceReadbackFixture;
use serde::Serialize;

pub const STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_SCHEMA_VERSION: &str =
    "status_canary_evidence_source_adapter_v1";
pub const STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_ID: &str =
    "status-canary-evidence-source-adapter/hepta-system-status/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceAdapterReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub schema_version: &'static str,
    pub adapter_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_required_evidence_collection_plan_ready: bool,
    pub source_adapter_count: usize,
    pub adapter_input_count: usize,
    pub known_adapter_input_count: usize,
    pub unknown_adapter_input_count: usize,
    pub duplicate_adapter_input_count: usize,
    pub generated_fixture_count: usize,
    pub recorded_fixture_count: usize,
    pub waived_fixture_count: usize,
    pub expired_fixture_count: usize,
    pub invalid_fixture_count: usize,
    pub missing_adapter_input_count: usize,
    pub rejected_adapter_input_count: usize,
    pub metadata_contract_count: usize,
    pub metadata_contract_ready_count: usize,
    pub input_contract_field_count: usize,
    pub readback_fixture_contract_field_count: usize,
    pub required_field_validator_count: usize,
    pub required_field_validator_ready_count: usize,
    pub required_field_rejected_count: usize,
    pub missing_required_field_count: usize,
    pub source_adapter_ready: bool,
    pub source_adapter_route: &'static str,
    pub entries: Vec<StatusCanaryEvidenceSourceAdapterEntry>,
    pub generated_fixtures: Vec<StatusCanaryEvidenceSourceReadbackFixture>,
    pub side_effects: StatusCanaryEvidenceSourceAdapterSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceAdapterEntry {
    pub source_blocker_id: &'static str,
    pub source_adapter_key: &'static str,
    pub source_adapter_route: &'static str,
    pub source_adapter_kind: &'static str,
    pub source_adapter_metadata_contract_route: &'static str,
    pub source_adapter_readback_fixture_contract_route: &'static str,
    pub source_adapter_required_field_validator_route: &'static str,
    pub source_adapter_input_contract_fields: Vec<&'static str>,
    pub source_adapter_readback_fixture_contract_fields: Vec<&'static str>,
    pub recorded_decision_required_fields: Vec<&'static str>,
    pub waived_decision_required_fields: Vec<&'static str>,
    pub expired_decision_required_fields: Vec<&'static str>,
    pub invalid_decision_required_fields: Vec<&'static str>,
    pub source_adapter_metadata_contract_ready: bool,
    pub source_adapter_required_field_validator_ready: bool,
    pub requested_decision_required_fields: Vec<&'static str>,
    pub missing_required_fields: Vec<&'static str>,
    pub missing_required_field_count: usize,
    pub required_field_rejected: bool,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub adapter_input_present: bool,
    pub duplicate_adapter_input: bool,
    pub requested_decision: StatusCanaryEvidenceDecision,
    pub adapter_source_route: &'static str,
    pub adapter_source_route_present: bool,
    pub evidence_artifact_present: bool,
    pub evidence_digest_present: bool,
    pub operator_authority_present: bool,
    pub freshness_attested: bool,
    pub waiver_reason_present: bool,
    pub expiry_attested: bool,
    pub invalidity_reason_present: bool,
    pub adapter_input_missing: bool,
    pub adapter_input_rejected: bool,
    pub rejection_reason: &'static str,
    pub fixture_generated: bool,
    pub generated_fixture: Option<StatusCanaryEvidenceSourceReadbackFixture>,
    pub operator_visible: bool,
    pub queryable: bool,
    pub selected_status_canary_bound: bool,
    pub preflight_only_connector_excluded: bool,
    pub source_adapter_execution_allowed: bool,
    pub source_read_persistence_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub waiver_recording_allowed: bool,
    pub credential_read_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceAdapterInput {
    pub source_blocker_id: &'static str,
    pub requested_decision: StatusCanaryEvidenceDecision,
    pub source_route: &'static str,
    pub evidence_artifact_present: bool,
    pub evidence_digest_present: bool,
    pub operator_authority_present: bool,
    pub freshness_attested: bool,
    pub waiver_reason_present: bool,
    pub expiry_attested: bool,
    pub invalidity_reason_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceAdapterSideEffects {
    pub source_adapter_executed: bool,
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

pub fn status_canary_evidence_source_adapter() -> StatusCanaryEvidenceSourceAdapterReport {
    status_canary_evidence_source_adapter_from_inputs(&[])
}

pub fn status_canary_evidence_source_adapter_from_inputs(
    inputs: &[StatusCanaryEvidenceSourceAdapterInput],
) -> StatusCanaryEvidenceSourceAdapterReport {
    let source = controlled_live_required_evidence_collection_plan_report();
    status_canary_evidence_source_adapter_from_plan(
        source.evidence_collection_plan_ready,
        source.entries,
        inputs,
    )
}

pub fn status_canary_evidence_source_adapter_from_plan(
    source_required_evidence_collection_plan_ready: bool,
    source_entries: Vec<ControlledLiveRequiredEvidenceCollectionPlanEntry>,
    inputs: &[StatusCanaryEvidenceSourceAdapterInput],
) -> StatusCanaryEvidenceSourceAdapterReport {
    let known_adapter_input_count = inputs
        .iter()
        .filter(|input| {
            source_entries
                .iter()
                .any(|entry| entry.source_blocker_id == input.source_blocker_id)
        })
        .count();
    let unknown_adapter_input_count = inputs.len().saturating_sub(known_adapter_input_count);
    let entries = source_entries
        .into_iter()
        .map(|entry| status_canary_evidence_source_adapter_entry(entry, inputs))
        .collect::<Vec<_>>();
    let duplicate_adapter_input_count = entries
        .iter()
        .filter(|entry| entry.duplicate_adapter_input)
        .count();
    let rejected_adapter_input_count = entries
        .iter()
        .filter(|entry| entry.adapter_input_rejected)
        .count();
    let metadata_contract_count = entries.len();
    let metadata_contract_ready_count = entries
        .iter()
        .filter(|entry| entry.source_adapter_metadata_contract_ready)
        .count();
    let input_contract_field_count = entries
        .iter()
        .map(|entry| entry.source_adapter_input_contract_fields.len())
        .sum();
    let readback_fixture_contract_field_count = entries
        .iter()
        .map(|entry| entry.source_adapter_readback_fixture_contract_fields.len())
        .sum();
    let required_field_validator_count = entries.len();
    let required_field_validator_ready_count = entries
        .iter()
        .filter(|entry| entry.source_adapter_required_field_validator_ready)
        .count();
    let required_field_rejected_count = entries
        .iter()
        .filter(|entry| entry.required_field_rejected)
        .count();
    let missing_required_field_count = entries
        .iter()
        .map(|entry| entry.missing_required_field_count)
        .sum();
    let generated_fixtures = entries
        .iter()
        .filter_map(|entry| entry.generated_fixture)
        .collect::<Vec<_>>();
    let missing_adapter_input_count = entries.len().saturating_sub(generated_fixtures.len());
    let side_effects = StatusCanaryEvidenceSourceAdapterSideEffects::none();
    let source_adapter_ready = source_required_evidence_collection_plan_ready
        && entries.len() == 7
        && unknown_adapter_input_count == 0
        && duplicate_adapter_input_count == 0
        && rejected_adapter_input_count == 0
        && metadata_contract_count == 7
        && metadata_contract_ready_count == 7
        && input_contract_field_count == 21
        && readback_fixture_contract_field_count == 70
        && required_field_validator_count == 7
        && required_field_validator_ready_count == 7
        && required_field_rejected_count == 0
        && missing_required_field_count == 0
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.source_adapter_metadata_contract_ready
                && entry.source_adapter_required_field_validator_ready
                && entry.selected_status_canary_bound
                && entry.preflight_only_connector_excluded
                && !entry.source_adapter_execution_allowed
                && !entry.source_read_persistence_allowed
                && !entry.evidence_recording_allowed
                && !entry.waiver_recording_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        })
        && side_effects == StatusCanaryEvidenceSourceAdapterSideEffects::none();

    StatusCanaryEvidenceSourceAdapterReport {
        runtime: "hepta",
        surface: "status_canary_evidence_source_adapter",
        schema_version: STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_SCHEMA_VERSION,
        adapter_id: STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_required_evidence_collection_plan_ready,
        source_adapter_count: entries.len(),
        adapter_input_count: inputs.len(),
        known_adapter_input_count,
        unknown_adapter_input_count,
        duplicate_adapter_input_count,
        generated_fixture_count: generated_fixtures.len(),
        recorded_fixture_count: generated_fixtures
            .iter()
            .filter(|fixture| fixture.requested_decision == StatusCanaryEvidenceDecision::Recorded)
            .count(),
        waived_fixture_count: generated_fixtures
            .iter()
            .filter(|fixture| fixture.requested_decision == StatusCanaryEvidenceDecision::Waived)
            .count(),
        expired_fixture_count: generated_fixtures
            .iter()
            .filter(|fixture| fixture.requested_decision == StatusCanaryEvidenceDecision::Expired)
            .count(),
        invalid_fixture_count: generated_fixtures
            .iter()
            .filter(|fixture| fixture.requested_decision == StatusCanaryEvidenceDecision::Invalid)
            .count(),
        missing_adapter_input_count,
        rejected_adapter_input_count,
        metadata_contract_count,
        metadata_contract_ready_count,
        input_contract_field_count,
        readback_fixture_contract_field_count,
        required_field_validator_count,
        required_field_validator_ready_count,
        required_field_rejected_count,
        missing_required_field_count,
        source_adapter_ready,
        source_adapter_route: status_canary_evidence_source_adapter_route(
            source_required_evidence_collection_plan_ready,
            inputs.len(),
            unknown_adapter_input_count,
            duplicate_adapter_input_count,
            rejected_adapter_input_count,
            generated_fixtures.len(),
        ),
        entries,
        generated_fixtures,
        side_effects,
    }
}

fn status_canary_evidence_source_adapter_entry(
    source: ControlledLiveRequiredEvidenceCollectionPlanEntry,
    inputs: &[StatusCanaryEvidenceSourceAdapterInput],
) -> StatusCanaryEvidenceSourceAdapterEntry {
    let matching_inputs = inputs
        .iter()
        .filter(|input| input.source_blocker_id == source.source_blocker_id)
        .collect::<Vec<_>>();
    let duplicate_adapter_input = matching_inputs.len() > 1;
    let input = matching_inputs.first().copied().copied();
    let source_adapter_required_field_validator_route =
        status_canary_evidence_source_adapter_required_field_validator_route(
            source.source_blocker_id,
        );
    let source_adapter_required_field_validator_ready =
        status_canary_evidence_source_adapter_required_field_validator_ready(
            source.source_blocker_id,
        );
    let requested_decision_required_fields = input
        .map(|input| {
            status_canary_evidence_source_adapter_required_fields_for_decision(
                input.requested_decision,
            )
        })
        .unwrap_or_default();
    let missing_required_fields = input
        .map(status_canary_evidence_source_adapter_missing_required_fields)
        .unwrap_or_default();
    let missing_required_field_count = missing_required_fields.len();
    let required_field_rejected = input.is_some() && missing_required_field_count > 0;
    let validation = input
        .map(|input| {
            status_canary_evidence_source_adapter_validation(
                input,
                duplicate_adapter_input,
                source_adapter_required_field_validator_ready,
            )
        })
        .unwrap_or(StatusCanaryEvidenceSourceAdapterValidation {
            valid: false,
            rejection_reason: "no_source_adapter_input",
        });
    let fixture_generated = input.is_some() && validation.valid;
    let generated_fixture = if fixture_generated {
        input.map(|input| StatusCanaryEvidenceSourceReadbackFixture {
            source_blocker_id: source.source_blocker_id,
            requested_decision: input.requested_decision,
            source_route: input.source_route,
            source_adapter_metadata_contract_route:
                status_canary_evidence_source_adapter_metadata_contract_route(
                    source.source_blocker_id,
                ),
            source_adapter_readback_fixture_contract_route:
                status_canary_evidence_source_adapter_readback_fixture_contract_route(
                    source.source_blocker_id,
                ),
            source_adapter_metadata_contract_ready:
                status_canary_evidence_source_adapter_metadata_contract_ready(
                    source.source_blocker_id,
                ),
            source_adapter_readback_fixture_contract_ready:
                status_canary_evidence_source_adapter_metadata_contract_ready(
                    source.source_blocker_id,
                ),
            evidence_artifact_present: input.evidence_artifact_present,
            evidence_digest_present: input.evidence_digest_present,
            operator_authority_present: input.operator_authority_present,
            freshness_attested: input.freshness_attested,
            waiver_reason_present: input.waiver_reason_present,
            expiry_attested: input.expiry_attested,
            invalidity_reason_present: input.invalidity_reason_present,
        })
    } else {
        None
    };

    StatusCanaryEvidenceSourceAdapterEntry {
        source_blocker_id: source.source_blocker_id,
        source_adapter_key: status_canary_evidence_source_adapter_key(source.source_blocker_id),
        source_adapter_route: status_canary_evidence_source_adapter_entry_route(
            source.source_blocker_id,
        ),
        source_adapter_kind: status_canary_evidence_source_adapter_kind(source.source_blocker_id),
        source_adapter_metadata_contract_route:
            status_canary_evidence_source_adapter_metadata_contract_route(source.source_blocker_id),
        source_adapter_readback_fixture_contract_route:
            status_canary_evidence_source_adapter_readback_fixture_contract_route(
                source.source_blocker_id,
            ),
        source_adapter_required_field_validator_route,
        source_adapter_input_contract_fields:
            status_canary_evidence_source_adapter_input_contract_fields(),
        source_adapter_readback_fixture_contract_fields:
            status_canary_evidence_source_adapter_readback_fixture_contract_fields(),
        recorded_decision_required_fields:
            status_canary_evidence_source_adapter_recorded_required_fields(),
        waived_decision_required_fields:
            status_canary_evidence_source_adapter_waived_required_fields(),
        expired_decision_required_fields:
            status_canary_evidence_source_adapter_expired_required_fields(),
        invalid_decision_required_fields:
            status_canary_evidence_source_adapter_invalid_required_fields(),
        source_adapter_metadata_contract_ready:
            status_canary_evidence_source_adapter_metadata_contract_ready(source.source_blocker_id),
        source_adapter_required_field_validator_ready,
        requested_decision_required_fields,
        missing_required_fields,
        missing_required_field_count,
        required_field_rejected,
        operator_label: source.operator_label,
        required_evidence: source.required_evidence,
        adapter_input_present: input.is_some(),
        duplicate_adapter_input,
        requested_decision: input
            .map(|input| input.requested_decision)
            .unwrap_or(StatusCanaryEvidenceDecision::Missing),
        adapter_source_route: input
            .map(|input| input.source_route)
            .unwrap_or("memory://status-canary/evidence-source/adapter/no-input"),
        adapter_source_route_present: input
            .map(|input| !input.source_route.is_empty())
            .unwrap_or(false),
        evidence_artifact_present: input
            .map(|input| input.evidence_artifact_present)
            .unwrap_or(false),
        evidence_digest_present: input
            .map(|input| input.evidence_digest_present)
            .unwrap_or(false),
        operator_authority_present: input
            .map(|input| input.operator_authority_present)
            .unwrap_or(false),
        freshness_attested: input.map(|input| input.freshness_attested).unwrap_or(false),
        waiver_reason_present: input
            .map(|input| input.waiver_reason_present)
            .unwrap_or(false),
        expiry_attested: input.map(|input| input.expiry_attested).unwrap_or(false),
        invalidity_reason_present: input
            .map(|input| input.invalidity_reason_present)
            .unwrap_or(false),
        adapter_input_missing: input.is_none(),
        adapter_input_rejected: input.is_some() && !validation.valid,
        rejection_reason: validation.rejection_reason,
        fixture_generated,
        generated_fixture,
        operator_visible: true,
        queryable: true,
        selected_status_canary_bound: true,
        preflight_only_connector_excluded: true,
        source_adapter_execution_allowed: false,
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
struct StatusCanaryEvidenceSourceAdapterValidation {
    valid: bool,
    rejection_reason: &'static str,
}

fn status_canary_evidence_source_adapter_validation(
    input: StatusCanaryEvidenceSourceAdapterInput,
    duplicate_adapter_input: bool,
    source_adapter_required_field_validator_ready: bool,
) -> StatusCanaryEvidenceSourceAdapterValidation {
    if duplicate_adapter_input {
        return StatusCanaryEvidenceSourceAdapterValidation {
            valid: false,
            rejection_reason: "duplicate_source_adapter_input",
        };
    }
    if input.source_route.is_empty() {
        return StatusCanaryEvidenceSourceAdapterValidation {
            valid: false,
            rejection_reason: "missing_source_adapter_route",
        };
    }
    if input.requested_decision == StatusCanaryEvidenceDecision::Missing {
        return StatusCanaryEvidenceSourceAdapterValidation {
            valid: false,
            rejection_reason: "missing_is_default_not_source_adapter_input",
        };
    }
    if !source_adapter_required_field_validator_ready {
        return StatusCanaryEvidenceSourceAdapterValidation {
            valid: false,
            rejection_reason: "source_adapter_required_field_validator_not_ready",
        };
    }
    if !status_canary_evidence_source_adapter_missing_required_fields(input).is_empty() {
        return StatusCanaryEvidenceSourceAdapterValidation {
            valid: false,
            rejection_reason: "source_adapter_required_fields_missing",
        };
    }

    StatusCanaryEvidenceSourceAdapterValidation {
        valid: true,
        rejection_reason: "accepted",
    }
}

fn status_canary_evidence_source_adapter_route(
    source_required_evidence_collection_plan_ready: bool,
    adapter_input_count: usize,
    unknown_adapter_input_count: usize,
    duplicate_adapter_input_count: usize,
    rejected_adapter_input_count: usize,
    generated_fixture_count: usize,
) -> &'static str {
    if !source_required_evidence_collection_plan_ready {
        "status_canary_evidence_source_adapter_blocked_source_plan_not_ready"
    } else if unknown_adapter_input_count > 0 {
        "status_canary_evidence_source_adapter_blocked_unknown_input"
    } else if duplicate_adapter_input_count > 0 {
        "status_canary_evidence_source_adapter_blocked_duplicate_input"
    } else if rejected_adapter_input_count > 0 {
        "status_canary_evidence_source_adapter_blocked_rejected_input"
    } else if adapter_input_count == 0 {
        "status_canary_evidence_source_adapter_ready_no_inputs"
    } else if generated_fixture_count > 0 {
        "status_canary_evidence_source_adapter_ready_fixtures_generated"
    } else {
        "status_canary_evidence_source_adapter_blocked_unknown"
    }
}

fn status_canary_evidence_source_adapter_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "status_canary.evidence_source_adapter.clean_worktree_snapshot"
        }
        "operator_live_approval_missing" => {
            "status_canary.evidence_source_adapter.operator_live_approval"
        }
        "fresh_soak_readback_missing" => {
            "status_canary.evidence_source_adapter.fresh_status_canary_soak"
        }
        "credential_boundary_attestation_missing" => {
            "status_canary.evidence_source_adapter.credential_boundary_attestation"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "status_canary.evidence_source_adapter.transport_boundary_approval"
        }
        "rollback_rehearsal_missing" => "status_canary.evidence_source_adapter.rollback_rehearsal",
        "kill_switch_rehearsal_missing" => {
            "status_canary.evidence_source_adapter.kill_switch_rehearsal"
        }
        _ => "status_canary.evidence_source_adapter.unknown",
    }
}

fn status_canary_evidence_source_adapter_entry_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "adapter://status-canary/evidence-source/clean-worktree-snapshot"
        }
        "operator_live_approval_missing" => {
            "adapter://status-canary/evidence-source/operator-live-approval"
        }
        "fresh_soak_readback_missing" => {
            "adapter://status-canary/evidence-source/fresh-status-canary-soak"
        }
        "credential_boundary_attestation_missing" => {
            "adapter://status-canary/evidence-source/credential-boundary-attestation"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "adapter://status-canary/evidence-source/transport-boundary-approval"
        }
        "rollback_rehearsal_missing" => {
            "adapter://status-canary/evidence-source/rollback-rehearsal"
        }
        "kill_switch_rehearsal_missing" => {
            "adapter://status-canary/evidence-source/kill-switch-rehearsal"
        }
        _ => "adapter://status-canary/evidence-source/unknown",
    }
}

fn status_canary_evidence_source_adapter_kind(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "clean_worktree_snapshot_source_adapter",
        "operator_live_approval_missing" => "operator_live_approval_source_adapter",
        "fresh_soak_readback_missing" => "fresh_status_canary_soak_source_adapter",
        "credential_boundary_attestation_missing" => "credential_attestation_source_adapter",
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "transport_approval_source_adapter"
        }
        "rollback_rehearsal_missing" => "rollback_rehearsal_source_adapter",
        "kill_switch_rehearsal_missing" => "kill_switch_rehearsal_source_adapter",
        _ => "unknown_status_canary_evidence_source_adapter",
    }
}

fn status_canary_evidence_source_adapter_metadata_contract_route(
    source_blocker_id: &str,
) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "metadata://status-canary/evidence-source-adapter-contract/clean-worktree-snapshot"
        }
        "operator_live_approval_missing" => {
            "metadata://status-canary/evidence-source-adapter-contract/operator-live-approval"
        }
        "fresh_soak_readback_missing" => {
            "metadata://status-canary/evidence-source-adapter-contract/fresh-status-canary-soak"
        }
        "credential_boundary_attestation_missing" => {
            "metadata://status-canary/evidence-source-adapter-contract/credential-boundary-attestation"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "metadata://status-canary/evidence-source-adapter-contract/transport-boundary-approval"
        }
        "rollback_rehearsal_missing" => {
            "metadata://status-canary/evidence-source-adapter-contract/rollback-rehearsal"
        }
        "kill_switch_rehearsal_missing" => {
            "metadata://status-canary/evidence-source-adapter-contract/kill-switch-rehearsal"
        }
        _ => "metadata://status-canary/evidence-source-adapter-contract/unknown",
    }
}

fn status_canary_evidence_source_adapter_readback_fixture_contract_route(
    source_blocker_id: &str,
) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "metadata://status-canary/evidence-source-readback-fixture-contract/clean-worktree-snapshot"
        }
        "operator_live_approval_missing" => {
            "metadata://status-canary/evidence-source-readback-fixture-contract/operator-live-approval"
        }
        "fresh_soak_readback_missing" => {
            "metadata://status-canary/evidence-source-readback-fixture-contract/fresh-status-canary-soak"
        }
        "credential_boundary_attestation_missing" => {
            "metadata://status-canary/evidence-source-readback-fixture-contract/credential-boundary-attestation"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "metadata://status-canary/evidence-source-readback-fixture-contract/transport-boundary-approval"
        }
        "rollback_rehearsal_missing" => {
            "metadata://status-canary/evidence-source-readback-fixture-contract/rollback-rehearsal"
        }
        "kill_switch_rehearsal_missing" => {
            "metadata://status-canary/evidence-source-readback-fixture-contract/kill-switch-rehearsal"
        }
        _ => "metadata://status-canary/evidence-source-readback-fixture-contract/unknown",
    }
}

fn status_canary_evidence_source_adapter_required_field_validator_route(
    source_blocker_id: &str,
) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "validator://status-canary/evidence-source-required-fields/clean-worktree-snapshot"
        }
        "operator_live_approval_missing" => {
            "validator://status-canary/evidence-source-required-fields/operator-live-approval"
        }
        "fresh_soak_readback_missing" => {
            "validator://status-canary/evidence-source-required-fields/fresh-status-canary-soak"
        }
        "credential_boundary_attestation_missing" => {
            "validator://status-canary/evidence-source-required-fields/credential-boundary-attestation"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "validator://status-canary/evidence-source-required-fields/transport-boundary-approval"
        }
        "rollback_rehearsal_missing" => {
            "validator://status-canary/evidence-source-required-fields/rollback-rehearsal"
        }
        "kill_switch_rehearsal_missing" => {
            "validator://status-canary/evidence-source-required-fields/kill-switch-rehearsal"
        }
        _ => "validator://status-canary/evidence-source-required-fields/unknown",
    }
}

fn status_canary_evidence_source_adapter_input_contract_fields() -> Vec<&'static str> {
    vec!["source_blocker_id", "requested_decision", "source_route"]
}

fn status_canary_evidence_source_adapter_readback_fixture_contract_fields() -> Vec<&'static str> {
    vec![
        "source_blocker_id",
        "requested_decision",
        "source_route",
        "evidence_artifact_present",
        "evidence_digest_present",
        "operator_authority_present",
        "freshness_attested",
        "waiver_reason_present",
        "expiry_attested",
        "invalidity_reason_present",
    ]
}

fn status_canary_evidence_source_adapter_recorded_required_fields() -> Vec<&'static str> {
    vec![
        "evidence_artifact_present",
        "evidence_digest_present",
        "operator_authority_present",
        "freshness_attested",
    ]
}

fn status_canary_evidence_source_adapter_waived_required_fields() -> Vec<&'static str> {
    vec!["operator_authority_present", "waiver_reason_present"]
}

fn status_canary_evidence_source_adapter_expired_required_fields() -> Vec<&'static str> {
    vec![
        "evidence_artifact_present",
        "evidence_digest_present",
        "expiry_attested",
    ]
}

fn status_canary_evidence_source_adapter_invalid_required_fields() -> Vec<&'static str> {
    vec![
        "evidence_artifact_present",
        "evidence_digest_present",
        "invalidity_reason_present",
    ]
}

fn status_canary_evidence_source_adapter_required_fields_for_decision(
    decision: StatusCanaryEvidenceDecision,
) -> Vec<&'static str> {
    match decision {
        StatusCanaryEvidenceDecision::Missing => Vec::new(),
        StatusCanaryEvidenceDecision::Recorded => {
            status_canary_evidence_source_adapter_recorded_required_fields()
        }
        StatusCanaryEvidenceDecision::Waived => {
            status_canary_evidence_source_adapter_waived_required_fields()
        }
        StatusCanaryEvidenceDecision::Expired => {
            status_canary_evidence_source_adapter_expired_required_fields()
        }
        StatusCanaryEvidenceDecision::Invalid => {
            status_canary_evidence_source_adapter_invalid_required_fields()
        }
    }
}

fn status_canary_evidence_source_adapter_missing_required_fields(
    input: StatusCanaryEvidenceSourceAdapterInput,
) -> Vec<&'static str> {
    status_canary_evidence_source_adapter_required_fields_for_decision(input.requested_decision)
        .into_iter()
        .filter(|field| !status_canary_evidence_source_adapter_input_field_present(input, field))
        .collect()
}

fn status_canary_evidence_source_adapter_input_field_present(
    input: StatusCanaryEvidenceSourceAdapterInput,
    field: &str,
) -> bool {
    match field {
        "evidence_artifact_present" => input.evidence_artifact_present,
        "evidence_digest_present" => input.evidence_digest_present,
        "operator_authority_present" => input.operator_authority_present,
        "freshness_attested" => input.freshness_attested,
        "waiver_reason_present" => input.waiver_reason_present,
        "expiry_attested" => input.expiry_attested,
        "invalidity_reason_present" => input.invalidity_reason_present,
        _ => false,
    }
}

fn status_canary_evidence_source_adapter_required_field_validator_ready(
    source_blocker_id: &str,
) -> bool {
    status_canary_evidence_source_adapter_metadata_contract_ready(source_blocker_id)
        && status_canary_evidence_source_adapter_required_field_validator_route(source_blocker_id)
            != "validator://status-canary/evidence-source-required-fields/unknown"
}

fn status_canary_evidence_source_adapter_metadata_contract_ready(source_blocker_id: &str) -> bool {
    status_canary_evidence_source_adapter_kind(source_blocker_id)
        != "unknown_status_canary_evidence_source_adapter"
        && status_canary_evidence_source_adapter_metadata_contract_route(source_blocker_id)
            != "metadata://status-canary/evidence-source-adapter-contract/unknown"
        && status_canary_evidence_source_adapter_readback_fixture_contract_route(source_blocker_id)
            != "metadata://status-canary/evidence-source-readback-fixture-contract/unknown"
        && status_canary_evidence_source_adapter_input_contract_fields().len() == 3
        && status_canary_evidence_source_adapter_readback_fixture_contract_fields().len() == 10
        && status_canary_evidence_source_adapter_recorded_required_fields().len() == 4
        && status_canary_evidence_source_adapter_waived_required_fields().len() == 2
        && status_canary_evidence_source_adapter_expired_required_fields().len() == 3
        && status_canary_evidence_source_adapter_invalid_required_fields().len() == 3
}

impl StatusCanaryEvidenceSourceAdapterInput {
    pub const fn recorded(source_blocker_id: &'static str) -> Self {
        Self {
            source_blocker_id,
            requested_decision: StatusCanaryEvidenceDecision::Recorded,
            source_route: "adapter://status-canary/evidence-source/test-recorded",
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
            source_route: "adapter://status-canary/evidence-source/test-waived",
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
            source_route: "adapter://status-canary/evidence-source/test-expired",
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
            source_route: "adapter://status-canary/evidence-source/test-invalid",
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

impl StatusCanaryEvidenceSourceAdapterSideEffects {
    pub const fn none() -> Self {
        Self {
            source_adapter_executed: false,
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
    use crate::status_canary_evidence_source_readback::status_canary_evidence_source_readback_from_fixtures;
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
    fn default_adapter_declares_seven_sources_without_fixtures() {
        let adapter = status_canary_evidence_source_adapter();

        assert_eq!(adapter.adapter_id, STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_ID);
        assert!(adapter.source_adapter_ready);
        assert_eq!(
            adapter.source_adapter_route,
            "status_canary_evidence_source_adapter_ready_no_inputs"
        );
        assert_eq!(adapter.source_adapter_count, 7);
        assert_eq!(adapter.adapter_input_count, 0);
        assert_eq!(adapter.generated_fixture_count, 0);
        assert_eq!(adapter.missing_adapter_input_count, 7);
        assert_eq!(adapter.metadata_contract_count, 7);
        assert_eq!(adapter.metadata_contract_ready_count, 7);
        assert_eq!(adapter.input_contract_field_count, 21);
        assert_eq!(adapter.readback_fixture_contract_field_count, 70);
        assert_eq!(adapter.required_field_validator_count, 7);
        assert_eq!(adapter.required_field_validator_ready_count, 7);
        assert_eq!(adapter.required_field_rejected_count, 0);
        assert_eq!(adapter.missing_required_field_count, 0);
        assert_eq!(
            adapter.side_effects,
            StatusCanaryEvidenceSourceAdapterSideEffects::none()
        );
        assert!(adapter.entries.iter().any(|entry| {
            entry.source_adapter_kind == "clean_worktree_snapshot_source_adapter"
        }));
        assert!(
            adapter.entries.iter().any(|entry| {
                entry.source_adapter_kind == "operator_live_approval_source_adapter"
            })
        );
        assert!(adapter.entries.iter().any(|entry| {
            entry.source_adapter_kind == "fresh_status_canary_soak_source_adapter"
        }));
        assert!(
            adapter.entries.iter().any(|entry| {
                entry.source_adapter_kind == "credential_attestation_source_adapter"
            })
        );
        assert!(
            adapter
                .entries
                .iter()
                .any(|entry| { entry.source_adapter_kind == "transport_approval_source_adapter" })
        );
        assert!(
            adapter
                .entries
                .iter()
                .any(|entry| { entry.source_adapter_kind == "rollback_rehearsal_source_adapter" })
        );
        assert!(
            adapter.entries.iter().any(|entry| {
                entry.source_adapter_kind == "kill_switch_rehearsal_source_adapter"
            })
        );
        assert!(adapter.entries.iter().all(|entry| {
            !entry.adapter_input_present
                && entry.adapter_input_missing
                && entry.source_adapter_metadata_contract_ready
                && entry.source_adapter_required_field_validator_ready
                && entry.source_adapter_input_contract_fields
                    == vec!["source_blocker_id", "requested_decision", "source_route"]
                && entry.source_adapter_readback_fixture_contract_fields.len() == 10
                && entry.requested_decision_required_fields.is_empty()
                && entry.missing_required_fields.is_empty()
                && entry.missing_required_field_count == 0
                && !entry.required_field_rejected
                && !entry.fixture_generated
                && entry.generated_fixture.is_none()
                && !entry.source_adapter_execution_allowed
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
    fn metadata_contracts_describe_per_source_input_and_readback_fields() {
        let adapter = status_canary_evidence_source_adapter();
        let clean_worktree = adapter
            .entries
            .iter()
            .find(|entry| entry.source_blocker_id == "dirty_worktree_boundary")
            .expect("clean worktree source contract is present");
        let transport = adapter
            .entries
            .iter()
            .find(|entry| {
                entry.source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing"
            })
            .expect("transport source contract is present");

        assert_eq!(
            clean_worktree.source_adapter_metadata_contract_route,
            "metadata://status-canary/evidence-source-adapter-contract/clean-worktree-snapshot"
        );
        assert_eq!(
            clean_worktree.source_adapter_readback_fixture_contract_route,
            "metadata://status-canary/evidence-source-readback-fixture-contract/clean-worktree-snapshot"
        );
        assert_eq!(
            clean_worktree.source_adapter_required_field_validator_route,
            "validator://status-canary/evidence-source-required-fields/clean-worktree-snapshot"
        );
        assert_eq!(
            transport.source_adapter_metadata_contract_route,
            "metadata://status-canary/evidence-source-adapter-contract/transport-boundary-approval"
        );
        assert_eq!(
            transport.source_adapter_readback_fixture_contract_route,
            "metadata://status-canary/evidence-source-readback-fixture-contract/transport-boundary-approval"
        );
        assert_eq!(
            clean_worktree.source_adapter_input_contract_fields,
            vec!["source_blocker_id", "requested_decision", "source_route"]
        );
        assert_eq!(
            clean_worktree.source_adapter_readback_fixture_contract_fields,
            vec![
                "source_blocker_id",
                "requested_decision",
                "source_route",
                "evidence_artifact_present",
                "evidence_digest_present",
                "operator_authority_present",
                "freshness_attested",
                "waiver_reason_present",
                "expiry_attested",
                "invalidity_reason_present",
            ]
        );
        assert_eq!(
            clean_worktree.recorded_decision_required_fields,
            vec![
                "evidence_artifact_present",
                "evidence_digest_present",
                "operator_authority_present",
                "freshness_attested",
            ]
        );
        assert_eq!(
            clean_worktree.waived_decision_required_fields,
            vec!["operator_authority_present", "waiver_reason_present"]
        );
        assert_eq!(
            clean_worktree.expired_decision_required_fields,
            vec![
                "evidence_artifact_present",
                "evidence_digest_present",
                "expiry_attested",
            ]
        );
        assert_eq!(
            clean_worktree.invalid_decision_required_fields,
            vec![
                "evidence_artifact_present",
                "evidence_digest_present",
                "invalidity_reason_present",
            ]
        );
        assert!(adapter.entries.iter().all(|entry| {
            entry.source_adapter_metadata_contract_ready
                && entry.source_adapter_required_field_validator_ready
                && entry
                    .source_adapter_metadata_contract_route
                    .starts_with("metadata://status-canary/evidence-source-adapter-contract/")
                && entry
                    .source_adapter_readback_fixture_contract_route
                    .starts_with(
                        "metadata://status-canary/evidence-source-readback-fixture-contract/",
                    )
                && entry
                    .source_adapter_required_field_validator_route
                    .starts_with("validator://status-canary/evidence-source-required-fields/")
        }));
    }

    #[test]
    fn recorded_and_waived_inputs_generate_readback_fixtures() {
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
        let readback =
            status_canary_evidence_source_readback_from_fixtures(&adapter.generated_fixtures);
        let validator =
            status_canary_evidence_source_validator_from_observations(&readback.observations);
        let acceptance =
            status_canary_evidence_acceptance_packet_from_requests(&validator.decision_requests);

        assert!(adapter.source_adapter_ready);
        assert_eq!(adapter.adapter_input_count, 7);
        assert_eq!(adapter.required_field_rejected_count, 0);
        assert_eq!(adapter.missing_required_field_count, 0);
        assert_eq!(adapter.generated_fixture_count, 7);
        assert_eq!(adapter.recorded_fixture_count, 4);
        assert_eq!(adapter.waived_fixture_count, 3);
        assert_eq!(
            adapter.source_adapter_route,
            "status_canary_evidence_source_adapter_ready_fixtures_generated"
        );
        assert_eq!(readback.observation_count, 7);
        assert_eq!(validator.generated_request_count, 7);
        assert_eq!(acceptance.generated_override_count, 7);
        assert!(acceptance.source_evidence_packet_complete);
    }

    #[test]
    fn expired_and_invalid_inputs_flow_to_start_guard_blocked() {
        let ids = blocker_ids();
        let inputs = ids
            .iter()
            .enumerate()
            .map(|(index, source_blocker_id)| match index {
                0 | 1 => StatusCanaryEvidenceSourceAdapterInput::recorded(source_blocker_id),
                2 | 3 => StatusCanaryEvidenceSourceAdapterInput::waived(source_blocker_id),
                4 | 5 => StatusCanaryEvidenceSourceAdapterInput::expired(source_blocker_id),
                _ => StatusCanaryEvidenceSourceAdapterInput::invalid(source_blocker_id),
            })
            .collect::<Vec<_>>();
        let adapter = status_canary_evidence_source_adapter_from_inputs(&inputs);
        let readback =
            status_canary_evidence_source_readback_from_fixtures(&adapter.generated_fixtures);
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

        assert!(adapter.source_adapter_ready);
        assert_eq!(adapter.generated_fixture_count, 7);
        assert_eq!(adapter.expired_fixture_count, 2);
        assert_eq!(adapter.invalid_fixture_count, 1);
        assert_eq!(readback.expired_observation_count, 2);
        assert_eq!(validator.generated_expired_request_count, 2);
        assert!(!acceptance.source_evidence_packet_complete);
        assert!(guard.canary_start_blocked);
        assert!(!guard.canary_start_allowed);
    }

    #[test]
    fn unknown_and_duplicate_inputs_fail_closed_without_fixtures() {
        let duplicate = StatusCanaryEvidenceSourceAdapterInput::recorded("dirty_worktree_boundary");
        let unknown = StatusCanaryEvidenceSourceAdapterInput::recorded("unknown_blocker");
        let adapter =
            status_canary_evidence_source_adapter_from_inputs(&[duplicate, duplicate, unknown]);

        assert!(!adapter.source_adapter_ready);
        assert_eq!(adapter.adapter_input_count, 3);
        assert_eq!(adapter.known_adapter_input_count, 2);
        assert_eq!(adapter.unknown_adapter_input_count, 1);
        assert_eq!(adapter.duplicate_adapter_input_count, 1);
        assert_eq!(adapter.generated_fixture_count, 0);
        assert_eq!(
            adapter.source_adapter_route,
            "status_canary_evidence_source_adapter_blocked_unknown_input"
        );
        assert!(adapter.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.duplicate_adapter_input
                && entry.adapter_input_rejected
                && entry.rejection_reason == "duplicate_source_adapter_input"
                && !entry.fixture_generated
        }));
    }

    #[test]
    fn missing_required_field_fails_closed_before_readback_fixture_generation() {
        let mut input =
            StatusCanaryEvidenceSourceAdapterInput::recorded("operator_live_approval_missing");
        input.evidence_digest_present = false;
        let adapter = status_canary_evidence_source_adapter_from_inputs(&[input]);
        let readback =
            status_canary_evidence_source_readback_from_fixtures(&adapter.generated_fixtures);
        let validator =
            status_canary_evidence_source_validator_from_observations(&readback.observations);

        assert!(!adapter.source_adapter_ready);
        assert_eq!(adapter.adapter_input_count, 1);
        assert_eq!(adapter.required_field_rejected_count, 1);
        assert_eq!(adapter.missing_required_field_count, 1);
        assert_eq!(adapter.generated_fixture_count, 0);
        assert_eq!(
            adapter.source_adapter_route,
            "status_canary_evidence_source_adapter_blocked_rejected_input"
        );
        assert_eq!(readback.observation_count, 0);
        assert_eq!(validator.source_validated_count, 0);
        assert_eq!(validator.source_rejected_count, 0);
        assert_eq!(validator.generated_request_count, 0);
        assert!(adapter.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.source_adapter_required_field_validator_ready
                && entry.required_field_rejected
                && entry.missing_required_fields == vec!["evidence_digest_present"]
                && entry.missing_required_field_count == 1
                && entry.rejection_reason == "source_adapter_required_fields_missing"
                && !entry.fixture_generated
                && entry.generated_fixture.is_none()
        }));
    }
}
