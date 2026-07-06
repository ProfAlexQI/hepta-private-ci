use crate::status_canary_evidence_packet::PREFLIGHT_ONLY_CONNECTOR_TOOL_ID;
use crate::status_canary_evidence_packet::SELECTED_STATUS_CANARY_TOOL_ID;
use crate::status_canary_evidence_packet::StatusCanaryEvidenceDecision;
use crate::status_canary_evidence_source_adapter::STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_ID;
use crate::status_canary_evidence_source_adapter::StatusCanaryEvidenceSourceAdapterEntry;
use crate::status_canary_evidence_source_adapter::StatusCanaryEvidenceSourceAdapterReport;
use crate::status_canary_evidence_source_adapter::status_canary_evidence_source_adapter;
use serde::Serialize;

pub const STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_SCHEMA_VERSION: &str =
    "status_canary_evidence_source_reason_packet_v1";
pub const STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_ID: &str =
    "status-canary-evidence-source-reason-packet/hepta-system-status/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceReasonPacketReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub schema_version: &'static str,
    pub reason_packet_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub source_adapter_id: &'static str,
    pub source_adapter_bound: bool,
    pub source_adapter_ready: bool,
    pub source_adapter_route: &'static str,
    pub source_count: usize,
    pub source_decision_reason_count: usize,
    pub source_decision_reason_ready_count: usize,
    pub decision_required_field_count: usize,
    pub missing_required_field_reason_count: usize,
    pub source_adapter_input_missing_reason_count: usize,
    pub source_adapter_input_other_decision_reason_count: usize,
    pub source_adapter_rejection_reason_count: usize,
    pub fixture_generation_allowed_count: usize,
    pub fixture_generation_blocked_count: usize,
    pub reason_packet_ready: bool,
    pub reason_packet_route: &'static str,
    pub entries: Vec<StatusCanaryEvidenceSourceDecisionReason>,
    pub side_effects: StatusCanaryEvidenceSourceReasonPacketSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusCanaryEvidenceSourceDecisionReason {
    pub source_blocker_id: &'static str,
    pub source_adapter_key: &'static str,
    pub source_adapter_kind: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub requested_decision: StatusCanaryEvidenceDecision,
    pub reason_key: String,
    pub source_adapter_metadata_contract_ready: bool,
    pub source_adapter_required_field_validator_ready: bool,
    pub source_adapter_input_present: bool,
    pub source_adapter_input_for_other_decision_present: bool,
    pub source_adapter_rejected: bool,
    pub source_adapter_rejection_reason: &'static str,
    pub decision_required_fields: Vec<&'static str>,
    pub missing_required_fields: Vec<&'static str>,
    pub missing_required_field_count: usize,
    pub fixture_generation_allowed: bool,
    pub fixture_generation_blocked: bool,
    pub fixture_generation_blocker_reason: &'static str,
    pub operator_visible: bool,
    pub queryable: bool,
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
pub struct StatusCanaryEvidenceSourceReasonPacketSideEffects {
    pub source_reason_packet_persisted: bool,
    pub source_adapter_executed: bool,
    pub source_read_persisted: bool,
    pub evidence_recorded: bool,
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

pub fn status_canary_evidence_source_reason_packet() -> StatusCanaryEvidenceSourceReasonPacketReport
{
    let source_adapter = status_canary_evidence_source_adapter();
    status_canary_evidence_source_reason_packet_from_adapter(&source_adapter)
}

pub fn status_canary_evidence_source_reason_packet_from_adapter(
    source_adapter: &StatusCanaryEvidenceSourceAdapterReport,
) -> StatusCanaryEvidenceSourceReasonPacketReport {
    let entries = source_adapter
        .entries
        .iter()
        .flat_map(status_canary_evidence_source_reason_packet_entries_for_source)
        .collect::<Vec<_>>();
    let source_adapter_bound =
        source_adapter.adapter_id == STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_ID;
    let source_decision_reason_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.source_adapter_metadata_contract_ready
                && entry.source_adapter_required_field_validator_ready
                && !entry.source_adapter_execution_allowed
                && !entry.source_read_persistence_allowed
                && !entry.evidence_recording_allowed
                && !entry.waiver_recording_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        })
        .count();
    let decision_required_field_count = entries
        .iter()
        .map(|entry| entry.decision_required_fields.len())
        .sum();
    let missing_required_field_reason_count = entries
        .iter()
        .map(|entry| entry.missing_required_field_count)
        .sum();
    let source_adapter_input_missing_reason_count = entries
        .iter()
        .filter(|entry| {
            entry.fixture_generation_blocker_reason == "source_adapter_input_missing_for_decision"
        })
        .count();
    let source_adapter_input_other_decision_reason_count = entries
        .iter()
        .filter(|entry| {
            entry.fixture_generation_blocker_reason == "source_adapter_input_for_different_decision"
        })
        .count();
    let source_adapter_rejection_reason_count = entries
        .iter()
        .filter(|entry| entry.source_adapter_rejected)
        .count();
    let fixture_generation_allowed_count = entries
        .iter()
        .filter(|entry| entry.fixture_generation_allowed)
        .count();
    let fixture_generation_blocked_count = entries
        .iter()
        .filter(|entry| entry.fixture_generation_blocked)
        .count();
    let side_effects = StatusCanaryEvidenceSourceReasonPacketSideEffects::none();
    let reason_packet_ready = source_adapter_bound
        && source_adapter.source_adapter_count == 7
        && entries.len() == 28
        && source_decision_reason_ready_count == 28
        && decision_required_field_count == 84
        && fixture_generation_allowed_count + fixture_generation_blocked_count == 28
        && side_effects == StatusCanaryEvidenceSourceReasonPacketSideEffects::none();

    StatusCanaryEvidenceSourceReasonPacketReport {
        runtime: "hepta",
        surface: "status_canary_evidence_source_reason_packet",
        schema_version: STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_SCHEMA_VERSION,
        reason_packet_id: STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_ID,
        selected_status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        source_adapter_id: source_adapter.adapter_id,
        source_adapter_bound,
        source_adapter_ready: source_adapter.source_adapter_ready,
        source_adapter_route: source_adapter.source_adapter_route,
        source_count: source_adapter.source_adapter_count,
        source_decision_reason_count: entries.len(),
        source_decision_reason_ready_count,
        decision_required_field_count,
        missing_required_field_reason_count,
        source_adapter_input_missing_reason_count,
        source_adapter_input_other_decision_reason_count,
        source_adapter_rejection_reason_count,
        fixture_generation_allowed_count,
        fixture_generation_blocked_count,
        reason_packet_ready,
        reason_packet_route: status_canary_evidence_source_reason_packet_route(
            reason_packet_ready,
            source_adapter.adapter_input_count,
            source_adapter_rejection_reason_count,
            fixture_generation_allowed_count,
        ),
        entries,
        side_effects,
    }
}

