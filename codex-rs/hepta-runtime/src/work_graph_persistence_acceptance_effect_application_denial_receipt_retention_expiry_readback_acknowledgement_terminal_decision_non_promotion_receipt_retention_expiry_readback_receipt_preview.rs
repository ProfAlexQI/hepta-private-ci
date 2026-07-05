use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewReport
{
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
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreview>,
    pub digest_checks:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDigestCheckPreview>,
    pub mismatch_denials:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackMismatchDenialPreview>,
    pub receipt_guards:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptGuardPreview>,
    pub local_views:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreview
{
    pub id: &'static str,
    pub source_retention_surface: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redaction_state: &'static str,
    pub persistence_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDigestCheckPreview
{
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_receipt_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackMismatchDenialPreview
{
    pub id: &'static str,
    pub applies_to_receipt_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
    pub blocks_persistence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptGuardPreview
{
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDurableIdentityEvidencePreview
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
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_persisted: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_persisted: bool,
    pub terminal_decision_receipt_acknowledgement_recorded: bool,
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
    pub public_claim_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewReport
{
    let readback_receipts =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipts();
    let digest_checks =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_digest_checks();
    let mismatch_denials =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_mismatch_denials();
    let receipt_guards =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_guards();
    let local_views =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_evidence();
    let invariants =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_invariants();

    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_no_receipt_write",
        readback_receipt_count: readback_receipts.len(),
        digest_check_count: digest_checks.len(),
        mismatch_denial_count: mismatch_denials.len(),
        receipt_guard_count: receipt_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_required_prior_gates(),
        readback_receipts,
        digest_checks,
        mismatch_denials,
        receipt_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = crate::work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_required_prior_gates();
    gates.retain(|gate| *gate != "hepta_work_graph_durable_identity_preview_gate");
    gates.push(
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate",
    );
    gates.push("hepta_work_graph_durable_identity_preview_gate");
    gates
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_ids()
-> Vec<&'static str> {
    vec![
        "terminal_receipt_retention_policy_readback_receipt",
        "terminal_receipt_expiry_guard_readback_receipt",
        "terminal_receipt_supersession_guard_readback_receipt",
        "terminal_receipt_gc_denial_readback_receipt",
        "terminal_receipt_zero_effect_digest_readback_receipt",
        "terminal_receipt_release_public_claim_denial_readback_receipt",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipts()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreview,
>{
    vec![
        readback_receipt(
            "terminal_receipt_retention_policy_readback_receipt",
            "terminal_receipt_retention_policies",
        ),
        readback_receipt(
            "terminal_receipt_expiry_guard_readback_receipt",
            "terminal_receipt_retention_expiry_guards",
        ),
        readback_receipt(
            "terminal_receipt_supersession_guard_readback_receipt",
            "terminal_receipt_retention_supersession_guards",
        ),
        readback_receipt(
            "terminal_receipt_gc_denial_readback_receipt",
            "terminal_receipt_garbage_collection_denials",
        ),
        readback_receipt(
            "terminal_receipt_zero_effect_digest_readback_receipt",
            "terminal_receipt_retention_zero_effect_digests",
        ),
        readback_receipt(
            "terminal_receipt_release_public_claim_denial_readback_receipt",
            "terminal_receipt_release_public_claim_external_delivery_denials",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_digest_checks()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDigestCheckPreview,
>{
    vec![
        digest_check(
            "check_durable_identity_digest",
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_field_ids(),
        ),
        digest_check(
            "check_terminal_receipt_retention_policy_digest",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "retentionPolicyIds",
                "retentionWindowHash",
                "hashOnly",
            ]),
        ),
        digest_check(
            "check_terminal_receipt_expiry_guard_digest",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "expiryGuardIds",
                "expired",
                "blocksPersistence",
            ]),
        ),
        digest_check(
            "check_terminal_receipt_supersession_digest",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "supersessionGuardIds",
                "scopeEpochHash",
                "blocksMutation",
            ]),
        ),
        digest_check(
            "check_terminal_receipt_gc_denial_digest",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "garbageCollectionDenialIds",
                "gcAllowed",
                "tombstonePersisted",
            ]),
        ),
        digest_check(
            "check_terminal_receipt_zero_effect_digest",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "zeroWriteHash",
                "zeroTrafficHash",
                "zeroExternalSendHash",
            ]),
        ),
        digest_check(
            "check_terminal_receipt_prior_gate_digest",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "priorGateId",
                "priorGateDigest",
                "readbackReceiptHash",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_mismatch_denials()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackMismatchDenialPreview,
>{
    let receipt_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_ids();

    vec![
        mismatch_denial(
            "durable_identity_evidence_missing",
            receipt_ids.clone(),
            "terminal receipt retention readback receipt is missing durable identity evidence",
        ),
        mismatch_denial(
            "missing_terminal_receipt_retention_policy_digest",
            receipt_ids.clone(),
            "terminal receipt retention readback is missing policy digest",
        ),
        mismatch_denial(
            "expired_terminal_receipt_replayed",
            receipt_ids.clone(),
            "expired terminal receipt acknowledgement evidence was replayed",
        ),
        mismatch_denial(
            "superseded_terminal_receipt_scope_replayed",
            receipt_ids.clone(),
            "superseded terminal receipt scope was replayed",
        ),
        mismatch_denial(
            "terminal_receipt_gc_tombstone_persistence_attempted",
            receipt_ids.clone(),
            "terminal receipt retention readback attempted to persist a GC tombstone",
        ),
        mismatch_denial(
            "terminal_receipt_zero_effect_digest_nonzero",
            receipt_ids.clone(),
            "terminal receipt retention readback does not prove zero side effects",
        ),
        mismatch_denial(
            "terminal_receipt_public_claim_attempted",
            receipt_ids.clone(),
            "terminal receipt retention readback cannot record public claims",
        ),
        mismatch_denial(
            "terminal_receipt_external_delivery_attempted",
            receipt_ids,
            "terminal receipt retention readback cannot send external delivery",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_guards()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptGuardPreview,
>{
    vec![
        receipt_guard(
            "hash_only_terminal_receipt_retention_receipt_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "retentionPolicyHash",
                "expiryGuardHash",
                "supersessionHash",
            ]),
        ),
        receipt_guard(
            "non_persistent_terminal_receipt_readback_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "persistenceEnabled",
                "receiptPersisted",
                "tombstonePersisted",
            ]),
        ),
        receipt_guard(
            "terminal_receipt_local_view_only_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "operatorViewHash",
                "auditorViewHash",
                "releaseOwnerViewHash",
            ]),
        ),
        receipt_guard(
            "terminal_receipt_bounded_retention_window_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "retentionWindow",
                "expiryState",
                "scopeEpoch",
            ]),
        ),
        receipt_guard(
            "terminal_receipt_next_gate_acknowledgement_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "recommendedNextGate",
                "acknowledgementAllowed",
                "acceptanceAllowed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_local_views()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackLocalViewPreview,
>{
    vec![
        local_view(
            "operator_terminal_receipt_retention_readback_receipt_view",
            "operator",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "readbackReceiptId",
                "retentionPolicyId",
                "expired",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_terminal_receipt_retention_readback_digest_view",
            "auditor",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "readbackReceiptHash",
                "retentionPolicyHash",
                "gcDenialHash",
                "zeroEffectHash",
            ]),
        ),
        local_view(
            "release_owner_terminal_receipt_retention_readback_denial_view",
            "release_owner",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "publicClaimDenied",
                "externalDeliveryDenied",
            ]),
        ),
        local_view(
            "runtime_terminal_receipt_retention_readback_zero_effect_view",
            "system",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(vec![
                "retentionStatePersisted",
                "readbackReceiptPersisted",
                "authorityGranted",
                "publicClaimRecorded",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDurableIdentityEvidencePreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_field_ids(),
        required_for_readback_receipt_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_invariants()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackInvariantPreview,
>{
    vec![
        invariant(
            "terminal_receipt_retention_readback_receipts_require_durable_identity_evidence",
            "terminal receipt retention readback receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "terminal_receipt_retention_readback_receipts_are_hash_only",
            "terminal receipt retention readback receipts contain hash-only redacted evidence",
        ),
        invariant(
            "terminal_receipt_retention_readback_receipts_are_non_persistent",
            "terminal receipt retention readback cannot write receipt, retention, expiry, or tombstone state",
        ),
        invariant(
            "terminal_receipt_retention_readback_receipts_block_acceptance",
            "terminal receipt retention readback cannot become acceptance or approval recording",
        ),
        invariant(
            "terminal_receipt_retention_readback_receipts_block_gc_mutation",
            "terminal receipt retention readback cannot perform garbage collection or persist tombstones",
        ),
        invariant(
            "terminal_receipt_retention_readback_views_are_local_only",
            "operator, auditor, release-owner, and runtime terminal receipt readback views cannot be sent externally",
        ),
        invariant(
            "terminal_receipt_retention_readback_preview_has_no_side_effects",
            "this gate cannot persist, grant authority, enable live execution, publish, record public claims, or send externally",
        ),
    ]
}

fn readback_receipt(
    id: &'static str,
    source_retention_surface: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreview {
        id,
        source_retention_surface,
        required_fields:
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(
                vec![
                    "readbackReceiptId",
                    "sourceRetentionSurface",
                    "retentionPolicyHash",
                    "expiryGuardHash",
                    "supersessionHash",
                    "garbageCollectionDenialHash",
                    "zeroEffectHash",
                    "nextGate",
                ],
            ),
        redaction_state: "hash_only_redacted",
        persistence_enabled: false,
        external_delivery_enabled: false,
    }
}

fn digest_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDigestCheckPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDigestCheckPreview {
        id,
        compared_fields,
        blocks_receipt_acceptance: true,
    }
}

fn mismatch_denial(
    id: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackMismatchDenialPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackMismatchDenialPreview {
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
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptGuardPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptGuardPreview {
        id,
        required_fields,
        blocks_recording: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackLocalViewPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackInvariantPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_persisted: false,
            terminal_decision_receipt_recorded: false,
            terminal_decision_receipt_persisted: false,
            terminal_decision_receipt_acknowledgement_recorded: false,
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
    fn terminal_receipt_retention_readback_declares_hash_only_receipts() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.readback_receipt_count, 6);
        assert_eq!(
            report
                .readback_receipts
                .iter()
                .map(|receipt| receipt.id)
                .collect::<Vec<_>>(),
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_ids()
        );
        assert!(report.readback_receipts.iter().all(|receipt| {
            receipt.redaction_state == "hash_only_redacted"
                && !receipt.persistence_enabled
                && !receipt.external_delivery_enabled
                && receipt.required_fields.len() >= 15
                && receipt.required_fields.contains(&"workflow_id")
                && receipt.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn terminal_receipt_retention_readback_checks_digests_and_mismatches() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.digest_check_count, 7);
        assert!(report.digest_checks.iter().all(|check| {
            check.blocks_receipt_acceptance
                && check.compared_fields.len() >= 7
                && check.compared_fields.contains(&"workflow_id")
                && check.compared_fields.contains(&"receipt_hash")
        }));
        assert_eq!(report.mismatch_denial_count, 8);
        assert!(
            report
                .mismatch_denials
                .iter()
                .any(|denial| denial.id == "durable_identity_evidence_missing")
        );
        assert!(report.mismatch_denials.iter().all(|denial| {
            denial.blocks_acceptance
                && denial.blocks_persistence
                && denial.applies_to_receipt_ids.len() == 6
        }));
    }

    #[test]
    fn terminal_receipt_retention_readback_requires_non_recording_guards() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.receipt_guard_count, 5);
        assert!(report.receipt_guards.iter().all(|guard| {
            guard.blocks_recording
                && guard.required_fields.len() >= 10
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn terminal_receipt_retention_readback_requires_retention_expiry_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_field_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview
        );
    }

    #[test]
    fn terminal_receipt_retention_readback_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert!(report.local_views.iter().all(|view| {
            !view.external_delivery_enabled
                && view.required_fields.len() >= 11
                && view.required_fields.contains(&"workflow_id")
                && view.required_fields.contains(&"receipt_hash")
        }));
        assert_eq!(report.invariant_count, 7);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(report.invariants.iter().any(|invariant| {
            invariant.id
                == "terminal_receipt_retention_readback_receipts_require_durable_identity_evidence"
        }));
    }

    #[test]
    fn terminal_receipt_retention_readback_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewSideEffects::none()
        );
    }
}
