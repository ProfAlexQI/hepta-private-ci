use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_record_receipt_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_record_receipt_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub receipt_contract_count: usize,
    pub digest_check_count: usize,
    pub receipt_denial_count: usize,
    pub readback_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub receipt_contracts: Vec<WorkGraphPersistenceAcceptanceRecordReceiptContractPreview>,
    pub digest_checks: Vec<WorkGraphPersistenceAcceptanceRecordReceiptDigestCheckPreview>,
    pub receipt_denials: Vec<WorkGraphPersistenceAcceptanceRecordReceiptDenialPreview>,
    pub readback_views: Vec<WorkGraphPersistenceAcceptanceRecordReceiptReadbackViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceRecordReceiptDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceAcceptanceRecordReceiptInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_record_receipt_acknowledgement_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceAcceptanceRecordReceiptPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptContractPreview {
    pub id: &'static str,
    pub source_record_template_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redaction_state: &'static str,
    pub persistence_enabled: bool,
    pub approval_recording_enabled: bool,
    pub authority_grant_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptDigestCheckPreview {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptDenialPreview {
    pub id: &'static str,
    pub applies_to_receipt_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_receipt_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptReadbackViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_receipt_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordReceiptPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub acceptance_record_persisted: bool,
    pub acceptance_record_receipt_persisted: bool,
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

pub fn hepta_work_graph_persistence_acceptance_record_receipt_preview_report()
-> WorkGraphPersistenceAcceptanceRecordReceiptPreviewReport {
    let receipt_contracts = work_graph_persistence_acceptance_record_receipt_contracts();
    let digest_checks = work_graph_persistence_acceptance_record_receipt_digest_checks();
    let receipt_denials = work_graph_persistence_acceptance_record_receipt_denials();
    let readback_views = work_graph_persistence_acceptance_record_receipt_readback_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_record_receipt_durable_identity_evidence();
    let invariants = work_graph_persistence_acceptance_record_receipt_invariants();

    WorkGraphPersistenceAcceptanceRecordReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_record_receipt_preview_no_receipt_write",
        receipt_contract_count: receipt_contracts.len(),
        digest_check_count: digest_checks.len(),
        receipt_denial_count: receipt_denials.len(),
        readback_view_count: readback_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_persistence_acceptance_record_receipt_required_prior_gates(
        ),
        receipt_contracts,
        digest_checks,
        receipt_denials,
        readback_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_record_receipt_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistenceAcceptanceRecordReceiptPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_record_receipt_required_prior_gates() -> Vec<&'static str>
{
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_record_receipt_ids() -> Vec<&'static str> {
    vec![
        "trusted_operator_acceptance_record_receipt",
        "approval_decision_record_receipt",
        "live_persistence_enablement_record_receipt",
        "rollback_quarantine_owner_attestation_receipt",
        "release_publication_owner_attestation_receipt",
        "external_delivery_consent_record_receipt",
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_contracts()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptContractPreview> {
    vec![
        receipt_contract(
            "trusted_operator_acceptance_record_receipt",
            "trusted_operator_acceptance_record",
        ),
        receipt_contract(
            "approval_decision_record_receipt",
            "approval_decision_record",
        ),
        receipt_contract(
            "live_persistence_enablement_record_receipt",
            "live_persistence_enablement_record",
        ),
        receipt_contract(
            "rollback_quarantine_owner_attestation_receipt",
            "rollback_quarantine_owner_attestation",
        ),
        receipt_contract(
            "release_publication_owner_attestation_receipt",
            "release_publication_owner_attestation",
        ),
        receipt_contract(
            "external_delivery_consent_record_receipt",
            "external_delivery_consent_record",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_digest_checks()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptDigestCheckPreview> {
    vec![
        digest_check(
            "check_durable_identity_digest",
            work_graph_persistence_acceptance_record_receipt_durable_identity_field_ids(),
        ),
        digest_check(
            "check_acceptance_record_template_digest",
            vec!["recordTemplateId", "recordDigestHash", "requiredFieldIds"],
        ),
        digest_check(
            "check_validation_denial_digest",
            vec!["validationDenialIds", "blockedEffectIds", "missingFieldIds"],
        ),
        digest_check(
            "check_redaction_digest",
            vec!["redactionDigestIds", "hashOnlyFieldIds", "redactionState"],
        ),
        digest_check(
            "check_authority_scope_digest",
            vec![
                "targetAuthoritySurfaceIds",
                "authorityDenied",
                "explicitAuthorityGrantDenied",
            ],
        ),
        digest_check(
            "check_side_effect_guard_digest",
            vec!["intakeGuardIds", "sideEffectHash", "persistenceDenied"],
        ),
        digest_check(
            "check_expiry_revocation_digest",
            vec!["expiresAtUnixMs", "revocationStatus", "supersessionId"],
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_denials()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptDenialPreview> {
    let receipt_ids = work_graph_persistence_acceptance_record_receipt_ids();

    vec![
        receipt_denial(
            "durable_identity_evidence_missing",
            receipt_ids.clone(),
            "receipt is missing durable identity evidence",
        ),
        receipt_denial(
            "missing_record_digest_hash",
            receipt_ids.clone(),
            "receipt is missing the hash-only acceptance record digest",
        ),
        receipt_denial(
            "record_template_absent",
            receipt_ids.clone(),
            "source acceptance record template is absent from preview intake",
        ),
        receipt_denial(
            "validation_denial_present",
            receipt_ids.clone(),
            "acceptance record validation denials are still present",
        ),
        receipt_denial(
            "recording_attempted",
            receipt_ids.clone(),
            "receipt cannot record acceptance or approval decisions",
        ),
        receipt_denial(
            "authority_grant_attempted",
            receipt_ids.clone(),
            "receipt cannot grant WorkGraph persistence authority",
        ),
        receipt_denial(
            "live_execution_attempted",
            receipt_ids.clone(),
            "receipt cannot enable live persistence, WAL, checkpoint, enforcement, or rollout execution",
        ),
        receipt_denial(
            "release_publication_attempted",
            receipt_ids.clone(),
            "receipt cannot publish release status or artifacts",
        ),
        receipt_denial(
            "external_delivery_attempted",
            receipt_ids,
            "receipt cannot send acknowledgements or readiness externally",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_readback_views()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptReadbackViewPreview> {
    vec![
        readback_view(
            "operator_acceptance_record_receipt_view",
            "operator",
            with_acceptance_record_receipt_durable_identity_fields(vec![
                "receiptId",
                "recordTemplateId",
                "receiptDenialIds",
                "acceptanceDenied",
            ]),
        ),
        readback_view(
            "auditor_acceptance_record_receipt_digest_view",
            "auditor",
            with_acceptance_record_receipt_durable_identity_fields(vec![
                "recordDigestHash",
                "receiptDigestHash",
                "redactionDigestIds",
                "sideEffectHash",
            ]),
        ),
        readback_view(
            "release_owner_acceptance_record_receipt_denial_view",
            "release_owner",
            with_acceptance_record_receipt_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "receiptDenialIds",
            ]),
        ),
        readback_view(
            "runtime_acceptance_record_receipt_zero_effect_view",
            "system",
            with_acceptance_record_receipt_durable_identity_fields(vec![
                "operatorAcceptanceRecorded",
                "authorityGranted",
                "livePersistenceEnabled",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_receipt_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceRecordReceiptDurableIdentityEvidencePreview {
    WorkGraphPersistenceAcceptanceRecordReceiptDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_record_receipt_durable_identity_field_ids(),
        required_for_receipt_ids: work_graph_persistence_acceptance_record_receipt_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_record_receipt_invariants()
-> Vec<WorkGraphPersistenceAcceptanceRecordReceiptInvariantPreview> {
    vec![
        invariant(
            "acceptance_record_receipts_require_durable_identity_evidence",
            "acceptance record receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "acceptance_record_receipts_are_hash_only",
            "acceptance record receipts contain hash-only redacted evidence",
        ),
        invariant(
            "acceptance_record_receipts_are_non_persistent",
            "receipt contracts cannot write acceptance, approval, authority, or graph state",
        ),
        invariant(
            "acceptance_record_receipts_block_live_effects",
            "receipts block live persistence, WAL, checkpoints, enforcement, rollout, traffic, release, and delivery",
        ),
        invariant(
            "acceptance_record_receipt_readback_views_are_local_only",
            "operator, auditor, release-owner, and runtime receipt views cannot be sent externally",
        ),
        invariant(
            "acceptance_record_receipt_requires_intake_gate",
            "receipt preview requires acceptance record intake preview as its direct prior gate",
        ),
        invariant(
            "acceptance_record_receipt_preview_has_no_side_effects",
            "this gate cannot persist receipts, record approval, grant authority, enable live execution, publish, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceAcceptanceRecordReceiptPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            acceptance_record_persisted: false,
            acceptance_record_receipt_persisted: false,
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

fn receipt_contract(
    id: &'static str,
    source_record_template_id: &'static str,
) -> WorkGraphPersistenceAcceptanceRecordReceiptContractPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptContractPreview {
        id,
        source_record_template_id,
        required_fields: with_acceptance_record_receipt_durable_identity_fields(vec![
            "receiptId",
            "sourceRecordTemplateId",
            "recordDigestHash",
            "intakeGuardIds",
            "validationDenialIds",
            "redactionDigestIds",
            "acceptanceDenied",
            "authorityDenied",
            "sideEffectHash",
        ]),
        redaction_state: "hash_only_redacted",
        persistence_enabled: false,
        approval_recording_enabled: false,
        authority_grant_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_acceptance_record_receipt_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged = work_graph_persistence_acceptance_record_receipt_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn digest_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceRecordReceiptDigestCheckPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptDigestCheckPreview {
        id,
        compared_fields,
        blocks_acceptance: true,
    }
}

fn receipt_denial(
    id: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceRecordReceiptDenialPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptDenialPreview {
        id,
        applies_to_receipt_ids,
        reason,
        blocks_receipt_acceptance: true,
    }
}

fn readback_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceRecordReceiptReadbackViewPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptReadbackViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceRecordReceiptInvariantPreview {
    WorkGraphPersistenceAcceptanceRecordReceiptInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceptance_record_receipt_declares_hash_only_receipts() {
        let report = hepta_work_graph_persistence_acceptance_record_receipt_preview_report();
        let receipt_ids = report
            .receipt_contracts
            .iter()
            .map(|receipt| receipt.id)
            .collect::<Vec<_>>();

        assert_eq!(
            receipt_ids,
            [
                "trusted_operator_acceptance_record_receipt",
                "approval_decision_record_receipt",
                "live_persistence_enablement_record_receipt",
                "rollback_quarantine_owner_attestation_receipt",
                "release_publication_owner_attestation_receipt",
                "external_delivery_consent_record_receipt",
            ]
        );
        assert_eq!(report.receipt_contract_count, 6);
        assert!(report.receipt_contracts.iter().all(|receipt| {
            receipt.redaction_state == "hash_only_redacted"
                && !receipt.persistence_enabled
                && !receipt.approval_recording_enabled
                && !receipt.authority_grant_enabled
                && !receipt.external_delivery_enabled
                && receipt.required_fields.len() >= 16
                && receipt.required_fields.contains(&"workflow_id")
                && receipt.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn acceptance_record_receipt_blocks_on_digest_or_denial_gap() {
        let report = hepta_work_graph_persistence_acceptance_record_receipt_preview_report();

        assert_eq!(report.digest_check_count, 7);
        assert!(
            report
                .digest_checks
                .iter()
                .all(|check| check.blocks_acceptance && check.compared_fields.len() >= 3)
        );
        assert_eq!(report.receipt_denial_count, 9);
        assert!(report.receipt_denials.iter().all(|denial| {
            denial.blocks_receipt_acceptance && denial.applies_to_receipt_ids.len() == 6
        }));
    }

    #[test]
    fn acceptance_record_receipt_keeps_readback_views_local() {
        let report = hepta_work_graph_persistence_acceptance_record_receipt_preview_report();
        let view_ids = report
            .readback_views
            .iter()
            .map(|view| view.id)
            .collect::<Vec<_>>();

        assert_eq!(
            view_ids,
            [
                "operator_acceptance_record_receipt_view",
                "auditor_acceptance_record_receipt_digest_view",
                "release_owner_acceptance_record_receipt_denial_view",
                "runtime_acceptance_record_receipt_zero_effect_view",
            ]
        );
        assert_eq!(report.readback_view_count, 4);
        assert!(
            report
                .readback_views
                .iter()
                .all(|view| !view.external_delivery_enabled
                    && view.required_fields.len() >= 11
                    && view.required_fields.contains(&"workflow_id")
                    && view.required_fields.contains(&"receipt_hash"))
        );
    }

    #[test]
    fn acceptance_record_receipt_requires_durable_identity_evidence() {
        let report = hepta_work_graph_persistence_acceptance_record_receipt_preview_report();

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
                .required_for_receipt_ids
                .len(),
            6
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn acceptance_record_receipt_requires_intake_gate() {
        let report = hepta_work_graph_persistence_acceptance_record_receipt_preview_report();

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
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_RECEIPT_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn acceptance_record_receipt_has_no_side_effects() {
        let report = hepta_work_graph_persistence_acceptance_record_receipt_preview_report();

        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceRecordReceiptPreviewSideEffects::none()
        );
        assert!(report.ready_for_acceptance_record_receipt_acknowledgement_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
