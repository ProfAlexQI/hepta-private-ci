use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_SCHEMA_VERSION: &str =
    "work_graph_persistence_operator_readiness_receipt_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub receipt_contract_count: usize,
    pub digest_check_count: usize,
    pub signature_denial_count: usize,
    pub acceptance_denial_count: usize,
    pub readback_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub receipt_contracts: Vec<WorkGraphPersistenceOperatorReadinessReceiptContractPreview>,
    pub digest_checks: Vec<WorkGraphPersistenceOperatorReadinessReceiptDigestCheckPreview>,
    pub signature_denials: Vec<WorkGraphPersistenceOperatorReadinessSignatureDenialPreview>,
    pub acceptance_denials: Vec<WorkGraphPersistenceOperatorReadinessAcceptanceDenialPreview>,
    pub readback_views: Vec<WorkGraphPersistenceOperatorReadinessReceiptReadbackViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceOperatorReadinessReceiptDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceOperatorReadinessReceiptInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_operator_readiness_receipt_acknowledgement_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceOperatorReadinessReceiptPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptContractPreview {
    pub id: &'static str,
    pub source_packet_template_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redaction_state: &'static str,
    pub persistence_enabled: bool,
    pub approval_recording_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptDigestCheckPreview {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessSignatureDenialPreview {
    pub id: &'static str,
    pub applies_to_receipt_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub blocks_receipt_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessAcceptanceDenialPreview {
    pub id: &'static str,
    pub applies_to_receipt_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptReadbackViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_receipt_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub readiness_receipt_persisted: bool,
    pub operator_acceptance_recorded: bool,
    pub approval_recorded: bool,
    pub live_readback_executed: bool,
    pub enforcement_enabled: bool,
    pub rollout_started: bool,
    pub traffic_routed: bool,
    pub release_published: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_operator_readiness_receipt_preview_report()
-> WorkGraphPersistenceOperatorReadinessReceiptPreviewReport {
    let receipt_contracts = work_graph_persistence_operator_readiness_receipt_contracts();
    let digest_checks = work_graph_persistence_operator_readiness_receipt_digest_checks();
    let signature_denials = work_graph_persistence_operator_readiness_signature_denials();
    let acceptance_denials = work_graph_persistence_operator_readiness_acceptance_denials();
    let readback_views = work_graph_persistence_operator_readiness_receipt_readback_views();
    let durable_identity_evidence =
        work_graph_persistence_operator_readiness_receipt_durable_identity_evidence();
    let invariants = work_graph_persistence_operator_readiness_receipt_invariants();

    WorkGraphPersistenceOperatorReadinessReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_operator_readiness_receipt_preview_no_receipt_write",
        receipt_contract_count: receipt_contracts.len(),
        digest_check_count: digest_checks.len(),
        signature_denial_count: signature_denials.len(),
        acceptance_denial_count: acceptance_denials.len(),
        readback_view_count: readback_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_operator_readiness_receipt_required_prior_gates(),
        receipt_contracts,
        digest_checks,
        signature_denials,
        acceptance_denials,
        readback_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_RECOMMENDED_NEXT_GATE,
        ready_for_operator_readiness_receipt_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistenceOperatorReadinessReceiptPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_operator_readiness_receipt_required_prior_gates() -> Vec<&'static str>
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_durable_identity_field_ids()
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

pub fn work_graph_persistence_operator_readiness_receipt_ids() -> Vec<&'static str> {
    vec![
        "store_persistence_readiness_receipt",
        "wal_checkpoint_readiness_receipt",
        "readback_receipt_readiness_receipt",
        "replay_execution_readiness_receipt",
        "external_publication_readiness_receipt",
        "full_rollout_abort_readiness_receipt",
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_contracts()
-> Vec<WorkGraphPersistenceOperatorReadinessReceiptContractPreview> {
    vec![
        receipt_contract(
            "store_persistence_readiness_receipt",
            "store_persistence_readiness_packet",
        ),
        receipt_contract(
            "wal_checkpoint_readiness_receipt",
            "wal_checkpoint_readiness_packet",
        ),
        receipt_contract(
            "readback_receipt_readiness_receipt",
            "readback_receipt_readiness_packet",
        ),
        receipt_contract(
            "replay_execution_readiness_receipt",
            "replay_execution_readiness_packet",
        ),
        receipt_contract(
            "external_publication_readiness_receipt",
            "external_publication_readiness_packet",
        ),
        receipt_contract(
            "full_rollout_abort_readiness_receipt",
            "full_rollout_abort_readiness_packet",
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_digest_checks()
-> Vec<WorkGraphPersistenceOperatorReadinessReceiptDigestCheckPreview> {
    vec![
        digest_check(
            "check_durable_identity_digest",
            work_graph_persistence_operator_readiness_receipt_durable_identity_field_ids(),
        ),
        digest_check(
            "check_packet_template_digest",
            vec!["packetTemplateId", "packetDigestHash", "requiredSectionIds"],
        ),
        digest_check(
            "check_section_completion_digest",
            vec![
                "readinessSectionsHash",
                "validationDenialIds",
                "completeSectionIds",
            ],
        ),
        digest_check(
            "check_operator_scope_digest",
            vec!["operatorScopeHash", "operatorIdHash", "signatureState"],
        ),
        digest_check(
            "check_expiry_revocation_digest",
            vec!["expiresAtUnixMs", "revocationStatus", "supersessionId"],
        ),
        digest_check(
            "check_release_publication_denial_digest",
            vec![
                "releaseDenialIds",
                "publicationDenied",
                "externalDeliveryDenied",
            ],
        ),
        digest_check(
            "check_side_effect_denial_digest",
            vec![
                "approvalRecordingDenied",
                "receiptPersistenceDenied",
                "sideEffectHash",
            ],
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_signature_denials()
-> Vec<WorkGraphPersistenceOperatorReadinessSignatureDenialPreview> {
    let receipt_ids = work_graph_persistence_operator_readiness_receipt_ids();

    vec![
        signature_denial(
            "durable_identity_evidence_missing",
            receipt_ids.clone(),
            "receipt does not include durable identity evidence",
        ),
        signature_denial(
            "missing_signature_hash",
            receipt_ids.clone(),
            "receipt does not include operator signature hash",
        ),
        signature_denial(
            "invalid_operator_scope_signature",
            receipt_ids.clone(),
            "operator signature does not match the required scope hash",
        ),
        signature_denial(
            "packet_expired",
            receipt_ids.clone(),
            "readiness packet expires before receipt readback",
        ),
        signature_denial(
            "packet_superseded",
            receipt_ids.clone(),
            "newer readiness packet supersedes this receipt",
        ),
        signature_denial(
            "operator_scope_revoked",
            receipt_ids.clone(),
            "operator authority scope was revoked",
        ),
        signature_denial(
            "rollback_owner_revoked",
            receipt_ids,
            "rollback or quarantine owner was revoked",
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_acceptance_denials()
-> Vec<WorkGraphPersistenceOperatorReadinessAcceptanceDenialPreview> {
    let receipt_ids = work_graph_persistence_operator_readiness_receipt_ids();

    vec![
        acceptance_denial(
            "durable_identity_evidence_missing",
            receipt_ids.clone(),
            "preview receipt attempted acceptance without durable identity evidence",
        ),
        acceptance_denial(
            "approval_recording_attempted",
            receipt_ids.clone(),
            "preview receipt attempted to record approval",
        ),
        acceptance_denial(
            "release_publication_attempted",
            receipt_ids.clone(),
            "preview receipt attempted release or publication",
        ),
        acceptance_denial(
            "external_delivery_attempted",
            receipt_ids.clone(),
            "preview receipt attempted external delivery",
        ),
        acceptance_denial(
            "receipt_persistence_attempted",
            receipt_ids.clone(),
            "preview receipt attempted durable receipt persistence",
        ),
        acceptance_denial(
            "enforcement_rollout_attempted",
            receipt_ids.clone(),
            "preview receipt attempted enforcement rollout",
        ),
        acceptance_denial(
            "live_readback_attempted",
            receipt_ids.clone(),
            "preview receipt attempted live readback",
        ),
        acceptance_denial(
            "readiness_receipt_not_hash_only",
            receipt_ids,
            "readiness receipt contains payload material instead of hashes",
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_readback_views()
-> Vec<WorkGraphPersistenceOperatorReadinessReceiptReadbackViewPreview> {
    vec![
        readback_view(
            "operator_readiness_receipt_summary_view",
            "operator",
            with_operator_readiness_receipt_durable_identity_fields(vec![
                "receiptId",
                "packetTemplateId",
                "signatureState",
                "acceptanceDenied",
            ]),
        ),
        readback_view(
            "auditor_readiness_receipt_digest_view",
            "auditor",
            with_operator_readiness_receipt_durable_identity_fields(vec![
                "packetDigestHash",
                "readinessSectionsHash",
                "sideEffectHash",
                "redactionState",
            ]),
        ),
        readback_view(
            "rollback_owner_revocation_view",
            "rollback_owner",
            with_operator_readiness_receipt_durable_identity_fields(vec![
                "rollbackOwnerId",
                "revocationStatus",
                "quarantineScope",
                "killSwitchId",
            ]),
        ),
        readback_view(
            "release_publication_denial_view",
            "release_owner",
            with_operator_readiness_receipt_durable_identity_fields(vec![
                "releaseDenialIds",
                "publicationDenied",
                "externalDeliveryDenied",
                "nextGate",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_durable_identity_evidence()
-> WorkGraphPersistenceOperatorReadinessReceiptDurableIdentityEvidencePreview {
    WorkGraphPersistenceOperatorReadinessReceiptDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_operator_readiness_receipt_durable_identity_field_ids(),
        required_for_receipt_ids: work_graph_persistence_operator_readiness_receipt_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_operator_readiness_receipt_invariants()
-> Vec<WorkGraphPersistenceOperatorReadinessReceiptInvariantPreview> {
    vec![
        invariant(
            "operator_readiness_receipts_require_durable_identity_evidence",
            "readiness receipt contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "readiness_receipts_are_hash_only",
            "readiness receipts expose packet hashes, receipt hashes, denial ids, and redaction state only",
        ),
        invariant(
            "signature_denials_block_acceptance",
            "missing, invalid, expired, superseded, or revoked signature state blocks acceptance",
        ),
        invariant(
            "receipt_readback_is_non_persistent",
            "receipt readback views are local preview shapes and cannot persist receipt state",
        ),
        invariant(
            "approval_recording_is_denied",
            "readiness receipt preview cannot record operator acceptance or approval",
        ),
        invariant(
            "release_publication_and_external_delivery_are_denied",
            "readiness receipt preview cannot release, publish, or send externally",
        ),
        invariant(
            "operator_readiness_receipt_preview_has_no_side_effects",
            "this gate cannot persist receipts, record approvals, execute readback, enable enforcement, route traffic, publish releases, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceOperatorReadinessReceiptPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            readiness_receipt_persisted: false,
            operator_acceptance_recorded: false,
            approval_recorded: false,
            live_readback_executed: false,
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
    source_packet_template_id: &'static str,
) -> WorkGraphPersistenceOperatorReadinessReceiptContractPreview {
    WorkGraphPersistenceOperatorReadinessReceiptContractPreview {
        id,
        source_packet_template_id,
        required_fields: with_operator_readiness_receipt_durable_identity_fields(vec![
            "receiptId",
            "packetTemplateId",
            "packetDigestHash",
            "operatorScopeHash",
            "readinessSectionsHash",
            "signatureState",
            "expiryState",
            "releasePublicationDenied",
            "approvalRecordingDenied",
            "externalDeliveryDenied",
        ]),
        redaction_state: "redacted_hash_only",
        persistence_enabled: false,
        approval_recording_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_operator_readiness_receipt_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged = work_graph_persistence_operator_readiness_receipt_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn digest_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphPersistenceOperatorReadinessReceiptDigestCheckPreview {
    WorkGraphPersistenceOperatorReadinessReceiptDigestCheckPreview {
        id,
        compared_fields,
        blocks_acceptance: true,
    }
}

fn signature_denial(
    id: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphPersistenceOperatorReadinessSignatureDenialPreview {
    WorkGraphPersistenceOperatorReadinessSignatureDenialPreview {
        id,
        applies_to_receipt_ids,
        trigger,
        blocks_receipt_acceptance: true,
    }
}

fn acceptance_denial(
    id: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceOperatorReadinessAcceptanceDenialPreview {
    WorkGraphPersistenceOperatorReadinessAcceptanceDenialPreview {
        id,
        applies_to_receipt_ids,
        reason,
        blocks_promotion: true,
    }
}

fn readback_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceOperatorReadinessReceiptReadbackViewPreview {
    WorkGraphPersistenceOperatorReadinessReceiptReadbackViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceOperatorReadinessReceiptInvariantPreview {
    WorkGraphPersistenceOperatorReadinessReceiptInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_readiness_receipt_preview_declares_hash_only_receipts() {
        let report = hepta_work_graph_persistence_operator_readiness_receipt_preview_report();
        let receipt_ids = report
            .receipt_contracts
            .iter()
            .map(|receipt| receipt.id)
            .collect::<Vec<_>>();

        assert_eq!(
            receipt_ids,
            [
                "store_persistence_readiness_receipt",
                "wal_checkpoint_readiness_receipt",
                "readback_receipt_readiness_receipt",
                "replay_execution_readiness_receipt",
                "external_publication_readiness_receipt",
                "full_rollout_abort_readiness_receipt",
            ]
        );
        assert_eq!(report.receipt_contract_count, 6);
        assert!(report.receipt_contracts.iter().all(|receipt| {
            receipt.redaction_state == "redacted_hash_only"
                && !receipt.persistence_enabled
                && !receipt.approval_recording_enabled
                && !receipt.external_delivery_enabled
                && receipt.required_fields.len() >= 17
                && receipt.required_fields.contains(&"workflow_id")
                && receipt.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn operator_readiness_receipt_preview_blocks_on_digest_or_signature_gap() {
        let report = hepta_work_graph_persistence_operator_readiness_receipt_preview_report();
        let signature_denial_ids = report
            .signature_denials
            .iter()
            .map(|denial| denial.id)
            .collect::<Vec<_>>();

        assert_eq!(report.digest_check_count, 7);
        let durable_digest = report
            .digest_checks
            .iter()
            .find(|check| check.id == "check_durable_identity_digest")
            .expect("durable identity digest check present");
        assert_eq!(
            durable_digest.compared_fields,
            work_graph_persistence_operator_readiness_receipt_durable_identity_field_ids()
        );
        assert!(
            report
                .digest_checks
                .iter()
                .all(|check| check.blocks_acceptance && check.compared_fields.len() >= 3)
        );
        assert_eq!(
            signature_denial_ids,
            [
                "durable_identity_evidence_missing",
                "missing_signature_hash",
                "invalid_operator_scope_signature",
                "packet_expired",
                "packet_superseded",
                "operator_scope_revoked",
                "rollback_owner_revoked",
            ]
        );
        assert_eq!(report.signature_denial_count, 7);
        assert!(report.signature_denials.iter().all(|denial| {
            denial.blocks_receipt_acceptance && denial.applies_to_receipt_ids.len() == 6
        }));
    }

    #[test]
    fn operator_readiness_receipt_preview_denies_acceptance_side_effects() {
        let report = hepta_work_graph_persistence_operator_readiness_receipt_preview_report();
        let acceptance_denial_ids = report
            .acceptance_denials
            .iter()
            .map(|denial| denial.id)
            .collect::<Vec<_>>();

        assert_eq!(
            acceptance_denial_ids,
            [
                "durable_identity_evidence_missing",
                "approval_recording_attempted",
                "release_publication_attempted",
                "external_delivery_attempted",
                "receipt_persistence_attempted",
                "enforcement_rollout_attempted",
                "live_readback_attempted",
                "readiness_receipt_not_hash_only",
            ]
        );
        assert_eq!(report.acceptance_denial_count, 8);
        assert!(
            report
                .acceptance_denials
                .iter()
                .all(|denial| denial.blocks_promotion)
        );
    }

    #[test]
    fn operator_readiness_receipt_preview_keeps_readback_views_local() {
        let report = hepta_work_graph_persistence_operator_readiness_receipt_preview_report();
        let view_ids = report
            .readback_views
            .iter()
            .map(|view| view.id)
            .collect::<Vec<_>>();

        assert_eq!(
            view_ids,
            [
                "operator_readiness_receipt_summary_view",
                "auditor_readiness_receipt_digest_view",
                "rollback_owner_revocation_view",
                "release_publication_denial_view",
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
    fn operator_readiness_receipt_preview_requires_packet_gate() {
        let report = hepta_work_graph_persistence_operator_readiness_receipt_preview_report();

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
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
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
            work_graph_persistence_operator_readiness_receipt_durable_identity_field_ids()
        );
        assert_eq!(
            report.durable_identity_evidence.required_for_receipt_ids,
            work_graph_persistence_operator_readiness_receipt_ids()
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn operator_readiness_receipt_preview_has_no_side_effects() {
        let report = hepta_work_graph_persistence_operator_readiness_receipt_preview_report();

        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceOperatorReadinessReceiptPreviewSideEffects::none()
        );
        assert!(report.ready_for_operator_readiness_receipt_acknowledgement_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
