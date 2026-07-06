use serde::Serialize;

use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptDigestCheckPreview;
use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptGuardPreview;
use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptInvariantPreview;
use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptLocalViewPreview;
use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptMismatchDenialPreview;
use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptPreview;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewReport
{
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub receipt_count: usize,
    pub digest_check_count: usize,
    pub mismatch_denial_count: usize,
    pub receipt_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub receipts: Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptPreview>,
    pub digest_checks:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptDigestCheckPreview>,
    pub mismatch_denials: Vec<
        WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptMismatchDenialPreview,
    >,
    pub receipt_guards:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptGuardPreview>,
    pub local_views:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptLocalViewPreview>,
    pub invariants:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptSideEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_persisted: bool,
    pub terminal_decision_receipt_retention_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub readback_acknowledgement_recorded: bool,
    pub readback_acknowledgement_replay_recorded: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report()
-> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewReport{
    let receipts =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipts();
    let digest_checks =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_digest_checks();
    let mismatch_denials =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_mismatch_denials();
    let receipt_guards =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_guards();
    let local_views =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_local_views();
    let invariants =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_invariants();

    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_decision_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_hash_only_no_recording",
        receipt_count: receipts.len(),
        digest_check_count: digest_checks.len(),
        mismatch_denial_count: mismatch_denials.len(),
        receipt_guard_count: receipt_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_required_prior_gates(),
        receipts,
        digest_checks,
        mismatch_denials,
        receipt_guards,
        local_views,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptSideEffects::none(),
    }
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        crate::work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_required_prior_gates();
    gates.push(
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate",
    );
    gates
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_ids()
-> Vec<&'static str> {
    vec![
        "operator_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt",
        "release_owner_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt",
        "authority_denial_terminal_decision_receipt_retention_readback_ack_receipt",
        "rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt",
        "release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt",
        "external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt",
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipts()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptPreview> {
    let surface_ids =
        crate::work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_surface_ids();
    vec![
        receipt(
            "operator_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt",
            surface_ids.clone(),
            "hash_only_operator_terminal_decision_receipt_retention_readback_ack_receipt",
        ),
        receipt(
            "release_owner_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt",
            surface_ids.clone(),
            "hash_only_release_owner_terminal_decision_receipt_retention_readback_ack_receipt",
        ),
        receipt(
            "authority_denial_terminal_decision_receipt_retention_readback_ack_receipt",
            surface_ids.clone(),
            "hash_only_authority_denial_terminal_decision_receipt_retention_readback_ack_receipt",
        ),
        receipt(
            "rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt",
            surface_ids.clone(),
            "hash_only_rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt",
        ),
        receipt(
            "release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt",
            surface_ids.clone(),
            "hash_only_release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt",
        ),
        receipt(
            "external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt",
            surface_ids,
            "hash_only_external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt",
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_digest_checks()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptDigestCheckPreview> {
    vec![
        digest_check(
            "terminal_decision_receipt_retention_readback_ack_decision_surface_digest_matches",
            vec![
                "terminalDecisionSurfaceId",
                "terminalDecisionHash",
                "sourceGateDigest",
            ],
        ),
        digest_check(
            "terminal_decision_receipt_retention_readback_ack_non_promotion_denial_digest_matches",
            vec!["nonPromotionDenialId", "denialHash", "zeroPromotionHash"],
        ),
        digest_check(
            "terminal_decision_receipt_retention_readback_ack_authority_guard_digest_matches",
            vec!["authorityGuardId", "authorityGuardHash", "authorityGranted"],
        ),
        digest_check(
            "terminal_decision_receipt_retention_readback_ack_release_delivery_guard_digest_matches",
            vec!["releaseDeliveryGuardId", "releaseHash", "deliveryHash"],
        ),
        digest_check(
            "terminal_decision_receipt_retention_readback_ack_local_view_digest_matches",
            vec!["localViewId", "localViewHash", "externalDeliveryEnabled"],
        ),
        digest_check(
            "terminal_decision_receipt_retention_readback_ack_zero_side_effect_digest_matches",
            vec!["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"],
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_mismatch_denials()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptMismatchDenialPreview> {
    let receipt_ids =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_ids();
    vec![
        mismatch_denial(
            "missing_terminal_decision_receipt_retention_readback_ack_surface_cannot_record_receipt",
            receipt_ids.clone(),
            "missing terminal decision receipt retention readback acknowledgement decision surface cannot record receipt",
        ),
        mismatch_denial(
            "mismatched_terminal_decision_receipt_retention_readback_ack_hash_cannot_accept",
            receipt_ids.clone(),
            "mismatched terminal decision receipt retention readback acknowledgement decision hash cannot become acceptance",
        ),
        mismatch_denial(
            "stale_terminal_decision_receipt_retention_readback_ack_replay_digest_cannot_grant_authority",
            receipt_ids.clone(),
            "stale terminal decision receipt retention readback acknowledgement replay digest cannot grant authority",
        ),
        mismatch_denial(
            "authority_guard_absence_after_terminal_decision_receipt_retention_readback_ack_cannot_start_rollout",
            receipt_ids.clone(),
            "absent authority guard after terminal decision receipt retention readback acknowledgement cannot start rollout",
        ),
        mismatch_denial(
            "release_delivery_guard_absence_after_terminal_decision_receipt_retention_readback_ack_cannot_publish",
            receipt_ids.clone(),
            "release and delivery guard absence after terminal decision receipt retention readback acknowledgement cannot publish release state",
        ),
        mismatch_denial(
            "external_delivery_terminal_decision_receipt_retention_readback_ack_receipt_cannot_send",
            receipt_ids.clone(),
            "external delivery receipt echo cannot send externally",
        ),
        mismatch_denial(
            "terminal_decision_receipt_retention_readback_ack_receipt_is_not_live_completion",
            receipt_ids,
            "terminal decision receipt retention readback acknowledgement receipt cannot claim live persistence completion",
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_guards()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptGuardPreview> {
    vec![
        receipt_guard(
            "terminal_decision_receipt_retention_readback_ack_receipt_is_hash_only",
            vec!["receiptId", "receiptHash", "redactionHash"],
        ),
        receipt_guard(
            "terminal_decision_receipt_retention_readback_ack_receipt_is_non_persistent",
            vec![
                "receiptPersisted",
                "receiptStorageScope",
                "receiptWriteHash",
            ],
        ),
        receipt_guard(
            "terminal_decision_receipt_retention_readback_ack_receipt_is_non_accepting",
            vec!["acceptanceAllowed", "approvalRecorded", "authorityGranted"],
        ),
        receipt_guard(
            "terminal_decision_receipt_retention_readback_ack_receipt_keeps_release_denied",
            vec![
                "releasePublished",
                "publicClaimRecorded",
                "artifactPublished",
            ],
        ),
        receipt_guard(
            "terminal_decision_receipt_retention_readback_ack_receipt_keeps_external_delivery_denied",
            vec![
                "externalDeliveryEnabled",
                "destinationPolicy",
                "externalSendPerformed",
            ],
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_local_views()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptLocalViewPreview> {
    vec![
        local_view(
            "operator_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt_view",
            "operator",
            vec!["receiptId", "receiptHash", "acceptanceAllowed", "nextGate"],
        ),
        local_view(
            "release_owner_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt_view",
            "release_owner",
            vec![
                "releasePublished",
                "publicClaimRecorded",
                "receiptPersisted",
                "externalDeliveryDenied",
            ],
        ),
        local_view(
            "auditor_terminal_decision_receipt_retention_readback_ack_receipt_digest_view",
            "auditor",
            vec![
                "sourceGateDigest",
                "receiptHash",
                "digestCheckId",
                "mismatchDenialId",
            ],
        ),
        local_view(
            "runtime_terminal_decision_receipt_retention_readback_ack_receipt_zero_effect_view",
            "system",
            vec![
                "terminalDecisionReceiptRecorded",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ],
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_invariants()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptInvariantPreview> {
    vec![
        invariant(
            "terminal_decision_receipt_retention_readback_ack_receipts_are_hash_only",
            "terminal decision receipt retention readback acknowledgement receipts expose hashes only",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_receipts_are_not_recorded",
            "terminal decision receipt retention readback acknowledgement cannot record receipt state",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_receipts_are_not_acceptance",
            "terminal decision receipt visibility cannot become acceptance or authority",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_receipts_keep_release_denied",
            "release publication, public claim, rollout, and traffic routing remain denied",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_receipt_views_are_local_only",
            "operator, release-owner, auditor, and runtime receipt views cannot be sent externally",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_receipt_preview_has_no_side_effects",
            "this gate cannot persist receipts, record approval, grant authority, publish, or send externally",
        ),
    ]
}

fn receipt(
    id: &'static str,
    source_terminal_decision_surface_ids: Vec<&'static str>,
    receipt_hash_mode: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptPreview {
        id,
        source_terminal_decision_surface_ids,
        receipt_hash_mode,
        required_fields: vec![
            "workflow_id",
            "run_id",
            "step_id",
            "checkpoint",
            "replay_key",
            "rollback_anchor",
            "receipt_hash",
            "receiptId",
            "sourceTerminalDecisionSurfaceIds",
            "receiptHashMode",
            "receiptHash",
            "redactionHash",
        ],
        persisted: false,
        receipt_recording_allowed: false,
        acceptance_allowed: false,
        external_delivery_enabled: false,
    }
}

fn digest_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptDigestCheckPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptDigestCheckPreview {
        id,
        compared_fields,
        blocks_receipt_recording: true,
    }
}

fn mismatch_denial(
    id: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptMismatchDenialPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptMismatchDenialPreview {
        id,
        applies_to_receipt_ids,
        reason,
        blocks_receipt_recording: true,
        blocks_acceptance: true,
        blocks_authority: true,
        blocks_rollout: true,
        blocks_release_publication: true,
        blocks_public_claim: true,
        blocks_external_delivery: true,
    }
}

fn receipt_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptGuardPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptGuardPreview {
        id,
        required_fields,
        receipt_recording_allowed: false,
        promotion_allowed: false,
        public_claim_allowed: false,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptLocalViewPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptInvariantPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptSideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
            terminal_decision_receipt_persisted: false,
            terminal_decision_receipt_retention_state_persisted: false,
            readback_receipt_persisted: false,
            readback_acknowledgement_recorded: false,
            readback_acknowledgement_replay_recorded: false,
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
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_requires_terminal_decision_gate()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(
                "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate"
            )
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview);
    }

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipts_are_hash_only()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(report.receipt_count, 6);
        assert!(report.receipts.iter().all(|receipt| {
            receipt.receipt_hash_mode.starts_with("hash_only_")
                && !receipt.persisted
                && !receipt.receipt_recording_allowed
                && !receipt.acceptance_allowed
                && !receipt.external_delivery_enabled
                && receipt.source_terminal_decision_surface_ids.len() == 6
        }));
    }

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipts_check_digests()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(report.digest_check_count, 6);
        assert!(
            report
                .digest_checks
                .iter()
                .all(|check| check.blocks_receipt_recording && check.compared_fields.len() >= 3)
        );
    }

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipts_deny_mismatches()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(report.mismatch_denial_count, 7);
        assert!(report.mismatch_denials.iter().all(|denial| {
            denial.blocks_receipt_recording
                && denial.blocks_acceptance
                && denial.blocks_authority
                && denial.blocks_rollout
                && denial.blocks_release_publication
                && denial.blocks_public_claim
                && denial.blocks_external_delivery
                && denial.applies_to_receipt_ids.len() == 6
        }));
    }

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipts_guard_recording_and_views()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(report.receipt_guard_count, 5);
        assert!(report.receipt_guards.iter().all(|guard| {
            !guard.receipt_recording_allowed
                && !guard.promotion_allowed
                && !guard.public_claim_allowed
                && guard.required_fields.len() >= 3
        }));
        assert_eq!(report.local_view_count, 4);
        assert!(
            report
                .local_views
                .iter()
                .all(|view| !view.external_delivery_enabled && view.required_fields.len() >= 4)
        );
    }

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_has_no_side_effects()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(report.invariant_count, 6);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptSideEffects::none()
        );
    }
}
