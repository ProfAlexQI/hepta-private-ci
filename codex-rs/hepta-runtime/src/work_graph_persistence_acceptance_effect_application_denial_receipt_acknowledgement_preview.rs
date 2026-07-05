use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_ACKNOWLEDGEMENT_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_ACKNOWLEDGEMENT_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementPreviewReport {
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
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementContractPreview>,
    pub non_acceptance_reasons:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementNonAcceptancePreview>,
    pub recording_denials:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementRecordingDenialPreview>,
    pub expiry_guards:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementExpiryGuardPreview>,
    pub local_views:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_replay_idempotency_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementContractPreview {
    pub id: &'static str,
    pub source_denial_receipt_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acceptance_allowed: bool,
    pub acknowledgement_recording_enabled: bool,
    pub authority_grant_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementNonAcceptancePreview {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementRecordingDenialPreview {
    pub id: &'static str,
    pub target_record: &'static str,
    pub reason: &'static str,
    pub blocks_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementExpiryGuardPreview {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub blocks_acknowledgement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementLocalViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementDurableIdentityEvidencePreview
{
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_acknowledgement_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub denial_receipt_persisted: bool,
    pub denial_receipt_acknowledgement_recorded: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_report()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementPreviewReport {
    let acknowledgement_contracts =
        work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_contracts();
    let non_acceptance_reasons =
        work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_non_acceptance_reasons();
    let recording_denials =
        work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_recording_denials();
    let expiry_guards =
        work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_expiry_guards();
    let local_views =
        work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_durable_identity_evidence();
    let invariants =
        work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_invariants();

    WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_ACKNOWLEDGEMENT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_ACKNOWLEDGEMENT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_guard_count: expiry_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_replay_idempotency_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_required_prior_gates()
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
        "hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_ids()
-> Vec<&'static str> {
    vec![
        "operator_acceptance_recording_denial_receipt_acknowledgement",
        "approval_ledger_write_denial_receipt_acknowledgement",
        "authority_grant_denial_receipt_acknowledgement",
        "graph_state_persistence_denial_receipt_acknowledgement",
        "wal_checkpoint_write_denial_receipt_acknowledgement",
        "enforcement_rollout_denial_receipt_acknowledgement",
        "release_publication_denial_receipt_acknowledgement",
        "external_delivery_denial_receipt_acknowledgement",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_contracts()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementContractPreview> {
    vec![
        acknowledgement_contract(
            "operator_acceptance_recording_denial_receipt_acknowledgement",
            "operator_acceptance_recording_denial_receipt",
        ),
        acknowledgement_contract(
            "approval_ledger_write_denial_receipt_acknowledgement",
            "approval_ledger_write_denial_receipt",
        ),
        acknowledgement_contract(
            "authority_grant_denial_receipt_acknowledgement",
            "authority_grant_denial_receipt",
        ),
        acknowledgement_contract(
            "graph_state_persistence_denial_receipt_acknowledgement",
            "graph_state_persistence_denial_receipt",
        ),
        acknowledgement_contract(
            "wal_checkpoint_write_denial_receipt_acknowledgement",
            "wal_checkpoint_write_denial_receipt",
        ),
        acknowledgement_contract(
            "enforcement_rollout_denial_receipt_acknowledgement",
            "enforcement_rollout_denial_receipt",
        ),
        acknowledgement_contract(
            "release_publication_denial_receipt_acknowledgement",
            "release_publication_denial_receipt",
        ),
        acknowledgement_contract(
            "external_delivery_denial_receipt_acknowledgement",
            "external_delivery_denial_receipt",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_non_acceptance_reasons()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementNonAcceptancePreview> {
    let acknowledgement_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_ids();

    vec![
        non_acceptance_reason(
            "durable_identity_evidence_missing",
            acknowledgement_ids.clone(),
            "denial receipt acknowledgement does not include durable identity evidence",
        ),
        non_acceptance_reason(
            "acknowledgement_is_not_effect_acceptance",
            acknowledgement_ids.clone(),
            "denial receipt acknowledgement only confirms local preview visibility",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_record_approval",
            acknowledgement_ids.clone(),
            "denial receipt acknowledgement cannot record approval or acceptance",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_grant_authority",
            acknowledgement_ids.clone(),
            "denial receipt acknowledgement cannot grant WorkGraph authority",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_enable_persistence_or_wal",
            acknowledgement_ids.clone(),
            "denial receipt acknowledgement cannot enable persistence, WAL, or checkpoints",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_start_rollout",
            acknowledgement_ids.clone(),
            "denial receipt acknowledgement cannot start enforcement rollout or route traffic",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_publish_release",
            acknowledgement_ids.clone(),
            "denial receipt acknowledgement cannot publish release status or artifacts",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_send_external_delivery",
            acknowledgement_ids,
            "denial receipt acknowledgement cannot send external delivery",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_recording_denials()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementRecordingDenialPreview> {
    vec![
        recording_denial(
            "deny_durable_identity_ack_recording",
            "durable_identity_acknowledgement_evidence",
            "acknowledgement recording is blocked without durable identity evidence",
        ),
        recording_denial(
            "denial_receipt_acknowledgement_recording_denied",
            "effect_denial_receipt_acknowledgement_store",
            "acknowledgement recording is disabled in preview",
        ),
        recording_denial(
            "operator_acceptance_recording_denied",
            "operator_acceptance_record",
            "denial receipt acknowledgement is not operator acceptance",
        ),
        recording_denial(
            "approval_ledger_recording_denied",
            "approval_ledger",
            "denial receipt acknowledgement cannot write approval ledger entries",
        ),
        recording_denial(
            "authority_grant_recording_denied",
            "authority_grant_record",
            "denial receipt acknowledgement cannot grant authority",
        ),
        recording_denial(
            "graph_state_persistence_denied",
            "work_graph_state_store",
            "denial receipt acknowledgement cannot persist graph state",
        ),
        recording_denial(
            "release_publication_recording_denied",
            "release_publication_record",
            "denial receipt acknowledgement cannot publish release state",
        ),
        recording_denial(
            "external_delivery_recording_denied",
            "external_delivery_record",
            "denial receipt acknowledgement cannot create external delivery records",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_expiry_guards()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementExpiryGuardPreview> {
    let acknowledgement_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_ids();

    vec![
        expiry_guard(
            "denial_receipt_expired",
            acknowledgement_ids.clone(),
            "denial receipt age exceeds local preview window",
        ),
        expiry_guard(
            "denial_receipt_scope_superseded",
            acknowledgement_ids.clone(),
            "denial receipt scope was superseded by a newer blocker report",
        ),
        expiry_guard(
            "denial_receipt_digest_mismatch",
            acknowledgement_ids.clone(),
            "denial receipt digest does not match the local readback digest",
        ),
        expiry_guard(
            "source_blocker_gate_superseded",
            acknowledgement_ids.clone(),
            "effect application blocker gate digest changed after receipt creation",
        ),
        expiry_guard(
            "acknowledgement_replay_detected",
            acknowledgement_ids,
            "acknowledgement idempotency key has already been observed",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_local_views()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementLocalViewPreview> {
    vec![
        local_view(
            "operator_effect_denial_receipt_acknowledgement_view",
            "operator",
            with_acceptance_effect_denial_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "sourceDenialReceiptId",
                "accepted",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_effect_denial_receipt_acknowledgement_view",
            "auditor",
            with_acceptance_effect_denial_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementHash",
                "sourceDenialReceiptHash",
                "scopeDigest",
                "zeroEffectHash",
            ]),
        ),
        local_view(
            "release_owner_effect_denial_receipt_acknowledgement_view",
            "release_owner",
            with_acceptance_effect_denial_receipt_acknowledgement_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "acknowledgementId",
            ]),
        ),
        local_view(
            "runtime_effect_denial_receipt_acknowledgement_zero_effect_view",
            "system",
            with_acceptance_effect_denial_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementRecorded",
                "authorityGranted",
                "statePersisted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementDurableIdentityEvidencePreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_durable_identity_field_ids(),
        required_for_acknowledgement_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_invariants()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementInvariantPreview> {
    vec![
        invariant(
            "effect_denial_receipt_acknowledgements_require_durable_identity_evidence",
            "effect denial receipt acknowledgement contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "effect_denial_receipt_acknowledgements_are_hash_only",
            "acknowledgements expose only local hash-only receipt references",
        ),
        invariant(
            "effect_denial_receipt_acknowledgements_are_non_accepting",
            "acknowledgement visibility cannot become effect acceptance",
        ),
        invariant(
            "effect_denial_receipt_acknowledgements_are_non_recording",
            "acknowledgement preview cannot record approval, acceptance, authority, or receipt state",
        ),
        invariant(
            "effect_denial_receipt_acknowledgement_views_are_local_only",
            "operator, auditor, release-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "effect_denial_receipt_acknowledgement_requires_denial_receipt_gate",
            "acknowledgement preview requires hash-only denial receipt evidence first",
        ),
        invariant(
            "effect_denial_receipt_acknowledgement_preview_has_no_side_effects",
            "this gate cannot persist, grant authority, enable live execution, publish, or send externally",
        ),
    ]
}

fn acknowledgement_contract(
    id: &'static str,
    source_denial_receipt_id: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementContractPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementContractPreview {
        id,
        source_denial_receipt_id,
        required_fields:
            with_acceptance_effect_denial_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "sourceDenialReceiptId",
                "denialReceiptHash",
                "receiptScope",
                "acknowledgementHash",
                "accepted",
                "recordingEnabled",
                "nextGate",
            ]),
        acceptance_allowed: false,
        acknowledgement_recording_enabled: false,
        authority_grant_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_acceptance_effect_denial_receipt_acknowledgement_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn non_acceptance_reason(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementNonAcceptancePreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementNonAcceptancePreview {
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
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementRecordingDenialPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementRecordingDenialPreview {
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
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementExpiryGuardPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementExpiryGuardPreview {
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
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementLocalViewPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementInvariantPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            denial_receipt_persisted: false,
            denial_receipt_acknowledgement_recorded: false,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_denial_receipt_acknowledgement_declares_non_accepting_contracts() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_report();

        assert_eq!(report.acknowledgement_contract_count, 8);
        assert_eq!(
            report.acknowledgement_contracts.len(),
            report.acknowledgement_contract_count
        );
        assert_eq!(
            report
                .acknowledgement_contracts
                .iter()
                .map(|contract| contract.id)
                .collect::<Vec<_>>(),
            work_graph_persistence_acceptance_effect_denial_receipt_acknowledgement_ids()
        );
        assert!(report.acknowledgement_contracts.iter().all(|contract| {
            !contract.acceptance_allowed
                && !contract.acknowledgement_recording_enabled
                && !contract.authority_grant_enabled
                && !contract.external_delivery_enabled
                && contract.required_fields.len() >= 15
                && contract.required_fields.contains(&"workflow_id")
                && contract.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn effect_denial_receipt_acknowledgement_blocks_acceptance_and_recording() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_report();

        assert_eq!(report.non_acceptance_reason_count, 8);
        assert!(report.non_acceptance_reasons.iter().all(|reason| {
            reason.blocks_acceptance
                && reason.applies_to_acknowledgement_ids.len() == 8
                && reason.reason.contains("acknowledgement")
        }));
        assert_eq!(report.recording_denial_count, 8);
        assert!(
            report
                .recording_denials
                .iter()
                .all(|denial| denial.blocks_recording)
        );
        assert!(
            report
                .recording_denials
                .iter()
                .any(|denial| denial.target_record == "approval_ledger")
        );
    }

    #[test]
    fn effect_denial_receipt_acknowledgement_guards_expiry_scope_and_replay() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_report();

        assert_eq!(report.expiry_guard_count, 5);
        assert!(report.expiry_guards.iter().all(|guard| {
            guard.blocks_acknowledgement && guard.applies_to_acknowledgement_ids.len() == 8
        }));
        assert!(
            report
                .expiry_guards
                .iter()
                .any(|guard| guard.id == "acknowledgement_replay_detected")
        );
    }

    #[test]
    fn effect_denial_receipt_acknowledgement_requires_durable_identity_evidence() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_report();

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
            8
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn effect_denial_receipt_acknowledgement_requires_denial_receipt_and_durable_identity_gates() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_report();

        assert!(
            report
                .required_prior_gates
                .contains(
                    &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_gate"
                )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_replay_idempotency_preview
        );
    }

    #[test]
    fn effect_denial_receipt_acknowledgement_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_report();

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
        assert_eq!(report.invariant_count, 7);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
    }

    #[test]
    fn effect_denial_receipt_acknowledgement_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_report();

        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectDenialReceiptAcknowledgementPreviewSideEffects::none(
            )
        );
    }
}
