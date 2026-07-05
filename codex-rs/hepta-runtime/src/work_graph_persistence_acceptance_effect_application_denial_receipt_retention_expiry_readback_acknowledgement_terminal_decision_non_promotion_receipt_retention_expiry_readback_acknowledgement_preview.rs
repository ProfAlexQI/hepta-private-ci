use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAcknowledgementPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub acknowledgement_contract_count: usize,
    pub non_acceptance_reason_count: usize,
    pub recording_denial_count: usize,
    pub expiry_replay_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub acknowledgement_contracts:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementContractPreview>,
    pub non_acceptance_reasons:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementNonAcceptancePreview>,
    pub recording_denials:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementRecordingDenialPreview>,
    pub expiry_replay_guards:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview>,
    pub local_views: Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphTerminalReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphTerminalReceiptRetentionReadbackAcknowledgementPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAcknowledgementContractPreview {
    pub id: &'static str,
    pub source_readback_receipt_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acceptance_allowed: bool,
    pub acknowledgement_recording_enabled: bool,
    pub receipt_recording_enabled: bool,
    pub authority_grant_enabled: bool,
    pub public_claim_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAcknowledgementNonAcceptancePreview {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAcknowledgementRecordingDenialPreview {
    pub id: &'static str,
    pub target_record: &'static str,
    pub reason: &'static str,
    pub blocks_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub blocks_acknowledgement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAcknowledgementLocalViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_acknowledgement_ids: Vec<&'static str>,
    pub durable_field_count: usize,
    pub preview_binding_count: usize,
    pub invariant_count: usize,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAcknowledgementInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAcknowledgementPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_receipt_retention_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub readback_acknowledgement_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub approval_recorded: bool,
    pub authority_granted: bool,
    pub live_persistence_enabled: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub enforcement_enabled: bool,
    pub rollout_started: bool,
    pub traffic_routed: bool,
    pub release_published: bool,
    pub public_claim_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report()
-> WorkGraphTerminalReceiptRetentionReadbackAcknowledgementPreviewReport {
    let acknowledgement_contracts =
        work_graph_terminal_receipt_retention_readback_acknowledgement_contracts();
    let non_acceptance_reasons =
        work_graph_terminal_receipt_retention_readback_acknowledgement_non_acceptance_reasons();
    let recording_denials =
        work_graph_terminal_receipt_retention_readback_acknowledgement_recording_denials();
    let expiry_replay_guards =
        work_graph_terminal_receipt_retention_readback_acknowledgement_expiry_replay_guards();
    let local_views = work_graph_terminal_receipt_retention_readback_acknowledgement_local_views();
    let durable_identity_evidence =
        work_graph_terminal_receipt_retention_readback_acknowledgement_durable_identity_evidence();
    let invariants = work_graph_terminal_receipt_retention_readback_acknowledgement_invariants();

    WorkGraphTerminalReceiptRetentionReadbackAcknowledgementPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_replay_guard_count: expiry_replay_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_terminal_receipt_retention_readback_acknowledgement_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_replay_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphTerminalReceiptRetentionReadbackAcknowledgementPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_receipt_retention_readback_acknowledgement_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = crate::work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_required_prior_gates();
    gates.retain(|gate| *gate != "hepta_work_graph_durable_identity_preview_gate");
    gates.push(
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate",
    );
    gates.push("hepta_work_graph_durable_identity_preview_gate");
    gates
}

pub fn work_graph_terminal_receipt_retention_readback_acknowledgement_ids() -> Vec<&'static str> {
    vec![
        "terminal_receipt_retention_policy_readback_acknowledgement",
        "terminal_receipt_expiry_guard_readback_acknowledgement",
        "terminal_receipt_supersession_guard_readback_acknowledgement",
        "terminal_receipt_gc_denial_readback_acknowledgement",
        "terminal_receipt_zero_effect_digest_readback_acknowledgement",
        "terminal_receipt_release_public_claim_denial_readback_acknowledgement",
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_acknowledgement_durable_identity_field_ids()
-> Vec<&'static str> {
    vec![
        "workflow_id",
        "run_id",
        "step_id",
        "checkpoint",
        "replay_key",
        "rollback_anchor",
        "receipt_hash",
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_acknowledgement_contracts()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementContractPreview> {
    vec![
        acknowledgement_contract(
            "terminal_receipt_retention_policy_readback_acknowledgement",
            "terminal_receipt_retention_policy_readback_receipt",
        ),
        acknowledgement_contract(
            "terminal_receipt_expiry_guard_readback_acknowledgement",
            "terminal_receipt_expiry_guard_readback_receipt",
        ),
        acknowledgement_contract(
            "terminal_receipt_supersession_guard_readback_acknowledgement",
            "terminal_receipt_supersession_guard_readback_receipt",
        ),
        acknowledgement_contract(
            "terminal_receipt_gc_denial_readback_acknowledgement",
            "terminal_receipt_gc_denial_readback_receipt",
        ),
        acknowledgement_contract(
            "terminal_receipt_zero_effect_digest_readback_acknowledgement",
            "terminal_receipt_zero_effect_digest_readback_receipt",
        ),
        acknowledgement_contract(
            "terminal_receipt_release_public_claim_denial_readback_acknowledgement",
            "terminal_receipt_release_public_claim_denial_readback_receipt",
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_acknowledgement_non_acceptance_reasons()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementNonAcceptancePreview> {
    let acknowledgement_ids = work_graph_terminal_receipt_retention_readback_acknowledgement_ids();

    vec![
        non_acceptance_reason(
            "durable_identity_evidence_missing",
            acknowledgement_ids.clone(),
            "terminal receipt retention readback acknowledgement does not include durable identity evidence",
        ),
        non_acceptance_reason(
            "terminal_retention_readback_ack_is_not_acceptance",
            acknowledgement_ids.clone(),
            "terminal retention readback acknowledgement only confirms local preview visibility",
        ),
        non_acceptance_reason(
            "terminal_retention_readback_ack_cannot_record_acknowledgement",
            acknowledgement_ids.clone(),
            "terminal retention readback acknowledgement cannot record acknowledgement state",
        ),
        non_acceptance_reason(
            "terminal_retention_readback_ack_cannot_record_approval",
            acknowledgement_ids.clone(),
            "terminal retention readback acknowledgement cannot record approval or acceptance",
        ),
        non_acceptance_reason(
            "terminal_retention_readback_ack_cannot_grant_authority",
            acknowledgement_ids.clone(),
            "terminal retention readback acknowledgement cannot grant WorkGraph authority",
        ),
        non_acceptance_reason(
            "terminal_retention_readback_ack_cannot_enable_persistence",
            acknowledgement_ids.clone(),
            "terminal retention readback acknowledgement cannot enable live persistence, WAL, or checkpoints",
        ),
        non_acceptance_reason(
            "terminal_retention_readback_ack_cannot_start_rollout",
            acknowledgement_ids.clone(),
            "terminal retention readback acknowledgement cannot start rollout or route traffic",
        ),
        non_acceptance_reason(
            "terminal_retention_readback_ack_cannot_publish_or_send",
            acknowledgement_ids,
            "terminal retention readback acknowledgement cannot publish release state, record public claims, or send externally",
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_acknowledgement_recording_denials()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementRecordingDenialPreview> {
    vec![
        recording_denial(
            "deny_durable_identity_terminal_receipt_retention_readback_ack_recording",
            "durable_identity_terminal_receipt_retention_readback_acknowledgement_evidence",
            "terminal receipt retention readback acknowledgement recording is blocked without durable identity evidence",
        ),
        recording_denial(
            "terminal_retention_readback_ack_recording_denied",
            "terminal_retention_readback_acknowledgement_store",
            "terminal receipt retention readback acknowledgement recording is disabled in preview",
        ),
        recording_denial(
            "terminal_receipt_retention_state_recording_denied",
            "terminal_receipt_retention_state_store",
            "readback acknowledgement cannot persist terminal receipt retention state",
        ),
        recording_denial(
            "terminal_retention_readback_receipt_recording_denied",
            "terminal_receipt_retention_readback_receipt_store",
            "readback acknowledgement cannot persist terminal receipt readback state",
        ),
        recording_denial(
            "terminal_operator_acceptance_recording_denied",
            "operator_acceptance_record",
            "terminal retention readback acknowledgement is not operator acceptance",
        ),
        recording_denial(
            "terminal_approval_ledger_recording_denied",
            "approval_ledger",
            "terminal retention readback acknowledgement cannot write approval ledger entries",
        ),
        recording_denial(
            "terminal_authority_grant_recording_denied",
            "authority_grant_record",
            "terminal retention readback acknowledgement cannot grant authority",
        ),
        recording_denial(
            "terminal_release_public_claim_delivery_recording_denied",
            "release_public_claim_external_delivery_record",
            "terminal retention readback acknowledgement cannot publish release state, record public claims, or create delivery records",
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_acknowledgement_expiry_replay_guards()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview> {
    let acknowledgement_ids = work_graph_terminal_receipt_retention_readback_acknowledgement_ids();

    vec![
        expiry_replay_guard(
            "terminal_retention_readback_receipt_expired",
            acknowledgement_ids.clone(),
            "terminal retention readback receipt exceeded the local preview window",
        ),
        expiry_replay_guard(
            "terminal_retention_readback_receipt_scope_superseded",
            acknowledgement_ids.clone(),
            "terminal retention readback receipt scope was superseded by a newer blocker report",
        ),
        expiry_replay_guard(
            "terminal_retention_readback_receipt_digest_mismatch",
            acknowledgement_ids.clone(),
            "terminal retention readback receipt digest does not match local evidence",
        ),
        expiry_replay_guard(
            "terminal_retention_gc_denial_receipt_replayed",
            acknowledgement_ids.clone(),
            "terminal garbage-collection denial readback receipt replay was observed",
        ),
        expiry_replay_guard(
            "terminal_retention_readback_ack_replay_detected",
            acknowledgement_ids,
            "terminal retention readback acknowledgement idempotency key has already been observed",
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_acknowledgement_local_views()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementLocalViewPreview> {
    vec![
        local_view(
            "operator_terminal_retention_readback_acknowledgement_view",
            "operator",
            with_terminal_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "sourceReadbackReceiptId",
                "accepted",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_terminal_retention_readback_acknowledgement_view",
            "auditor",
            with_terminal_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementHash",
                "sourceReadbackReceiptHash",
                "scopeDigest",
                "zeroEffectHash",
            ]),
        ),
        local_view(
            "release_owner_terminal_retention_readback_acknowledgement_view",
            "release_owner",
            with_terminal_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "publicClaimDenied",
                "externalDeliveryDenied",
            ]),
        ),
        local_view(
            "runtime_terminal_retention_readback_ack_zero_effect_view",
            "system",
            with_terminal_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementRecorded",
                "retentionStatePersisted",
                "authorityGranted",
                "publicClaimRecorded",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_acknowledgement_durable_identity_evidence()
-> WorkGraphTerminalReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview {
    WorkGraphTerminalReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_terminal_receipt_retention_readback_acknowledgement_durable_identity_field_ids(
            ),
        required_for_acknowledgement_ids:
            work_graph_terminal_receipt_retention_readback_acknowledgement_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_terminal_receipt_retention_readback_acknowledgement_invariants()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAcknowledgementInvariantPreview> {
    vec![
        invariant(
            "terminal_receipt_retention_readback_acknowledgements_require_durable_identity_evidence",
            "terminal receipt retention readback acknowledgement contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "terminal_retention_readback_acknowledgements_are_hash_only",
            "acknowledgements expose only local hash-only readback receipt references",
        ),
        invariant(
            "terminal_retention_readback_acknowledgements_are_non_accepting",
            "terminal retention readback acknowledgement visibility cannot become acceptance",
        ),
        invariant(
            "terminal_retention_readback_acknowledgements_are_non_recording",
            "acknowledgement preview cannot record receipt, approval, acceptance, authority, public claim, or retention state",
        ),
        invariant(
            "terminal_retention_readback_acknowledgement_views_are_local_only",
            "operator, auditor, release-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "terminal_retention_readback_acknowledgement_requires_readback_receipt_gate",
            "acknowledgement preview requires terminal receipt retention readback receipt evidence first",
        ),
        invariant(
            "terminal_retention_readback_acknowledgement_preview_has_no_side_effects",
            "this gate cannot persist, grant authority, enable live execution, publish, record public claims, or send externally",
        ),
    ]
}

fn acknowledgement_contract(
    id: &'static str,
    source_readback_receipt_id: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAcknowledgementContractPreview {
    WorkGraphTerminalReceiptRetentionReadbackAcknowledgementContractPreview {
        id,
        source_readback_receipt_id,
        required_fields:
            with_terminal_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "sourceReadbackReceiptId",
                "readbackReceiptHash",
                "retentionScope",
                "acknowledgementHash",
                "accepted",
                "recordingEnabled",
                "nextGate",
            ]),
        acceptance_allowed: false,
        acknowledgement_recording_enabled: false,
        receipt_recording_enabled: false,
        authority_grant_enabled: false,
        public_claim_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_terminal_receipt_retention_readback_acknowledgement_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_terminal_receipt_retention_readback_acknowledgement_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn non_acceptance_reason(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAcknowledgementNonAcceptancePreview {
    WorkGraphTerminalReceiptRetentionReadbackAcknowledgementNonAcceptancePreview {
        id,
        applies_to_acknowledgement_ids,
        reason,
        blocks_acceptance: true,
    }
}

fn recording_denial(
    id: &'static str,
    target_record: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAcknowledgementRecordingDenialPreview {
    WorkGraphTerminalReceiptRetentionReadbackAcknowledgementRecordingDenialPreview {
        id,
        target_record,
        reason,
        blocks_recording: true,
    }
}

fn expiry_replay_guard(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview {
    WorkGraphTerminalReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview {
        id,
        applies_to_acknowledgement_ids,
        trigger,
        blocks_acknowledgement: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTerminalReceiptRetentionReadbackAcknowledgementLocalViewPreview {
    WorkGraphTerminalReceiptRetentionReadbackAcknowledgementLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAcknowledgementInvariantPreview {
    WorkGraphTerminalReceiptRetentionReadbackAcknowledgementInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphTerminalReceiptRetentionReadbackAcknowledgementPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
            terminal_receipt_retention_state_persisted: false,
            readback_receipt_persisted: false,
            readback_acknowledgement_recorded: false,
            operator_acceptance_recorded: false,
            approval_recorded: false,
            authority_granted: false,
            live_persistence_enabled: false,
            wal_written: false,
            checkpoint_written: false,
            enforcement_enabled: false,
            rollout_started: false,
            traffic_routed: false,
            release_published: false,
            public_claim_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_retention_readback_acknowledgement_declares_non_accepting_contracts() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.acknowledgement_contract_count, 6);
        assert_eq!(
            report
                .acknowledgement_contracts
                .iter()
                .map(|contract| contract.id)
                .collect::<Vec<_>>(),
            work_graph_terminal_receipt_retention_readback_acknowledgement_ids()
        );
        assert!(report.acknowledgement_contracts.iter().all(|contract| {
            !contract.acceptance_allowed
                && !contract.acknowledgement_recording_enabled
                && !contract.receipt_recording_enabled
                && !contract.authority_grant_enabled
                && !contract.public_claim_enabled
                && !contract.external_delivery_enabled
                && contract.required_fields.contains(&"workflow_id")
                && contract.required_fields.contains(&"receipt_hash")
                && contract.required_fields.len() >= 15
        }));
    }

    #[test]
    fn terminal_retention_readback_acknowledgement_blocks_acceptance_and_recording() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.non_acceptance_reason_count, 8);
        assert!(report.non_acceptance_reasons.iter().all(|reason| {
            reason.blocks_acceptance && reason.applies_to_acknowledgement_ids.len() == 6
        }));
        assert!(
            report
                .non_acceptance_reasons
                .iter()
                .any(|reason| reason.id == "durable_identity_evidence_missing")
        );
        assert_eq!(report.recording_denial_count, 8);
        assert!(
            report
                .recording_denials
                .iter()
                .all(|denial| denial.blocks_recording)
        );
        assert!(report.recording_denials.iter().any(|denial| denial.target_record
            == "durable_identity_terminal_receipt_retention_readback_acknowledgement_evidence"));
    }

    #[test]
    fn terminal_retention_readback_acknowledgement_guards_expiry_scope_and_replay() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.expiry_replay_guard_count, 5);
        assert!(report.expiry_replay_guards.iter().all(|guard| {
            guard.blocks_acknowledgement && guard.applies_to_acknowledgement_ids.len() == 6
        }));
    }

    #[test]
    fn terminal_retention_readback_acknowledgement_requires_readback_receipt_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview
        );
    }

    #[test]
    fn terminal_retention_readback_acknowledgement_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert!(
            report
                .local_views
                .iter()
                .all(|view| !view.external_delivery_enabled
                    && view.required_fields.contains(&"workflow_id")
                    && view.required_fields.contains(&"receipt_hash")
                    && view.required_fields.len() >= 11)
        );
        let durable_evidence = report.durable_identity_evidence;
        assert_eq!(
            durable_evidence.required_prior_gate,
            "hepta_work_graph_durable_identity_preview_gate"
        );
        assert_eq!(durable_evidence.durable_field_count, 7);
        assert_eq!(durable_evidence.preview_binding_count, 5);
        assert_eq!(durable_evidence.invariant_count, 7);
        assert!(!durable_evidence.currently_satisfied);
        assert_eq!(report.invariant_count, 7);
        assert!(
            report
                .invariants
                .iter()
                .any(|invariant| invariant.id
                    == "terminal_receipt_retention_readback_acknowledgements_require_durable_identity_evidence")
        );
        assert!(report.invariants.iter().all(|invariant| invariant.required));
    }

    #[test]
    fn terminal_retention_readback_acknowledgement_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalReceiptRetentionReadbackAcknowledgementPreviewSideEffects::none()
        );
    }
}
