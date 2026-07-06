use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_receipt_count: usize,
    pub digest_check_count: usize,
    pub mismatch_denial_count: usize,
    pub receipt_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub readback_receipts:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreview>,
    pub digest_checks:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackDigestCheckPreview>,
    pub mismatch_denials: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackMismatchDenialPreview,
    >,
    pub receipt_guards:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptGuardPreview>,
    pub local_views:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreview {
    pub id: &'static str,
    pub source_retention_surface: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redaction_state: &'static str,
    pub persistence_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackDigestCheckPreview {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_receipt_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackMismatchDenialPreview {
    pub id: &'static str,
    pub applies_to_receipt_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
    pub blocks_persistence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptGuardPreview {
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackLocalViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackDurableIdentityEvidencePreview
{
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_readback_receipt_ids: Vec<&'static str>,
    pub durable_field_count: usize,
    pub preview_binding_count: usize,
    pub invariant_count: usize,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub retention_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub receipt_acknowledgement_recorded: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_report()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreviewReport {
    let readback_receipts =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_receipts();
    let digest_checks =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_digest_checks();
    let mismatch_denials =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_mismatch_denials(
        );
    let receipt_guards =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_receipt_guards();
    let local_views =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_durable_identity_evidence();
    let invariants =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_invariants();

    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_no_receipt_write",
        readback_receipt_count: readback_receipts.len(),
        digest_check_count: digest_checks.len(),
        mismatch_denial_count: mismatch_denials.len(),
        receipt_guard_count: receipt_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_required_prior_gates(),
        readback_receipts,
        digest_checks,
        mismatch_denials,
        receipt_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_required_prior_gates()
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
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_receipt_ids()
-> Vec<&'static str> {
    vec![
        "retention_policy_readback_receipt",
        "expiry_guard_readback_receipt",
        "supersession_guard_readback_receipt",
        "garbage_collection_denial_readback_receipt",
        "zero_effect_digest_readback_receipt",
        "release_external_denial_readback_receipt",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackDurableIdentityEvidencePreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_durable_identity_field_ids(),
        required_for_readback_receipt_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_receipt_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_receipts()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreview> {
    vec![
        readback_receipt(
            "retention_policy_readback_receipt",
            "effect_denial_receipt_retention_policies",
        ),
        readback_receipt("expiry_guard_readback_receipt", "retention_expiry_guards"),
        readback_receipt(
            "supersession_guard_readback_receipt",
            "retention_supersession_guards",
        ),
        readback_receipt(
            "garbage_collection_denial_readback_receipt",
            "retention_garbage_collection_denials",
        ),
        readback_receipt(
            "zero_effect_digest_readback_receipt",
            "retention_zero_effect_digests",
        ),
        readback_receipt(
            "release_external_denial_readback_receipt",
            "release_publication_external_delivery_denials",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_digest_checks()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackDigestCheckPreview> {
    vec![
        digest_check(
            "check_durable_identity_digest",
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_durable_identity_field_ids(),
        ),
        digest_check(
            "check_retention_policy_digest",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "retentionPolicyIds",
                "retentionWindowHash",
                "hashOnly",
            ]),
        ),
        digest_check(
            "check_expiry_guard_digest",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "expiryGuardIds",
                "expired",
                "blocksPersistence",
            ]),
        ),
        digest_check(
            "check_supersession_digest",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "supersessionGuardIds",
                "scopeEpochHash",
                "blocksMutation",
            ]),
        ),
        digest_check(
            "check_garbage_collection_denial_digest",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "garbageCollectionDenialIds",
                "gcAllowed",
                "tombstonePersisted",
            ]),
        ),
        digest_check(
            "check_zero_effect_digest",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "zeroWriteHash",
                "zeroTrafficHash",
                "zeroExternalSendHash",
            ]),
        ),
        digest_check(
            "check_prior_gate_digest",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "priorGateId",
                "priorGateDigest",
                "readbackReceiptHash",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_mismatch_denials()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackMismatchDenialPreview> {
    let receipt_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_receipt_ids();

    vec![
        mismatch_denial(
            "durable_identity_evidence_missing",
            receipt_ids.clone(),
            "retention readback receipt is missing durable identity evidence",
        ),
        mismatch_denial(
            "missing_retention_policy_digest",
            receipt_ids.clone(),
            "retention readback receipt is missing policy digest",
        ),
        mismatch_denial(
            "expired_receipt_replayed",
            receipt_ids.clone(),
            "expired retention receipt was replayed",
        ),
        mismatch_denial(
            "superseded_scope_replayed",
            receipt_ids.clone(),
            "superseded retention scope was replayed",
        ),
        mismatch_denial(
            "garbage_collection_tombstone_persistence_attempted",
            receipt_ids.clone(),
            "retention readback attempted to persist a GC tombstone",
        ),
        mismatch_denial(
            "zero_effect_digest_nonzero",
            receipt_ids.clone(),
            "retention readback does not prove zero side effects",
        ),
        mismatch_denial(
            "release_publication_attempted",
            receipt_ids.clone(),
            "retention readback cannot publish release status",
        ),
        mismatch_denial(
            "external_delivery_attempted",
            receipt_ids,
            "retention readback cannot send external delivery",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_receipt_guards()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptGuardPreview> {
    vec![
        receipt_guard(
            "hash_only_retention_receipt_required",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "retentionPolicyHash",
                "expiryGuardHash",
                "supersessionHash",
            ]),
        ),
        receipt_guard(
            "non_persistent_readback_required",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "persistenceEnabled",
                "receiptPersisted",
                "tombstonePersisted",
            ]),
        ),
        receipt_guard(
            "local_view_only_required",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "operatorViewHash",
                "auditorViewHash",
                "releaseOwnerViewHash",
            ]),
        ),
        receipt_guard(
            "bounded_retention_window_required",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "retentionWindow",
                "expiryState",
                "scopeEpoch",
            ]),
        ),
        receipt_guard(
            "next_gate_acknowledgement_required",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "recommendedNextGate",
                "acknowledgementAllowed",
                "acceptanceAllowed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_local_views()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackLocalViewPreview> {
    vec![
        local_view(
            "operator_retention_readback_receipt_view",
            "operator",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "readbackReceiptId",
                "retentionPolicyId",
                "expired",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_retention_readback_digest_view",
            "auditor",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "readbackReceiptHash",
                "retentionPolicyHash",
                "gcDenialHash",
                "zeroEffectHash",
            ]),
        ),
        local_view(
            "release_owner_retention_readback_denial_view",
            "release_owner",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "readbackReceiptId",
            ]),
        ),
        local_view(
            "runtime_retention_readback_zero_effect_view",
            "system",
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "retentionStatePersisted",
                "readbackReceiptPersisted",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_invariants()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackInvariantPreview> {
    vec![
        invariant(
            "retention_readback_receipts_require_durable_identity_evidence",
            "retention expiry readback receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "retention_readback_receipts_are_hash_only",
            "retention expiry readback receipts contain hash-only redacted evidence",
        ),
        invariant(
            "retention_readback_receipts_are_non_persistent",
            "retention expiry readback cannot write receipt, retention, expiry, or tombstone state",
        ),
        invariant(
            "retention_readback_receipts_block_acceptance",
            "retention expiry readback cannot become acceptance or approval recording",
        ),
        invariant(
            "retention_readback_receipts_block_gc_mutation",
            "retention expiry readback cannot perform garbage collection or persist tombstones",
        ),
        invariant(
            "retention_readback_receipt_views_are_local_only",
            "operator, auditor, release-owner, and runtime readback views cannot be sent externally",
        ),
        invariant(
            "retention_readback_receipt_preview_has_no_side_effects",
            "this gate cannot persist, grant authority, enable live execution, publish, or send externally",
        ),
    ]
}

fn readback_receipt(
    id: &'static str,
    source_retention_surface: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreview {
        id,
        source_retention_surface,
        required_fields:
            with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(vec![
                "readbackReceiptId",
                "sourceRetentionSurface",
                "retentionPolicyHash",
                "expiryGuardHash",
                "supersessionHash",
                "garbageCollectionDenialHash",
                "zeroEffectHash",
                "nextGate",
            ]),
        redaction_state: "hash_only_redacted",
        persistence_enabled: false,
        external_delivery_enabled: false,
    }
}

fn digest_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackDigestCheckPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackDigestCheckPreview {
        id,
        compared_fields,
        blocks_receipt_acceptance: true,
    }
}

