use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: String,
    pub schema_version: String,
    pub preview_mode: &'static str,
    pub receipt_count: usize,
    pub digest_check_count: usize,
    pub mismatch_denial_count: usize,
    pub receipt_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<String>,
    pub receipts: Vec<DeepTd8RetentionAckTerminalDecisionReceipt>,
    pub digest_checks: Vec<DeepTd8RetentionAckTerminalDecisionReceiptDigestCheck>,
    pub mismatch_denials: Vec<DeepTd8RetentionAckTerminalDecisionReceiptMismatchDenial>,
    pub receipt_guards: Vec<DeepTd8RetentionAckTerminalDecisionReceiptGuard>,
    pub local_views: Vec<DeepTd8RetentionAckTerminalDecisionReceiptLocalView>,
    pub invariants: Vec<DeepTd8RetentionAckTerminalDecisionReceiptInvariant>,
    pub recommended_next_gate: String,
    pub ready_for_terminal_decision_receipt_acknowledgement_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8RetentionAckTerminalDecisionReceiptSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceipt {
    pub id: &'static str,
    pub source_surface_ids: Vec<&'static str>,
    pub receipt_visibility: &'static str,
    pub receipt_recording_allowed: bool,
    pub acceptance_allowed: bool,
    pub authority_grant_allowed: bool,
    pub public_claim_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptDigestCheck {
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub digest_must_match: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptMismatchDenial {
    pub id: &'static str,
    pub applies_to_receipt_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_receipt_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_rollout: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptGuard {
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub recording_allowed: bool,
    pub persistence_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptLocalView {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptInvariant {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_persisted: bool,
    pub acknowledgement_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub approval_recorded: bool,
    pub authority_granted: bool,
    pub live_persistence_enabled: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub rollout_started: bool,
    pub release_published: bool,
    pub public_claim_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_gate()
-> String {
    format!(
        "{}_receipt_acknowledgement_preview_gate",
        crate::deep_td8_retention_ack_terminal_decision_base()
    )
}

pub fn hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_preview_report()
-> DeepTd8RetentionAckTerminalDecisionReceiptPreviewReport {
    let receipts = deep_td8_receipt_retention_readback_ack_terminal_decision_receipts();
    let digest_checks =
        deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_digest_checks();
    let mismatch_denials =
        deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_mismatch_denials();
    let receipt_guards = deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_guards();
    let local_views =
        deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_local_views();
    let invariants = deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_invariants();
    let gate = crate::deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_gate();

    DeepTd8RetentionAckTerminalDecisionReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_preview_hash_only_no_recording",
        receipt_count: receipts.len(),
        digest_check_count: digest_checks.len(),
        mismatch_denial_count: mismatch_denials.len(),
        receipt_guard_count: receipt_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_required_prior_gates(),
        receipts,
        digest_checks,
        mismatch_denials,
        receipt_guards,
        local_views,
        invariants,
        recommended_next_gate:
            deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_gate(),
        ready_for_terminal_decision_receipt_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8RetentionAckTerminalDecisionReceiptSideEffects::none(),
    }
}

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_required_prior_gates()
-> Vec<String> {
    let mut gates =
        crate::deep_td8_receipt_retention_readback_ack_terminal_decision_required_prior_gates();
    gates.push(crate::deep_td8_receipt_retention_readback_ack_terminal_decision_gate());
    gates
}

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ids() -> Vec<&'static str>
{
    vec![
        "operator_deep_td8_receipt_retention_readback_ack_terminal_decision_non_promotion_receipt",
        "release_owner_deep_td8_receipt_retention_readback_ack_terminal_decision_non_promotion_receipt",
        "authority_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
        "rollout_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
        "release_publication_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
        "external_delivery_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
    ]
}

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipts()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceipt> {
    let surface_ids =
        crate::deep_td8_receipt_retention_readback_ack_terminal_decision_surface_ids();
    vec![
        receipt(
            "operator_deep_td8_receipt_retention_readback_ack_terminal_decision_non_promotion_receipt",
            surface_ids.clone(),
            "hash_only_operator_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
        ),
        receipt(
            "release_owner_deep_td8_receipt_retention_readback_ack_terminal_decision_non_promotion_receipt",
            surface_ids.clone(),
            "hash_only_release_owner_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
        ),
        receipt(
            "authority_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
            surface_ids.clone(),
            "hash_only_authority_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
        ),
        receipt(
            "rollout_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
            surface_ids.clone(),
            "hash_only_rollout_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
        ),
        receipt(
            "release_publication_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
            surface_ids.clone(),
            "hash_only_release_publication_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
        ),
        receipt(
            "external_delivery_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
            surface_ids,
            "hash_only_external_delivery_denial_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt",
        ),
    ]
}

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_digest_checks()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptDigestCheck> {
    vec![
        digest_check(
            "deep_td8_receipt_retention_readback_ack_terminal_decision_surface_digest_matches",
            vec![
                "terminalDecisionSurfaceId",
                "terminalDecisionHash",
                "sourceGateDigest",
            ],
        ),
        digest_check(
            "deep_td8_non_promotion_denial_digest_matches",
            vec!["nonPromotionDenialId", "denialHash", "zeroPromotionHash"],
        ),
        digest_check(
            "deep_td8_authority_guard_digest_matches",
            vec!["authorityGuardId", "authorityGuardHash", "authorityGranted"],
        ),
        digest_check(
            "deep_td8_release_delivery_guard_digest_matches",
            vec!["releaseDeliveryGuardId", "releaseHash", "deliveryHash"],
        ),
        digest_check(
            "deep_td8_local_view_digest_matches",
            vec!["localViewId", "localViewHash", "externalDeliveryEnabled"],
        ),
        digest_check(
            "deep_td8_zero_side_effect_digest_matches",
            vec!["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"],
        ),
    ]
}

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_mismatch_denials()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptMismatchDenial> {
    let receipt_ids = deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ids();
    vec![
        mismatch_denial(
            "missing_deep_td8_receipt_retention_readback_ack_terminal_decision_surface_cannot_record_receipt",
            receipt_ids.clone(),
            "missing terminal decision surface cannot record receipt",
        ),
        mismatch_denial(
            "mismatched_deep_td8_receipt_retention_readback_ack_terminal_decision_hash_cannot_accept",
            receipt_ids.clone(),
            "mismatched terminal decision hash cannot become acceptance",
        ),
        mismatch_denial(
            "stale_deep_td8_replay_digest_cannot_grant_authority",
            receipt_ids.clone(),
            "stale replay digest cannot grant authority",
        ),
        mismatch_denial(
            "deep_td8_authority_guard_absence_cannot_start_rollout",
            receipt_ids.clone(),
            "absent authority guard cannot start rollout",
        ),
        mismatch_denial(
            "deep_td8_release_delivery_guard_absence_cannot_publish",
            receipt_ids.clone(),
            "release and delivery guard absence cannot publish release state",
        ),
        mismatch_denial(
            "deep_td8_external_delivery_receipt_cannot_send",
            receipt_ids.clone(),
            "external delivery receipt echo cannot send externally",
        ),
        mismatch_denial(
            "deep_td8_receipt_is_not_live_completion",
            receipt_ids,
            "terminal decision receipt cannot claim live persistence completion",
        ),
    ]
}

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_guards()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptGuard> {
    vec![
        receipt_guard(
            "deep_td8_receipt_source_gate_digest_required",
            vec!["sourceGate", "sourceGateDigest", "schemaVersion"],
        ),
        receipt_guard(
            "deep_td8_receipt_terminal_decision_hash_required",
            vec![
                "terminalDecisionSurfaceId",
                "terminalDecisionHash",
                "zeroPromotionHash",
            ],
        ),
        receipt_guard(
            "deep_td8_receipt_authority_denial_hash_required",
            vec![
                "authorityGuardId",
                "authorityGranted",
                "authorityDenialHash",
            ],
        ),
        receipt_guard(
            "deep_td8_receipt_release_delivery_denial_hash_required",
            vec![
                "releaseDeliveryGuardId",
                "releasePublished",
                "externalDeliveryDenied",
            ],
        ),
        receipt_guard(
            "deep_td8_receipt_zero_side_effect_hash_required",
            vec![
                "terminalDecisionRecorded",
                "livePersistenceEnabled",
                "externalSendPerformed",
            ],
        ),
    ]
}

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_local_views()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptLocalView> {
    vec![
        local_view(
            "operator_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_view",
            "operator",
            vec![
                "receiptId",
                "receiptHash",
                "terminalDecisionHash",
                "nextGate",
            ],
        ),
        local_view(
            "release_owner_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_view",
            "release_owner",
            vec![
                "receiptId",
                "releasePublished",
                "publicClaimRecorded",
                "externalDeliveryDenied",
            ],
        ),
        local_view(
            "auditor_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_digest_view",
            "auditor",
            vec![
                "sourceGateDigest",
                "receiptHash",
                "denialHash",
                "zeroSideEffectHash",
            ],
        ),
        local_view(
            "runtime_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_zero_effect_view",
            "system",
            vec![
                "receiptRecorded",
                "graphStatePersisted",
                "walWritten",
                "externalSendPerformed",
            ],
        ),
    ]
}

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_invariants()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptInvariant> {
    vec![
        invariant(
            "deep_td8_receipt_retention_readback_ack_terminal_decision_receipts_are_hash_only",
            "terminal decision receipts expose hashes and local visibility only",
        ),
        invariant(
            "deep_td8_receipt_retention_readback_ack_terminal_decision_receipts_do_not_record",
            "terminal decision receipts cannot record terminal decisions or receipts",
        ),
        invariant(
            "deep_td8_receipt_retention_readback_ack_terminal_decision_receipts_block_acceptance",
            "terminal decision receipts cannot become acceptance or approval",
        ),
        invariant(
            "deep_td8_receipt_retention_readback_ack_terminal_decision_receipts_block_authority",
            "terminal decision receipts cannot grant authority",
        ),
        invariant(
            "deep_td8_receipt_retention_readback_ack_terminal_decision_receipts_block_release_delivery",
            "terminal decision receipts cannot publish, claim, roll out, or send externally",
        ),
        invariant(
            "deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_preview_has_no_side_effects",
            "this gate cannot persist state, write WAL/checkpoints, or invoke models",
        ),
    ]
}

fn receipt(
    id: &'static str,
    source_surface_ids: Vec<&'static str>,
    receipt_visibility: &'static str,
) -> DeepTd8RetentionAckTerminalDecisionReceipt {
    DeepTd8RetentionAckTerminalDecisionReceipt {
        id,
        source_surface_ids,
        receipt_visibility,
        receipt_recording_allowed: false,
        acceptance_allowed: false,
        authority_grant_allowed: false,
        public_claim_enabled: false,
        external_delivery_enabled: false,
    }
}

fn digest_check(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> DeepTd8RetentionAckTerminalDecisionReceiptDigestCheck {
    DeepTd8RetentionAckTerminalDecisionReceiptDigestCheck {
        id,
        required_fields,
        digest_must_match: true,
    }
}

fn mismatch_denial(
    id: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
    reason: &'static str,
) -> DeepTd8RetentionAckTerminalDecisionReceiptMismatchDenial {
    DeepTd8RetentionAckTerminalDecisionReceiptMismatchDenial {
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
) -> DeepTd8RetentionAckTerminalDecisionReceiptGuard {
    DeepTd8RetentionAckTerminalDecisionReceiptGuard {
        id,
        required_fields,
        recording_allowed: false,
        persistence_allowed: false,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> DeepTd8RetentionAckTerminalDecisionReceiptLocalView {
    DeepTd8RetentionAckTerminalDecisionReceiptLocalView {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> DeepTd8RetentionAckTerminalDecisionReceiptInvariant {
    DeepTd8RetentionAckTerminalDecisionReceiptInvariant {
        id,
        required: true,
        reason,
    }
}

impl DeepTd8RetentionAckTerminalDecisionReceiptSideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
            terminal_decision_receipt_persisted: false,
            acknowledgement_recorded: false,
            operator_acceptance_recorded: false,
            approval_recorded: false,
            authority_granted: false,
            live_persistence_enabled: false,
            wal_written: false,
            checkpoint_written: false,
            rollout_started: false,
            release_published: false,
            public_claim_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }

    #[cfg(test)]
    fn all_false(self) -> bool {
        !self.filesystem_written
            && !self.graph_state_persisted
            && !self.terminal_decision_recorded
            && !self.terminal_decision_receipt_recorded
            && !self.terminal_decision_receipt_persisted
            && !self.acknowledgement_recorded
            && !self.operator_acceptance_recorded
            && !self.approval_recorded
            && !self.authority_granted
            && !self.live_persistence_enabled
            && !self.wal_written
            && !self.checkpoint_written
            && !self.rollout_started
            && !self.release_published
            && !self.public_claim_recorded
            && !self.external_send_performed
            && !self.model_invoked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_requires_terminal_decision_gate()
     {
        let report = hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(&crate::deep_td8_receipt_retention_readback_ack_terminal_decision_gate())
        );
    }

    #[test]
    fn work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipts_are_hash_only()
    {
        let report = hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_preview_report();

        assert_eq!(report.receipt_count, 6);
        assert!(report.receipts.iter().all(|receipt| {
            receipt.source_surface_ids.len() == 6
                && !receipt.receipt_recording_allowed
                && !receipt.acceptance_allowed
                && !receipt.authority_grant_allowed
                && !receipt.public_claim_enabled
                && !receipt.external_delivery_enabled
        }));
    }

    #[test]
    fn work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipts_check_digests()
    {
        let report = hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_preview_report();

        assert_eq!(report.digest_check_count, 6);
        assert!(
            report
                .digest_checks
                .iter()
                .all(|check| check.digest_must_match && check.required_fields.len() >= 3)
        );
    }

    #[test]
    fn work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipts_deny_mismatches()
     {
        let report = hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_preview_report();

        assert_eq!(report.mismatch_denial_count, 7);
        assert!(report.mismatch_denials.iter().all(|denial| {
            denial.applies_to_receipt_ids.len() == 6
                && denial.blocks_receipt_recording
                && denial.blocks_acceptance
                && denial.blocks_authority
                && denial.blocks_release_publication
                && denial.blocks_public_claim
                && denial.blocks_external_delivery
        }));
    }

    #[test]
    fn work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipts_guard_recording_and_views()
     {
        let report = hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_preview_report();

        assert_eq!(report.receipt_guard_count, 5);
        assert!(
            report
                .receipt_guards
                .iter()
                .all(|guard| !guard.recording_allowed && !guard.persistence_allowed)
        );
        assert_eq!(report.local_view_count, 4);
        assert!(
            report
                .local_views
                .iter()
                .all(|view| !view.external_delivery_enabled)
        );
    }

    #[test]
    fn work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_has_no_side_effects()
     {
        let report = hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_preview_report();

        assert_eq!(report.invariant_count, 6);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert_eq!(
            report.recommended_next_gate,
            deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_gate(
            )
        );
        assert!(report.ready_for_terminal_decision_receipt_acknowledgement_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert!(report.side_effects.all_false());
    }
}