fn status_canary_evidence_source_reason_packet_entries_for_source(
    source: &StatusCanaryEvidenceSourceAdapterEntry,
) -> Vec<StatusCanaryEvidenceSourceDecisionReason> {
    status_canary_evidence_source_reason_packet_decisions()
        .into_iter()
        .map(|decision| status_canary_evidence_source_reason_packet_entry(source, decision))
        .collect()
}

fn status_canary_evidence_source_reason_packet_entry(
    source: &StatusCanaryEvidenceSourceAdapterEntry,
    decision: StatusCanaryEvidenceDecision,
) -> StatusCanaryEvidenceSourceDecisionReason {
    let source_adapter_input_present =
        source.adapter_input_present && source.requested_decision == decision;
    let source_adapter_input_for_other_decision_present =
        source.adapter_input_present && source.requested_decision != decision;
    let decision_required_fields =
        status_canary_evidence_source_reason_packet_required_fields_for_decision(source, decision);
    let missing_required_fields = if source_adapter_input_present {
        source.missing_required_fields.clone()
    } else {
        decision_required_fields.clone()
    };
    let source_adapter_rejected = source_adapter_input_present && source.adapter_input_rejected;
    let fixture_generation_allowed = source_adapter_input_present && source.fixture_generated;
    let fixture_generation_blocker_reason =
        status_canary_evidence_source_reason_packet_blocker_reason(
            source,
            source_adapter_input_present,
            source_adapter_input_for_other_decision_present,
            source_adapter_rejected,
            fixture_generation_allowed,
        );
    let decision_key = status_canary_evidence_source_reason_packet_decision_key(decision);
    let source_adapter_key = source.source_adapter_key;

    StatusCanaryEvidenceSourceDecisionReason {
        source_blocker_id: source.source_blocker_id,
        source_adapter_key: source.source_adapter_key,
        source_adapter_kind: source.source_adapter_kind,
        operator_label: source.operator_label,
        required_evidence: source.required_evidence,
        requested_decision: decision,
        reason_key: format!("{source_adapter_key}:{decision_key}"),
        source_adapter_metadata_contract_ready: source.source_adapter_metadata_contract_ready,
        source_adapter_required_field_validator_ready: source
            .source_adapter_required_field_validator_ready,
        source_adapter_input_present,
        source_adapter_input_for_other_decision_present,
        source_adapter_rejected,
        source_adapter_rejection_reason: if source_adapter_rejected {
            source.rejection_reason
        } else {
            "none"
        },
        decision_required_fields,
        missing_required_field_count: missing_required_fields.len(),
        missing_required_fields,
        fixture_generation_allowed,
        fixture_generation_blocked: !fixture_generation_allowed,
        fixture_generation_blocker_reason,
        operator_visible: source.operator_visible,
        queryable: source.queryable,
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

fn status_canary_evidence_source_reason_packet_required_fields_for_decision(
    source: &StatusCanaryEvidenceSourceAdapterEntry,
    decision: StatusCanaryEvidenceDecision,
) -> Vec<&'static str> {
    match decision {
        StatusCanaryEvidenceDecision::Missing => Vec::new(),
        StatusCanaryEvidenceDecision::Recorded => source.recorded_decision_required_fields.clone(),
        StatusCanaryEvidenceDecision::Waived => source.waived_decision_required_fields.clone(),
        StatusCanaryEvidenceDecision::Expired => source.expired_decision_required_fields.clone(),
        StatusCanaryEvidenceDecision::Invalid => source.invalid_decision_required_fields.clone(),
    }
}

fn status_canary_evidence_source_reason_packet_blocker_reason(
    source: &StatusCanaryEvidenceSourceAdapterEntry,
    source_adapter_input_present: bool,
    source_adapter_input_for_other_decision_present: bool,
    source_adapter_rejected: bool,
    fixture_generation_allowed: bool,
) -> &'static str {
    if fixture_generation_allowed {
        "fixture_generation_allowed"
    } else if !source.source_adapter_metadata_contract_ready {
        "source_adapter_metadata_contract_not_ready"
    } else if !source.source_adapter_required_field_validator_ready {
        "source_adapter_required_field_validator_not_ready"
    } else if source_adapter_rejected {
        source.rejection_reason
    } else if source_adapter_input_for_other_decision_present {
        "source_adapter_input_for_different_decision"
    } else if !source_adapter_input_present {
        "source_adapter_input_missing_for_decision"
    } else {
        "fixture_generation_blocked_unknown"
    }
}

