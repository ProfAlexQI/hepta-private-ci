use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8TerminalDecisionReceiptAckPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: String,
    pub schema_version: String,
    pub preview_mode: &'static str,
    pub acknowledgement_contract_count: usize,
    pub non_acceptance_reason_count: usize,
    pub recording_denial_count: usize,
    pub expiry_replay_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<String>,
    pub acknowledgement_contracts: Vec<DeepTd8ReceiptAckContract>,
    pub non_acceptance_reasons: Vec<DeepTd8ReceiptAckNonAcceptanceReason>,
    pub recording_denials: Vec<DeepTd8ReceiptAckRecordingDenial>,
    pub expiry_replay_guards: Vec<DeepTd8ReceiptAckExpiryReplayGuard>,
    pub local_views: Vec<DeepTd8ReceiptAckLocalView>,
    pub invariants: Vec<DeepTd8ReceiptAckInvariant>,
    pub recommended_next_gate: String,
    pub ready_for_terminal_decision_receipt_acknowledgement_replay_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8ReceiptAckSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8ReceiptAckContract {
    pub id: &'static str,
    pub source_receipt_ids: Vec<&'static str>,
    pub acknowledgement_visibility: &'static str,
    pub acknowledgement_recording_allowed: bool,
    pub receipt_recording_allowed: bool,
    pub acceptance_allowed: bool,
    pub authority_grant_allowed: bool,
    pub public_claim_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8ReceiptAckNonAcceptanceReason {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8ReceiptAckRecordingDenial {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub target_record: &'static str,
    pub reason: &'static str,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_receipt_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8ReceiptAckExpiryReplayGuard {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub required_fields: Vec<&'static str>,
    pub blocks_acknowledgement_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8ReceiptAckLocalView {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8ReceiptAckInvariant {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8ReceiptAckSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub receipt_recorded: bool,
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

pub fn deep_td8_terminal_decision_receipt_ack_replay_gate() -> String {
    format!(
        "{}_receipt_acknowledgement_replay_idempotency_preview_gate",
        crate::deep_td8_base()
    )
}

pub fn hepta_work_graph_deep_td8_terminal_decision_receipt_ack_preview_report()
-> DeepTd8TerminalDecisionReceiptAckPreviewReport {
    let acknowledgement_contracts = deep_td8_receipt_ack_contracts();
    let non_acceptance_reasons = deep_td8_receipt_ack_non_acceptance_reasons();
    let recording_denials = deep_td8_receipt_ack_recording_denials();
    let expiry_replay_guards = deep_td8_receipt_ack_expiry_replay_guards();
    let local_views = deep_td8_receipt_ack_local_views();
    let invariants = deep_td8_receipt_ack_invariants();
    let gate = crate::deep_td8_terminal_decision_receipt_acknowledgement_gate();

    DeepTd8TerminalDecisionReceiptAckPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_terminal_decision_receipt_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_replay_guard_count: expiry_replay_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates: deep_td8_terminal_decision_receipt_ack_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_replay_guards,
        local_views,
        invariants,
        recommended_next_gate: deep_td8_terminal_decision_receipt_ack_replay_gate(),
        ready_for_terminal_decision_receipt_acknowledgement_replay_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8ReceiptAckSideEffects::none(),
    }
}

pub fn deep_td8_terminal_decision_receipt_ack_required_prior_gates() -> Vec<String> {
    let mut gates = crate::deep_td8_terminal_decision_receipt_required_prior_gates();
    gates.push(crate::deep_td8_terminal_decision_receipt_gate());
    gates
}

pub fn deep_td8_terminal_decision_receipt_ack_ids() -> Vec<&'static str> {
    vec![
        "operator_deep_td8_terminal_decision_receipt_acknowledgement",
        "release_owner_deep_td8_terminal_decision_receipt_acknowledgement",
        "authority_denial_deep_td8_terminal_decision_receipt_acknowledgement",
        "rollout_denial_deep_td8_terminal_decision_receipt_acknowledgement",
        "release_publication_denial_deep_td8_terminal_decision_receipt_acknowledgement",
        "external_delivery_denial_deep_td8_terminal_decision_receipt_acknowledgement",
    ]
}

pub fn deep_td8_receipt_ack_contracts() -> Vec<DeepTd8ReceiptAckContract> {
    let source_receipt_ids = crate::deep_td8_terminal_decision_receipt_ids();
    vec![
        ack_contract(
            "operator_deep_td8_terminal_decision_receipt_acknowledgement",
            source_receipt_ids.clone(),
            "local_operator_deep_td8_receipt_acknowledgement_visibility",
        ),
        ack_contract(
            "release_owner_deep_td8_terminal_decision_receipt_acknowledgement",
            source_receipt_ids.clone(),
            "local_release_owner_deep_td8_receipt_acknowledgement_visibility",
        ),
        ack_contract(
            "authority_denial_deep_td8_terminal_decision_receipt_acknowledgement",
            source_receipt_ids.clone(),
            "local_authority_denial_deep_td8_receipt_acknowledgement_visibility",
        ),
        ack_contract(
            "rollout_denial_deep_td8_terminal_decision_receipt_acknowledgement",
            source_receipt_ids.clone(),
            "local_rollout_denial_deep_td8_receipt_acknowledgement_visibility",
        ),
        ack_contract(
            "release_publication_denial_deep_td8_terminal_decision_receipt_acknowledgement",
            source_receipt_ids.clone(),
            "local_release_publication_denial_deep_td8_receipt_acknowledgement_visibility",
        ),
        ack_contract(
            "external_delivery_denial_deep_td8_terminal_decision_receipt_acknowledgement",
            source_receipt_ids,
            "external_delivery_acknowledgement_echo_denied",
        ),
    ]
}

pub fn deep_td8_receipt_ack_non_acceptance_reasons() -> Vec<DeepTd8ReceiptAckNonAcceptanceReason> {
    let acknowledgement_ids = deep_td8_terminal_decision_receipt_ack_ids();
    vec![
        non_acceptance_reason(
            "deep_td8_receipt_acknowledgement_is_not_acceptance",
            acknowledgement_ids.clone(),
            "terminal decision receipt acknowledgement only confirms local preview visibility",
        ),
        non_acceptance_reason(
            "deep_td8_receipt_acknowledgement_cannot_record_receipt",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot record terminal decision receipt state",
        ),
        non_acceptance_reason(
            "deep_td8_receipt_acknowledgement_cannot_record_acknowledgement",
            acknowledgement_ids.clone(),
            "receipt acknowledgement recording remains denied",
        ),
        non_acceptance_reason(
            "deep_td8_receipt_acknowledgement_cannot_grant_authority",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot grant acceptance, approval, or delivery authority",
        ),
        non_acceptance_reason(
            "deep_td8_receipt_acknowledgement_cannot_publish_release",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot publish release state",
        ),
        non_acceptance_reason(
            "deep_td8_receipt_acknowledgement_cannot_claim_public_status",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot record public claims",
        ),
        non_acceptance_reason(
            "deep_td8_receipt_acknowledgement_cannot_send_externally",
            acknowledgement_ids,
            "receipt acknowledgement cannot send externally",
        ),
    ]
}

pub fn deep_td8_receipt_ack_recording_denials() -> Vec<DeepTd8ReceiptAckRecordingDenial> {
    let acknowledgement_ids = deep_td8_terminal_decision_receipt_ack_ids();
    vec![
        recording_denial(
            "deep_td8_ack_recording_denied_for_terminal_decision_receipt",
            acknowledgement_ids.clone(),
            "terminalDecisionReceiptAcknowledgement",
            "receipt acknowledgement is preview-only",
        ),
        recording_denial(
            "deep_td8_receipt_recording_denied_from_acknowledgement",
            acknowledgement_ids.clone(),
            "terminalDecisionReceipt",
            "acknowledgement visibility cannot record receipt state",
        ),
        recording_denial(
            "deep_td8_acceptance_recording_denied_from_acknowledgement",
            acknowledgement_ids.clone(),
            "operatorAcceptance",
            "acknowledgement is not acceptance",
        ),
        recording_denial(
            "deep_td8_approval_recording_denied_from_acknowledgement",
            acknowledgement_ids.clone(),
            "operatorApproval",
            "acknowledgement is not approval",
        ),
        recording_denial(
            "deep_td8_authority_recording_denied_from_acknowledgement",
            acknowledgement_ids.clone(),
            "authorityGrant",
            "acknowledgement cannot grant authority",
        ),
        recording_denial(
            "deep_td8_release_recording_denied_from_acknowledgement",
            acknowledgement_ids.clone(),
            "releasePublication",
            "acknowledgement cannot publish release state",
        ),
        recording_denial(
            "deep_td8_external_delivery_denied_from_acknowledgement",
            acknowledgement_ids,
            "externalDelivery",
            "acknowledgement cannot send externally",
        ),
    ]
}

pub fn deep_td8_receipt_ack_expiry_replay_guards() -> Vec<DeepTd8ReceiptAckExpiryReplayGuard> {
    let acknowledgement_ids = deep_td8_terminal_decision_receipt_ack_ids();
    vec![
        expiry_replay_guard(
            "deep_td8_ack_scope_epoch_required",
            acknowledgement_ids.clone(),
            vec!["scopeId", "scopeEpoch", "receiptHash"],
        ),
        expiry_replay_guard(
            "deep_td8_ack_expiry_window_required",
            acknowledgement_ids.clone(),
            vec!["expiresAt", "observedAt", "clockSkewBound"],
        ),
        expiry_replay_guard(
            "deep_td8_ack_supersession_guard_required",
            acknowledgement_ids.clone(),
            vec!["supersessionId", "supersededReceiptId", "replacementHash"],
        ),
        expiry_replay_guard(
            "deep_td8_ack_replay_idempotency_required",
            acknowledgement_ids.clone(),
            vec!["idempotencyKey", "priorGateDigest", "zeroEffectHash"],
        ),
        expiry_replay_guard(
            "deep_td8_ack_external_delivery_denial_required",
            acknowledgement_ids,
            vec![
                "externalDeliveryDenied",
                "deliveryPolicyHash",
                "localViewHash",
            ],
        ),
    ]
}

pub fn deep_td8_receipt_ack_local_views() -> Vec<DeepTd8ReceiptAckLocalView> {
    vec![
        local_view(
            "operator_deep_td8_receipt_acknowledgement_view",
            "operator",
            vec![
                "acknowledgementId",
                "receiptHash",
                "acceptanceAllowed",
                "nextGate",
            ],
        ),
        local_view(
            "release_owner_deep_td8_receipt_acknowledgement_denial_view",
            "release_owner",
            vec![
                "releasePublished",
                "publicClaimRecorded",
                "rolloutStarted",
                "externalDeliveryDenied",
            ],
        ),
        local_view(
            "auditor_deep_td8_receipt_acknowledgement_digest_view",
            "auditor",
            vec![
                "priorReceiptDigest",
                "acknowledgementHash",
                "recordingDenialHash",
                "zeroSideEffectHash",
            ],
        ),
        local_view(
            "runtime_deep_td8_receipt_acknowledgement_zero_effect_view",
            "system",
            vec![
                "acknowledgementRecorded",
                "acceptanceRecorded",
                "authorityGranted",
                "externalSendPerformed",
            ],
        ),
    ]
}

pub fn deep_td8_receipt_ack_invariants() -> Vec<DeepTd8ReceiptAckInvariant> {
    vec![
        invariant(
            "deep_td8_receipt_acknowledgement_is_preview_only",
            "receipt acknowledgement confirms local visibility only",
        ),
        invariant(
            "deep_td8_receipt_acknowledgement_blocks_recording",
            "receipt acknowledgement cannot record acknowledgement or receipt state",
        ),
        invariant(
            "deep_td8_receipt_acknowledgement_blocks_acceptance",
            "receipt acknowledgement cannot become acceptance or approval",
        ),
        invariant(
            "deep_td8_receipt_acknowledgement_blocks_authority",
            "receipt acknowledgement cannot grant authority",
        ),
        invariant(
            "deep_td8_receipt_acknowledgement_blocks_release_delivery",
            "receipt acknowledgement cannot publish, claim, roll out, or send externally",
        ),
        invariant(
            "deep_td8_receipt_acknowledgement_has_no_side_effects",
            "this gate cannot persist state, write WAL/checkpoints, or invoke models",
        ),
    ]
}

fn ack_contract(
    id: &'static str,
    source_receipt_ids: Vec<&'static str>,
    acknowledgement_visibility: &'static str,
) -> DeepTd8ReceiptAckContract {
    DeepTd8ReceiptAckContract {
        id,
        source_receipt_ids,
        acknowledgement_visibility,
        acknowledgement_recording_allowed: false,
        receipt_recording_allowed: false,
        acceptance_allowed: false,
        authority_grant_allowed: false,
        public_claim_enabled: false,
        external_delivery_enabled: false,
    }
}

fn non_acceptance_reason(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    reason: &'static str,
) -> DeepTd8ReceiptAckNonAcceptanceReason {
    DeepTd8ReceiptAckNonAcceptanceReason {
        id,
        applies_to_acknowledgement_ids,
        reason,
        blocks_acceptance: true,
        blocks_authority: true,
    }
}

fn recording_denial(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    target_record: &'static str,
    reason: &'static str,
) -> DeepTd8ReceiptAckRecordingDenial {
    DeepTd8ReceiptAckRecordingDenial {
        id,
        applies_to_acknowledgement_ids,
        target_record,
        reason,
        blocks_acknowledgement_recording: true,
        blocks_receipt_recording: true,
        blocks_acceptance: true,
        blocks_authority: true,
        blocks_release_publication: true,
        blocks_public_claim: true,
        blocks_external_delivery: true,
    }
}

fn expiry_replay_guard(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    required_fields: Vec<&'static str>,
) -> DeepTd8ReceiptAckExpiryReplayGuard {
    DeepTd8ReceiptAckExpiryReplayGuard {
        id,
        applies_to_acknowledgement_ids,
        required_fields,
        blocks_acknowledgement_recording: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> DeepTd8ReceiptAckLocalView {
    DeepTd8ReceiptAckLocalView {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(id: &'static str, reason: &'static str) -> DeepTd8ReceiptAckInvariant {
    DeepTd8ReceiptAckInvariant {
        id,
        required: true,
        reason,
    }
}

impl DeepTd8ReceiptAckSideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            receipt_recorded: false,
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
            && !self.receipt_recorded
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
    fn work_graph_deep_td8_receipt_ack_requires_receipt_gate() {
        let report = hepta_work_graph_deep_td8_terminal_decision_receipt_ack_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(&crate::deep_td8_terminal_decision_receipt_gate())
        );
    }

    #[test]
    fn work_graph_deep_td8_receipt_ack_declares_non_accepting_contracts() {
        let report = hepta_work_graph_deep_td8_terminal_decision_receipt_ack_preview_report();

        assert_eq!(report.acknowledgement_contract_count, 6);
        assert!(report.acknowledgement_contracts.iter().all(|contract| {
            contract.source_receipt_ids.len() == 6
                && !contract.acknowledgement_recording_allowed
                && !contract.receipt_recording_allowed
                && !contract.acceptance_allowed
                && !contract.authority_grant_allowed
                && !contract.public_claim_enabled
                && !contract.external_delivery_enabled
        }));
    }

    #[test]
    fn work_graph_deep_td8_receipt_ack_blocks_acceptance_and_authority() {
        let report = hepta_work_graph_deep_td8_terminal_decision_receipt_ack_preview_report();

        assert_eq!(report.non_acceptance_reason_count, 7);
        assert!(
            report
                .non_acceptance_reasons
                .iter()
                .all(|reason| reason.blocks_acceptance && reason.blocks_authority)
        );
    }

    #[test]
    fn work_graph_deep_td8_receipt_ack_denies_recording_and_delivery() {
        let report = hepta_work_graph_deep_td8_terminal_decision_receipt_ack_preview_report();

        assert_eq!(report.recording_denial_count, 7);
        assert!(report.recording_denials.iter().all(|denial| {
            denial.blocks_acknowledgement_recording
                && denial.blocks_receipt_recording
                && denial.blocks_acceptance
                && denial.blocks_authority
                && denial.blocks_release_publication
                && denial.blocks_public_claim
                && denial.blocks_external_delivery
        }));
    }

    #[test]
    fn work_graph_deep_td8_receipt_ack_guards_expiry_scope_and_replay() {
        let report = hepta_work_graph_deep_td8_terminal_decision_receipt_ack_preview_report();

        assert_eq!(report.expiry_replay_guard_count, 5);
        assert!(
            report
                .expiry_replay_guards
                .iter()
                .all(|guard| guard.blocks_acknowledgement_recording)
        );
    }

    #[test]
    fn work_graph_deep_td8_receipt_ack_has_no_side_effects() {
        let report = hepta_work_graph_deep_td8_terminal_decision_receipt_ack_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert_eq!(report.invariant_count, 6);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert_eq!(
            report.recommended_next_gate,
            deep_td8_terminal_decision_receipt_ack_replay_gate()
        );
        assert!(report.ready_for_terminal_decision_receipt_acknowledgement_replay_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert!(report.side_effects.all_false());
    }
}
