use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_ACKNOWLEDGEMENT_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_ACKNOWLEDGEMENT_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub acknowledgement_contract_count: usize,
    pub non_acceptance_reason_count: usize,
    pub recording_denial_count: usize,
    pub expiry_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub acknowledgement_contracts:
        Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementContractPreview>,
    pub non_acceptance_reasons:
        Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementNonAcceptanceReasonPreview>,
    pub recording_denials:
        Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementRecordingDenialPreview>,
    pub expiry_guards:
        Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementExpiryGuardPreview>,
    pub local_views:
        Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_blocker_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementContractPreview {
    pub id: &'static str,
    pub source_acceptance_record_receipt_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acceptance_allowed: bool,
    pub acknowledgement_recording_enabled: bool,
    pub authority_grant_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementNonAcceptanceReasonPreview {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementRecordingDenialPreview {
    pub id: &'static str,
    pub target_record: &'static str,
    pub reason: &'static str,
    pub blocks_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementExpiryGuardPreview {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub blocks_acknowledgement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementLocalViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementDurableIdentityEvidencePreview
{
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_acknowledgement_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub acceptance_record_persisted: bool,
    pub acceptance_record_receipt_persisted: bool,
    pub acknowledgement_recorded: bool,
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
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_report()
-> WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementPreviewReport {
    let acknowledgement_contracts =
        work_graph_persistence_acceptance_record_receipt_acknowledgement_contracts();
    let non_acceptance_reasons =
        work_graph_persistence_acceptance_record_receipt_acknowledgement_non_acceptance_reasons();
    let recording_denials =
        work_graph_persistence_acceptance_record_receipt_acknowledgement_recording_denials();
    let expiry_guards =
        work_graph_persistence_acceptance_record_receipt_acknowledgement_expiry_guards();
    let local_views =
        work_graph_persistence_acceptance_record_receipt_acknowledgement_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_record_receipt_acknowledgement_durable_identity_evidence(
        );
    let invariants = work_graph_persistence_acceptance_record_receipt_acknowledgement_invariants();

    WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_ACKNOWLEDGEMENT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_ACKNOWLEDGEMENT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_record_receipt_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_guard_count: expiry_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_record_receipt_acknowledgement_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_blocker_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_record_receipt_acknowledgement_required_prior_gates()
-> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
        "hepta_work_graph_adapter_projection_fixture_gate",
        "hepta_work_graph_state_store_persistence_preview_gate",
        "hepta_work_graph_replay_readback_preview_gate",
        "hepta_work_graph_promotion_precondition_preview_gate",
        "hepta_work_graph_activation_enforcement_blocker_preview_gate",
        "hepta_work_graph_shadow_adapter_readback_preview_gate",
        "hepta_work_graph_persistence_feature_flag_preview_gate",
        "hepta_work_graph_persistence_canary_dry_run_preview_gate",
        "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
        "hepta_work_graph_persistence_promotion_blocker_preview_gate",
        "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate",
        "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate",
        "hepta_work_graph_persistence_operator_readiness_packet_preview_gate",
        "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate",
        "hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate",
        "hepta_work_graph_persistence_acceptance_authority_blocker_preview_gate",
        "hepta_work_graph_persistence_acceptance_record_intake_preview_gate",
        "hepta_work_graph_persistence_acceptance_record_receipt_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_acknowledgement_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_record_receipt_acknowledgement_ids() -> Vec<&'static str> {
    vec![
        "trusted_operator_acceptance_record_receipt_acknowledgement",
        "approval_decision_record_receipt_acknowledgement",
        "live_persistence_enablement_record_receipt_acknowledgement",
        "rollback_quarantine_owner_attestation_receipt_acknowledgement",
        "release_publication_owner_attestation_receipt_acknowledgement",
        "external_delivery_consent_record_receipt_acknowledgement",
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_acknowledgement_contracts()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementContractPreview> {
    vec![
        acknowledgement_contract(
            "trusted_operator_acceptance_record_receipt_acknowledgement",
            "trusted_operator_acceptance_record_receipt",
        ),
        acknowledgement_contract(
            "approval_decision_record_receipt_acknowledgement",
            "approval_decision_record_receipt",
        ),
        acknowledgement_contract(
            "live_persistence_enablement_record_receipt_acknowledgement",
            "live_persistence_enablement_record_receipt",
        ),
        acknowledgement_contract(
            "rollback_quarantine_owner_attestation_receipt_acknowledgement",
            "rollback_quarantine_owner_attestation_receipt",
        ),
        acknowledgement_contract(
            "release_publication_owner_attestation_receipt_acknowledgement",
            "release_publication_owner_attestation_receipt",
        ),
        acknowledgement_contract(
            "external_delivery_consent_record_receipt_acknowledgement",
            "external_delivery_consent_record_receipt",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_acknowledgement_non_acceptance_reasons()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementNonAcceptanceReasonPreview> {
    let acknowledgement_ids =
        work_graph_persistence_acceptance_record_receipt_acknowledgement_ids();

    vec![
        non_acceptance_reason(
            "durable_identity_evidence_missing",
            acknowledgement_ids.clone(),
            "receipt acknowledgement does not include durable identity evidence",
        ),
        non_acceptance_reason(
            "acknowledgement_is_not_acceptance_record",
            acknowledgement_ids.clone(),
            "receipt acknowledgement only confirms local preview visibility",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_record_approval",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot record approval or acceptance",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_grant_authority",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot grant WorkGraph persistence authority",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_enable_live_persistence",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot enable persistence, WAL, checkpoints, enforcement, or rollout",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_release_or_publish",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot publish release status or artifacts",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_send_externally",
            acknowledgement_ids,
            "receipt acknowledgement cannot send externally",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_acknowledgement_recording_denials()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementRecordingDenialPreview> {
    vec![
        recording_denial(
            "deny_durable_identity_ack_recording",
            "durable_identity_acknowledgement_evidence",
            "acknowledgement recording is blocked without durable identity evidence",
        ),
        recording_denial(
            "deny_acceptance_record_receipt_seen_recording",
            "acceptance_record_receipt_seen",
            "receipt seen visibility cannot become recorded acceptance",
        ),
        recording_denial(
            "deny_acceptance_record_receipt_confirmed_recording",
            "acceptance_record_receipt_confirmed",
            "receipt confirmation cannot be recorded from preview",
        ),
        recording_denial(
            "deny_approval_ack_recording",
            "approval_acknowledgement",
            "approval acknowledgement recording is blocked",
        ),
        recording_denial(
            "deny_authority_ack_recording",
            "authority_acknowledgement",
            "authority acknowledgement recording is blocked",
        ),
        recording_denial(
            "deny_release_ack_recording",
            "release_acknowledgement",
            "release acknowledgement recording is blocked",
        ),
        recording_denial(
            "deny_external_delivery_ack_send",
            "external_delivery_ack_send",
            "external acknowledgement send is blocked",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_acknowledgement_expiry_guards()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementExpiryGuardPreview> {
    let acknowledgement_ids =
        work_graph_persistence_acceptance_record_receipt_acknowledgement_ids();

    vec![
        expiry_guard(
            "acceptance_record_acknowledgement_expired",
            acknowledgement_ids.clone(),
            "acknowledgement preview expires before receipt readback",
        ),
        expiry_guard(
            "acceptance_record_acknowledgement_superseded",
            acknowledgement_ids.clone(),
            "newer acknowledgement packet supersedes this preview",
        ),
        expiry_guard(
            "acceptance_record_acknowledgement_scope_revoked",
            acknowledgement_ids.clone(),
            "operator or authority scope is revoked before acknowledgement",
        ),
        expiry_guard(
            "acceptance_record_acknowledgement_receipt_digest_mismatch",
            acknowledgement_ids,
            "acknowledgement digest does not match acceptance record receipt digest",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_acknowledgement_local_views()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementLocalViewPreview> {
    vec![
        local_view(
            "operator_acceptance_record_acknowledgement_non_acceptance_view",
            "operator",
            with_acceptance_record_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "receiptId",
                "nonAcceptanceReasonIds",
                "acceptanceDenied",
            ]),
        ),
        local_view(
            "auditor_acceptance_record_acknowledgement_digest_view",
            "auditor",
            with_acceptance_record_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementHash",
                "receiptHash",
                "recordingDenialIds",
                "sideEffectHash",
            ]),
        ),
        local_view(
            "release_owner_acceptance_record_acknowledgement_denial_view",
            "release_owner",
            with_acceptance_record_receipt_acknowledgement_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "nextGate",
            ]),
        ),
        local_view(
            "runtime_acceptance_record_acknowledgement_zero_effect_view",
            "system",
            with_acceptance_record_receipt_acknowledgement_durable_identity_fields(vec![
                "authorityGranted",
                "approvalRecorded",
                "livePersistenceEnabled",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_acknowledgement_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementDurableIdentityEvidencePreview {
    WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_record_receipt_acknowledgement_durable_identity_field_ids(),
        required_for_acknowledgement_ids:
            work_graph_persistence_acceptance_record_receipt_acknowledgement_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_record_receipt_acknowledgement_invariants()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementInvariantPreview> {
    vec![
        invariant(
            "acceptance_record_receipt_acknowledgements_require_durable_identity_evidence",
            "acceptance record receipt acknowledgement contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "acceptance_record_receipt_acknowledgement_is_non_acceptance",
            "acknowledgement contracts explicitly deny acceptance and authority",
        ),
        invariant(
            "acceptance_record_receipt_acknowledgement_recording_is_blocked",
            "all acknowledgement recording surfaces are blocked",
        ),
        invariant(
            "acceptance_record_receipt_acknowledgement_cannot_grant_authority",
            "acknowledgement cannot grant persistence, WAL, checkpoint, rollout, release, or delivery authority",
        ),
        invariant(
            "acceptance_record_receipt_acknowledgement_expiry_and_digest_guards_block",
            "expired, superseded, revoked, or mismatched acknowledgement digests block preview",
        ),
        invariant(
            "acceptance_record_receipt_acknowledgement_views_are_local_only",
            "operator, auditor, release-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "acceptance_record_receipt_acknowledgement_preview_has_no_side_effects",
            "this gate cannot record acknowledgement, acceptance, approval, authority, live execution, release, or external sends",
        ),
    ]
}

impl WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            acceptance_record_persisted: false,
            acceptance_record_receipt_persisted: false,
            acknowledgement_recorded: false,
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
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn acknowledgement_contract(
    id: &'static str,
    source_acceptance_record_receipt_id: &'static str,
) -> WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementContractPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementContractPreview {
        id,
        source_acceptance_record_receipt_id,
        required_fields: with_acceptance_record_receipt_acknowledgement_durable_identity_fields(
            vec![
                "acknowledgementId",
                "receiptId",
                "acknowledgementHash",
                "receiptHash",
                "nonAcceptanceReasonIds",
                "recordingDenied",
                "authorityDenied",
                "liveExecutionDenied",
                "externalDeliveryDenied",
            ],
        ),
        acceptance_allowed: false,
        acknowledgement_recording_enabled: false,
        authority_grant_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_acceptance_record_receipt_acknowledgement_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_acceptance_record_receipt_acknowledgement_durable_identity_field_ids(
        );
    merged.extend(fields);
    merged
}

fn non_acceptance_reason(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementNonAcceptanceReasonPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementNonAcceptanceReasonPreview {
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
) -> WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementRecordingDenialPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementRecordingDenialPreview {
        id,
        target_record,
        reason,
        blocks_recording: true,
    }
}

fn expiry_guard(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementExpiryGuardPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementExpiryGuardPreview {
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
) -> WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementLocalViewPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementInvariantPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceptance_record_receipt_acknowledgement_declares_non_accepting_contracts() {
        let report =
            hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_report();
        let acknowledgement_ids = report
            .acknowledgement_contracts
            .iter()
            .map(|acknowledgement| acknowledgement.id)
            .collect::<Vec<_>>();

        assert_eq!(
            acknowledgement_ids,
            [
                "trusted_operator_acceptance_record_receipt_acknowledgement",
                "approval_decision_record_receipt_acknowledgement",
                "live_persistence_enablement_record_receipt_acknowledgement",
                "rollback_quarantine_owner_attestation_receipt_acknowledgement",
                "release_publication_owner_attestation_receipt_acknowledgement",
                "external_delivery_consent_record_receipt_acknowledgement",
            ]
        );
        assert_eq!(report.acknowledgement_contract_count, 6);
        assert!(report.acknowledgement_contracts.iter().all(|ack| {
            !ack.acceptance_allowed
                && !ack.acknowledgement_recording_enabled
                && !ack.authority_grant_enabled
                && !ack.external_delivery_enabled
                && ack.required_fields.len() >= 16
                && ack.required_fields.contains(&"workflow_id")
                && ack.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn acceptance_record_receipt_acknowledgement_blocks_acceptance_and_recording() {
        let report =
            hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_report();

        assert_eq!(report.non_acceptance_reason_count, 7);
        assert!(report.non_acceptance_reasons.iter().all(|reason| {
            reason.blocks_acceptance && reason.applies_to_acknowledgement_ids.len() == 6
        }));
        assert_eq!(report.recording_denial_count, 7);
        assert!(
            report
                .recording_denials
                .iter()
                .all(|denial| denial.blocks_recording)
        );
    }

    #[test]
    fn acceptance_record_receipt_acknowledgement_guards_expiry_scope_and_digest() {
        let report =
            hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_report();
        let guard_ids = report
            .expiry_guards
            .iter()
            .map(|guard| guard.id)
            .collect::<Vec<_>>();

        assert_eq!(
            guard_ids,
            [
                "acceptance_record_acknowledgement_expired",
                "acceptance_record_acknowledgement_superseded",
                "acceptance_record_acknowledgement_scope_revoked",
                "acceptance_record_acknowledgement_receipt_digest_mismatch",
            ]
        );
        assert_eq!(report.expiry_guard_count, 4);
        assert!(report.expiry_guards.iter().all(|guard| {
            guard.blocks_acknowledgement && guard.applies_to_acknowledgement_ids.len() == 6
        }));
    }

    #[test]
    fn acceptance_record_receipt_acknowledgement_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_report();
        let view_ids = report
            .local_views
            .iter()
            .map(|view| view.id)
            .collect::<Vec<_>>();

        assert_eq!(
            view_ids,
            [
                "operator_acceptance_record_acknowledgement_non_acceptance_view",
                "auditor_acceptance_record_acknowledgement_digest_view",
                "release_owner_acceptance_record_acknowledgement_denial_view",
                "runtime_acceptance_record_acknowledgement_zero_effect_view",
            ]
        );
        assert_eq!(report.local_view_count, 4);
        assert!(
            report
                .local_views
                .iter()
                .all(|view| !view.external_delivery_enabled
                    && view.required_fields.len() >= 11
                    && view.required_fields.contains(&"workflow_id")
                    && view.required_fields.contains(&"receipt_hash"))
        );
    }

    #[test]
    fn acceptance_record_receipt_acknowledgement_requires_durable_identity_evidence() {
        let report =
            hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_report();

        assert_eq!(
            report.durable_identity_evidence.schema_version,
            "work_graph_durable_identity_preview_v1"
        );
        assert_eq!(
            report.durable_identity_evidence.required_prior_gate,
            "hepta_work_graph_durable_identity_preview_gate"
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            [
                "workflow_id",
                "run_id",
                "step_id",
                "checkpoint",
                "replay_key",
                "rollback_anchor",
                "receipt_hash",
            ]
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_acknowledgement_ids
                .len(),
            6
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn acceptance_record_receipt_acknowledgement_requires_receipt_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_report();

        assert_eq!(
            report.required_prior_gates,
            [
                "hepta_work_graph_contract_preview_gate",
                "hepta_work_graph_task_result_contract_preview_gate",
                "hepta_work_graph_scheduler_admission_controller_preview_gate",
                "hepta_work_graph_observability_timeline_preview_gate",
                "hepta_work_graph_role_manifest_contract_preview_gate",
                "hepta_work_graph_unified_state_store_preview_gate",
                "hepta_work_graph_adapter_projection_fixture_gate",
                "hepta_work_graph_state_store_persistence_preview_gate",
                "hepta_work_graph_replay_readback_preview_gate",
                "hepta_work_graph_promotion_precondition_preview_gate",
                "hepta_work_graph_activation_enforcement_blocker_preview_gate",
                "hepta_work_graph_shadow_adapter_readback_preview_gate",
                "hepta_work_graph_persistence_feature_flag_preview_gate",
                "hepta_work_graph_persistence_canary_dry_run_preview_gate",
                "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
                "hepta_work_graph_persistence_promotion_blocker_preview_gate",
                "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate",
                "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate",
                "hepta_work_graph_persistence_operator_readiness_packet_preview_gate",
                "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate",
                "hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate",
                "hepta_work_graph_persistence_acceptance_authority_blocker_preview_gate",
                "hepta_work_graph_persistence_acceptance_record_intake_preview_gate",
                "hepta_work_graph_persistence_acceptance_record_receipt_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn acceptance_record_receipt_acknowledgement_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_report();

        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceRecordReceiptAcknowledgementPreviewSideEffects::none()
        );
        assert!(report.ready_for_acceptance_effect_application_blocker_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
