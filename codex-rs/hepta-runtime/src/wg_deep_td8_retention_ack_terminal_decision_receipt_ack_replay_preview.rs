use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptAckReplayPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: String,
    pub schema_version: String,
    pub preview_mode: &'static str,
    pub replay_scenario_count: usize,
    pub idempotency_guard_count: usize,
    pub replay_denial_count: usize,
    pub monotonicity_check_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<String>,
    pub replay_scenarios: Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckReplayScenario>,
    pub idempotency_guards: Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckIdempotencyGuard>,
    pub replay_denials: Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckReplayDenial>,
    pub monotonicity_checks: Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckMonotonicityCheck>,
    pub local_views: Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckReplayLocalView>,
    pub invariants: Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckReplayInvariant>,
    pub recommended_next_gate: String,
    pub ready_for_receipt_retention_expiry_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8RetentionAckTerminalDecisionReceiptAckReplaySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptAckReplayScenario {
    pub id: &'static str,
    pub source_acknowledgement_ids: Vec<&'static str>,
    pub replay_mode: &'static str,
    pub acknowledgement_recording_allowed: bool,
    pub receipt_recording_allowed: bool,
    pub mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptAckIdempotencyGuard {
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_replay_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptAckReplayDenial {
    pub id: &'static str,
    pub applies_to_replay_scenario_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_receipt_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_rollout: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptAckMonotonicityCheck {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_out_of_order_replay: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptAckReplayLocalView {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptAckReplayInvariant {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetentionAckTerminalDecisionReceiptAckReplaySideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub acknowledgement_replay_recorded: bool,
    pub acknowledgement_recorded: bool,
    pub receipt_recorded: bool,
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

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_retention_expiry_gate()
-> String {
    format!(
        "{}_receipt_retention_expiry_preview_gate",
        crate::deep_td8_retention_ack_terminal_decision_base()
    )
}

pub fn hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_preview_report()
-> DeepTd8RetentionAckTerminalDecisionReceiptAckReplayPreviewReport {
    let replay_scenarios = deep_td8_retention_ack_terminal_decision_receipt_ack_replay_scenarios();
    let idempotency_guards =
        deep_td8_retention_ack_terminal_decision_receipt_ack_idempotency_guards();
    let replay_denials = deep_td8_retention_ack_terminal_decision_receipt_ack_replay_denials();
    let monotonicity_checks =
        deep_td8_retention_ack_terminal_decision_receipt_ack_monotonicity_checks();
    let local_views = deep_td8_retention_ack_terminal_decision_receipt_ack_replay_local_views();
    let invariants = deep_td8_retention_ack_terminal_decision_receipt_ack_replay_invariants();
    let gate =
        crate::deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_gate();

    DeepTd8RetentionAckTerminalDecisionReceiptAckReplayPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_replay_idempotency_preview_no_replay_write",
        replay_scenario_count: replay_scenarios.len(),
        idempotency_guard_count: idempotency_guards.len(),
        replay_denial_count: replay_denials.len(),
        monotonicity_check_count: monotonicity_checks.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates: deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_required_prior_gates(),
        replay_scenarios,
        idempotency_guards,
        replay_denials,
        monotonicity_checks,
        local_views,
        invariants,
        recommended_next_gate:
            deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_retention_expiry_gate(
            ),
        ready_for_receipt_retention_expiry_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8RetentionAckTerminalDecisionReceiptAckReplaySideEffects::none(),
    }
}

pub fn deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_required_prior_gates()
-> Vec<String> {
    let mut gates = crate::deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_required_prior_gates();
    gates.push(crate::deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_gate());
    gates
}

pub fn deep_td8_retention_ack_terminal_decision_receipt_ack_replay_scenario_ids()
-> Vec<&'static str> {
    vec![
        "duplicate_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_replay",
        "duplicate_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_replay",
        "stale_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_digest_replay",
        "superseded_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_scope_replay",
        "cross_scope_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_replay",
        "out_of_order_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_replay",
    ]
}

pub fn deep_td8_retention_ack_terminal_decision_receipt_ack_replay_scenarios()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckReplayScenario> {
    let acknowledgement_ids =
        crate::deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_ids();
    vec![
        replay_scenario(
            "duplicate_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_replay",
            acknowledgement_ids.clone(),
            "duplicate_receipt",
        ),
        replay_scenario(
            "duplicate_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "duplicate_acknowledgement",
        ),
        replay_scenario(
            "stale_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_digest_replay",
            acknowledgement_ids.clone(),
            "stale_receipt_digest",
        ),
        replay_scenario(
            "superseded_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_scope_replay",
            acknowledgement_ids.clone(),
            "superseded_receipt_scope",
        ),
        replay_scenario(
            "cross_scope_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "cross_scope_acknowledgement",
        ),
        replay_scenario(
            "out_of_order_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_replay",
            acknowledgement_ids,
            "out_of_order_acknowledgement",
        ),
    ]
}

pub fn deep_td8_retention_ack_terminal_decision_receipt_ack_idempotency_guards()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckIdempotencyGuard> {
    vec![
        idempotency_guard(
            "deep_td8_receipt_idempotency_key_required",
            vec!["receiptId", "receiptHash", "terminalDecisionHash"],
        ),
        idempotency_guard(
            "deep_td8_retention_ack_terminal_decision_receipt_acknowledgement_idempotency_key_required",
            vec!["acknowledgementId", "acknowledgementHash", "localViewHash"],
        ),
        idempotency_guard(
            "deep_td8_prior_gate_digest_binding_required",
            vec!["priorGate", "priorGateDigest", "schemaVersion"],
        ),
        idempotency_guard(
            "deep_td8_scope_epoch_binding_required",
            vec!["scopeId", "scopeEpoch", "supersessionId"],
        ),
        idempotency_guard(
            "deep_td8_zero_effect_hash_required",
            vec!["zeroWriteHash", "zeroAuthorityHash", "zeroDeliveryHash"],
        ),
        idempotency_guard(
            "deep_td8_release_denial_binding_required",
            vec!["releasePublished", "publicClaimRecorded", "rolloutStarted"],
        ),
        idempotency_guard(
            "deep_td8_external_delivery_denial_binding_required",
            vec![
                "externalDeliveryDenied",
                "deliveryPolicyHash",
                "externalSendPerformed",
            ],
        ),
    ]
}

pub fn deep_td8_retention_ack_terminal_decision_receipt_ack_replay_denials()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckReplayDenial> {
    let scenario_ids = deep_td8_retention_ack_terminal_decision_receipt_ack_replay_scenario_ids();
    vec![
        replay_denial(
            "deep_td8_duplicate_receipt_replay_denied",
            scenario_ids.clone(),
            "duplicate receipt replay cannot record acknowledgement or receipt state",
        ),
        replay_denial(
            "deep_td8_duplicate_acknowledgement_replay_denied",
            scenario_ids.clone(),
            "duplicate acknowledgement replay cannot record acknowledgement state",
        ),
        replay_denial(
            "deep_td8_stale_receipt_digest_replay_denied",
            scenario_ids.clone(),
            "stale receipt digest replay cannot become acceptance",
        ),
        replay_denial(
            "deep_td8_superseded_receipt_scope_replay_denied",
            scenario_ids.clone(),
            "superseded receipt scope replay cannot grant authority",
        ),
        replay_denial(
            "deep_td8_cross_scope_acknowledgement_replay_denied",
            scenario_ids.clone(),
            "cross-scope acknowledgement replay cannot publish release state",
        ),
        replay_denial(
            "deep_td8_out_of_order_acknowledgement_replay_denied",
            scenario_ids.clone(),
            "out-of-order acknowledgement replay cannot route traffic",
        ),
        replay_denial(
            "deep_td8_external_delivery_replay_denied",
            scenario_ids,
            "acknowledgement replay cannot send externally",
        ),
    ]
}

pub fn deep_td8_retention_ack_terminal_decision_receipt_ack_monotonicity_checks()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckMonotonicityCheck> {
    vec![
        monotonicity_check(
            "deep_td8_check_receipt_sequence",
            vec!["receiptSequence", "receiptHash", "priorReceiptHash"],
        ),
        monotonicity_check(
            "deep_td8_check_acknowledgement_sequence",
            vec![
                "acknowledgementSequence",
                "acknowledgementHash",
                "priorAcknowledgementHash",
            ],
        ),
        monotonicity_check(
            "deep_td8_check_digest_epoch",
            vec!["scopeEpoch", "digestEpoch", "supersessionId"],
        ),
        monotonicity_check(
            "deep_td8_check_release_public_claim_epoch",
            vec!["releaseEpoch", "publicClaimEpoch", "rolloutEpoch"],
        ),
        monotonicity_check(
            "deep_td8_check_next_gate_order",
            vec!["currentGate", "recommendedNextGate", "requiredPriorGate"],
        ),
    ]
}

pub fn deep_td8_retention_ack_terminal_decision_receipt_ack_replay_local_views()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckReplayLocalView> {
    vec![
        local_view(
            "operator_deep_td8_ack_replay_view",
            "operator",
            vec![
                "replayScenarioId",
                "idempotencyKey",
                "mutationAllowed",
                "nextGate",
            ],
        ),
        local_view(
            "release_owner_deep_td8_ack_replay_denial_view",
            "release_owner",
            vec![
                "releasePublished",
                "publicClaimRecorded",
                "rolloutStarted",
                "externalDeliveryDenied",
            ],
        ),
        local_view(
            "auditor_deep_td8_ack_replay_digest_view",
            "auditor",
            vec![
                "priorGateDigest",
                "replayHash",
                "monotonicityHash",
                "zeroSideEffectHash",
            ],
        ),
        local_view(
            "runtime_deep_td8_ack_replay_zero_effect_view",
            "system",
            vec![
                "replayRecorded",
                "acknowledgementRecorded",
                "authorityGranted",
                "externalSendPerformed",
            ],
        ),
    ]
}

pub fn deep_td8_retention_ack_terminal_decision_receipt_ack_replay_invariants()
-> Vec<DeepTd8RetentionAckTerminalDecisionReceiptAckReplayInvariant> {
    vec![
        invariant(
            "deep_td8_ack_replay_is_idempotent",
            "acknowledgement replay must be idempotent and zero-effect",
        ),
        invariant(
            "deep_td8_ack_replay_blocks_recording",
            "acknowledgement replay cannot record acknowledgement or receipt state",
        ),
        invariant(
            "deep_td8_ack_replay_blocks_acceptance_authority",
            "acknowledgement replay cannot become acceptance, approval, or authority",
        ),
        invariant(
            "deep_td8_ack_replay_blocks_release_delivery",
            "acknowledgement replay cannot publish, claim, roll out, or send externally",
        ),
        invariant(
            "deep_td8_ack_replay_views_are_local_only",
            "operator, auditor, release-owner, and runtime replay views are local only",
        ),
        invariant(
            "deep_td8_ack_replay_preview_has_no_side_effects",
            "this gate cannot persist state, write WAL/checkpoints, or invoke models",
        ),
    ]
}

fn replay_scenario(
    id: &'static str,
    source_acknowledgement_ids: Vec<&'static str>,
    replay_mode: &'static str,
) -> DeepTd8RetentionAckTerminalDecisionReceiptAckReplayScenario {
    DeepTd8RetentionAckTerminalDecisionReceiptAckReplayScenario {
        id,
        source_acknowledgement_ids,
        replay_mode,
        acknowledgement_recording_allowed: false,
        receipt_recording_allowed: false,
        mutation_allowed: false,
    }
}

fn idempotency_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> DeepTd8RetentionAckTerminalDecisionReceiptAckIdempotencyGuard {
    DeepTd8RetentionAckTerminalDecisionReceiptAckIdempotencyGuard {
        id,
        required_fields,
        blocks_replay_mutation: true,
    }
}

fn replay_denial(
    id: &'static str,
    applies_to_replay_scenario_ids: Vec<&'static str>,
    reason: &'static str,
) -> DeepTd8RetentionAckTerminalDecisionReceiptAckReplayDenial {
    DeepTd8RetentionAckTerminalDecisionReceiptAckReplayDenial {
        id,
        applies_to_replay_scenario_ids,
        reason,
        blocks_acknowledgement_recording: true,
        blocks_receipt_recording: true,
        blocks_acceptance: true,
        blocks_authority: true,
        blocks_rollout: true,
        blocks_release_publication: true,
        blocks_public_claim: true,
        blocks_external_delivery: true,
    }
}

fn monotonicity_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> DeepTd8RetentionAckTerminalDecisionReceiptAckMonotonicityCheck {
    DeepTd8RetentionAckTerminalDecisionReceiptAckMonotonicityCheck {
        id,
        compared_fields,
        blocks_out_of_order_replay: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> DeepTd8RetentionAckTerminalDecisionReceiptAckReplayLocalView {
    DeepTd8RetentionAckTerminalDecisionReceiptAckReplayLocalView {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> DeepTd8RetentionAckTerminalDecisionReceiptAckReplayInvariant {
    DeepTd8RetentionAckTerminalDecisionReceiptAckReplayInvariant {
        id,
        required: true,
        reason,
    }
}

impl DeepTd8RetentionAckTerminalDecisionReceiptAckReplaySideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
            acknowledgement_replay_recorded: false,
            acknowledgement_recorded: false,
            receipt_recorded: false,
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
            && !self.acknowledgement_replay_recorded
            && !self.acknowledgement_recorded
            && !self.receipt_recorded
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
    fn work_graph_deep_td8_ack_replay_requires_acknowledgement_gate() {
        let report =
            hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(&crate::deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_gate())
        );
    }

    #[test]
    fn work_graph_deep_td8_ack_replay_declares_blocked_scenarios() {
        let report =
            hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_preview_report();

        assert_eq!(report.replay_scenario_count, 6);
        assert!(report.replay_scenarios.iter().all(|scenario| {
            scenario.source_acknowledgement_ids.len() == 6
                && !scenario.acknowledgement_recording_allowed
                && !scenario.receipt_recording_allowed
                && !scenario.mutation_allowed
        }));
    }

    #[test]
    fn work_graph_deep_td8_ack_replay_requires_idempotency_guards() {
        let report =
            hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_preview_report();

        assert_eq!(report.idempotency_guard_count, 7);
        assert!(
            report
                .idempotency_guards
                .iter()
                .all(|guard| guard.blocks_replay_mutation)
        );
    }

    #[test]
    fn work_graph_deep_td8_ack_replay_denies_every_mutating_outcome() {
        let report =
            hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_preview_report();

        assert_eq!(report.replay_denial_count, 7);
        assert!(report.replay_denials.iter().all(|denial| {
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
    fn work_graph_deep_td8_ack_replay_enforces_monotonicity_and_next_gate() {
        let report =
            hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_preview_report();

        assert_eq!(report.monotonicity_check_count, 5);
        assert!(
            report
                .monotonicity_checks
                .iter()
                .all(|check| check.blocks_out_of_order_replay)
        );
        assert_eq!(
            report.recommended_next_gate,
            deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_retention_expiry_gate(
            )
        );
    }

    #[test]
    fn work_graph_deep_td8_ack_replay_has_no_side_effects() {
        let report =
            hepta_work_graph_deep_td8_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert_eq!(report.invariant_count, 6);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(report.ready_for_receipt_retention_expiry_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert!(report.side_effects.all_false());
    }
}