fn status_canary_evidence_source_reason_packet_decisions() -> Vec<StatusCanaryEvidenceDecision> {
    vec![
        StatusCanaryEvidenceDecision::Recorded,
        StatusCanaryEvidenceDecision::Waived,
        StatusCanaryEvidenceDecision::Expired,
        StatusCanaryEvidenceDecision::Invalid,
    ]
}

fn status_canary_evidence_source_reason_packet_decision_key(
    decision: StatusCanaryEvidenceDecision,
) -> &'static str {
    match decision {
        StatusCanaryEvidenceDecision::Missing => "missing",
        StatusCanaryEvidenceDecision::Recorded => "recorded",
        StatusCanaryEvidenceDecision::Waived => "waived",
        StatusCanaryEvidenceDecision::Expired => "expired",
        StatusCanaryEvidenceDecision::Invalid => "invalid",
    }
}

fn status_canary_evidence_source_reason_packet_route(
    reason_packet_ready: bool,
    adapter_input_count: usize,
    source_adapter_rejection_reason_count: usize,
    fixture_generation_allowed_count: usize,
) -> &'static str {
    if !reason_packet_ready {
        "status_canary_evidence_source_reason_packet_blocked_not_ready"
    } else if adapter_input_count == 0 {
        "status_canary_evidence_source_reason_packet_ready_no_adapter_inputs"
    } else if source_adapter_rejection_reason_count > 0 {
        "status_canary_evidence_source_reason_packet_ready_adapter_rejections_reported"
    } else if fixture_generation_allowed_count > 0 {
        "status_canary_evidence_source_reason_packet_ready_fixture_reasons_reported"
    } else {
        "status_canary_evidence_source_reason_packet_ready_reasons_reported"
    }
}

