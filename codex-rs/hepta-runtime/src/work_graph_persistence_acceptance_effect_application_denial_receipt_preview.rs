use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_effect_application_denial_receipt_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectApplicationDenialReceiptPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub denial_receipt_count: usize,
    pub digest_check_count: usize,
    pub mismatch_denial_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub denial_receipts: Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptContractPreview>,
    pub digest_checks: Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptDigestCheckPreview>,
    pub mismatch_denials: Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptMismatchPreview>,
    pub local_views: Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_acknowledgement_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphPersistenceAcceptanceEffectApplicationDenialReceiptPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptContractPreview {
    pub id: &'static str,
    pub source_effect_surface_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redaction_state: &'static str,
    pub persistence_enabled: bool,
    pub effect_applied: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptDigestCheckPreview {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_receipt_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptMismatchPreview {
    pub id: &'static str,
    pub applies_to_receipt_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_receipt_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptLocalViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_receipt_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectApplicationDenialReceiptPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub denial_receipt_persisted: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_report()
-> WorkGraphPersistenceAcceptanceEffectApplicationDenialReceiptPreviewReport {
    let denial_receipts = work_graph_persistence_acceptance_effect_denial_receipts();
    let digest_checks = work_graph_persistence_acceptance_effect_denial_receipt_digest_checks();
    let mismatch_denials = work_graph_persistence_acceptance_effect_denial_receipt_mismatches();
    let local_views = work_graph_persistence_acceptance_effect_denial_receipt_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_denial_receipt_durable_identity_evidence();
    let invariants = work_graph_persistence_acceptance_effect_denial_receipt_invariants();

    WorkGraphPersistenceAcceptanceEffectApplicationDenialReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_preview_no_receipt_write",
        denial_receipt_count: denial_receipts.len(),
        digest_check_count: digest_checks.len(),
        mismatch_denial_count: mismatch_denials.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_denial_receipt_required_prior_gates(),
        denial_receipts,
        digest_checks,
        mismatch_denials,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectApplicationDenialReceiptPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_required_prior_gates()
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_ids() -> Vec<&'static str> {
    vec![
        "operator_acceptance_recording_denial_receipt",
        "approval_ledger_write_denial_receipt",
        "authority_grant_denial_receipt",
        "graph_state_persistence_denial_receipt",
        "wal_checkpoint_write_denial_receipt",
        "enforcement_rollout_denial_receipt",
        "release_publication_denial_receipt",
        "external_delivery_denial_receipt",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipts()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptContractPreview> {
    vec![
        denial_receipt(
            "operator_acceptance_recording_denial_receipt",
            "operator_acceptance_recording_effect",
        ),
        denial_receipt(
            "approval_ledger_write_denial_receipt",
            "approval_ledger_write_effect",
        ),
        denial_receipt("authority_grant_denial_receipt", "authority_grant_effect"),
        denial_receipt(
            "graph_state_persistence_denial_receipt",
            "graph_state_persistence_effect",
        ),
        denial_receipt(
            "wal_checkpoint_write_denial_receipt",
            "wal_checkpoint_write_effect",
        ),
        denial_receipt(
            "enforcement_rollout_denial_receipt",
            "enforcement_rollout_effect",
        ),
        denial_receipt(
            "release_publication_denial_receipt",
            "release_publication_effect",
        ),
        denial_receipt(
            "external_delivery_denial_receipt",
            "external_delivery_effect",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_digest_checks()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptDigestCheckPreview> {
    vec![
        digest_check(
            "check_durable_identity_digest",
            work_graph_persistence_acceptance_effect_denial_receipt_durable_identity_field_ids(),
        ),
        digest_check(
            "check_effect_surface_digest",
            vec!["effectSurfaceId", "requestedEffect", "requiredFieldIds"],
        ),
        digest_check(
            "check_blocker_digest",
            vec!["effectBlockerIds", "blockerReasonHash", "effectBlocked"],
        ),
        digest_check(
            "check_apply_guard_digest",
            vec!["applyGuardIds", "requiredDenialFieldsHash", "applyBlocked"],
        ),
        digest_check(
            "check_rollback_quarantine_digest",
            vec![
                "rollbackQuarantineIds",
                "rollbackOwnerRequired",
                "quarantineRequired",
            ],
        ),
        digest_check(
            "check_zero_side_effect_digest",
            vec![
                "sideEffectHash",
                "zeroWriteProofHash",
                "zeroTrafficProofHash",
            ],
        ),
        digest_check(
            "check_prior_gate_digest",
            vec!["priorGateId", "priorGateReportHash", "receiptSourceHash"],
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_mismatches()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptMismatchPreview> {
    let receipt_ids = work_graph_persistence_acceptance_effect_denial_receipt_ids();

    vec![
        mismatch_denial(
            "durable_identity_evidence_missing",
            receipt_ids.clone(),
            "denial receipt is missing durable identity evidence",
        ),
        mismatch_denial(
            "missing_effect_surface_digest",
            receipt_ids.clone(),
            "denial receipt is missing effect surface digest",
        ),
        mismatch_denial(
            "missing_effect_blocker_digest",
            receipt_ids.clone(),
            "denial receipt is missing blocker digest",
        ),
        mismatch_denial(
            "missing_apply_guard_digest",
            receipt_ids.clone(),
            "denial receipt is missing apply guard digest",
        ),
        mismatch_denial(
            "side_effect_digest_nonzero",
            receipt_ids.clone(),
            "denial receipt does not prove zero side effects",
        ),
        mismatch_denial(
            "rollback_quarantine_digest_missing",
            receipt_ids.clone(),
            "denial receipt is missing rollback or quarantine digest",
        ),
        mismatch_denial(
            "denial_receipt_persistence_attempted",
            receipt_ids.clone(),
            "denial receipt cannot be persisted in preview",
        ),
        mismatch_denial(
            "release_publication_attempted",
            receipt_ids.clone(),
            "denial receipt cannot publish release status",
        ),
        mismatch_denial(
            "external_delivery_attempted",
            receipt_ids,
            "denial receipt cannot be sent externally",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_local_views()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptLocalViewPreview> {
    vec![
        local_view(
            "operator_effect_denial_receipt_view",
            "operator",
            with_acceptance_effect_denial_receipt_durable_identity_fields(vec![
                "denialReceiptId",
                "effectSurfaceId",
                "effectBlockerIds",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_effect_denial_receipt_digest_view",
            "auditor",
            with_acceptance_effect_denial_receipt_durable_identity_fields(vec![
                "denialReceiptHash",
                "applyGuardDigestHash",
                "rollbackQuarantineHash",
                "sideEffectHash",
            ]),
        ),
        local_view(
            "release_owner_effect_denial_receipt_view",
            "release_owner",
            with_acceptance_effect_denial_receipt_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "denialReceiptId",
            ]),
        ),
        local_view(
            "runtime_effect_denial_receipt_zero_effect_view",
            "system",
            with_acceptance_effect_denial_receipt_durable_identity_fields(vec![
                "authorityGranted",
                "statePersisted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptDurableIdentityEvidencePreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_durable_identity_field_ids(),
        required_for_receipt_ids: work_graph_persistence_acceptance_effect_denial_receipt_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_invariants()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptInvariantPreview> {
    vec![
        invariant(
            "acceptance_effect_application_denial_receipts_require_durable_identity_evidence",
            "effect application denial receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "effect_denial_receipts_are_hash_only",
            "denial receipts contain hash-only redacted evidence",
        ),
        invariant(
            "effect_denial_receipts_are_non_persistent",
            "denial receipts cannot write graph state or receipt state",
        ),
        invariant(
            "effect_denial_receipts_prove_zero_side_effects",
            "denial receipts must prove zero writes, zero traffic, zero release, and zero external sends",
        ),
        invariant(
            "effect_denial_receipt_views_are_local_only",
            "operator, auditor, release-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "effect_denial_receipt_requires_application_blocker_gate",
            "denial receipt preview requires the effect application blocker gate",
        ),
        invariant(
            "effect_denial_receipt_preview_has_no_side_effects",
            "this gate cannot persist receipts, record approval, grant authority, enable live execution, publish, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceAcceptanceEffectApplicationDenialReceiptPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            denial_receipt_persisted: false,
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

fn denial_receipt(
    id: &'static str,
    source_effect_surface_id: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptContractPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptContractPreview {
        id,
        source_effect_surface_id,
        required_fields: with_acceptance_effect_denial_receipt_durable_identity_fields(vec![
            "denialReceiptId",
            "effectSurfaceId",
            "effectBlockerIds",
            "applyGuardIds",
            "rollbackQuarantineIds",
            "effectApplied",
            "sideEffectHash",
            "zeroEffectProofHash",
        ]),
        redaction_state: "hash_only_redacted",
        persistence_enabled: false,
        effect_applied: false,
        external_delivery_enabled: false,
    }
}

fn with_acceptance_effect_denial_receipt_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_acceptance_effect_denial_receipt_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn digest_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptDigestCheckPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptDigestCheckPreview {
        id,
        compared_fields,
        blocks_receipt_acceptance: true,
    }
}

fn mismatch_denial(
    id: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptMismatchPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptMismatchPreview {
        id,
        applies_to_receipt_ids,
        reason,
        blocks_receipt_acceptance: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptLocalViewPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptInvariantPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_denial_receipt_declares_hash_only_receipts() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_report(
            );
        let receipt_ids = report
            .denial_receipts
            .iter()
            .map(|receipt| receipt.id)
            .collect::<Vec<_>>();

        assert_eq!(
            receipt_ids,
            [
                "operator_acceptance_recording_denial_receipt",
                "approval_ledger_write_denial_receipt",
                "authority_grant_denial_receipt",
                "graph_state_persistence_denial_receipt",
                "wal_checkpoint_write_denial_receipt",
                "enforcement_rollout_denial_receipt",
                "release_publication_denial_receipt",
                "external_delivery_denial_receipt",
            ]
        );
        assert_eq!(report.denial_receipt_count, 8);
        assert!(report.denial_receipts.iter().all(|receipt| {
            receipt.redaction_state == "hash_only_redacted"
                && !receipt.persistence_enabled
                && !receipt.effect_applied
                && !receipt.external_delivery_enabled
                && receipt.required_fields.len() >= 15
                && receipt.required_fields.contains(&"workflow_id")
                && receipt.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn effect_denial_receipt_checks_digests_and_mismatches() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_report(
            );

        assert_eq!(report.digest_check_count, 7);
        assert!(
            report.digest_checks.iter().all(|check| {
                check.blocks_receipt_acceptance && check.compared_fields.len() >= 3
            })
        );
        assert_eq!(report.mismatch_denial_count, 9);
        assert!(report.mismatch_denials.iter().all(|denial| {
            denial.blocks_receipt_acceptance && denial.applies_to_receipt_ids.len() == 8
        }));
    }

    #[test]
    fn effect_denial_receipt_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_report(
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
    fn effect_denial_receipt_requires_durable_identity_evidence() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_report(
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
            8
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn effect_denial_receipt_requires_blocker_and_durable_identity_gates() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_report(
            );

        assert!(report.required_prior_gates.contains(
            &"hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_gate"
        ));
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn effect_denial_receipt_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_report(
            );

        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectApplicationDenialReceiptPreviewSideEffects::none()
        );
        assert!(
            report.ready_for_acceptance_effect_application_denial_receipt_acknowledgement_preview
        );
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