fn mismatch_denial(
    id: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackMismatchDenialPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackMismatchDenialPreview {
        id,
        applies_to_receipt_ids,
        reason,
        blocks_acceptance: true,
        blocks_persistence: true,
    }
}

fn receipt_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptGuardPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptGuardPreview {
        id,
        required_fields,
        blocks_recording: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackLocalViewPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackInvariantPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackInvariantPreview {
        id,
        required: true,
        reason,
    }
}

fn with_acceptance_effect_denial_receipt_retention_readback_durable_identity_fields(
    mut fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut durable_fields =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_durable_identity_field_ids(
        );
    durable_fields.append(&mut fields);
    durable_fields
}

impl WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            retention_state_persisted: false,
            readback_receipt_persisted: false,
            receipt_acknowledgement_recorded: false,
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
    fn retention_readback_receipt_declares_hash_only_receipts() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.readback_receipt_count, 6);
        assert_eq!(
            report
                .readback_receipts
                .iter()
                .map(|receipt| receipt.id)
                .collect::<Vec<_>>(),
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_receipt_ids(
            )
        );
        assert!(report.readback_receipts.iter().all(|receipt| {
            receipt.redaction_state == "hash_only_redacted"
                && !receipt.persistence_enabled
                && !receipt.external_delivery_enabled
                && receipt.required_fields.contains(&"workflow_id")
                && receipt.required_fields.contains(&"receipt_hash")
                && receipt.required_fields.len() >= 15
        }));
    }

    #[test]
    fn retention_readback_receipt_checks_digests_and_mismatches() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.digest_check_count, 7);
        assert!(report.digest_checks.iter().all(|check| {
            check.blocks_receipt_acceptance
                && check.compared_fields.contains(&"workflow_id")
                && check.compared_fields.contains(&"receipt_hash")
                && check.compared_fields.len() >= 7
        }));
        assert_eq!(report.mismatch_denial_count, 8);
        assert!(report.mismatch_denials.iter().all(|denial| {
            denial.blocks_acceptance
                && denial.blocks_persistence
                && denial.applies_to_receipt_ids.len() == 6
        }));
        assert!(
            report
                .mismatch_denials
                .iter()
                .any(|denial| denial.id == "durable_identity_evidence_missing")
        );
    }

    #[test]
    fn retention_readback_receipt_requires_non_recording_guards() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.receipt_guard_count, 5);
        assert!(report.receipt_guards.iter().all(|guard| {
            guard.blocks_recording
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
                && guard.required_fields.len() >= 10
        }));
    }

    #[test]
    fn retention_readback_receipt_requires_retention_expiry_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview
        );
    }

    #[test]
    fn retention_readback_receipt_requires_durable_identity_evidence() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_report();

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
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_readback_receipt_ids,
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_receipt_ids(
            )
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn retention_readback_receipt_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert!(report.local_views.iter().all(|view| {
            !view.external_delivery_enabled
                && view.required_fields.contains(&"workflow_id")
                && view.required_fields.contains(&"receipt_hash")
                && view.required_fields.len() >= 11
        }));
        assert_eq!(report.invariant_count, 7);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
    }

    #[test]
    fn retention_readback_receipt_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_report();

        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackReceiptPreviewSideEffects::none()
        );
    }
}