impl StatusCanaryEvidenceSourceReasonPacketSideEffects {
    pub const fn none() -> Self {
        Self {
            source_reason_packet_persisted: false,
            source_adapter_executed: false,
            source_read_persisted: false,
            evidence_recorded: false,
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
    use crate::status_canary_evidence_source_adapter::StatusCanaryEvidenceSourceAdapterInput;
    use crate::status_canary_evidence_source_adapter::status_canary_evidence_source_adapter_from_inputs;

    #[test]
    fn default_reason_packet_explains_all_source_decisions_without_inputs() {
        let packet = status_canary_evidence_source_reason_packet();

        assert_eq!(
            packet.reason_packet_id,
            STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_ID
        );
        assert!(packet.reason_packet_ready);
        assert_eq!(
            packet.reason_packet_route,
            "status_canary_evidence_source_reason_packet_ready_no_adapter_inputs"
        );
        assert!(packet.source_adapter_bound);
        assert!(packet.source_adapter_ready);
        assert_eq!(packet.source_count, 7);
        assert_eq!(packet.source_decision_reason_count, 28);
        assert_eq!(packet.source_decision_reason_ready_count, 28);
        assert_eq!(packet.decision_required_field_count, 84);
        assert_eq!(packet.missing_required_field_reason_count, 84);
        assert_eq!(packet.source_adapter_input_missing_reason_count, 28);
        assert_eq!(packet.source_adapter_rejection_reason_count, 0);
        assert_eq!(packet.fixture_generation_allowed_count, 0);
        assert_eq!(packet.fixture_generation_blocked_count, 28);
        assert_eq!(
            packet.side_effects,
            StatusCanaryEvidenceSourceReasonPacketSideEffects::none()
        );
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.requested_decision == StatusCanaryEvidenceDecision::Recorded
                && entry.decision_required_fields
                    == vec![
                        "evidence_artifact_present",
                        "evidence_digest_present",
                        "operator_authority_present",
                        "freshness_attested",
                    ]
                && entry.missing_required_fields == entry.decision_required_fields
                && entry.fixture_generation_blocker_reason
                    == "source_adapter_input_missing_for_decision"
                && !entry.fixture_generation_allowed
                && !entry.evidence_recording_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.live_mutation_allowed
        }));
    }

    #[test]
    fn missing_required_field_reports_source_decision_rejection_reason() {
        let mut input =
            StatusCanaryEvidenceSourceAdapterInput::recorded("operator_live_approval_missing");
        input.evidence_digest_present = false;
        let source_adapter = status_canary_evidence_source_adapter_from_inputs(&[input]);
        let packet = status_canary_evidence_source_reason_packet_from_adapter(&source_adapter);

        assert!(packet.reason_packet_ready);
        assert!(!packet.source_adapter_ready);
        assert_eq!(
            packet.reason_packet_route,
            "status_canary_evidence_source_reason_packet_ready_adapter_rejections_reported"
        );
        assert_eq!(packet.source_adapter_rejection_reason_count, 1);
        assert_eq!(packet.fixture_generation_allowed_count, 0);
        assert_eq!(packet.fixture_generation_blocked_count, 28);
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.requested_decision == StatusCanaryEvidenceDecision::Recorded
                && entry.source_adapter_input_present
                && entry.source_adapter_rejected
                && entry.source_adapter_rejection_reason == "source_adapter_required_fields_missing"
                && entry.fixture_generation_blocker_reason
                    == "source_adapter_required_fields_missing"
                && entry.missing_required_fields == vec!["evidence_digest_present"]
                && entry.missing_required_field_count == 1
                && !entry.fixture_generation_allowed
        }));
        assert!(packet.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.requested_decision == StatusCanaryEvidenceDecision::Waived
                && entry.source_adapter_input_for_other_decision_present
                && entry.fixture_generation_blocker_reason
                    == "source_adapter_input_for_different_decision"
        }));
    }
}
